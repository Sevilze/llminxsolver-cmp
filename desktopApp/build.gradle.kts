import org.gradle.jvm.toolchain.JavaLanguageVersion
import org.jetbrains.compose.desktop.application.dsl.TargetFormat
import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
}

kotlin {
    jvmToolchain(25)

    jvm("desktop") {
        @OptIn(ExperimentalKotlinGradlePluginApi::class)
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_25)
        }
    }

    sourceSets {
        val desktopMain by getting {
            dependencies {
                implementation(project(":shared"))
                implementation(compose.desktop.currentOs)
            }
        }
    }
}

compose.desktop {
    application {
        javaHome =
            javaToolchains
                .launcherFor {
                    languageVersion.set(JavaLanguageVersion.of(25))
                }
                .get()
                .metadata
                .installationPath
                .asFile
                .absolutePath

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

        buildTypes.release.proguard {
            version.set("7.8.2")
            configurationFiles.from(project.file("proguard-rules.pro"))
        }
    }
}
