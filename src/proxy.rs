use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("Failed to set environment variable: {0}")]
    SetEnv(#[from] std::env::VarError),
}

const PROXY_URL_TEMPLATE: &str = "http://127.0.0.1:{port}";

/// 启用代理（设置会话级环境变量）
pub fn enable_proxy(port: u16) -> Result<(), ProxyError> {
    let proxy_url = PROXY_URL_TEMPLATE.replace("{port}", &port.to_string());
    
    // 设置环境变量（会话级，仅对当前进程及其子进程有效）
    // 同时设置小写和大写版本，确保兼容性
    env::set_var("http_proxy", &proxy_url);
    env::set_var("https_proxy", &proxy_url);
    env::set_var("HTTP_PROXY", &proxy_url);
    env::set_var("HTTPS_PROXY", &proxy_url);
    
    Ok(())
}

/// 禁用代理（移除会话级环境变量）
pub fn disable_proxy() -> Result<(), ProxyError> {
    // 移除环境变量
    env::remove_var("http_proxy");
    env::remove_var("https_proxy");
    env::remove_var("HTTP_PROXY");
    env::remove_var("HTTPS_PROXY");
    
    Ok(())
}

/// 获取当前代理状态（从环境变量读取）
pub fn get_current_proxy() -> (bool, Option<String>) {
    // 优先读取小写版本的环境变量
    let http_proxy = env::var("http_proxy")
        .or_else(|_| env::var("HTTP_PROXY"))
        .ok();
    
    let https_proxy = env::var("https_proxy")
        .or_else(|_| env::var("HTTPS_PROXY"))
        .ok();
    
    // 如果任一代理变量存在，则认为代理已启用
    let enabled = http_proxy.is_some() || https_proxy.is_some();
    
    // 返回 HTTP 代理地址（如果存在）
    (enabled, http_proxy)
}
