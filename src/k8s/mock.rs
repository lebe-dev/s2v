use anyhow::anyhow;

use crate::k8s::KubectlTool;

pub struct KubectlToolMockImpl {
    pub get_secret_names_output: Vec<String>,
    pub get_secret_names_error: bool,
    pub get_secret_manifest_output: String,
    pub get_secret_manifest_error: bool,
}

impl KubectlToolMockImpl {
    pub fn new(
        get_secret_names_output: Vec<String>,
        get_secret_names_error: bool,
        get_secret_manifest_output: String,
        get_secret_manifest_error: bool,
    ) -> Self {
        Self {
            get_secret_names_output,
            get_secret_names_error,
            get_secret_manifest_output,
            get_secret_manifest_error,
        }
    }
}

impl KubectlTool for KubectlToolMockImpl {
    fn get_secret_names(&self, _namespace: &str, _mask: &str) -> anyhow::Result<Vec<String>> {
        if !self.get_secret_names_error {
            Ok(self.get_secret_names_output.clone())

        } else {
            Err(anyhow!("error"))
        }
    }

    fn get_secret_manifest(&self, _namespace: &str, _secret_name: &str) -> anyhow::Result<String> {
        if !self.get_secret_manifest_error {
            Ok(self.get_secret_manifest_output.clone())

        } else {
            Err(anyhow!("error"))
        }
    }
}