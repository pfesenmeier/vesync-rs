#![allow(non_snake_case)]

use attohttpc;

pub mod dto;
pub use dto::*;

const BASE_URL: &str = "https://smartapi.vesync.com";

fn build_path(relative: &str) -> String {
    let mut full_path = String::with_capacity(BASE_URL.len() + relative.len());
    full_path.push_str(BASE_URL);
    full_path.push_str(relative);
    full_path
}

#[derive(Debug)]
pub struct VeSync {
    account: AccountResponse,
    pub devices: Option<Vec<dto::Device>>,
}

impl VeSync {
    pub fn get_account(account: &str, password: &str) -> Result<Self, ()> {
        let request = AccountRequest::new(account, password);

        let response = attohttpc::post(&build_path("/vold/user/login"))
            .json(&request) // set the request body (json feature required)
            .map_err(|_e| ())?
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        let account: AccountResponse = response.json().map_err(|_e| ())?;

        Ok(VeSync {
            account,
            devices: None,
        })
    }

    pub fn get_devices(&mut self) -> Result<&Option<Vec<dto::Device>>, ()> {
        let response = attohttpc::get(&build_path("/vold/user/devices"))
            .header("tk", &self.account.tk)
            .header("accountid", &self.account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        let devices: Vec<dto::Device> = response.json().map_err(|_e| ())?;

        self.devices = Some(devices);

        Ok(&self.devices)
    }

    pub fn device_on(&self, device: &Device) -> Result<(), ()> {
        self.device_put(device, "on")
    }

    pub fn device_off(&self, device: &Device) -> Result<(), ()> {
        self.device_put(device, "off")
    }

    fn device_put(&self, device: &Device, state: &str) -> Result<(), ()> {
        // `wifi-switch-1.3` also happens to match my `deviceType`, so we may want to use that here
        let path = format!("/v1/wifi-switch-1.3/{}/status/{}", device.cid, state);

        let response = attohttpc::put(&build_path(&path))
            .header("tk", &self.account.tk)
            .header("accountid", &self.account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        Ok(())
    }
}
