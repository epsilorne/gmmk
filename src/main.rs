use hidapi::{HidApi, HidDevice, HidError};
use std::{cmp, env};

const VID: u16 = 0x320f;
const PID: u16 = 0x5064;

struct Keeb {
    device: HidDevice,
}

impl Keeb {
    pub fn new() -> Result<Self, HidError> {
        match HidApi::new() {
            Ok(api) => {
                let dev = api.device_list().find(|info| {
                    info.vendor_id() == VID
                        && info.product_id() == PID
                        && info.interface_number() == 1
                });
                if dev.is_some() {
                    Ok(Keeb {
                        device: api.open_path(dev.unwrap().path())?,
                    })
                } else {
                    Err(HidError::HidApiError {
                        message: "could not connect to device".to_string(),
                    })
                }
            }
            Err(_) => Err(HidError::HidApiError {
                message: "could not init hidapi".to_string(),
            }),
        }
    }

    fn send(&self, buf: &[u8]) -> Result<(), HidError> {
        self.device.write(&pad(&[0x04, 0x01, 0x00, 0x01]))?;
        self.device.write(&pad(buf))?;
        self.device.write(&pad(&[0x04, 0x02, 0x00, 0x02]))?;
        Ok(())
    }

    pub fn led_on(&self) -> Result<(), HidError> {
        self.send(&[0x04, 0x0c, 0x00, 0x06, 0x01, 0x04, 0x00, 0x00, 0x01])
    }

    pub fn led_off(&self) -> Result<(), HidError> {
        self.send(&[0x04, 0x0b, 0x00, 0x06, 0x01, 0x04])
    }
}

fn pad(buf: &[u8]) -> [u8; 64] {
    let mut padded = [0x0; 64];
    padded[..cmp::min(buf.len(), 64)].copy_from_slice(buf);
    padded
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: gmmk_rs <on|off>");
        return;
    }

    let keyboard = Keeb::new().expect("");
    let choice = &args[1];
    match choice.as_str() {
        "on" => keyboard.led_on().expect("failed to turn on led"),
        "off" => keyboard.led_off().expect("failed to turn off led"),
        _ => {
            eprintln!("invalid option");
        }
    }
}
