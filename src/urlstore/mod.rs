use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, io::Error};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

pub struct UrlStore {
    urls: HashMap<String, String>,
}

impl UrlStore {
    pub fn new() -> UrlStore {
        UrlStore {
            urls: HashMap::new(),
        }
    }

    pub fn shorten(&mut self, url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        let result = hasher.finalize();
        let hash = format!("{:x}", result);
        if !self.urls.contains_key(&hash) {
            self.urls.insert(hash.clone(), url.to_owned());
        }
        format!("localhost:8080/api/{}", hash)
    }

    pub fn redirect(&mut self, hash: &str) -> Result<&String, Error> {
        self.urls.get(hash).ok_or(Error::new(
            std::io::ErrorKind::NotFound,
            "short URL not found",
        ))
    }
}
