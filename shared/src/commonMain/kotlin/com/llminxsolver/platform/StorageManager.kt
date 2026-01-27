package com.llminxsolver.platform

data class PruningTableInfo(
    val filename: String,
    val displayName: String,
    val depth: Int?,
    val sizeBytes: Long,
    val lastModified: Long
) {
    val sizeMB: Float get() = sizeBytes / (1024f * 1024f)

    companion object {
        private val DEPTH_PATTERN = Regex("""^d(\d+)_(.+)\.prn\.lz4$""")
        fun parseDepthFromFilename(filename: String): Int? =
            DEPTH_PATTERN.matchEntire(filename)?.groupValues?.get(1)?.toIntOrNull()

        fun parseDisplayNameFromFilename(filename: String): String {
            val match = DEPTH_PATTERN.matchEntire(filename)
            return if (match != null) {
                "${match.groupValues[2]} (depth ${match.groupValues[1]})"
            } else {
                filename.removeSuffix(".lz4").removeSuffix(".prn")
            }
        }
    }
}

expect class StorageManager() {
    fun getDataDirectory(): String
    fun getPruningTables(): List<PruningTableInfo>
    fun deletePruningTable(filename: String): Boolean
    fun getTotalStorageUsed(): Long
    fun getAvailableStorage(): Long
}
