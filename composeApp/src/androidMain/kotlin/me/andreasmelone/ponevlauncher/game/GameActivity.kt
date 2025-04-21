package me.andreasmelone.ponevlauncher.game

import android.app.Activity
import android.os.Bundle

class GameActivity : Activity() {
    private val glSurface = MinecraftGLSurface(baseContext)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        glSurface.start()
    }
}