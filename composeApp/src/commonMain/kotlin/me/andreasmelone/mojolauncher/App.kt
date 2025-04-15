package me.andreasmelone.mojolauncher

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.RadioButton
import androidx.compose.material.Scaffold
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import me.andreasmelone.mojolauncher.minecraft.MinecraftAssetDownloader
import me.andreasmelone.mojolauncher.minecraft.PistonAPI
import me.andreasmelone.mojolauncher.minecraft.PistonVersion
import me.andreasmelone.mojolauncher.ui.LauncherTheme
import me.andreasmelone.mojolauncher.utils.state
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
@Preview
fun App() {
    var isEnabled by state(true)
    var options by state(listOf<String>())
    var selectedOption by remember { mutableStateOf<String?>(null) }
    val coroutineScope = rememberCoroutineScope()

    LaunchedEffect(Unit) {
        val versions = PistonAPI.versions()

        options = versions.versions
            .filter { version ->
                version.type == "release"
            }
            .map(PistonVersion::id)
    }

    LauncherTheme {
        Scaffold(
            bottomBar = {
                Column(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    var progress by state(0f)

                    if (progress > 0) {
                        Text(
                            "Downloading assets: $progress%",
                            modifier = Modifier
                                .padding(bottom = 5.dp)
                        )
                        LinearProgressIndicator(
                            progress = progress / 100f,
                            modifier = Modifier
                                .fillMaxWidth(0.5f)
                                .padding(bottom = 10.dp)
                        )
                    }

                    Button(
                        onClick = {
                            isEnabled = false
                            coroutineScope.launch {
                                val selected = selectedOption ?: return@launch
                                progress = 0f
                                MinecraftAssetDownloader.setupJar(platform.homeDir, selected) { progress = it }
                                isEnabled = true
                                logger.info("Main", "Downloaded jar!")
                            }
                        },
                        enabled = isEnabled,
                        modifier = Modifier.padding(bottom = 10.dp)
                    ) {
                        Text("Download Minecraft")
                    }
                }
            }
        ) {
            LazyColumn {
                items(options) { option ->
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .clickable { selectedOption = option }
                            .padding(16.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        RadioButton(
                            selected = selectedOption == option,
                            onClick = { selectedOption = option }
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text(text = option)
                    }
                }
            }
        }
    }
}
