import org.jetbrains.compose.desktop.application.dsl.TargetFormat

plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
}

kotlin {
    jvm("desktop")

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
