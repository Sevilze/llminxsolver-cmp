import java.util.Properties
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.androidApplication)
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
    alias(libs.plugins.kotlinSerialization)
}

kotlin {
    compilerOptions {
        freeCompilerArgs.add("-Xexpect-actual-classes")
        freeCompilerArgs.add(
            "-opt-in=androidx.compose.material3.ExperimentalMaterial3ExpressiveApi"
        )
        freeCompilerArgs.add("-opt-in=androidx.compose.material3.ExperimentalMaterial3Api")
    }

    androidTarget {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_21)
        }
    }

    jvm("desktop")

    sourceSets {
        val desktopMain by getting

        androidMain.dependencies {
            implementation(compose.preview)
            implementation(libs.androidx.activity.compose)
            implementation(libs.kotlinx.coroutines.android)
            implementation(libs.datastore.preferences.android)
            implementation("net.java.dev.jna:jna:5.18.1@aar")
        }

        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material3)
            implementation(compose.materialIconsExtended)
            implementation(compose.ui)
            implementation(compose.components.resources)
            implementation(compose.components.uiToolingPreview)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.datastore.preferences)
            implementation("org.jetbrains.compose.material3:material3:1.10.0-alpha05")
        }

        desktopMain.dependencies {
            implementation(compose.desktop.currentOs)
            implementation(libs.kotlinx.coroutines.swing)
            implementation(libs.jna)
            implementation(libs.kotlinx.serialization.json)
        }
    }
}

android {
    namespace = "com.llminxsolver"
    compileSdk =
        libs.versions.android.compileSdk
            .get()
            .toInt()

    defaultConfig {
        applicationId = "com.llminxsolver"
        minSdk =
            libs.versions.android.minSdk
                .get()
                .toInt()
        targetSdk =
            libs.versions.android.targetSdk
                .get()
                .toInt()
        versionCode = 4
        versionName = "1.1.1"
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

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

dependencies {
    debugImplementation(compose.uiTooling)
}

tasks.register("renameReleaseBundle") {
    dependsOn("bundleRelease")
    doLast {
        val bundleDir = layout.buildDirectory.dir("outputs/bundle/release").get().asFile
        val originalFile = bundleDir.resolve("composeApp-release.aab")
        if (originalFile.exists()) {
            val versionName = android.defaultConfig.versionName
            val newFile = bundleDir.resolve("llminxsolver-v$versionName-release.aab")
            originalFile.renameTo(newFile)
            println("Renamed bundle to: ${newFile.name}")
        }
    }
}

compose.desktop {
    application {
        mainClass = "com.llminxsolver.MainKt"

        nativeDistributions {
            targetFormats(TargetFormat.Exe, TargetFormat.Dmg, TargetFormat.Deb, TargetFormat.Rpm)
            packageName = "LLMinx Solver"
            packageVersion = System.getenv("APP_VERSION") ?: "1.0.0"
            description = "Megaminx Last Layer Solver with Compose Multiplatform GUI"
            vendor = "Sevilze"
            licenseFile.set(project.rootProject.file("LICENSE"))

            linux {
                iconFile.set(project.file("icons/manimicon.png"))
                debMaintainer = "sevilzcubing@gmail.com"
                menuGroup = "Utilities"
                appCategory = "Utilities"
                rpmLicenseType = "MIT"
            }

            windows {
                modules("java.management", "jdk.management")
                iconFile.set(project.file("icons/manimicon.ico"))
                menuGroup = "LLMinx Solver"
                upgradeUuid = "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
            }

            macOS {
                iconFile.set(project.file("icons/icon.icns"))
                bundleID = "com.llminxsolver"
                dockName = "LLMinx Solver"
            }
        }

        buildTypes.release {
            proguard {
                configurationFiles.from(project.file("proguard-rules.pro"))
            }
        }
    }
}
