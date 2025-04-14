package me.andreasmelone.mojolauncher.minecraft

import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.request.get
import io.ktor.client.statement.readBytes
import io.ktor.http.Url
import kotlinx.io.IOException
import kotlinx.io.buffered
import kotlinx.io.files.SystemFileSystem
import kotlinx.io.files.Path
import me.andreasmelone.mojolauncher.json
import me.andreasmelone.mojolauncher.logger
import org.kotlincrypto.hash.sha1.SHA1

object MinecraftAssetDownloader {
    const val PISTON = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    private val client = HttpClient(CIO)

    private suspend fun download(url: String): ByteArray {
        val response = client.get(Url(url))
        if(response.status.value < 200 || response.status.value >= 400) throw IOException("Failed to fetch versions! Failed with status ${response.status}")
        return response.readBytes()
    }

    private suspend inline fun <reified T> downloadAndParse(url: String): T? {
        val string = download(url).decodeToString();
        try {
            return json.decodeFromString<T>(string)
        } catch (e: Exception) {
            logger.error("MinecraftAssetDownloader", "Failed to parse received json!", e)
            return null;
        }
    }

    suspend fun fetchVersions(): PistonVersionsResponse? {
        return downloadAndParse(PISTON)
    }

    @OptIn(ExperimentalStdlibApi::class)
    suspend fun downloadJar(path: String, version: String) {
        val versions = fetchVersions()
        val foundVersion = versions?.versions?.find { it.id == version } ?: throw IllegalArgumentException("Version $version not found!")

        val versionMeta: PistonVersionResponse = downloadAndParse(foundVersion.url) ?: throw IOException("Failed to fetch meta!")
        val clientJar = download(versionMeta.downloads.client.url)
        if(clientJar.size != versionMeta.downloads.client.size) throw IllegalStateException("Downloaded jar does not match expected size! Expected ${versionMeta.downloads.client.size}, got ${clientJar.size}")
        val actualHash = SHA1().digest(clientJar).toHexString();
        if(actualHash != versionMeta.downloads.client.sha1) throw IllegalStateException("Downloaded jar does not match expected sha1! Expected ${versionMeta.downloads.client.sha1}, got $actualHash")

        val currentPath = Path(path)
        SystemFileSystem.createDirectories(currentPath.parent!!)
        val sink = SystemFileSystem.sink(currentPath).buffered().use { it.write(clientJar, 0, versionMeta.downloads.client.size) }
    }
}