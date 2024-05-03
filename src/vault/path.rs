
const DATA_PATH_PART: &str = "data";

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
    fn remove_data_part_test() {
        assert_eq!(remove_data_part_from_vault_path("kv/data/demo/app"), "kv/demo/app")
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