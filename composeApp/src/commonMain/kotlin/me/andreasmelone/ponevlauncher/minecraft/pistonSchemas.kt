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
    val libraries: List<PistonLibrary>
)

@Serializable
data class PistonVersionDownloads(
    val client: PistonVersionDownload,
    val server: PistonVersionDownload? = null,
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

@Serializable
data class PistonLibraryDownloads(
    val artifact: PistonLibraryArtifact
)

@Serializable
data class PistonLibraryArtifact(
    val path: String,
    val sha1: String,
    val size: Int,
    val url: String
)

@Serializable
data class PistonLibrary(
    val downloads: PistonLibraryDownloads,
    val name: String,
    val rules: List<PistonLibraryRule> = listOf()
)

@Serializable
data class PistonLibraryRule(
    val action: String,
    val os: PistonOsObject? = null,
)

@Serializable
data class PistonOsObject(
    val name: String,
)
