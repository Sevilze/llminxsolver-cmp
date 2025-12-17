package com.llminxsolver

actual object NativeLib {
    private var loaded = false
    
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
}
