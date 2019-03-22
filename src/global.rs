use std::ffi::CStr;
use std::os::raw::c_void;

pub type DOMReference = i32;
pub type CString = i32;
pub type Element = i32;
pub type EventListener = i32;
pub type Event = i32;

pub const UNDEFINED: i32 = -1;

pub fn to_cstring(s: &str) -> CString {
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}

pub fn to_string(c: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(c as *const i8) };
    s.to_str().unwrap().to_owned()
}

extern "C" {
    fn global_debugger();
    fn global_get_window() -> Element;
    fn global_release(handle: i32);
    fn global_create_event_listener() -> EventListener;
    fn global_get_property(element: Element, property_name: CString) -> i32;
    fn global_sys_call(
        op: i32,
        sub_op: i32,
        param_a: i32,
        param_b: i32,
        param_c: i32,
        param_d: i32,
    ) -> i32;
}

pub fn sys_call(
    op: i32,
    sub_op: i32,
    param_a: i32,
    param_b: i32,
    param_c: i32,
    param_d: i32,
) -> i32 {
    unsafe { global_sys_call(op, sub_op, param_a, param_b, param_c, param_d) }
}

pub fn debugger() {
    unsafe {
        global_debugger();
    }
}

pub fn window() -> Element {
    unsafe { global_get_window() }
}

pub fn document() -> Element {
    crate::window::get_document(window())
}

pub fn release(handle: i32) {
    unsafe { global_release(handle) }
}

pub fn create_event_listener() -> EventListener {
    unsafe { global_create_event_listener() }
}

pub fn get_property(element: Element, property_name: &str) -> i32 {
    unsafe { global_get_property(element, to_cstring(property_name)) }
}

#[no_mangle]
pub fn malloc(size: i32) -> *mut c_void {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr as *mut c_void;
}
