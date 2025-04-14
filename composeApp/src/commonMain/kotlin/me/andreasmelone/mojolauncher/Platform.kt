package me.andreasmelone.mojolauncher

import kotlinx.serialization.json.Json

interface Platform {
    val name: String
    val homeDir: String
}

// TODO replace String by String... or whatever varargs are in kotlin
interface PlatformlessLogger {
    fun info(tag: String, message: String)
    fun debug(tag: String, message: String)
    fun warn(tag: String, message: String)
    fun error(tag: String, message: String)
    fun verbose(tag: String, message: String)

    fun info(tag: String, message: String, ex: Exception)
    fun debug(tag: String, message: String, ex: Exception)
    fun warn(tag: String, message: String, ex: Exception)
    fun error(tag: String, message: String, ex: Exception)
    fun verbose(tag: String, message: String, ex: Exception)
}

expect val platform: Platform
expect val logger: PlatformlessLogger
val json = Json { ignoreUnknownKeys = true }