package me.andreasmelone.mojolauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import okio.Path.Companion.toPath

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val dataDir = getExternalFilesDir(null)?.absolutePath ?: filesDir.absolutePath.also { path ->
            logger.error("MainActivity", "Failed to fetch data dir! Falling back to: $path")
        }
        initPlatform(dataDir.toPath())

        setContent {
            App()
        }
    }
}

@Preview
@Composable
fun AppAndroidPreview() {
    App()
}
