package me.andreasmelone.ponevlauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import me.andreasmelone.ponevlauncher.utils.checkInternetConnection
import me.andreasmelone.ponevlauncher.utils.hasInternetConnection
import okio.Path.Companion.toPath

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val dataDir = getExternalFilesDir(null)?.absolutePath ?: filesDir.absolutePath.also { path ->
            logger.error("MainActivity", "Failed to fetch data dir! Falling back to: $path")
        }
        val cacheDir = externalCacheDir?.absolutePath ?: cacheDir.absolutePath.also { path ->
            logger.error("MainActivity", "Failed to fetch cache dir! Falling back to: $path")
        }
        homeDir0 = dataDir.toPath()
        cacheDir0 = cacheDir.toPath()

        runBlocking {
            checkInternetConnection()
        }

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
