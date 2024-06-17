use log::warn;
use reqwasm::http::{Method, Request, Response};
use reqwasm::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use shared::{OrderInfo, OrderPayload};

fn uri(i: impl Into<String>) -> String {
    let ssl = match shared::env::APP_SSL_ENABLED
        .parse::<bool>()
        .expect("invalid env variable APP_SSL_ENABLED")
    {
        true => "https://",
        false => "http://",
    };
    format!("{}{}{}", ssl, shared::env::APP_BACK_URI, i.into())
}

pub struct Client(Request);


impl Client {
    pub async fn new<T: serde::Serialize>(
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
        Self::new("/order", Some(body), Method::POST).await?.json().await
    }
}

// this section is for operating
impl Client {
    pub async fn fetch() -> Result<Vec<OrderInfo>, Error> {
        Self::new("/fetch", None::<()>, Method::GET).await?.json().await
        // let a = Self::new("/fetch", None::<()>, Method::GET).await?;
        // warn!("body: {:?}", a);
        // todo!()

    }
    pub async fn complete(item: u8) -> Result<(), Error> {
        Self::new("/complete", Some(item), Method::DELETE).await.map(|_| {})
    }
}
