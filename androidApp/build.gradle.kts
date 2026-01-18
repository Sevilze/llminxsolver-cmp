import java.util.Properties

plugins {
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.composeCompiler)
}

android {
    namespace = "com.llminxsolver"
    compileSdk = libs.versions.android.compileSdk.get().toInt()

    defaultConfig {
        applicationId = "com.llminxsolver"
        minSdk = libs.versions.android.minSdk.get().toInt()
        targetSdk = libs.versions.android.targetSdk.get().toInt()
        versionCode = 5
        versionName = "1.3.1"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }

    val localProperties = Properties().apply {
        val localPropertiesFile = rootProject.file("local.properties")
        if (localPropertiesFile.exists()) {
            load(localPropertiesFile.inputStream())
        }
    }

    signingConfigs {
        create("release") {
            storeFile =
                file(localProperties.getProperty("RELEASE_STORE_FILE", "../release-keystore.jks"))
            storePassword = localProperties.getProperty("RELEASE_STORE_PASSWORD", "")
            keyAlias = localProperties.getProperty("RELEASE_KEY_ALIAS", "release")
            keyPassword = localProperties.getProperty("RELEASE_KEY_PASSWORD", "")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }

    buildFeatures {
        compose = true
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

dependencies {
    implementation(project(":shared"))
    implementation(libs.androidx.activity.compose)
    debugImplementation("org.jetbrains.compose.ui:ui-tooling:1.11.0-alpha01")
}

tasks.register("renameReleaseBundle") {
    dependsOn("bundleRelease")

    val versionName = android.defaultConfig.versionName ?: "unknown"
    val bundleDirProvider = layout.buildDirectory.dir("outputs/bundle/release")

    doLast {
        val bundleDir = bundleDirProvider.get().asFile
        val originalFile = bundleDir.resolve("androidApp-release.aab")
        if (originalFile.exists()) {
            val newFile = bundleDir.resolve("llminxsolver-v$versionName-release.aab")
            originalFile.renameTo(newFile)
            println("Renamed bundle to: ${newFile.name}")
        }
    }
}
