package com.llminxsolver

import uniffi.llminxsolver.setDataDirectory

actual object NativeLib {
    private var loaded = false
    private var initialized = false

    actual fun ensureLoaded() {
        if (!loaded) {
            try {
                System.loadLibrary("llminxsolver_uniffi")
                loaded = true
            } catch (e: UnsatisfiedLinkError) {
                println("Warning: Native library not found. Using mock implementation.")
            }
        }
    }

    actual fun initialize(dataDirectory: String) {
        ensureLoaded()
        if (loaded && !initialized) {
            setDataDirectory(dataDirectory)
            initialized = true
        }
    }
}
