#![no_std]

use internal_bluefox_lib::plugin;

pub struct Plugin {
    pub name: &'static str,
    pub startup_func: fn(),
    pub update_func: fn(),
    pub fixed_update_func: fn(),
}

extern "C" {
    static mut plugin_interface: Plugin;
}

#[no_mangle]
pub extern fn get_plugin() -> plugin::Plugin {
    unsafe {
        plugin::Plugin {
            name: plugin_interface.name.as_ptr() as *const i8,
            startup_func: plugin_interface.startup_func as *const fn(),
            update_func: plugin_interface.update_func as *const fn(),
            fixed_update_func: plugin_interface.fixed_update_func as *const fn()
        }
    }
}