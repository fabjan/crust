mod backend;

use backend::Backend;

#[no_mangle]
pub extern "C" fn be_make() -> *mut Backend {
    let be = Backend::new();
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
