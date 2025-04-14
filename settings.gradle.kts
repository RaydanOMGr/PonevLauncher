rootProject.name = "MojoLauncher"
enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

pluginManagement {
    repositories {
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositories {
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
        mavenCentral()
        versionCatalogs {
            create("kotlincrypto") {
                // https://github.com/KotlinCrypto/version-catalog/blob/master/gradle/kotlincrypto.versions.toml
                from("org.kotlincrypto:version-catalog:0.7.0")
            }
        }
    }
}

include(":composeApp")