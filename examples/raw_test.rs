use std::thread::sleep;
use std::time::Duration;

extern crate uinput_sys;

use uinput_sys::*;

fn main() {
    let mut device = uinput::default().unwrap()
        .name("test").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();

    sleep(Duration::from_secs(1));

    for i in 1..10 {
        device.write(EV_KEY, KEY_B, 1);
        sleep(Duration::from_millis(100));
        device.write(EV_KEY, KEY_B, 0);
    }
    device.synchronize().unwrap();
}