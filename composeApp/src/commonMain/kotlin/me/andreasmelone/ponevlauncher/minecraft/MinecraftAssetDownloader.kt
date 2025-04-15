package me.andreasmelone.ponevlauncher.minecraft

import io.ktor.util.sha1
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import me.andreasmelone.ponevlauncher.logger
import me.andreasmelone.ponevlauncher.utils.createParentDirectories
import me.andreasmelone.ponevlauncher.utils.exists
import me.andreasmelone.ponevlauncher.utils.sha1
import me.andreasmelone.ponevlauncher.utils.bufferedSink
import okio.Buffer
import okio.Path
import okio.use
import kotlin.concurrent.atomics.AtomicInt
import kotlin.concurrent.atomics.incrementAndFetch
import kotlin.math.round

object MinecraftAssetDownloader {
    suspend fun setupJar(dir: Path, version: String, progressReporter: (Float) -> Unit) {
        val path = dir.resolve("client-$version.jar")
        val versions = Piston.versions()
        val foundVersion = requireNotNull(versions.versions.find { it.id == version }) { "invalid version: $version" }
        val versionMeta = Piston.version(version, foundVersion.url)

        if (path.exists() && path.sha1() == versionMeta.downloads.client.sha1) {
            logger.info("MinecraftDownloader", "Client already downloaded, skipping")
            downloadAssets(dir, versionMeta, progressReporter)
            return
        }
        val clientJar = Piston.download(version, versionMeta.downloads.client.url)
        require(clientJar.size == versionMeta.downloads.client.size) {
            "Downloaded client jar does not match expected size of ${versionMeta.downloads.client.size}"
        }
        val actualHash = sha1(clientJar).toHexString()
        require(actualHash == versionMeta.downloads.client.sha1) {
            "Downloaded client jar hash does not match expected hash of ${versionMeta.downloads.client.sha1}"
        }
        path.createParentDirectories()
        path.bufferedSink().use { sink ->
            sink.write(clientJar)
        }

        downloadAssets(dir, versionMeta, progressReporter)
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
                            val target = dir.resolve(asset.hash.substring(0, 2)).resolve(asset.hash)
                            target.createParentDirectories()

                            if (!target.exists()) {
                                Buffer().use { buf ->
                                    buf.write(ResourcesAPI.get(asset.hash.substring(0, 2), asset.hash))

                                    target.bufferedSink().use { sink ->
                                        sink.writeAll(buf)
                                    }
                                }
                            }
                            val newDownloaded = downloaded.incrementAndFetch()
                            val progress = newDownloaded.toFloat() / assetIndex.size.toFloat()
                            val percentage = round(progress * 100f * 100f) / 100f
                            progressReporter(percentage)
                            logger.debug("AssetDownloader", "Downloaded asset $path ($percentage%)")
                        }
                    }
                }
            }

            // Sometimes the UI doesn't refresh quick enough
            progressReporter(100f)
        }
}
