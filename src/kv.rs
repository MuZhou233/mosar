use worker::*;
use crate::config::config;

pub async fn get(ctx: &RouteContext<()>, key: &str) -> Result<Option<String>> {
    let kv = ctx.kv(&config().kv)?;
    Ok(kv.get(key).text().await?)
}

pub async fn put(ctx: &RouteContext<()>, key: &str, val: &str) -> Result<()>{
    let kv = ctx.kv(&config().kv)?;
    kv.put(key, val)?.execute().await?;
    Ok(())
}