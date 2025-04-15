package me.andreasmelone.ponevlauncher

import okio.Path
import okio.Path.Companion.toPath
import platform.Foundation.NSDocumentDirectory
import platform.Foundation.NSFileManager
import platform.Foundation.NSLog
import platform.Foundation.NSURL
import platform.Foundation.NSUserDomainMask
import platform.UIKit.UIDevice
import kotlin.experimental.ExperimentalNativeApi

class IOSPlatform : Platform {
    override val name: String = UIDevice.currentDevice.systemName() + " " + UIDevice.currentDevice.systemVersion
    override val homeDir: Path
    override val cacheDir: Path

    init {
        val fileManager = NSFileManager.defaultManager()

        val documentUrls = fileManager.URLsForDirectory(NSDocumentDirectory, inDomains = NSUserDomainMask)
        val documentDirectoryURL = documentUrls.firstOrNull() as NSURL?
        val rootHomeDir = (documentDirectoryURL?.path ?: "").toPath()

        homeDir = rootHomeDir/"files"
        cacheDir = rootHomeDir/"cache"
    }
}

object IOSLogger : PlatformlessLogger {
    @OptIn(ExperimentalNativeApi::class)
    private val debug: Boolean = kotlin.native.Platform.isDebugBinary

    override fun info(tag: String, message: String) {
        NSLog("INFO | [$tag] $message")
    }

    override fun info(tag: String, message: String, ex: Exception) {
        info(tag, message + "\n${ex.stackTraceToString()}")
    }

    override fun debug(tag: String, message: String) {
        if(debug) NSLog("DEBUG | [$tag] $message")
    }

    override fun debug(tag: String, message: String, ex: Exception) {
        debug(tag, message + "\n${ex.stackTraceToString()}")
    }

    override fun warn(tag: String, message: String) {
        NSLog("WARN | [$tag] $message")
    }

    override fun warn(tag: String, message: String, ex: Exception) {
        warn(tag, message + "\n${ex.stackTraceToString()}")
    }

    override fun error(tag: String, message: String) {
        NSLog("ERROR | [$tag] $message")
    }

    override fun error(tag: String, message: String, ex: Exception) {
        error(tag, message + "\n${ex.stackTraceToString()}")
    }

    override fun verbose(tag: String, message: String) {
        if(debug) NSLog("VERBOSE | [$tag] $message")
    }

    override fun verbose(tag: String, message: String, ex: Exception) {
        verbose(tag, message + "\n${ex.stackTraceToString()}")
    }
}

actual val platform: Platform = IOSPlatform()
actual val logger: PlatformlessLogger = IOSLogger
