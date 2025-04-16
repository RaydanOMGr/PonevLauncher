package me.andreasmelone.ponevlauncher.minecraft

import de.jensklingenberg.ktorfit.Ktorfit
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.Path

const val BASE_URL = "https://resources.download.minecraft.net/"
interface ResourcesAPI {
    @GET("{hashSubstr}/{hash}")
    suspend fun get(@Path("hashSubstr") hashSubstr: String, @Path("hash") fullHash: String): ByteArray

    companion object : ResourcesAPI by Ktorfit.Builder()
        .baseUrl(BASE_URL)
        .httpClient(client)
        .build()
        .createResourcesAPI()
}
