package me.andreasmelone.ponevlauncher

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import me.andreasmelone.ponevlauncher.utils.checkInternetConnection
import org.apache.logging.log4j.Level
import org.apache.logging.log4j.core.config.Configurator

suspend fun main() {
    checkInternetConnection()

    if (java.lang.Boolean.getBoolean("debug")) {
        Configurator.setRootLevel(Level.DEBUG)
    }

    application {
        Window(
            onCloseRequest = ::exitApplication,
            title = "Ponev Launcher"
        ) {
            App()
        }
    }
}
