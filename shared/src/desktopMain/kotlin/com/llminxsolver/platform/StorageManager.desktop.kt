package com.llminxsolver.platform

import java.io.File
import java.nio.file.Files
import java.nio.file.Paths

actual class StorageManager actual constructor() {

    actual fun getDataDirectory(): String {
        val userHome = System.getProperty("user.home")
        val tablesDir = File(userHome, ".llminxsolver/tables")
        return tablesDir.absolutePath
    }

    actual fun getPruningTables(): List<PruningTableInfo> {
        val dataDir = File(getDataDirectory())
        if (!dataDir.exists()) return emptyList()

        return dataDir.listFiles()
            ?.filter { it.isFile && it.name.endsWith(".prn.lz4") }
            ?.map { file ->
                PruningTableInfo(
                    filename = file.name,
                    displayName = PruningTableInfo.parseDisplayNameFromFilename(file.name),
                    depth = PruningTableInfo.parseDepthFromFilename(file.name),
                    sizeBytes = file.length(),
                    lastModified = file.lastModified()
                )
            }
            ?.sortedByDescending { it.sizeBytes }
            ?: emptyList()
    }

    actual fun deletePruningTable(filename: String): Boolean {
        val file = File(getDataDirectory(), filename)
        return if (file.exists()) file.delete() else false
    }

    actual fun getTotalStorageUsed(): Long {
        val dataDir = File(getDataDirectory())
        if (!dataDir.exists()) return 0
        return dataDir.listFiles()?.sumOf { it.length() } ?: 0
    }

    actual fun getAvailableStorage(): Long = try {
        val path = Paths.get(getDataDirectory())
        val parent = path.parent ?: Paths.get(System.getProperty("user.home"))
        val store = Files.getFileStore(if (Files.exists(path)) path else parent)
        store.usableSpace
    } catch (e: Exception) {
        0L
    }
}
