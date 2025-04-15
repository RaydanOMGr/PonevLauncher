@file:OptIn(ExperimentalStdlibApi::class)

package me.andreasmelone.ponevlauncher.utils

import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.HttpTimeout
import io.ktor.client.request.get
import io.ktor.util.sha1
import okio.BufferedSink
import okio.FileSystem
import okio.Path
import okio.SYSTEM
import okio.Sink
import okio.Source
import okio.buffer

fun Path.createParentDirectories() {
    parent?.let { parent ->
        FileSystem.SYSTEM.createDirectories(parent)
    }
}

fun Path.sink(): Sink {
    return FileSystem.SYSTEM.sink(this)
}

fun Path.bufferedSink(): BufferedSink {
    return sink().buffer()
}

fun Path.source(): Source {
    return FileSystem.SYSTEM.source(this)
}

fun Path.exists(): Boolean {
    return FileSystem.SYSTEM.exists(this)
}

fun Path.bytes(): ByteArray {
    return FileSystem.SYSTEM.read(this) { readByteArray() }
}

fun Path.text(): String {
    return bytes().decodeToString()
}

fun Path.sha1(): String {
    return sha1(bytes()).toHexString()
}

val connectionCheckerClient = HttpClient(CIO) {
    install(HttpTimeout) {
        connectTimeoutMillis = 2000
    }
}

private var hasInternetConnection: Boolean? = null

suspend fun checkInternetConnection(): Boolean {
    val result = runCatching {
        connectionCheckerClient.get("https://piston-meta.mojang.com")
    }.isSuccess
    hasInternetConnection = result
    return result
}

fun hasInternetConnection(): Boolean {
    return hasInternetConnection!!
}

