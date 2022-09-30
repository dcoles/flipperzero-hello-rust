//! Hello, Rust!
//! Prints "Hello, Rust! 🦀" to the console, waits for one second then exits.

#![no_main]
#![no_std]

use core::ffi::c_void;
use core::fmt::Write;
use core::ptr;
use core::time::Duration;

extern crate flipperzero;
use flipperzero::furi::io::Stdout;
use flipperzero::furi::thread::sleep;
use flipperzero_sys as sys;
use flipperzero_sys::c_string;
use flipperzero_sys::canvas::Canvas;
use flipperzero_sys::gui::{RECORD_GUI, Gui, GuiLayer};

/// View draw handler.
pub extern "C" fn draw_callback(canvas: *mut Canvas, _context: *mut c_void) {
    unsafe {
        sys::canvas::draw_str(canvas, 39, 31, c_string!("Hello, Rust!"));
    }
}

/// Application entry point.
#[no_mangle]
pub extern "C" fn hello_rust_app(_args: *mut u8) -> i32 {
    let mut stdout = Stdout;

    write!(&mut stdout, "Hello, Rust! \u{1F980}\r\n").unwrap();
    stdout.flush().unwrap();

    unsafe {
        let view_port = sys::view_port::alloc();
        sys::view_port::draw_callback_set(view_port, draw_callback, ptr::null_mut());

        let gui = sys::furi::record::open(RECORD_GUI) as *mut Gui;
        sys::gui::add_view_port(gui, view_port, GuiLayer::Fullscreen);

        sleep(Duration::from_secs(1));

        sys::view_port::enabled_set(view_port, false);
        sys::gui::remove_view_port(gui, view_port);
        sys::furi::record::close(RECORD_GUI);
        sys::view_port::free(view_port);
    }

    0
}
