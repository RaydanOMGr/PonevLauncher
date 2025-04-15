package me.andreasmelone.mojolauncher.minecraft

import de.jensklingenberg.ktorfit.Ktorfit
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.Url
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.HttpTimeout
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json

val client = HttpClient(CIO) {
    install(ContentNegotiation) {
        json(Json {
            ignoreUnknownKeys = true
        })
    }

    install(HttpTimeout) {
        connectTimeoutMillis = 20000
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

    @Suppress("DEPRECATION")
    companion object : PistonAPI by Ktorfit.Builder()
        .baseUrl("https://piston-meta.mojang.com/")
        .httpClient(client)
        .build()
        .create()
}
