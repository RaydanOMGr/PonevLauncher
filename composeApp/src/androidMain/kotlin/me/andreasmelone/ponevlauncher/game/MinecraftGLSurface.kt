package me.andreasmelone.ponevlauncher.game

import android.content.Context
import android.graphics.SurfaceTexture
import android.view.Surface
import android.view.TextureView
import android.view.View
import android.view.ViewGroup
import me.andreasmelone.ponevlauncher.jaba.JREUtils

class MinecraftGLSurface(context: Context) : View(context) {
    lateinit var surfaceView: View

    fun start() {
        val textureView = TextureView(context)
        textureView.isOpaque = true
        textureView.alpha = 1.0f
        surfaceView = textureView

        textureView.surfaceTextureListener = object : TextureView.SurfaceTextureListener {
            private var isCalled = false

            override fun onSurfaceTextureAvailable(surfaceTexture: SurfaceTexture, p1: Int, p2: Int) {
                val surface = Surface(surfaceTexture)
                if(isCalled) {
                    JREUtils.setupBridgeWindow(surface)
                    return
                }
                isCalled = true
            }

            override fun onSurfaceTextureSizeChanged(surfaceTexture: SurfaceTexture, p1: Int, p2: Int) {
            }

            override fun onSurfaceTextureDestroyed(surfaceTexture: SurfaceTexture): Boolean {
                return true
            }

            override fun onSurfaceTextureUpdated(surfaceTexture: SurfaceTexture) {
            }
        }

        (parent as ViewGroup).addView(textureView)
    }
}