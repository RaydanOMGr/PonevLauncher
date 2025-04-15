package me.andreasmelone.mojolauncher.ui

import androidx.compose.material.darkColors
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.material.MaterialTheme

val LightGreen = Color(0xff90ceaa)
val LightBlue = Color(0xff86aaec)
val LightMagenta = Color(0xffc296eb)
val Foreground = Color(0xffa5b6cf)
val Background = Color(0xff0d0f18)
val BackgroundSecondary = Color(0xff040508)
val Red = Color(0xffdd6777)

val DarkColorScheme = darkColors(
    primary = LightGreen,
    primaryVariant = LightGreen,
    secondary = LightBlue,
    secondaryVariant = LightMagenta,
    error = Red,
    background = Background,
    surface = Background,
)

@Composable
fun LauncherTheme(
    content: @Composable () -> Unit
) {
    MaterialTheme(
        colors = DarkColorScheme,
        content = content
    )
}
