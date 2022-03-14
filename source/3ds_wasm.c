#include "m3_env.h"	
#include "3ds_wasm.h"
#include <3ds.h>

m3ApiRawFunction(m3_aptMainLoop) {
    m3ApiReturnType(bool)
    m3ApiReturn(aptMainLoop());
}

m3ApiRawFunction(m3_gspWaitForVBlank) {
    aptMainLoop();
    m3ApiSuccess();
}

m3ApiRawFunction(m3_gfxSwapBuffers) {
    gfxSwapBuffers();
    m3ApiSuccess();
}

m3ApiRawFunction(m3_hidScanInput) {
    hidScanInput();
    m3ApiSuccess();
}

m3ApiRawFunction(m3_hidKeysDown) {
    m3ApiReturnType(u32)
    m3ApiReturn(hidKeysDown());
}

m3ApiRawFunction(m3_printf) {
    m3ApiGetArgMem(const char *, i_ptr);
    printf(i_ptr);
    m3ApiSuccess();
}

m3ApiRawFunction(m3_consoleClear) {
    consoleClear();
    m3ApiSuccess();
}

m3ApiRawFunction(m3_gfxFlushBuffers) {
    gfxFlushBuffers();
    m3ApiSuccess();
}

m3ApiRawFunction(m3_consoleInit) {
    m3ApiGetArg(u8, gfxSelector);
    PrintConsole *p = consoleInit(gfxSelector, NULL);
    intptr_t ip = p;

    m3ApiReturnType(intptr_t);
    m3ApiReturn(ip);
}

m3ApiRawFunction(m3_consoleSelect) {
    m3ApiGetArg(intptr_t, ptr);
    PrintConsole *p = ptr;

    consoleSelect(p);
    m3ApiSuccess();
}


void Link3DSFunctions(IM3Module module) {
    m3_LinkRawFunction(module, "3ds", "aptMainLoop", "i()", &m3_aptMainLoop);
    m3_LinkRawFunction(module, "3ds", "gspWaitForVBlank", "v()", &m3_gspWaitForVBlank);
    m3_LinkRawFunction(module, "3ds", "gfxSwapBuffers", "v()", &m3_gfxSwapBuffers);
    m3_LinkRawFunction(module, "3ds", "gfxFlushBuffers", "v()", &m3_gfxFlushBuffers);
    m3_LinkRawFunction(module, "3ds", "hidScanInput", "v()", &m3_hidScanInput);
    m3_LinkRawFunction(module, "3ds", "hidKeysDown", "i()", &m3_hidKeysDown);
    m3_LinkRawFunction(module, "3ds", "printf", "v(i)", &m3_printf);
    m3_LinkRawFunction(module, "3ds", "consoleClear", "v()", &m3_consoleClear);
    m3_LinkRawFunction(module, "3ds", "consoleInit", "i(i)", &m3_consoleInit);
    m3_LinkRawFunction(module, "3ds", "consoleSelect", "i(i)", &m3_consoleSelect);
}