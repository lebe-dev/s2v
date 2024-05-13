use std::collections::HashMap;

const DATA_PATH_PART: &str = "data";

pub fn get_secrets_with_vault_paths(secrets: &HashMap<String, String>, vault_base_path: &str) -> HashMap<String, String> {
    for x in secrets {
        
    }
    
    unimplemented!()
}

#[cfg(test)]
mod get_secret_with_vault_paths_tests {
    use std::collections::HashMap;

    use crate::vault::path::get_secrets_with_vault_paths;

    #[test]
    fn values_should_be_properly_encoded_with_base64() {
        let input_secrets = HashMap::from([
            ("DB_USER".to_string(), "demo".to_string()),
            ("DB_PASS".to_string(), "g4958gj24g90j349fm".to_string()),
        ]);

        let expected_results = HashMap::from([
            ("DB_USER".to_string(), "dmF1bHQ6a3YvZGF0YS9kZW1vL2FwcCNEQl9VU0VS".to_string()),
            ("DB_PASS".to_string(), "dmF1bHQ6a3YvZGF0YS9kZW1vL2FwcCNEQl9QQVNT".to_string()),
        ]);

        let results = get_secrets_with_vault_paths(&input_secrets, "demo");

        for (key, expected_value) in expected_results {
            assert!(results.contains_key(&key));

            let value = results.get(&key).unwrap();
            assert_eq!(value, &expected_value);
        }
    }
}

/// Adds ../data/.. part to given vault path
///
/// Example: kv/demo/app -> kv/data/demo/app
pub fn add_data_part_to_vault_path(vault_path: &str) -> String {
    let mut result = vault_path.to_string();

    let mut parts = vault_path.split("/").collect::<Vec<&str>>();

    if parts.len() > 1 {
        match parts.get(1) {
            Some(second_part) => {
                if *second_part != DATA_PATH_PART {
                    parts.insert(1, DATA_PATH_PART);
                    result = parts.join("/")
                }
            }
            None => {}
        }
    }

    result
}

/// Removes ../data/.. part from given vault path
///
/// Example: kv/data/demo/app -> kv/demo/app
pub fn remove_data_part_from_vault_path(vault_path: &str) -> String {
    let mut result = vault_path.to_string();

    let mut parts = vault_path.split("/").collect::<Vec<&str>>();

    if parts.len() > 1 {
        match parts.get(1) {
            Some(second_part) => {
                if *second_part == DATA_PATH_PART {
                    parts.remove(1);
                    result = parts.join("/")
                }
            }
            None => {}
        }

    }

    result
}

#[cfg(test)]
mod tests {
    use crate::vault::path::{add_data_part_to_vault_path, remove_data_part_from_vault_path};

    #[test]
    fn add_data_part_test() {
        assert_eq!(add_data_part_to_vault_path("kv/demo/app"), "kv/data/demo/app")
    }

    #[test]
    fn multiple_add_data_part() {
        assert_eq!(add_data_part_to_vault_path("kv/data/demo/app"), "kv/data/demo/app")
    }

    #[test]
    fn remove_data_part_test() {
        assert_eq!(remove_data_part_from_vault_path("kv/data/demo/app"), "kv/demo/app")
    }

    #[test]
    fn missing_data_block_removal() {
        assert_eq!(remove_data_part_from_vault_path("kv/demo/app"), "kv/demo/app")
    }

    #[test]
    fn return_input_value_for_unsupported_format() {
        let values = vec!["38gj93jg434g", "", "    ", "vault", "kv123912312239gj49gj3434g34"];

        for value in values {
            assert_eq!(add_data_part_to_vault_path(value), value);
            assert_eq!(remove_data_part_from_vault_path(value), value);
        }
    }
}