package me.andreasmelone.ponevlauncher.minecraft

import de.jensklingenberg.ktorfit.Ktorfit
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.Url
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.HttpRequestRetry
import io.ktor.client.plugins.HttpTimeout
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import me.andreasmelone.ponevlauncher.cacheDir
import me.andreasmelone.ponevlauncher.utils.exists
import me.andreasmelone.ponevlauncher.utils.bufferedSink
import me.andreasmelone.ponevlauncher.utils.bytes
import me.andreasmelone.ponevlauncher.utils.createParentDirectories
import me.andreasmelone.ponevlauncher.utils.hasInternetConnection
import me.andreasmelone.ponevlauncher.utils.text
import okio.use

val JSON = Json {
    ignoreUnknownKeys = true
}

val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json(JSON)
    }

    install(HttpTimeout) {
        connectTimeoutMillis = 60000
    }

    install(HttpRequestRetry) {
        retryOnException(10)
        exponentialDelay()
    }
}

interface PistonAPI {
    @GET("mc/game/version_manifest_v2.json")
    suspend fun versions(): PistonVersionsResponse

    @GET
    suspend fun version(@Url url: String): PistonVersionResponse

    @GET
    suspend fun download(@Url url: String): ByteArray

    @GET
    suspend fun assets(@Url url: String): PistonAssetIndexResponse

    companion object : PistonAPI by Ktorfit.Builder()
        .baseUrl("https://piston-meta.mojang.com/")
        .httpClient(client)
        .build()
        .createPistonAPI()
}

object Piston {
    private inline fun <reified T> cache(fileName: String, action: () -> T): T {
        val path = cacheDir / "piston" / fileName
        if (path.exists() && !hasInternetConnection()) {
            return if (T::class == ByteArray::class) {
                path.bytes() as T
            } else {
                JSON.decodeFromString(path.text())
            }
        }

        val data = action()

        path.createParentDirectories()
        path.bufferedSink().use { sink ->
            if (T::class == ByteArray::class) {
                sink.write(data as ByteArray)
            } else {
                sink.writeUtf8(JSON.encodeToString(data))
            }
        }

        return data
    }

    suspend fun versions() = cache("versions.json") { PistonAPI.versions() }
    suspend fun version(version: String, url: String) = cache("$version.json") { PistonAPI.version(url) }
    suspend fun download(version: String, url: String) = cache("client-$version.jar") { PistonAPI.download(url) }
    suspend fun assets(version: String, url: String) = cache("assets-$version.json") { PistonAPI.assets(url) }
}
