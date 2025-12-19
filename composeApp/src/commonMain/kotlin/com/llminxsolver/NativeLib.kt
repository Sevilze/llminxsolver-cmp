package com.llminxsolver

expect object NativeLib {
    fun ensureLoaded()
    fun initialize(dataDirectory: String)
}
