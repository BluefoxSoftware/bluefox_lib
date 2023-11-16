import Foundation
@_exported import plugin

@_cdecl("get_plugin")
func get_plugin() -> plugin.Plugin {
    return plugin.plugin_interface
}