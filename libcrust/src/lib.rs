mod backend;

use std::ffi::CStr;
use std::os::raw::c_char;

use backend::Backend;

#[no_mangle]
pub extern "C" fn be_make(name: *const c_char) -> *mut Backend {
    let name = unsafe {
        CStr::from_ptr(name).to_str()
            .expect("cannot get name")
    };
    let be = Backend::new(name);
    Box::into_raw(Box::new(be))
}

#[no_mangle]
pub extern "C" fn be_start(be: &mut Backend) {
    be.start();
}

#[no_mangle]
pub extern "C" fn be_del(be: *mut Backend) {
    unsafe {
        drop(Box::from_raw(be));
    }
}

#[no_mangle]
pub extern "C" fn be_poke_later(be: &Backend, delay_ms: i32, msg: i32) {
    be.spawn_poker(delay_ms, msg);
}

#[no_mangle]
pub extern "C" fn be_poke_now(be: &Backend, msg: i32) {
    be.poke(msg);
}
