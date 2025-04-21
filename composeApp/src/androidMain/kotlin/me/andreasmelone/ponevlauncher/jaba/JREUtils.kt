package me.andreasmelone.ponevlauncher.jaba

object JREUtils {
    external fun setupBridgeWindow(surface: Any)
    external fun releaseBridgeWindow()
    external fun launchJVM()
    external fun dlopen(path: String): Boolean
}