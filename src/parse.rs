use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError(String);

pub fn parse_config(config_content: String) -> Result<HashMap<String, String>, ParseError> {
    let key_value_delimiter = "=";

    let mut config_map = HashMap::new();

    for line in config_content.split('\n') {
        if line.is_empty() {
            continue;
        }

        let key_value: Vec<&str> = line.split(key_value_delimiter).collect();

        if key_value.len() > 2 {
            return Err(ParseError("Error reading config file".to_string()));
        }

        config_map.insert(key_value[0].to_string(), key_value[1].to_string());
    }

    Ok(config_map)
}
