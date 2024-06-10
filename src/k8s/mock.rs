use std::fs;
use std::path::Path;

use anyhow::anyhow;

use crate::k8s::KubectlTool;

pub struct KubectlToolMockImpl {
    pub get_secret_names_output: Vec<String>,
    pub get_secret_names_error: bool,
    pub get_secret_manifest_a_error: bool,
    pub get_secret_manifest_b_error: bool,
}

impl KubectlToolMockImpl {
    pub fn new(
        get_secret_names_output: Vec<String>,

        get_secret_names_error: bool,
        get_secret_manifest_a_error: bool,
        get_secret_manifest_b_error: bool,
    ) -> Self {
        Self {
            get_secret_names_output,
            get_secret_names_error,
            get_secret_manifest_a_error,
            get_secret_manifest_b_error,
        }
    }
}

impl KubectlTool for KubectlToolMockImpl {
    fn get_secret_names(&self, _namespace: &str, _mask: &str, _ignore_mask: Option<&String>) -> anyhow::Result<Vec<String>> {
        if !self.get_secret_names_error {
            Ok(self.get_secret_names_output.clone())

        } else {
            Err(anyhow!("error"))
        }
    }

    fn get_secret_manifest(&self, _namespace: &str, secret_name: &str) -> anyhow::Result<String> {
        if secret_name == "a" {

            if !self.get_secret_manifest_a_error {
                let manifest_file = Path::new("test-data").join("a.yaml");
                let manifest = fs::read_to_string(manifest_file).unwrap();
                Ok(manifest)

            } else {
                Err(anyhow!("error"))
            }

        } else if secret_name == "b" {

            if !self.get_secret_manifest_b_error {
                let manifest_file = Path::new("test-data").join("b.yaml");
                let manifest = fs::read_to_string(manifest_file).unwrap();
                Ok(manifest)

            } else {
                Err(anyhow!("error"))
            }

        } else {
            Err(anyhow!("error"))
        }
    }
}