#![allow(non_snake_case)]

//use attohttpc;
//use serde_json::json;

//pub mod dto;
pub mod device;
pub use device::{ConnectionStatus, DeviceStatus, VeSyncDevice};

pub mod account;
pub use account::VeSyncAccount;

const BASE_URL: &str = "https://smartapi.vesync.com";

fn build_path(relative: &str) -> String {
    let mut full_path = String::with_capacity(BASE_URL.len() + relative.len());
    full_path.push_str(BASE_URL);
    full_path.push_str(relative);
    full_path
}
