use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Item;

pub struct Token {
    pub header: Header,
    pub secret: String,
    pub validation: Validation,
}
impl  Token {
    pub fn default() -> Self {
        let sec=std::env::var("TOKEN_SECRET").unwrap();
        Token {
            header: Header::new(Algorithm::HS512),
            secret: sec,
            validation: Validation::new(Algorithm::HS512),
        }
    }
    pub fn gen_token<'a,T>(&self, item: &'a T) -> Result<String, jsonwebtoken::errors::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        encode(
            &self.header,
            item,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    pub fn decode_token<T>(
        &self,
        token: String,
    ) -> Result<TokenData<T>, jsonwebtoken::errors::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let t = decode::<T>(
            token.as_str(),
            &DecodingKey::from_secret(self.secret.as_ref()),
            &self.validation,
        );
        t
    }
}
