use reqwest::Client;
use serde_json::{Value, Map};
use anyhow::{anyhow, Result};

const USER_AGENT: &str = "Varyag/0.1.0";

pub async fn send_get_request(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn send_post_request(url: &str, body_items: Vec<String>) -> Result<String> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .build()?;

    let mut body_map = Map::new();
    for item in body_items {
        let (key, value) = parse_request_item(&item)?;
        body_map.insert(key, value);
    }

    let response = client.post(url)
        .json(&body_map)
        .send()
        .await?;

    let response_body = response.text().await?;
    Ok(response_body)
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