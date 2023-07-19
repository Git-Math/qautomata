use super::types;
use std::fs;
use std::io::Error;

pub fn get_state_from_file(state_file: &str) -> Result<types::State, Error> {
    let content = fs::read_to_string(state_file)?;
    let state: types::State = serde_json::from_str(&content)?;
    Ok(state)
}

pub fn get_rules_from_file(
    rules_file: &str,
) -> Result<types::YamlRules, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(rules_file)?;
    let yaml_rules: types::YamlRules = serde_yaml::from_str(&content)?;
    Ok(yaml_rules)
}
