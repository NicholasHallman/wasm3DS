

mod lib3ds;
use lib3ds::*;

fn main() {

    let top = Console::init(GFX_SELECT::TOP);
    
    Console::select(top);

    let mut old_keys:u32 = 0;
    print("Hey there! try pressing a button.\n");
    print("\x1b[29;16HPress Start to exit");

    // Main loop
    while main_loop()
    {  
        Hid::scan_input();
        let keys:u32 = Hid::keys_down();
        
        if keys != old_keys {
            Console::clear();

            // We print these again because we just cleared the screen above
            print("Hi there! Try pressing a button");
            print("\x1b[29;16HPress Start to exit");

            // Move the cursor back to the top of the screen
            print("\x1b[3;0H");

            //consoleClear();
            if (keys & 1) == 1 {
                print("You held A!\n");
            }

            if (keys & 2) == 2 {
                print("You held B!\n");
            }

            if (keys & 4) == 4 {
                print("You held Select!\n");
            }

            if (keys & 8) == 8 {
                //let start_msg = CString::new("You held Start!").expect("failed"); 
                //printf(start_msg.as_ptr());
                break;
            }

            if (keys & 16) == 16 {
                print("You held Right!\n");
            }

            if (keys & 32) == 32 {
                print("You held Left!\n");
            }

            if (keys & 64) == 64 {
                print("You held Up!\n");
            }

            if (keys & 128) == 128 {
                print("You held Down!\n");
            }
        }

        old_keys = keys;

        Gfx::flush_buffers();
        Gfx::swap_buffers();
        Gsp::wait_for_vblank();
    }
}
