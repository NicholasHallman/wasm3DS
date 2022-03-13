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
    fn consoleClear();
}

fn main() {

    
    let mut old_keys:u32 = 0;

    let return_char = CString::new("\x1b[3;0H").expect("failed");
    let hello = CString::new("Hey there! try pressing a button.\n").expect("failed");
    let exit_ins = CString::new("Press Start to exit.\n").expect("failed");
    let mut l;
    unsafe {
        printf(hello.as_ptr());
        printf(exit_ins.as_ptr());
        printf(return_char.as_ptr());
        l = aptMainLoop();
    }
    // Main loop
    while l
    {  
        let keys: u32;
        unsafe {
            gspWaitForVBlank();
            gfxSwapBuffers();
            hidScanInput();
            keys = hidKeysDown();
        }
        
        //let current_key = CString::new(format!("keys {} old_keys {}", keys, old_keys)).expect("failed");
        //printf(current_key.as_ptr());

        if keys != old_keys {
            old_keys = keys;
            //consoleClear();
            if (keys & 1) == 1 {
                let a_msg = CString::new("You held A!").expect("failed"); 
                unsafe { printf(a_msg.as_ptr()); }
            }

            if (keys & 2) == 2 {
                let b_msg = CString::new("You held B!").expect("failed"); 
                unsafe { printf(b_msg.as_ptr()); }
            }

            if (keys & 4) == 4 {
                let select_msg = CString::new("You held Select!").expect("failed"); 
                unsafe { printf(select_msg.as_ptr()); }
            }

            if (keys & 8) == 8 {
                //let start_msg = CString::new("You held Start!").expect("failed"); 
                //printf(start_msg.as_ptr());
                break;
            }

            if (keys & 16) == 16 {
                let right_msg = CString::new("You held Right!").expect("failed"); 
                unsafe { printf(right_msg.as_ptr()); }
            }

            if (keys & 32) == 32 {
                let left_msg = CString::new("You held Left!").expect("failed"); 
                unsafe { printf(left_msg.as_ptr()); }
            }

            if (keys & 64) == 64 {
                let up_msg = CString::new("You held Up!").expect("failed"); 
                unsafe { printf(up_msg.as_ptr()); }
            }

            if (keys & 128) == 128 {
                let down_msg = CString::new("You held Down!").expect("failed"); 
                unsafe { printf(down_msg.as_ptr()); }
            }
        }
        unsafe {
            l = aptMainLoop();
        }
    }
}
