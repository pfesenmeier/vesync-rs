use serde::{Deserialize, Serialize};
use std::io::Read; // for Reqwest

#[derive(Serialize, Debug)]
pub struct AccountRequest {
    pub account: String,
    pub devToken: String,
    pub password: String,
}

impl AccountRequest {
    pub fn new(account: &str, password: &str) -> Self {
        AccountRequest {
            account: account.to_string(),
            devToken: "".to_string(),
            password: hexdigest(password),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AccountResponse {
    pub tk: String,
    pub accountID: String,
    pub nickName: String,
    pub avatarIcon: String, // is a URL
    pub userType: i32,      // probably an enum?
    pub acceptLanguage: String,
    pub termsStatus: bool,
}

#[derive(Deserialize, Debug)]
pub enum DeviceStatus {
    on,
    off,
}

#[derive(Deserialize, Debug)]
pub enum ConnectionStatus {
    online,
    offline,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    pub deviceName: String, // plaintext name of device
    pub deviceImg: String,  // URL
    pub cid: String,        // GUID
    pub deviceStatus: DeviceStatus,
    pub connectionType: String, // 'wifi',
    pub connectionStatus: ConnectionStatus,
    pub deviceType: String,         // 'wifi-switch-1.3',
    pub model: String,              // What are the known values? 'wifi-switch',
    pub currentFirmVersion: String, // eg, '1.99'; maybe we can convert to a number?
}

fn hexdigest(s: &str) -> String {
    let bytes = s.as_bytes();
    format!("{:x}", md5::compute(bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash() {
        const RAW_TEXT: &str = "abcdefghijklmnopqrstuvwxyz";
        let hashed = super::hexdigest(RAW_TEXT);
        assert_eq!(hashed, "c3fcd3d76192e4007dfb496cca67e13b");
    }
}
