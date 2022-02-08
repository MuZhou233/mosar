use worker::*;

mod utils;
mod handler;
mod config;
use config::config;
mod kv; 

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    config::load_config_from_env(&env);

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |req, ctx| {
            config::load_config_from_ctx(&ctx);
            handler::index_handler(req, ctx)
        })
        .get_async("/:key", |req, ctx| {
            config::load_config_from_ctx(&ctx);
            handler::redirect_handler(req, ctx)
        })
        .post_async(&config().api_path, |req, ctx| {
            config::load_config_from_ctx(&ctx);
            handler::api_handler(req, ctx)
        })
        .run(req, env)
        .await
}
