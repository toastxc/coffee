use crate::env;
use reqwasm::http::{Method, Request, Response};
use reqwasm::Error;
use shared::{OrderInfo, OrderPayload};

fn uri(i: impl Into<String>) -> String {
    let ssl = match env::APP_SSL_ENABLED
        .parse::<bool>()
        .expect("invalid env variable APP_SSL_ENABLED")
    {
        true => "https://",
        false => "http://",
    };
    format!("{}{}{}", ssl, env::APP_BACK_ADDR, i.into())
}

pub struct Client();

impl Client {
    pub async fn send<T: serde::Serialize>(
        path: impl Into<String>,
        body: Option<T>,
        method: Method,
    ) -> Result<Response, Error> {
        let mut client = Request::new(&uri(path))
            .header("content-type", "application/json")
            .method(method);

        if let Some(body) = body {
            client = client.body(serde_json::to_string(&body).unwrap());
        };

        client.send().await
    }
}

// this section of client is for ordering
impl Client {
    pub async fn order(body: OrderPayload) -> Result<u8, Error> {
        Self::send("/order", Some(body), Method::POST)
            .await?
            .json()
            .await
    }
}

// this section is for operating
impl Client {
    pub async fn fetch() -> Result<Vec<OrderInfo>, Error> {
        Self::send("/fetch", None::<()>, Method::GET)
            .await?
            .json()
            .await
    }
    pub async fn complete(item: u8) -> Result<(), Error> {
        Self::send("/complete", Some(item), Method::DELETE)
            .await
            .map(|_| {})
    }
}
