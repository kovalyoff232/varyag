use reqwest::{Client, Method, header::HeaderMap};
use serde_json::{Value, Map};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;

const USER_AGENT: &str = "Varyag/0.1.0";

pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<String>,
    pub body: Vec<String>,
    pub data_file: Option<PathBuf>,
    pub noproxy: bool,
}

pub struct HttpResponse {
    pub status: String,
    pub headers: HeaderMap,
    pub body: String,
}

pub async fn send_request(request: HttpRequest) -> Result<HttpResponse> {
    let mut client_builder = Client::builder()
        .user_agent(USER_AGENT);

    if request.noproxy {
        client_builder = client_builder.no_proxy();
    }

    let client = client_builder.build()?;

    let method = Method::from_bytes(request.method.to_uppercase().as_bytes())?;

    let mut request_builder = client.request(method, &request.url);

    // Add headers
    for header in request.headers {
        let parts: Vec<&str> = header.splitn(2, ':').collect();
        if parts.len() == 2 {
            request_builder = request_builder.header(parts[0].trim(), parts[1].trim());
        } else {
            return Err(anyhow!("Invalid header format: {}", header));
        }
    }

    // Prepare body
    let body_content: Option<String> = if let Some(file_path) = request.data_file {
        Some(fs::read_to_string(file_path)?)
    } else if !request.body.is_empty() {
        let mut body_map = Map::new();
        for item in request.body {
            let (key, value) = parse_request_item(&item)?;
            body_map.insert(key, value);
        }
        Some(serde_json::to_string(&body_map)?)
    } else {
        None
    };

    if let Some(body) = body_content {
        request_builder = request_builder.header("Content-Type", "application/json");
        request_builder = request_builder.body(body);
    }

    let response = request_builder.send().await?;
    
    let status = response.status().to_string();
    let headers = response.headers().clone();
    let body = response.text().await?;

    Ok(HttpResponse { status, headers, body })
}

fn parse_request_item(item: &str) -> Result<(String, Value)> {
    if let Some((key, value)) = item.split_once(":=") {
        let parsed_value: Value = serde_json::from_str(value)
            .map_err(|e| anyhow!("Invalid JSON value for key '{}': {}", key, e))?;
        Ok((key.to_string(), parsed_value))
    } else if let Some((key, value)) = item.split_once('=') {
        Ok((key.to_string(), Value::String(value.to_string())))
    } else {
        Err(anyhow!("Invalid request item format: '{}'. Use key=value or key:=json_value.", item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_request_item_string() {
        let (key, value) = parse_request_item("name=Varyag").unwrap();
        assert_eq!(key, "name");
        assert_eq!(value, Value::String("Varyag".to_string()));
    }

    #[test]
    fn test_parse_request_item_json_object() {
        let (key, value) = parse_request_item(r#"data:={"id":123,"active":true}"#).unwrap();
        assert_eq!(key, "data");
        assert_eq!(value, json!({"id": 123, "active": true}));
    }

    #[test]
    fn test_parse_request_item_json_array() {
        let (key, value) = parse_request_item(r#"items:=[1, "two", 3.0]"#).unwrap();
        assert_eq!(key, "items");
        assert_eq!(value, json!([1, "two", 3.0]));
    }

    #[test]
    fn test_parse_request_item_invalid_json() {
        let result = parse_request_item(r#"data:={"id":123,"active":true"#); // Missing closing brace
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid JSON value for key 'data'"));
    }

    #[test]
    fn test_parse_request_item_invalid_format() {
        let result = parse_request_item("name:Varyag"); // Missing '=' or ':='
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid request item format"));
    }
}
