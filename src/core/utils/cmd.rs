// write a function to parse a vector of strings of parameters in the form of
// val1=1 val2=2 val3 = 3
// spaces should be ignored if its not a string parameter
// also the param name should be cleaned from spaces and the value
// if the parameter is a string, it should be in quotes

use std::collections::HashMap;

pub fn parse_params(params: Option<Vec<String>>) -> HashMap<String, String> {
    let mut params_map: HashMap<String, String> = HashMap::new();

    if params.is_none() {
        return params_map;
    }
    for param in params.unwrap() {
        let param_parts: Vec<&str> = param.split('=').collect();
        let param_name = param_parts[0].trim().to_string();
        // join all other param_parts in case there is '=' inside
        let value_str = param_parts[1..].join("=");
        let param_value = value_str.trim().to_string();

        params_map.insert(param_name, param_value);
    }

    params_map
}

// write a test for parse_params with all kinds of inputs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_params() {
        let params = vec![
            "param1=1".to_string(),
            "param2=2".to_string(),
            "param3 = 3".to_string(),
            "param4 = =".to_string(),
        ];
        let params_map = parse_params(Some(params));

        assert_eq!(params_map.get("param1").unwrap(), "1");
        assert_eq!(params_map.get("param2").unwrap(), "2");
        assert_eq!(params_map.get("param3").unwrap(), "3");
        assert_eq!(params_map.get("param4").unwrap(), "=");
    }
}
