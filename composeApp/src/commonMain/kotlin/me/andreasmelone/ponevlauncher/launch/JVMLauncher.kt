package me.andreasmelone.ponevlauncher.launch

import okio.Path

interface JVMLauncher {
    fun launch(dir: Path, args: List<String>): Int
}