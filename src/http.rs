use reqwest::{
    Method as ReqwestMethod,
    blocking::{
        ClientBuilder as ReqwestClientBuilder,
        Client as ReqwestClient,
        Response as ReqwestResponse,
        RequestBuilder as ReqwestRequestBuilder
    }
};


#[allow(unused)]
pub type Method = ReqwestMethod;

#[allow(unused)]
pub type Response = ReqwestResponse;

pub type RequestBuilder = ReqwestRequestBuilder;



pub type FetchClient = ReqwestClient;
pub type FetchBuilder = ReqwestClientBuilder;



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FetchType {
    HTTP1,
    HTTP2,
    HTTP3,
    TCP,
    UDP
}

#[derive(Debug, Clone)]
pub struct Fetch {}

impl Fetch {
    
    pub fn new() -> Self {
        Self {}
    }

    pub fn builder(&self) -> FetchBuilder {
        FetchBuilder::new()
    }

    fn with_agent(&self) -> FetchBuilder {
        self.builder().user_agent(format!("ZeroUI-Lib/{}", std::env!("CARGO_PKG_VERSION")))
    }

    fn client(&self, http_type: FetchType) -> FetchClient {
        match http_type {
            FetchType::HTTP1 => self.with_agent().http1_only(),
            FetchType::HTTP2 => self.with_agent().http2_prior_knowledge(),
            FetchType::HTTP3 => self.with_agent().http3_prior_knowledge(),
            _ => self.with_agent().http1_only(),            
        }.build().unwrap().into()
    }

    pub fn req(&self, typ: FetchType, url: &str, method: Method) -> RequestBuilder {
        self.client(typ).request(method, url)
    }

}