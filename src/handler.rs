use serde::{Serialize, Deserialize};
use serde_json::json;
use worker::*;
use crate::{kv, utils};
use crate::config::config;

pub async fn api_handler(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    #[derive(Deserialize, Debug)]
    struct Req {
        token: Option<String>,
        key: Option<String>,
        url: String,
    }
    #[derive(Serialize)]
    struct Resp {
        key: String,
    }

    if let Ok(param) = req.json::<Req>().await {
        console_log!("{:?}", param);
        if config().token.len() > 0 && param.token.unwrap_or("".to_string()) != config().token {
            return Response::error("Forbidden", 403)
        }

        let key = match param.key {
            Some(key) if key.len() > 0 => key,
            _ => if let Some(k) = utils::gen_new_key(&ctx).await {
                k
            } else {
                return Response::error("Generate Key Failed", 500)
            }
        };

        if let Err(_) = Url::parse(&param.url) {
            return Response::error("Illegal Url", 403)
        }

        kv::put(&ctx, &key, &param.url).await?;
        
        return Response::from_json(&json!(Resp{key}))
    }
    Response::error("Bad Request", 400)
}

pub async fn redirect_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(key) = ctx.param("key") {
        if let Ok(Some(url)) = kv::get(&ctx, key).await {
            if let Ok(url) = Url::parse(&url) {
                return Response::redirect(url)
            } else {
                return Response::error("Bad Data", 500)
            }
        }
    }

    Response::error("Not Found", 404)
}

pub fn index_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    return Response::from_html(include_str!("page/index.html"))
}