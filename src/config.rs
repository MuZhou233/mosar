use std::cell::RefCell;
use worker::*;

use worker::RouteContext;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_path: String,
    pub kv: String,
    pub key_length: usize,
    pub token: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            api_path: "/api".to_string(),
            kv: "BUCKET".to_string(),
            key_length: 6,
            token: "mosar".to_string(),
        }
    }
}

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

macro_rules! get_env {
    ($env: expr, $binding: expr, $value: expr) => {
        if let Ok(v) = $env.var($binding) {
            if let Ok(v) = v.to_string().parse() {
                $value = v;
            }
        }
    };
}

macro_rules! load_config {
    ($env: expr) => {
        CONFIG.with(|config| {
            get_env!{$env, "API_PATH", config.borrow_mut().api_path};
            get_env!{$env, "KV", config.borrow_mut().kv};
            get_env!{$env, "KEY_LENGTH", config.borrow_mut().key_length};
            get_env!{$env, "TOKEN", config.borrow_mut().token};
        })
    };
}

pub fn load_config_from_ctx(ctx: &RouteContext<()>) {
    load_config!(ctx);
}
pub fn load_config_from_env(env: &Env) {
    load_config!(env);
}

pub fn config() -> Config {
    let mut config = Config::default();
    CONFIG.with(|c| {
        config = c.borrow().clone()
    });
    return config
}
