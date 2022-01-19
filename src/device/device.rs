use std::{mem, ptr, slice};
use libc::c_int;
use libc::{timeval, gettimeofday};
use nix::unistd;
use ffi::*;
use {Result as Res, event};
use device::Builder;
use event::{Kind, Code, Keyboard, ButtonsVec};
use event::controller::Mouse::{Left, Middle, Right};
use event::keyboard::Key;
use event::relative::Position::{X, Y};
use event::relative::Wheel;


/// The virtual device.
pub struct Device {
    fd: c_int,
}

impl Device {
    /// Wrap a file descriptor in a `Device`.
    pub fn new(fd: c_int) -> Self {
        Device {
            fd: fd
        }
    }

    pub fn init_mouse_keyboard() -> Self {
        let mut _device = Builder::default().unwrap()
            .name("fakeinputs").unwrap()
            .event(Keyboard::All).unwrap()
            .event(Left).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
            .event(Right).unwrap()
            .event(Middle).unwrap()
            .event(X).unwrap()
            .event(Y).unwrap()
            .event(Wheel::Vertical).unwrap()
            .event(Wheel::Horizontal).unwrap()
            .create().unwrap();
        _device
    }

    #[doc(hidden)]
    pub fn write(&self, kind: c_int, code: c_int, value: c_int) -> Res<()> {
        unsafe {
            let mut event = input_event {
                time: timeval { tv_sec: 0, tv_usec: 0 },
                kind: kind as u16,
                code: code as u16,
                value: value as i32,
            };

            gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &event as *const _ as *const u8;
            let size = mem::size_of_val(&event);

            unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
        }

        Ok(())
    }

    /// Synchronize the device.
    pub fn synchronize(&self) {
        self.write(EV_SYN, SYN_REPORT, 0).unwrap();
    }

    /// Send an event.
    pub fn send<T: Into<event::Event>>(&self, event: T, value: i32) {
        let event = event.into();
        self.write(event.kind(), event.code(), value).unwrap();
    }

    pub fn write_button(&self, button: &Key, value: c_int) {
        // let (kind, code) = button.value();
        let kind = button.kind();
        let code = button.code();
        self.write(kind, code, value).unwrap();
    }

    /// Send a press event.
    pub fn press(&self, button: &Key) {
        self.write_button(button, 1);
        self.synchronize();
    }

    /// Send a release event.
    pub fn release(&self, button: &Key) {
        self.write_button(button, 0);
        self.synchronize();
    }

    /// Send a press and release event.
    pub fn click(&self, button: &Key) {
        self.press(button);
        self.release(button);
    }

    pub fn press_sequence(&self, sequence: &ButtonsVec) {
        for button in sequence {
            self.press(button);
        }
    }

    pub fn release_sequence(&self, sequence: &ButtonsVec) {
        for button in sequence.into_iter().rev() {
            self.release(button);
        }
    }

}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            ui_dev_destroy(self.fd);
        }
    }
}
