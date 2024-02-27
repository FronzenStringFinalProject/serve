use std::sync::OnceLock;

use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use persistence::sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};

pub mod child;
pub mod parent;

static SECRET_KEY: std::sync::OnceLock<[u8; 256]> = OnceLock::new();

fn get_secret_key() -> &'static [u8] {
    SECRET_KEY.get_or_init(|| {
        // let mut ran = rand::thread_rng();
        // let mut buf = [0u8; 256];
        // ran.fill_bytes(&mut buf);
        // buf
        [0u8; 256]
    })
}

pub trait JwtConvert: Sized + Serialize
where
    for<'de> Self: Deserialize<'de>,
{
    fn algorithm() -> Algorithm {
        Algorithm::HS384
    }

    fn encode(&self) -> Result<String, errors::Error> {
        encode(
            &Header::new(Self::algorithm()),
            self,
            &EncodingKey::from_secret(get_secret_key()),
        )
    }
    fn decode(token: &str) -> Result<Self, errors::Error> {
        let res = decode(
            token,
            &DecodingKey::from_secret(get_secret_key()),
            &Validation::new(Self::algorithm()),
        )?;
        Ok(res.claims)
    }
}

pub trait FromModel {
    type Model: ModelTrait;

    fn from_model(model: &Self::Model) -> Self;
}
