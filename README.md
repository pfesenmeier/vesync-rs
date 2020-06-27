# vesync-rs
This crate lets you access and control your [VeSync](https://www.vesync.com/) smart outlets including, for example, [Etekcity smart plugs](https://smile.amazon.com/gp/product/B074GVPYPY/). You must have a VeSync account (which requires you install their iOS or Android app) in order to use this crate.

```toml
[depenencies]
vesync-rs = "0.1"
```

```rust
use vesync_rs::{DeviceStatus, VeSync};

const VESYNC_ACCOUNT: &str = "me@example.com";
const VESYNC_KEY: &str = "my-secret-password";

fn main() -> Result<(), ()> {
    let mut account = VeSync::get_account(VESYNC_ACCOUNT, VESYNC_KEY)?;
    account.get_devices()?;

    let devices = account.devices.as_ref().unwrap();

    let outside_light = devices
        .iter()
        .find(|device| device.cid == "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee")
        .unwrap();

    // Toggle the state of the device
    match outside_light.deviceStatus {
        DeviceStatus::On => account.device_off(outside_light),
        DeviceStatus::Off => account.device_on(outside_light),
    }?;

    Ok(())
}
```

## TODO
[ ] Switch to [`nanoserde`](https://docs.rs/nanoserde/0.1.2/nanoserde/)
[ ] Improve initial `vesync-rs` API
[ ] Strengthen VeSync API types (using Enums instead of Strings, whenever possible)

## Changelog
* Switched to [`attohttpc`](https://github.com/sbstp/attohttpc). This dropped the number of crates to build from 106 to 62 and the build time from about 1min30sec to 55sec.