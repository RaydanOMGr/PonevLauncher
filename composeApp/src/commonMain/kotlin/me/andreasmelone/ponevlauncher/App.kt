package me.andreasmelone.ponevlauncher

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
import me.andreasmelone.ponevlauncher.minecraft.MinecraftAssetDownloader
import me.andreasmelone.ponevlauncher.minecraft.Piston
import me.andreasmelone.ponevlauncher.minecraft.PistonVersion
import me.andreasmelone.ponevlauncher.ui.LauncherTheme
import me.andreasmelone.ponevlauncher.utils.state
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
@Preview
fun App() {
    var isEnabled by state(true)
    var options by state(listOf<String>())
    var selectedOption by remember { mutableStateOf<String?>(null) }
    val coroutineScope = rememberCoroutineScope()

    LaunchedEffect(Unit) {
        val versions = Piston.versions()

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
                            "Downloading Minecraft: $progress%",
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
                                MinecraftAssetDownloader.setupJar(dataDir, selected) { progress = it }
                                isEnabled = true
                                logger.info("Ponev", "Jar and assets set up!")
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
