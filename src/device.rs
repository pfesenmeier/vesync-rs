use crate::account::VeSyncAccount;
use serde::Deserialize;
//use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum DeviceStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum ConnectionStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
}

pub struct VeSyncDevice<'a> {
    pub deviceName: String, // plaintext name of device
    pub cid: String,        // GUID
    pub deviceStatus: DeviceStatus,
    pub connectionStatus: ConnectionStatus,

    pub account: &'a VeSyncAccount,
}

impl<'a> VeSyncDevice<'a> {
    pub fn get_devices(account: &'a VeSyncAccount) -> Result<Vec<VeSyncDevice>, ()> {
        let response = attohttpc::get(&crate::build_path("/vold/user/devices"))
            .header("tk", &account.tk)
            .header("accountid", &account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        let devices: Vec<DeviceResponse> = response.json().map_err(|_e| ())?;
        Ok(devices
            .iter()
            .map(|d| Self {
                deviceName: d.deviceName.to_string(),
                cid: d.cid.to_string(),
                deviceStatus: d.deviceStatus,
                connectionStatus: d.connectionStatus,
                account,
            })
            .collect())
    }

    /// Turns the specified `VeSyncDevice` on. If the status is already on, nothing is done.
    pub fn device_on(&self) -> Result<(), ()> {
        match &self.deviceStatus {
            DeviceStatus::On => Ok(()),
            DeviceStatus::Off => self.device_put("on"),
        }
    }

    /// Turns the specified `VeSyncDevice` off. If the status is already off, nothing is done.
    pub fn device_off(&self) -> Result<(), ()> {
        match &self.deviceStatus {
            DeviceStatus::Off => Ok(()),
            DeviceStatus::On => self.device_put("off"),
        }
    }

    /// Toggles the specified `VeSyncDevice`'s state
    pub fn device_toggle(&self) -> Result<(), ()> {
        match &self.deviceStatus {
            DeviceStatus::On => self.device_off()?,
            DeviceStatus::Off => self.device_on()?,
        };

        Ok(())
    }

    fn device_put(&self, state: &str) -> Result<(), ()> {
        // `wifi-switch-1.3` also happens to match my `deviceType`, so we may want to use that here
        let path = format!("/v1/wifi-switch-1.3/{}/status/{}", self.cid, state);

        let _response = attohttpc::put(&crate::build_path(&path))
            .header("tk", &self.account.tk)
            .header("accountid", &self.account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        Ok(())
    }

    /*
    /// For dimming the brightness of a light - no clue what the right API is here
    pub fn device_dim(&self, device: &VeSyncDevice, brightness: u8) -> Result<(), ()> {
        let body = json!({
            "uuid": device.cid,
            "brightness": brightness,
        });

        let path = format!("/v1/wifi-switch-1.3/{}/updatebrightness", device.cid);
        let path = "/dimmer/v1/device/updatebrightness";
        let path = "/v1/device/updatebrightness";
        let path = format!("/v1/wifi-switch-1.3/{}/brightness", device.cid);

        let _response = attohttpc::put(&path)
            .header("tk", &self.tk)
            .header("accountid", &self.accountID)
            .json(&body).map_err(|_| ())? // set the request body (json feature required)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;
        //r, _ = helpers.call_api(
        //    '/dimmer/v1/device/updatebrightness',
        //    'put',
        //    headers=head,
        //    json=body)
        Ok(())
    }
    */
}

/// Response from VeSync
#[derive(Deserialize, Debug)]
struct DeviceResponse {
    pub deviceName: String, // plaintext name of device
    pub deviceImg: String,  // URL
    pub cid: String,        // GUID
    pub deviceStatus: DeviceStatus,
    pub connectionType: String, // 'wifi',
    pub connectionStatus: ConnectionStatus,
    pub deviceType: String,         // 'wifi-switch-1.3',
    pub model: String,              // What are the known values? 'wifi-switch',
    pub currentFirmVersion: String, // eg, '1.99', '2.123'; maybe we can convert to a number?
}
