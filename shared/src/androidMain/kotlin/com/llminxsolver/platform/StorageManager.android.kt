package com.llminxsolver.platform

import android.content.Context
import android.os.StatFs
import java.io.File

actual class StorageManager actual constructor() {

    actual fun getDataDirectory(): String =
        appContext?.filesDir?.resolve("pruning_tables")?.absolutePath ?: ""

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

    actual fun getAvailableStorage(): Long {
        val dataDir = File(getDataDirectory())
        val parent = if (dataDir.exists()) dataDir else appContext?.filesDir ?: return 0
        val statFs = StatFs(parent.absolutePath)
        return statFs.availableBytes
    }

    companion object {
        private var appContext: Context? = null

        fun initialize(context: Context) {
            appContext = context.applicationContext
        }
    }
}
