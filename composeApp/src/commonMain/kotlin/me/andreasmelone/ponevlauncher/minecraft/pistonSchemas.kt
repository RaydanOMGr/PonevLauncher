package me.andreasmelone.ponevlauncher.minecraft

import kotlinx.serialization.Serializable

@Serializable
data class PistonVersion(
    val id: String,
    val type: String,
    val url: String,
)

@Serializable
data class PistonLatestVersions(
    val release: String,
    val snapshot: String,
)

@Serializable
data class PistonVersionsResponse(
    val latest: PistonLatestVersions,
    val versions: List<PistonVersion>,
)

@Serializable
data class PistonVersionResponse(
    val id: String,
    val downloads: PistonVersionDownloads,
    val assetIndex: PistonAssetIndex,
)

@Serializable
data class PistonVersionDownloads(
    val client: PistonVersionDownload,
    val server: PistonVersionDownload?,
)

@Serializable
data class PistonVersionDownload(
    val size: Int,
    val url: String,
    val sha1: String,
)

@Serializable
data class PistonAssetIndex(
    val url: String,
)

@Serializable
data class PistonAsset(
    val hash: String,
    val size: Int
)

@Serializable
data class PistonAssetIndexResponse(
    val objects: Map<String, PistonAsset>
)
