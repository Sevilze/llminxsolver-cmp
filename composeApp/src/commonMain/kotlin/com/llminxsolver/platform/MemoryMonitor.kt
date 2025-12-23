package com.llminxsolver.platform

data class MemoryInfo(
    val usedMemoryBytes: Long,
    val totalMemoryBytes: Long,
    val usagePercent: Float,
    val appMemoryBytes: Long = 0L
) {
    val usedMB: Long get() = usedMemoryBytes / (1024 * 1024)
    val totalMB: Long get() = totalMemoryBytes / (1024 * 1024)
    val appMB: Long get() = appMemoryBytes / (1024 * 1024)
}

expect class MemoryMonitor() {
    fun getMemoryInfo(): MemoryInfo
    fun startMonitoring(intervalMs: Long, onUpdate: (MemoryInfo) -> Unit)
    fun stopMonitoring()
}
