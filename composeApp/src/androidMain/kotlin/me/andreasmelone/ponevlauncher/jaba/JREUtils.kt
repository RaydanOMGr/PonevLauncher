package me.andreasmelone.ponevlauncher.jaba

object JREUtils {
    external fun setupBridgeWindow(surface: Any)
    external fun releaseBridgeWindow()
    // this does not correspond to the launch_jvm method in jre_launcher.rs
    // but rather to the java_launch_jvm method
    external fun launchJVM(args: Array<String>): Int
    external fun setupExitMethod()
    external fun dlopen(path: String): Boolean
}