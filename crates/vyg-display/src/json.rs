use serde_json::{Value, from_str};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// Formats and syntax-highlights a JSON string for terminal output.
///
/// If the string is not valid JSON, it returns the original string unmodified.
pub fn get_pretty_json(json_str: &str) -> String {
    if let Ok(parsed) = from_str::<Value>(json_str) {
        let pretty_json = serde_json::to_string_pretty(&parsed).unwrap();

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let syntax = ps.find_syntax_by_extension("json").unwrap();
        let theme = &ts.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);

        let mut highlighted_output = String::new();
        for line in LinesWithEndings::from(&pretty_json) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            highlighted_output.push_str(&escaped);
        }
        highlighted_output.push_str("\x1b[0m"); // Reset terminal colors
        highlighted_output
    } else {
        // Not a json, return as is
        json_str.to_string()
    }
}

/// A convenience function that gets the formatted JSON and prints it to stdout.
pub fn pretty_print_json(json_str: &str) {
    println!("{}", get_pretty_json(json_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pretty_json_valid() {
        let input = r#"{"a": 1, "b": "hello"}"#;
        let output = get_pretty_json(input);
        
        // Check that it's not the same as the input (it should be prettified)
        assert_ne!(input, output);
        // Check that it contains ANSI escape codes for coloring
        assert!(output.contains("\x1b["));
        // Check that the key and value are present, even if they have color codes around them.
        assert!(output.contains("a"));
        assert!(output.contains("1"));
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_get_pretty_json_invalid() {
        let input = "this is not json";
        let output = get_pretty_json(input);

        // Should return the original string unchanged
        assert_eq!(input, output);
    }

    #[test]
    fn test_get_pretty_json_empty_string() {
        let input = "";
        let output = get_pretty_json(input);
        assert_eq!(input, output);
    }
}