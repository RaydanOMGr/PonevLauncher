@file:JvmName("JNI")

package me.andreasmelone.ponevlauncher

external fun sayHello(name: String): String

external fun spawnJvm(jvmFlags: Array<String>, programArgs: Array<String>, mainClass: String)
