use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("Failed to set environment variable: {0}")]
    SetEnv(#[from] std::env::VarError),
}

const PROXY_URL_TEMPLATE: &str = "http://127.0.0.1:{port}";

pub fn enable_proxy(port: u16) -> Result<(), ProxyError> {
    let proxy_url = PROXY_URL_TEMPLATE.replace("{port}", &port.to_string());
    env::set_var("http_proxy", &proxy_url);
    env::set_var("https_proxy", &proxy_url);
    env::set_var("HTTP_PROXY", &proxy_url);
    env::set_var("HTTPS_PROXY", &proxy_url);
    Ok(())
}

pub fn disable_proxy() -> Result<(), ProxyError> {
    env::remove_var("http_proxy");
    env::remove_var("https_proxy");
    env::remove_var("HTTP_PROXY");
    env::remove_var("HTTPS_PROXY");
    Ok(())
}

pub fn get_current_proxy() -> (Option<String>, Option<String>) {
    let http = env::var("http_proxy").ok();
    let https = env::var("https_proxy").ok();
    (http, https)
}