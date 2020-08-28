use std::panic;

use console_error_panic_hook;

pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
