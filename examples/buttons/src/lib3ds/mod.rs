
use std::ffi::CString;
use core::ffi::c_void;
use std::os::raw::c_char;

#[link(wasm_import_module = "3ds")]
extern { 
    fn aptMainLoop() -> bool;
    fn gspWaitForVBlank();
    fn gfxSwapBuffers();
    fn hidScanInput();
    fn hidKeysDown() -> u32;
    fn printf(message: *const c_char);
    fn consoleClear();
    fn consoleSelect(p: usize) -> usize;
    fn consoleInit(selector: u8) -> usize;
    fn gfxFlushBuffers();
}

pub fn print(message: &str) {
    let cs = CString::new(message).expect("failed");
    unsafe { printf(cs.as_ptr()); }
}

pub fn main_loop() -> bool {
    return unsafe { aptMainLoop() };
}

pub struct Gsp {}

impl Gsp {
    pub fn wait_for_vblank() {
        unsafe { gspWaitForVBlank(); };
    }
}

pub struct Gfx {}

impl Gfx {
    pub fn swap_buffers() {
        unsafe { gfxSwapBuffers(); };
    }

    pub fn flush_buffers() {
        unsafe { gfxFlushBuffers(); };
    }
}

pub struct Hid {}

impl Hid {
    pub fn scan_input() {
        unsafe { hidScanInput(); };
    }

    pub fn keys_down() -> u32 {
        let keys = unsafe { hidKeysDown() };
        return keys;
    }
}

#[repr(u8)]
pub enum GFX_SELECT {
    TOP = 0,
    BOTTOM = 1
}

pub struct Console {}

impl Console {
    pub fn clear() {
        unsafe { consoleClear(); };
    }

    pub fn init(gfx_selector: GFX_SELECT) -> usize {
        return unsafe { consoleInit(gfx_selector as u8) };
    }

    pub fn select(p: usize) {
        unsafe { consoleSelect(p); };
    }
}