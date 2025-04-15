package me.andreasmelone.ponevlauncher

import android.os.Build
import android.util.Log
import okio.Path

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

actual val logger: PlatformlessLogger = AndroidLogger
actual val platformName: String = "Android ${Build.VERSION.SDK_INT}"
lateinit var homeDir0: Path
lateinit var cacheDir0: Path
actual val homeDir: Path get() = homeDir0
actual val cacheDir: Path get() = cacheDir0
