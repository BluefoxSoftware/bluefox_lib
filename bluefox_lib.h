#pragma once

#ifndef BLUEFOX_PLUGIN

#ifdef __cplusplus
// C++ specific bindings

struct Plugin {
    const char* name;
};

extern "C" Plugin new_plugin(const char*);

extern "C" Plugin plugin_interface;

extern "C" Plugin get_plugin() {
    return plugin_interface;
}

#else // __cplusplus
// C specific Bindings

typedef struct {
    const char* name;
} Plugin;

extern Plugin new_plugin(const char*);

extern Plugin plugin_interface;

extern Plugin get_plugin() {
    return plugin_interface;
}

#endif // __cplusplus

#define BLUEFOX_PLUGIN
#endif // BLUEFOX_PLUGIN