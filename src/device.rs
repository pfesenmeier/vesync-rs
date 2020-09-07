use crate::account::VeSyncAccount;
use serde::Deserialize;

const TK: &str = "tk";
const ACCOUNTID: &str = "accountid";

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Status {
    Unknown,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum ConnectionStatus {
    Unknown,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "offline")]
    Offline,
}

pub struct VeSyncDevice<'a> {
    pub deviceName: String,
    pub cid: String,
    pub deviceStatus: Status,
    pub connectionStatus: ConnectionStatus,

    pub account: &'a VeSyncAccount,
}

impl<'a> VeSyncDevice<'a> {
    pub fn from_id(account: &'a VeSyncAccount, cid: &str) -> Self {
        VeSyncDevice {
            deviceName: String::new(),
            cid: cid.to_string(),
            deviceStatus: Status::Unknown,
            connectionStatus: ConnectionStatus::Unknown,
            account,
        }
    }

    pub fn update(&mut self) -> Result<(), ()> {
        let details = self.details()?;
        self.deviceStatus = details.deviceStatus;

        Ok(())
    }

    pub fn get_devices(account: &'a VeSyncAccount) -> Result<Vec<VeSyncDevice>, ()> {
        let response = attohttpc::get(&crate::build_path("/vold/user/devices"))
            .header(TK, &account.tk)
            .header(ACCOUNTID, &account.accountID)
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
    pub fn device_on(&mut self) -> Result<(), ()> {
        match &self.deviceStatus {
            Status::On => Ok(()),
            Status::Off | Status::Unknown => {
                self.device_put("on")?;
                self.deviceStatus = Status::On;
                Ok(())
            }
        }
    }

    /// Turns the specified `VeSyncDevice` off. If the status is already off, nothing is done.
    pub fn device_off(&mut self) -> Result<(), ()> {
        match &self.deviceStatus {
            Status::Off => Ok(()),
            Status::On | Status::Unknown => {
                self.device_put("off")?;
                self.deviceStatus = Status::Off;
                Ok(())
            }
        }
    }

    /// Toggles the specified `VeSyncDevice`'s state
    pub fn device_toggle(&mut self) -> Result<(), ()> {
        match &self.deviceStatus {
            Status::On => self.device_off(),
            Status::Off => self.device_on(),
            Status::Unknown => {
                // first get the details for this device
                self.update()?;

                // then just recurse
                self.device_toggle()
            }
        }
    }

    fn device_put(&self, state: &str) -> Result<(), ()> {
        // `wifi-switch-1.3` also happens to match my `deviceType`, so we may want to use that here
        let path = format!("/v1/wifi-switch-1.3/{}/status/{}", self.cid, state);

        let _response = attohttpc::put(&crate::build_path(&path))
            .header(TK, &self.account.tk)
            .header(ACCOUNTID, &self.account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        Ok(())
    }

    pub fn details(&self) -> Result<Details, ()> {
        let path = format!("/v1/device/{}/detail", self.cid);
        self.query_get(&path)
    }

    pub fn energy_week(&self) -> Result<EnergyConsumption, ()> {
        let path = format!("/v1/device/{}/energy/week", self.cid);
        self.query_get(&path)
    }

    pub fn configurations(&self) -> Result<Configuration, ()> {
        let path = format!("/v1/device/{}/configurations", self.cid);
        self.query_get(&path)
    }

    fn query_get<T>(&self, path: &str) -> Result<T, ()>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = attohttpc::get(&crate::build_path(&path))
            .header(TK, &self.account.tk)
            .header(ACCOUNTID, &self.account.accountID)
            .send() // send the request
            .map_err(|_e| ())? // TODO: handle errors
            ;

        response.json().map_err(|_| ())
    }
}

#[derive(Deserialize, Debug)]
pub struct Details {
    pub deviceStatus: Status,
    pub deviceImg: String,
    pub activeTime: u64, // ?
    pub energy: u64,     // ?
    pub power: f64,      // ?
    pub voltage: f64,    // ?
}

/// Response from VeSync
#[derive(Deserialize, Debug)]
struct DeviceResponse {
    pub deviceName: String, // plaintext name of device
    pub deviceImg: String,  // URL
    pub cid: String,        // GUID
    pub deviceStatus: Status,
    pub connectionType: String, // 'wifi',
    pub connectionStatus: ConnectionStatus,
    pub deviceType: String,         // 'wifi-switch-1.3',
    pub model: String,              // What are the known values? 'wifi-switch',
    pub currentFirmVersion: String, // eg, '1.99', '2.123'; maybe we can convert to a number?
}

#[derive(Deserialize, Debug)]
pub struct EnergyConsumption {
    pub energyConsumptionOfToday: f32,
    pub costPerKWH: f32,
    pub maxEnergy: f32,
    pub totalEnergy: f32,
    pub currency: String,
    pub data: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub deviceName: String,
    pub deviceImg: String,
    pub allowNotify: Status,
    pub currentFirmVersion: f32,
    pub latestFirmVersion: f32,
    pub ownerShip: bool,
    pub energySavingStatus: Status,
    pub powerProtectionStatus: Status,
    pub maxCost: u32,
    pub costPerKWH: u32,
    pub threshHold: u32,
    pub maxPower: u32,
    pub saleschannel: String,
    pub isUpgrading: bool,
}
