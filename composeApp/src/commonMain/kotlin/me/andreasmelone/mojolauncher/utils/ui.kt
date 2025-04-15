package me.andreasmelone.mojolauncher.utils

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisallowComposableCalls
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember

@Composable
fun <T> state(provider: @DisallowComposableCalls () -> T): MutableState<T> {
    return remember { mutableStateOf(provider()) }
}

@Composable
fun <T> state(value: T): MutableState<T> {
    return state { value }
}
