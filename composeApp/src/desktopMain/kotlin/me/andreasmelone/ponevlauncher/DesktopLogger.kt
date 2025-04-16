package me.andreasmelone.ponevlauncher

import org.slf4j.LoggerFactory.getLogger

object DesktopLogger : PlatformlessLogger {
    override fun info(tag: String, message: String) {
        getLogger(tag).info(message)
    }

    override fun info(tag: String, message: String, ex: Exception) {
        getLogger(tag).info(message, ex)
    }

    override fun debug(tag: String, message: String) {
        getLogger(tag).debug(message)
    }

    override fun debug(tag: String, message: String, ex: Exception) {
        getLogger(tag).debug(message, ex)
    }

    override fun warn(tag: String, message: String) {
        getLogger(tag).warn(message)
    }

    override fun warn(tag: String, message: String, ex: Exception) {
        getLogger(tag).warn(message, ex)
    }

    override fun error(tag: String, message: String) {
        getLogger(tag).error(message)
    }

    override fun error(tag: String, message: String, ex: Exception) {
        getLogger(tag).error(message, ex)
    }

    override fun verbose(tag: String, message: String) {
        getLogger(tag).trace(message)
    }

    override fun verbose(tag: String, message: String, ex: Exception) {
        getLogger(tag).trace(message, ex)
    }
}
