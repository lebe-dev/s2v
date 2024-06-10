pub mod copy;
pub mod manifests;
pub mod append;
pub mod update;

pub struct SecretOptions<'a> {
    secret_mask: String,
    secret_ignore_mask: Option<&'a String>,
    ignore_base64_errors: bool,
    ignore_utf8_errors: bool
}

impl SecretOptions<'_> {
    pub fn new<'a>(secret_mask: &'a str, secret_ignore_mask: Option<&'a String>,
               ignore_base64_errors: bool, ignore_utf8_errors: bool) -> SecretOptions<'a> {
        SecretOptions
        {
            secret_mask: secret_mask.to_string(),
            secret_ignore_mask,
            ignore_base64_errors,
            ignore_utf8_errors,
        }
    }
}
