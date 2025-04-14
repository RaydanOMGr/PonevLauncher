package me.andreasmelone.mojolauncher

import android.os.Build
import android.util.Log

class AndroidPlatform(override val homeDir: String) : Platform {
    override val name: String = "Android ${Build.VERSION.SDK_INT}"
}

object AndroidLogger : PlatformlessLogger {
    override fun info(tag: String, message: String) {
        Log.i(tag, message)
    }

    override fun info(tag: String, message: String, ex: Exception) {
        Log.i(tag, message, ex)
    }

    override fun debug(tag: String, message: String) {
        Log.d(tag, message)
    }

    override fun debug(tag: String, message: String, ex: Exception) {
        Log.d(tag, message, ex)
    }

    override fun warn(tag: String, message: String) {
        Log.w(tag, message)
    }

    override fun warn(tag: String, message: String, ex: Exception) {
        Log.w(tag, message, ex)
    }

    override fun error(tag: String, message: String) {
        Log.e(tag, message)
    }

    override fun error(tag: String, message: String, ex: Exception) {
        Log.e(tag, message, ex)
    }

    override fun verbose(tag: String, message: String) {
        Log.v(tag, message)
    }

    override fun verbose(tag: String, message: String, ex: Exception) {
        Log.v(tag, message, ex)
    }
}

@Volatile
lateinit var currentPlatform: AndroidPlatform
fun initPlatform(homeDir: String) {
    currentPlatform = AndroidPlatform(homeDir)
}

actual val platform: Platform
    get() {
        if (!::currentPlatform.isInitialized) {
            throw IllegalStateException("Platform not yet initialized!")
        }
        return currentPlatform
    }
actual val logger: PlatformlessLogger = AndroidLogger