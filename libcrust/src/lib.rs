#[derive(Copy, Clone)]
#[repr(C)]
pub struct Baz {
    pub qux: f64,
}

pub struct Foo {
    pub bar: i32,
    pub baz: Baz,
}

#[no_mangle]
pub extern "C" fn make(bar: i32) -> *mut Foo {
    let baz = Baz { qux: 1.0 / (bar as f64) };
    let foo = Foo { bar, baz };
    Box::into_raw(Box::new(foo))
}

#[no_mangle]
pub extern "C" fn bump(f: &mut Foo) {
    f.bar += 1;
}

#[no_mangle]
pub extern "C" fn get(f: &Foo) -> i32 {
    f.bar
}

#[no_mangle]
pub extern "C" fn baz(f: &Foo) -> Baz {
    f.baz
}

#[no_mangle]
pub extern "C" fn fix(f: &mut Foo) {
    f.baz.qux = 1.0 / (f.bar as f64);
}

#[no_mangle]
pub extern fn del(f: *mut Foo) {
    unsafe {
        drop(Box::from_raw(f));
    }
}
