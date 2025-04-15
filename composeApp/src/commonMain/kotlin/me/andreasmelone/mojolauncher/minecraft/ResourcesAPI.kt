package me.andreasmelone.mojolauncher.minecraft

import de.jensklingenberg.ktorfit.Ktorfit
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.Path

interface ResourcesAPI {
    @GET("{hashSubstr}/{hash}")
    suspend fun get(@Path("hashSubstr") hashSubstr: String, @Path("hash") fullHash: String): ByteArray

    @Suppress("DEPRECATION")
    companion object : ResourcesAPI by Ktorfit.Builder()
        .baseUrl("https://resources.download.minecraft.net/")
        .httpClient(client)
        .build()
        .create()
}
