# vesync-rs
This crate lets you access and control your [VeSync](https://www.vesync.com/) smart outlets including, for example, [Etekcity smart plugs](https://smile.amazon.com/gp/product/B074GVPYPY/). You must have a VeSync account (which requires you install their iOS or Android app) in order to use this crate.

```toml
[depenencies]
vesync-rs = "0.1"
```

```rust
use vesync_rs::{VeSyncAccount, VeSyncdevice, DeviceStatus};

const VESYNC_ACCOUNT: &str = "me@example.com";
const VESYNC_KEY: &str = "my-secret-password";

fn main() -> Result<(), ()> {
    let account = VeSyncAccount::login(VESYNC_ACCOUNT, VESYNC_KEY)?;
    let devices = account.devices()?;

    let outside_light = devices
        .iter()
        .find(|device| device.cid == "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
        .unwrap();

    // Toggle the state of the device
    outside_light.device_toggle()?;

    match outside_light.deviceStatus {
        DeviceStatus::On => println!("Outside light is on"),
        DeviceStatus::Off => println!("Outside light is off"),
    }?;

    Ok(())
}
```

## TODO
- [ ] Switch to [`nanoserde`](https://docs.rs/nanoserde/0.1.2/nanoserde/)
- [x] Improve initial `vesync-rs` API
- [ ] Strengthen VeSync API types (using Enums instead of Strings, whenever possible)
- [ ] Investigate brightness (dimming) API

## Changelog
* Switched to [`attohttpc`](https://github.com/sbstp/attohttpc). This dropped the number of crates to build from 106 to 62 and the build time from about 1min30sec to 55sec.
* Updated API to (hopefully) be more idiomatic.
* Added ability to create `VeSyncAccount` using the `accountID` and `tk` directly (ie, without logging in). This lets you add _those_ values to your source code instead of the raw credentials:

```rust
let account = VeSyncAccount { accountID: "1234".to_string(), tk: "ABCXYZ==".to_string() };
```

* Added ability to create `VeSyncDevice` from the account and `cid`:

```rust
let inside_light = VeSyncDevice {
    deviceName: "inside light".to_string(),
    cid : "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee".to_string(),
    account: &account,
    deviceStatus: DeviceStatus::Off,
    connectionStatus: vesync_rs::ConnectionStatus::Online,
};
```
