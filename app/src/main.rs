use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(wasm_import_module = "3ds")]
extern { 
    fn aptMainLoop() -> bool;
    fn gspWaitForVBlank();
    fn gfxSwapBuffers();
    fn hidScanInput();
    fn hidKeysDown() -> u32;
    fn printf(message: *const c_char);
}

const KEY_START: u32 = 0x4;

fn main() {

    let message = CString::new("Hello, from rust!\n").expect("CString::new failed");
    unsafe {
        // Main loop
        while aptMainLoop()
        {  
            gspWaitForVBlank();
            gfxSwapBuffers();
            hidScanInput();
            printf(message.as_ptr());
            // Your code goes here
            let kDown: u32 = hidKeysDown();
            if (kDown & KEY_START) == 0x4 {
                break; // break in order to return to hbmenu
            }
        }
    }
    return;
}
