extern crate web_sys;

use std::panic;

use console_error_panic_hook;

pub fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}

pub fn init() {
    log("Setting panic hook");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
