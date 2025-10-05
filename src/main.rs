use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::kb::Keeb;
use crate::model::{Colour, KeyCode};

mod kb;
mod model;

fn main() -> Result<(), Box<dyn Error>> {
    let keyboard = Keeb::new()?;

    loop {
        keyboard
            .set_key_colour(KeyCode::Space, Colour::new(255, 255, 255))
            .expect("failed to turn on leds");
        sleep(Duration::from_secs(1));
        keyboard
            .set_key_colour(KeyCode::Space, Colour::new(0, 0, 0))
            .expect("failed to turn off leds");
        sleep(Duration::from_secs(1));
    }
}
