use std::ptr;

#[link(wasm_import_module = "3ds")]
extern { 
    fn gfxInitDefault(); 
    fn consoleInit(screen: u32); 
    fn printf(message: &str);
    fn aptMainLoop() -> bool;
    fn gspWaitForVBlank();
    fn gfxSwapBuffers();
    fn hidScanInput();
    fn hidKeysDown() -> u32;
    fn gfxExit();
}

const KEY_START: u32 = 0x4;

fn main() {
    unsafe {
        const GFX_TOP: u32 = 0;
        gfxInitDefault();
        consoleInit(GFX_TOP);

        printf("Hello, world!\n");

        // Main loop
        while aptMainLoop()
        {
            gspWaitForVBlank();
            gfxSwapBuffers();
            hidScanInput();

            // Your code goes here
            let kDown: u32 = hidKeysDown();
            if (kDown & KEY_START) == 0x4 {
                break; // break in order to return to hbmenu
            }
        }

        gfxExit();
        return;
    }
}
