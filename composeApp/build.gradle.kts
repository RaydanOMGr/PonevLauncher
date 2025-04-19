import android.databinding.tool.ext.capitalizeUS
import org.ajoberstar.grgit.Grgit
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.compose.reload.ComposeHotRun
import org.jetbrains.kotlin.compose.compiler.gradle.ComposeFeatureFlag
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import java.time.LocalDate
import java.time.format.DateTimeFormatter

plugins {
    alias(libs.plugins.kmp)
    alias(libs.plugins.android.app)
    alias(libs.plugins.cmp)
    alias(libs.plugins.compose.compiler)
    alias(libs.plugins.download)
    alias(libs.plugins.grgit)
    alias(libs.plugins.ksp)
    alias(libs.plugins.ktorfit)
    alias(libs.plugins.kotlinx.serialization)
    alias(libs.plugins.chr)
    alias(libs.plugins.rust)
}

// Dependencies
kotlin {
    jvm("desktop")

    sourceSets {
        androidMain.dependencies {
            implementation(compose.preview)
            implementation(libs.androidx.activity.compose)
            implementation(libs.kotlinx.coroutines.android)
        }
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material)
            implementation(compose.ui)
            implementation(compose.components.resources)
            implementation(compose.components.uiToolingPreview)
            implementation(libs.androidx.lifecycle.viewmodel)
            implementation(libs.androidx.lifecycle.runtime.compose)
            implementation(libs.bundles.ktor)
            implementation(libs.kotlinx.serialization.json)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.kotlinx.io.core)
            implementation(libs.okio)
        }
        iosMain.dependencies {
            implementation(libs.ktor.client.darwin)
        }

        val desktopMain by getting
        desktopMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.log4j.core)
            implementation(libs.log4j.slf4j)
        }
    }
}

dependencies {
    debugImplementation(compose.uiTooling)
}

// Versioning

val grgit = if (extra.has("grgit")) {
    the<Grgit>()
} else {
    null
}
val date = LocalDate.now().format(DateTimeFormatter.ofPattern("yyyyMMdd"))
val dateSeconds = if (System.getenv("GITHUB_ACTIONS") == "true") {
    Integer.parseInt(System.getenv("GITHUB_RUN_NUMBER"))
} else {
    172005
}

fun appVersionName(): String {
    val currentBranch = grgit?.branch?.current()
    val latestCommit = grgit?.log(mapOf("maxCommits" to 1))?.get(0)
    val dateToday = date
    if (currentBranch == null || latestCommit == null) {
        return "LOCAL-${dateToday}"
    } else {
        val branchName = currentBranch.getName()
        val commitAbbreviation = latestCommit.abbreviatedId
        return "hebe-${dateToday}-${commitAbbreviation}-${branchName}"
    }
}

fun cfApiKey(): String {
    val key = System.getenv("CURSEFORGE_API_KEY")
    if (key != null) return key
    val curseforgeKeyFile = File("./curseforge_key.txt")
    if (curseforgeKeyFile.canRead() && curseforgeKeyFile.isFile) {
        return curseforgeKeyFile.readText()
    }
    logger.warn("You have no CurseForge key, the curseforge api will be disabled!")
    return ""
}

kotlin {
    androidTarget {
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_1_8)
        }
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach { iosTarget ->
        iosTarget.binaries.framework {
            baseName = "ComposeApp"
            isStatic = true
        }
    }

    compilerOptions {
        freeCompilerArgs.addAll(
            listOf(
                "-opt-in=kotlinx.coroutines.ExperimentalCoroutinesApi",
                "-opt-in=kotlinx.coroutines.DelicateCoroutinesApi",
                "-opt-in=kotlin.contracts.ExperimentalContracts",
                "-opt-in=kotlin.ExperimentalStdlibApi",
                "-opt-in=kotlin.concurrent.atomics.ExperimentalAtomicApi"
            )
        )
    }
}

android {
    namespace = "me.andreasmelone.ponevlauncher"
    compileSdk = 35
    ndkVersion = "29.0.13113456"

    lint {
        abortOnError = false
    }

    defaultConfig {
        applicationId = "me.andreasmelone.ponevlauncher"
        minSdk = 26
        targetSdk = 35
        versionCode = dateSeconds
        versionName = appVersionName()
        multiDexEnabled = true
        resValue("string", "curseforge_api_key", cfApiKey())
    }

    signingConfigs {
        create("customDebug") {
            storeFile = file("debug.keystore")
            storePassword = "android"
            keyAlias = "androiddebugkey"
            keyPassword = "android"
        }
    }

    buildTypes {
        debug {
            applicationIdSuffix = ".debug"
            isMinifyEnabled = false
            isShrinkResources = false
            proguardFiles.addAll(listOf(getDefaultProguardFile("proguard-android-optimize.txt"), file("proguard-rules.pro")))
            signingConfig = signingConfigs["customDebug"]
            resValue("string", "application_package", "me.andreasmelone.ponevlauncher.debug")
            resValue("string", "storageProviderAuthorities", "me.andreasmelone.ponevlauncher.scoped.gamefolder.debug")
        }
        create("proguard") {
            initWith(buildTypes["debug"])
            isMinifyEnabled = true
            isShrinkResources = true
        }
        create("proguardNoDebug") {
            initWith(buildTypes["proguard"])
            isDebuggable = false
        }

        release {
            isMinifyEnabled = false
            proguardFiles.addAll(listOf(getDefaultProguardFile("proguard-android.txt"), file("proguard-rules.pro")))
            // defaultConfig already set
            // multiDexEnabled = true
            // debuggable = true
            resValue("string", "application_package", "me.andreasmelone.ponevlauncher")
            resValue("string", "storageProviderAuthorities", "me.andreasmelone.ponevlauncher.scoped.gamefolder")
        }
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
    buildFeatures {
        buildConfig = true
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
}

compose.desktop {
    application {
        buildTypes.release.proguard {
            obfuscate.set(false)
            optimize.set(false)
            version.set("7.6.0")
        }

        mainClass = "me.andreasmelone.ponevlauncher.DesktopKt"

        nativeDistributions {
            targetFormats(*TargetFormat.values())
            packageName = "ponev-launcher"
            packageVersion = "1.0.0"
        }
    }
}

composeCompiler {
    featureFlags.add(ComposeFeatureFlag.OptimizeNonSkippingGroups)
}

tasks.withType<ComposeHotRun> {
    mainClass = compose.desktop.application.mainClass
    jvmArgs("-Ddebug=true")
}

androidRust {
    targets = listOf("arm", "arm64", "x86", "x86_64")
    minimumSupportedRustVersion = "1.86.0"

    module("ponev-jni") {
        path = rootProject.file("jni")
    }
}

fun jniBuild(profile: String) {
    tasks.register("buildJni${profile.capitalizeUS()}") {
        val targets = listOf("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
        targets.forEach { target ->
            dependsOn("build${profile.capitalizeUS()}Ponev-jniRust[$target]")
        }
    }
}

jniBuild("debug")
jniBuild("release")
