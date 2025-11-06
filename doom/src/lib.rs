#![no_main]

use std::ffi::{CStr, CString, c_char};

use gooseboy::framebuffer::init_fb;
use gooseboy::{log, mem};

type DoomExitFn = extern "C" fn(i32);
type DoomPrintFn = extern "C" fn(*const c_char);
// type DoomMallocFn = extern "C" fn(i32) -> *mut std::ffi::c_void;
// type DoomFreeFn = extern "C" fn(*mut std::ffi::c_void);
// type DoomGetTimeFn = extern "C" fn(*mut i32, *mut i32);

unsafe extern "C" {
    fn doom_set_print(cb: DoomPrintFn);
    fn doom_set_exit(cb: DoomExitFn);
    // fn doom_set_malloc(cb: DoomMallocFn, cb2: DoomFreeFn);
    // fn doom_set_gettime(cb: DoomGetTimeFn);
    fn doom_init(argc: i32, argv: *mut *mut c_char, flags: i32);
    // fn doom_force_update();
    // fn doom_update();
    // fn doom_get_framebuffer(channels: i32) -> *const u8;
}

// extern "C" fn doom_malloc(size: i32) -> *mut std::ffi::c_void {
//     let ptr = mem::alloc_bytes(size as usize);
//     ptr as *mut std::ffi::c_void
// }

// extern "C" fn doom_free(_ptr: *mut std::ffi::c_void) {
//     // for now, keep this as a stub
// }

extern "C" fn doom_print_override(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }

    log!("[puredoom] about to start printing a string");

    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, 1024);
        let len = slice.iter().position(|&b| b == 0).unwrap_or(1024);
        str::from_utf8(&slice[..len]).unwrap_or("[invalid utf8]")
    };

    log!("[puredoom] {}", s);
}

extern "C" fn doom_exit_override(code: i32) {
    log!("[puredoom] doom requested exit with code: {}", code);
}

// extern "C" fn doom_gettime(sec: *mut i32, usec: *mut i32) {
//     unsafe {
//         *sec = 0_i32;
//         *usec = 0_i32;
//     }
// }

#[gooseboy::main]
fn main() {
    init_fb();

    unsafe {
        doom_set_exit(doom_exit_override);
        doom_set_print(doom_print_override);
        // trace!(
        //     doom_set_malloc(doom_malloc, doom_free),
        //     "set malloc override"
        // );
        // trace!(doom_set_gettime(doom_gettime), "set gettime override");

        let arg0 = CString::new("doom.wad").unwrap();
        let arg0_ptr = arg0.into_raw();
        let mut argv: [*mut c_char; 1] = [arg0_ptr];
        doom_init(1, argv.as_mut_ptr(), 0);
        // doom_force_update();

        let _ = CString::from_raw(arg0_ptr);
    }
}

#[gooseboy::update]
fn update(_nano_time: i64) {
    // unsafe {
    //     doom_update();
    // }
}
