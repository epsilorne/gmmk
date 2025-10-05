use crate::model::{Colour, KeyCode};
use hidapi::{HidApi, HidDevice, HidError};
use std::cmp;

const VID: u16 = 0x320f;
const PID: u16 = 0x5064;

pub struct Keeb {
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
                    let keeb = Keeb {
                        device: api.open_path(dev.unwrap().path())?,
                    };
                    keeb.set_custom_mode()?;
                    Ok(keeb)
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

    fn set_custom_mode(&self) -> Result<(), HidError> {
        self.send(&[0x04, 0x1b, 0x00, 0x06, 0x01, 0x00, 0x00, 0x00, 0x14])
    }

    pub fn set_key_colour(&self, key: KeyCode, colour: Colour) -> Result<(), HidError> {
        // this is set to 1 for some keys
        let idk: u8 = KeyCode::sets_other_bit(&key) as u8;

        let keycode: u8 = key as u8;
        let checksum: u8 = 0x20_u8
            .wrapping_add(keycode)
            .wrapping_add(colour.r)
            .wrapping_add(colour.b)
            .wrapping_add(colour.g);
        let buf: &[u8] = &[
            0x04,
            checksum,
            Colour::sizeof(&colour),
            0x11,
            0x3,
            keycode,
            idk,
            0x0,
            colour.r,
            colour.g,
            colour.b,
        ];

        self.send(buf)
    }
}

fn pad(buf: &[u8]) -> [u8; 64] {
    let mut padded = [0x0; 64];
    padded[..cmp::min(buf.len(), 64)].copy_from_slice(buf);
    padded
}
