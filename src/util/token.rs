use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Item;

pub struct Token<'a> {
    pub header: Header,
    pub secret: &'a str,
    pub validation: Validation,
}
impl<'a> Token<'a> {
    pub fn default() -> Self {
        Token {
            header: Header::new(Algorithm::HS512),
            secret: "secret_key",
            validation: Validation::new(Algorithm::HS512),
        }
    }
    pub fn gen_token<T>(&self, item: &'a T) -> Result<String, jsonwebtoken::errors::Error>
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
