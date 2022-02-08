use cfg_if::cfg_if;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use worker::*;
use crate::config::config;
use crate::kv;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub async fn gen_new_key(ctx: &RouteContext<()>) -> Option<String> {
    for _ in 1..30 {
        let try_key = random_string(config().key_length);
        if let Ok(None) = kv::get(ctx, &try_key).await {
            return Some(try_key)
        }
    }
    return None
}

fn random_string(len: usize) -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}