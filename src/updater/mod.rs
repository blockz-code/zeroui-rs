use cargo_packager_updater::{Config, Update, check_update};
use reqwest::Url;


#[derive(Debug, Clone)]
pub struct Updater {
    current_version: String,

    endpoints: Vec<Url>,
    token: Option<String>,

    client: Option<Update>,
    data: Vec<u8>
}

impl Updater {
    pub fn new() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            endpoints: Vec::new(),
            token: None,
            client: None,
            data: Vec::new()
        }
    }

    pub fn set_endpoints(mut self, endpoints: Vec<Url>) -> Self {
        self.endpoints = endpoints;
        self
    }

    pub fn set_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn check(mut self) -> crate::Result<Self> {
        let config = Config {
            endpoints: self.endpoints.clone(),
            pubkey: self.token.clone().unwrap_or(String::new()),
            ..Default::default()
        };
        match check_update(self.current_version.parse().unwrap(), config)? {
            Some(u) => {
                self.client = Some(u);
            },
            None => {},
        }
        Ok(self)
    }

    pub fn is_update_available(&self) -> bool {
        self.client.is_some()
    }

    pub fn download(&mut self, cb: fn(usize, u64)) {
        let data = self.client.as_ref().unwrap().download_extended(|size, d| {
            if let Some(d) = d {
                cb(size, d);
            }
        }, || {
            cb(0, 0);
        }).unwrap();
        self.data = data;
    }

    pub fn install(&mut self) {
        self.client.as_ref().unwrap().install(self.data.clone()).unwrap();
    }
}