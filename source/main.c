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
    consoleInit(GFX_TOP, NULL);

    printf("Loading WASM Module...!\n");

    IM3Environment env = m3_NewEnvironment();
    IM3Runtime runtime = m3_NewRuntime(env, 1000000, NULL);
    IM3Module module;
    m3_ParseModule(env, &module, __target_wasm32_unknown_unknown_debug_app_wasm, __target_wasm32_unknown_unknown_debug_app_wasm_len);
    m3_LoadModule(runtime, module);
    Link3DSFunctions(module);
    IM3Function f;
    m3_FindFunction(&f, runtime, "main");
    M3Result result = m3_CallV(f, 10);

    printf(result);

    while (aptMainLoop()) {}

    gfxExit();
    return 0;
}