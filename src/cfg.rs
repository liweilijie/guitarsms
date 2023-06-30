use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub magic_p: String,
    pub rs_p: String,
    pub me_p: String,
    pub access_key: String,
    pub secret_key: String,
}
