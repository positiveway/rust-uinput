extern crate uinput;
extern crate uinput_sys;

use std::thread::sleep;
use std::time::Duration;
use uinput_sys::*;
use uinput::Device;
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::{Left, Middle, Right};
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;
use uinput::event::relative::Wheel;

fn main() {
	let mut device = Device::init_mouse_keyboard();

	for _ in 1..4 {
		sleep(Duration::from_secs(1));

		// device.write(EV_REL, REL_WHEEL, 10);
		// sleep(Duration::from_millis(100));
		// device.write(EV_REL, KEY_B, 0);

		// device.click(&Mouse(Left));
		device.send(Wheel::Vertical, 5);

		// device.send(X, 50);
		// device.send(Y, 50);
		device.synchronize();
	}
}
