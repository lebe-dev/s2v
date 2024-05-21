use std::collections::HashMap;

use anyhow::anyhow;

use crate::vault::VaultTool;

pub struct VaultToolMockImpl {
    pub create_secrets_error: bool,
    pub expected_secrets: HashMap<String,String>
}

impl VaultToolMockImpl {
    pub fn new_with_error() -> Self {
        Self {
            create_secrets_error: true,
            expected_secrets: HashMap::new()
        }
    }

    pub fn new(expected_secrets: &HashMap<String, String>) -> Self {
        Self {
            create_secrets_error: false,
            expected_secrets: expected_secrets.clone()
        }
    }

    pub fn new_without_expectations() -> Self {
        Self {
            create_secrets_error: false,
            expected_secrets: HashMap::new()
        }
    }
}

impl VaultTool for VaultToolMockImpl {
    fn create_secrets(&self, _vault_path: &str, secrets: &HashMap<String, String>) -> anyhow::Result<()> {
        if self.create_secrets_error {
            Err(anyhow!("error"))

        } else {
            if self.expected_secrets.is_empty() {
                Ok(())
            } else {
                for (k, v) in secrets {
                    assert!(&self.expected_secrets.contains_key(k));

                    let expected_value = self.expected_secrets.get(k).unwrap();

                    assert_eq!(expected_value, v)
                }

                Ok(())
            }
        }
    }
}