use std::collections::HashMap;

use anyhow::anyhow;

use crate::vault::VaultTool;

pub struct VaultToolMockImpl {
    pub create_secrets_error: bool
}

impl VaultToolMockImpl {
    pub fn new(create_secrets_error: bool) -> Self {
        Self {
            create_secrets_error
        }
    }
}

impl VaultTool for VaultToolMockImpl {
    fn create_secrets(&self, _vault_path: &str, _secrets: &HashMap<String, String>) -> anyhow::Result<()> {
        if self.create_secrets_error {
            Err(anyhow!("error"))
        } else {
            Ok(())
        }
    }
}