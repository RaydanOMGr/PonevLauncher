package me.andreasmelone.ponevlauncher.minecraft

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import me.andreasmelone.ponevlauncher.logger
import me.andreasmelone.ponevlauncher.utils.bufferedSink
import me.andreasmelone.ponevlauncher.utils.bytes
import me.andreasmelone.ponevlauncher.utils.createParentDirectories
import me.andreasmelone.ponevlauncher.utils.exists
import me.andreasmelone.ponevlauncher.utils.sha1
import okio.Buffer
import okio.Path
import okio.use
import kotlin.concurrent.atomics.AtomicInt
import kotlin.concurrent.atomics.incrementAndFetch
import kotlin.math.round

object MinecraftAssetDownloader {
    suspend fun setupJar(dir: Path, version: String, progressReporter: (Float) -> Unit) {
        val path = dir / "versions" / "$version.jar"
        val versions = Piston.versions()
        val foundVersion = requireNotNull(versions.versions.find { it.id == version }) { "invalid version: $version" }
        val versionMeta = Piston.version(version, foundVersion.url)

        downloadFile(path, versionMeta.downloads.client.sha1, versionMeta.downloads.client.url, versionMeta.downloads.client.size)
        downloadAssets(dir / "assets" / "objects", versionMeta, progressReporter)
        downloadLibraries(dir / "libraries", versionMeta, progressReporter)
    }

    suspend fun downloadAssets(dir: Path, response: PistonVersionResponse, progressReporter: (percentage: Float) -> Unit) =
        withContext(Dispatchers.IO) {
            val assetIndex = Piston.assets(response.id, response.assetIndex.url).objects
            logger.debug("Extracting", "Got asset index")
            val downloaded = AtomicInt(0)

            coroutineScope {
                // Download two assets per job to not spam the API too much
                assetIndex.entries.chunked(2).forEach { assets ->
                    launch {
                        assets.forEach { (path, asset) ->
                            val target = dir / asset.hash.substring(0, 2) / asset.hash
                            val assetDownloaded = downloadAsset(target, asset.hash)

                            val newDownloaded = downloaded.incrementAndFetch()
                            val progress = newDownloaded.toFloat() / assetIndex.size.toFloat()
                            val percentage = round(progress * 100f * 100f) / 100f
                            progressReporter(percentage)
                            logger.debug(
                                "AssetDownloader",
                                if(assetDownloaded) "Downloaded asset $path ($percentage%)"
                                else "Skipped asset ${path} ($percentage%)"
                            )
                        }
                    }
                }
            }

            // Sometimes the UI doesn't refresh quick enough
            progressReporter(100f)
        }

    suspend fun downloadLibraries(dir: Path, response: PistonVersionResponse, progressReporter: (Float) -> Unit) =
        withContext(Dispatchers.IO) {
            val librariesResponse = response.libraries
            val downloaded = AtomicInt(0)

            coroutineScope {
                // Download two libraries per job to not spam the API too much
                librariesResponse.chunked(2).forEach { libraries ->
                    launch {
                        libraries.forEach { library ->
                            val artifact = library.downloads.artifact
                            val target = dir / artifact.path
                            var assetDownloaded = false
                            if (library.rules.isEmpty() || library.rules.any { it.action == "allow" && it.os?.name == "linux" }) {
                                assetDownloaded = downloadFile(target, artifact.sha1, artifact.url, artifact.size)
                            }

                            val newDownloaded = downloaded.incrementAndFetch()
                            val progress = newDownloaded.toFloat() / librariesResponse.size.toFloat()
                            val percentage = round(progress * 100f * 100f) / 100f
                            progressReporter(percentage)
                            logger.debug(
                                "AssetDownloader",
                                if(assetDownloaded) "Downloaded library ${library.name} ($percentage%)"
                                else "Skipped library ${library.name} ($percentage%)"
                            )
                        }
                    }
                }
            }

            // Sometimes the UI doesn't refresh quick enough
            progressReporter(100f)
        }

    suspend fun downloadFile(file: Path, hash: String, link: String, size: Int = -1): Boolean {
        val fileExists = file.exists()
        val sizesMatch = !fileExists || size == -1 || file.bytes().size == size
        val hashMatches = fileExists && file.sha1() == hash

        if (!sizesMatch || !hashMatches) {
            file.createParentDirectories()
            Buffer().use { buf ->
                buf.write(Piston.download(hash, link))

                file.bufferedSink().use { sink ->
                    sink.writeAll(buf)
                }
            }
            return true
        }
        return false
    }

    suspend fun downloadAsset(file: Path, hash: String): Boolean {
        if (!file.exists() || file.sha1() != hash) {
            file.createParentDirectories()
            Buffer().use { buf ->
                buf.write(ResourcesAPI.get(hash.substring(0, 2), hash))

                file.bufferedSink().use { sink ->
                    sink.writeAll(buf)
                }
            }
            return true
        }
        return false
    }
}
