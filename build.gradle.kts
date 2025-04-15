plugins {
    // this is necessary to avoid the plugins to be loaded multiple times
    // in each subproject's classloader
    alias(libs.plugins.android.app) apply false
    alias(libs.plugins.android.lib) apply false
    alias(libs.plugins.cmp) apply false
    alias(libs.plugins.compose.compiler) apply false
    alias(libs.plugins.kmp) apply false
}
