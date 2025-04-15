@file:OptIn(ExperimentalStdlibApi::class)

package me.andreasmelone.ponevlauncher.utils

import io.ktor.util.sha1
import okio.BufferedSink
import okio.FileSystem
import okio.Path
import okio.SYSTEM
import okio.Source
import okio.buffer

fun Path.createParentDirectories() {
    parent?.let { parent ->
        FileSystem.SYSTEM.createDirectories(parent)
    }
}

fun Path.sink(): BufferedSink {
    return FileSystem.SYSTEM.sink(this).buffer()
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

fun Path.sha1(): String {
    return sha1(bytes()).toHexString()
}

suspend fun <T> tryNTimes(n: Int, action: suspend (Int) -> T): T {
    repeat(n) { current ->
        runCatching { action(current) }
            .onSuccess { return it }
            .onFailure(Throwable::printStackTrace)
    }

    error("Failed to run action $n times")
}
