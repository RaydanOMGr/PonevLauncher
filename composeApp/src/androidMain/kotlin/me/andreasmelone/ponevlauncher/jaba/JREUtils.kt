package me.andreasmelone.ponevlauncher.jaba

object JREUtils {
    external fun setupBridgeWindow(surface: Any)
    external fun releaseBridgeWindow()
    // this does not correspond to the launch_jvm method in jre_launcher.rs
    external fun launchJVM(args: List<String>): Int
    external fun setupExitMethod()
    external fun dlopen(path: String): Boolean
}