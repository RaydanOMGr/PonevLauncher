package me.andreasmelone.ponevlauncher

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import kotlinx.coroutines.runBlocking
import me.andreasmelone.ponevlauncher.utils.checkInternetConnection
import okio.Path.Companion.toPath

class MainActivity : ComponentActivity() {
    companion object {
        init {
            System.loadLibrary("ponev_jni")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val dataDir = getExternalFilesDir(null)?.absolutePath ?: filesDir.absolutePath.also { path ->
            logger.error("MainActivity", "Failed to fetch data dir! Falling back to: $path")
        }
        val cacheDir = externalCacheDir?.absolutePath ?: cacheDir.absolutePath.also { path ->
            logger.error("MainActivity", "Failed to fetch cache dir! Falling back to: $path")
        }
        dataDir0 = dataDir.toPath()
        cacheDir0 = cacheDir.toPath()

        logger.info("Ponav", sayHello("rad"))

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
