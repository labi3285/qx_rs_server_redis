#![allow(unused)]

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::{collections::HashMap, str::FromStr};
use tracing::{self, instrument};

use qx_rs_server::err::{Error, Result};
use qx_rs_server::env::{self, DEFAULT};

use redis::{Client, Connection};
use redis::Commands;


static POOLS: Lazy<Mutex<HashMap<&'static str, Client>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[instrument]
pub async fn setup() -> Result<()> {
    _setup(DEFAULT).await
}

#[instrument]
pub async fn setup_redis(which_redis: &'static str) -> Result<()> {
    _setup(which_redis).await
}

#[instrument]
pub async fn get_connect() -> Result<Connection> {
    _get_connect(DEFAULT).await
}

#[instrument]
pub async fn get_redis_connect(which_redis: &'static str) -> Result<Connection> {
    _get_connect(which_redis).await
}

async fn _get_connect(which_redis: &'static str) -> Result<Connection> {
    let map = POOLS.lock().await;
    let res = map.get(which_redis);
    if let Some(client) = res {
        let conn  = client.get_connection().unwrap();
        Ok(conn)
    } else {
        let err = format!("redis.pool _get_connect failed");
        tracing::error!(err);
        return Err(Error::Redis(err));
    }
}

async fn _setup(which_redis: &'static str) -> Result<()> {
    let mut which = "REDIS".to_string();
    if which_redis != DEFAULT {
        which = format!("REDIS.{}", which_redis);
    }
    let url = env::str(&format!("{}.URL", which))?;
    let port: String = env::str(&format!("{}.PORT", which))?;
    let full_url = format!("redis://{}:{}", url, port);
    tracing::info!("connecting redis: {}", full_url);

    let res = Client::open(full_url);
    match res {
        Ok(client) => {
            let mut map = POOLS.lock().await;
            map.insert(which_redis, client);
            tracing::info!("redis connect success");
            Ok(())
        },
        Err(err) => {
            let err = format!("redis.pool _setup open failed{:?}", err);
            tracing::error!(err);
            return Err(Error::Redis(err));
        }
    }
}
