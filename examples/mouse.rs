extern crate uinput;

use std::thread::sleep;
use std::time::Duration;
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::{Left, Middle, Right};
use uinput::event::Event::{Controller, Relative};
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;

fn main() {
	let mut device = uinput::default().unwrap()
		.name("test").unwrap()
		.event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
		.event(Relative(Position(X))).unwrap()
		.event(Relative(Position(Y))).unwrap()
		.create().unwrap();

	for _ in 1..4 {
		sleep(Duration::from_secs(1));

		// device.click(&Mouse(Left));

		device.send(X, 50);
		device.synchronize();
		device.send(Y, 50);
		device.synchronize();
	}
}
