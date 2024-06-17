
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OrderPayload {
    pub coffee: u8,
    pub milk: u8,
    pub temp: u8,
}
pub mod env;



#[derive(Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    pub order_no: u8,
    pub coffee_info: OrderPayload,
    pub date: SystemTime
}