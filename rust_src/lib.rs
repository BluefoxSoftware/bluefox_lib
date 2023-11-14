#![no_std]

use internal_bluefox_lib::plugin;

pub struct Plugin {
    name: &'static str
}
impl Plugin {
    pub fn new<T: AsRef<str>>(name: &'static str) -> Self {
        Self { name }
    }
}

pub static mut plugin_interface: Plugin = Plugin {name: "Defualt" };

#[no_mangle]
pub extern fn get_plugin() -> plugin::Plugin {
    unsafe {
        plugin::Plugin {
            name: plugin_interface.name.as_ptr() as *const i8
        }
    }
}