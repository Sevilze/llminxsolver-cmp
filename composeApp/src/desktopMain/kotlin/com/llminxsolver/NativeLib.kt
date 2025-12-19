package com.llminxsolver

import java.io.File
import uniffi.llminxsolver.setDataDirectory

actual object NativeLib {
    private var loaded = false
    private var initialized = false

    actual fun ensureLoaded() {
        if (!loaded) {
            try {
                val osName = System.getProperty("os.name").lowercase()
                val libName =
                    when {
                        osName.contains("linux") -> "libllminxsolver_uniffi.so"

                        osName.contains(
                            "mac"
                        ) || osName.contains("darwin") -> "libllminxsolver_uniffi.dylib"

                        osName.contains("win") -> "llminxsolver_uniffi.dll"

                        else -> throw UnsupportedOperationException("Unsupported OS: $osName")
                    }

                val resourceStream = NativeLib::class.java.getResourceAsStream("/$libName")
                if (resourceStream != null) {
                    val tempFile = File.createTempFile("llminxsolver_uniffi", null)
                    tempFile.deleteOnExit()
                    resourceStream.use { input ->
                        tempFile.outputStream().use { output ->
                            input.copyTo(output)
                        }
                    }
                    System.load(tempFile.absolutePath)
                    loaded = true
                } else {
                    System.loadLibrary("llminxsolver_uniffi")
                    loaded = true
                }
            } catch (e: Exception) {
                println(
                    "Warning: Native library not found. Using mock implementation. Error: ${e.message}"
                )
            }
        }
    }

    actual fun initialize(dataDirectory: String) {
        ensureLoaded()
        if (loaded && !initialized) {
            val dataDir = File(dataDirectory)
            if (!dataDir.exists()) {
                dataDir.mkdirs()
            }
            setDataDirectory(dataDirectory)
            initialized = true
        }
    }
}
