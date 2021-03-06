#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <3ds.h>

#include <math.h>
#include <time.h>

#include <wasm3.h>
#include <m3_env.h>	
#include "app.wasm.h"
#include "3ds_wasm.h"

int main(void) {

    gfxInitDefault();
    PrintConsole *bottom = consoleInit(GFX_BOTTOM, NULL);
    consoleSelect(bottom);

    printf("Loading WASM Module...!\n");

    IM3Environment env = m3_NewEnvironment();
    IM3Runtime runtime = m3_NewRuntime(env, 2048, NULL);
    IM3Module module;
    m3_ParseModule(env, &module, a_wasm, a_wasm_len);
    m3_LoadModule(runtime, module);
    Link3DSFunctions(module);
    IM3Function f;
    m3_FindFunction(&f, runtime, "main");
    M3Result result = m3_CallV(f, 10);

    consoleClear();
    consoleSelect(bottom);
    gfxFlushBuffers();
    gfxSwapBuffers();
    gspWaitForVBlank();
    printf("WASM Module Finished\n");
    printf(result);

    while (aptMainLoop()) {
        hidScanInput();
        
        u32 keys = hidKeysDown();
        if ((keys & 8 ) == 8) {
            break;
        }

        gfxFlushBuffers();
        gfxSwapBuffers();
        gspWaitForVBlank();
    };

    gfxExit();
    return 0;
}