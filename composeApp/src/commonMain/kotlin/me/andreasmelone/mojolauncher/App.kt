package me.andreasmelone.mojolauncher

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.RadioButton
import androidx.compose.material.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import me.andreasmelone.mojolauncher.minecraft.MinecraftAssetDownloader
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
@Preview
fun App() {
    var isEnabled by remember { mutableStateOf(true) }
    val options = remember { mutableStateListOf<String>() }
    var selectedOption by remember { mutableStateOf<String?>(null) }
    val coroutineScope = rememberCoroutineScope()

    LaunchedEffect(Unit) {
        val versions = MinecraftAssetDownloader.fetchVersions() ?: return@LaunchedEffect
        options.clear()
        versions.versions.forEach {
            if(it.type == "release") options.add(it.id)
        }
    }
    MaterialTheme {
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
        Column(Modifier.fillMaxWidth(), horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.Bottom) {
            Button(onClick = {
                isEnabled = false
                coroutineScope.launch {
                    val selected = selectedOption ?: return@launch
                    MinecraftAssetDownloader.downloadJar(platform.homeDir + "/client-$selected.jar", selected)
                    isEnabled = true
                    logger.info("Main", "Downloaded jar!")
                }
            }, enabled = isEnabled) {
                Text("Download minecraft")
            }
        }
    }
}