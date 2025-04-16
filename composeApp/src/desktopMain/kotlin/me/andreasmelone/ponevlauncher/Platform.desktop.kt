package me.andreasmelone.ponevlauncher

import me.andreasmelone.ponevlauncher.utils.exists
import okio.Path
import okio.Path.Companion.toPath

actual val logger: PlatformlessLogger = DesktopLogger
actual val platformName: String = System.getProperty("os.name")

actual val dataDir: Path = when {
    "win" in platformName -> {
        System.getenv("APPDATA")?.toPath()
            ?: (System.getProperty("user.home").toPath() / "AppData" / "Roaming")
    }
    "mac" in platformName -> {
        System.getProperty("user.home").toPath() / "Library" / "Application Support"
    }
    else -> {
        val xdg = System.getenv("XDG_DATA_HOME")?.toPath()
        if (xdg != null && !xdg.isRoot && xdg.exists()) {
            xdg
        } else {
            System.getProperty("user.home").toPath() / ".local" / "share"
        }
    }
} / "ponev-launcher"

actual val cacheDir: Path = run {
    when {
        "win" in platformName -> {
            System.getenv("LOCALAPPDATA")?.toPath()
                ?: System.getenv("APPDATA")?.toPath()
                ?: (System.getProperty("user.home").toPath() / "AppData" / "Local")
        }
        "mac" in platformName -> {
            System.getProperty("user.home").toPath() / "Library" / "Caches"
        }
        else -> {
            val xdg = System.getenv("XDG_CACHE_HOME")?.toPath()
            if (xdg != null && !xdg.isRoot && xdg.exists()) {
                xdg
            } else {
                System.getProperty("user.home").toPath() / ".cache"
            }
        }
    }
} / "ponev-launcher"
