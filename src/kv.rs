#![allow(unused)]

use redis::{Commands, RedisError, SetExpiry, SetOptions};
use qx_rs_server::err::{Error, Result};

use crate::pool::{self, get_connect};

pub async fn set(key: &String, value: &String) -> Result<()> {
    let mut conn = get_connect().await?;
    let res = conn.set(key, value);
    match res {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            let err = format!("redis.kv set for key:{:?} failed{:?}", key, err);
            tracing::error!(err);
            return Err(Error::Redis(err));
        },
    }
}

pub async fn set_with_expire_secs(key: &String, value: &String, expire_secs: usize) -> Result<()> {
    let mut conn = get_connect().await?;
    let mut opt = SetOptions::default();
    let exp = SetExpiry::EX(expire_secs);
    opt = opt.with_expiration(exp);
    let res = conn.set_options(key, value, opt);
    match res {
        Ok(()) => {
            Ok(())
        },
        Err(err) => {
            let err = format!("redis.kv set_with_expire_secs for key:{:?} failed{:?}", key, err);
            tracing::error!(err);
            return Err(Error::Redis(err));
        },
    }
}

pub async fn get(key: &String) -> Result<Option<String>> {
    let mut conn = get_connect().await?;
    let res = conn.get(key);
    match res {
        Ok(val) => {
            Ok(val)
        },
        Err(err) => {
            let err = format!("redis.kv get for key:{:?} failed{:?}", key, err);
            tracing::error!(err);
            return Err(Error::Redis(err));
        },
    }
}