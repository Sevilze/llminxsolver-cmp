package com.llminxsolver.platform

data class PruningTableInfo(
    val filename: String,
    val displayName: String,
    val sizeBytes: Long,
    val lastModified: Long
) {
    val sizeMB: Float get() = sizeBytes / (1024f * 1024f)
}

expect class StorageManager() {
    fun getDataDirectory(): String
    fun getPruningTables(): List<PruningTableInfo>
    fun deletePruningTable(filename: String): Boolean
    fun getTotalStorageUsed(): Long
    fun getAvailableStorage(): Long
}
