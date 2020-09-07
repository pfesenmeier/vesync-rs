use crate::device::VeSyncDevice;
use attohttpc;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct VeSyncAccount {
    pub tk: String,
    pub accountID: String,
}

impl VeSyncAccount {
    pub fn login(account: &str, password: &str) -> Result<Self, ()> {
        let request = AccountRequest {
            account: account.to_string(),
            devToken: "".to_string(),
            password: hexdigest(password),
        };

        let response = attohttpc::post(&crate::build_path("/vold/user/login"))
            .json(&request) // set the request body (json feature required)
            .map_err(|_e| ())?
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        let response: AccountResponse = response.json().map_err(|_e| ())?;

        Ok(VeSyncAccount {
            tk: response.tk.to_string(),
            accountID: response.accountID.to_string(),
        })
    }

    /// Private function to query for devices
    pub fn get_devices(&self) -> Result<Vec<VeSyncDevice>, ()> {
        VeSyncDevice::get_devices(&self)
    }
}

#[derive(Serialize, Debug)]
struct AccountRequest {
    /// The email address associated with the account
    pub account: String,
    pub devToken: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
struct AccountResponse {
    pub tk: String,
    pub accountID: String,
    pub nickName: String,
    pub avatarIcon: String, // is a URL
    pub userType: i32,      // probably an enum?
    pub acceptLanguage: String,
    pub termsStatus: bool,
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
