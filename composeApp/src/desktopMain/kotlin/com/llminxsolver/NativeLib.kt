package com.llminxsolver

import java.io.File

actual object NativeLib {
    private var loaded = false

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
}
