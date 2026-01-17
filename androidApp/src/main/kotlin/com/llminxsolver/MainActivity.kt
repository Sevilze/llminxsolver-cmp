package com.llminxsolver

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import com.llminxsolver.data.initDataStore
import com.llminxsolver.platform.MemoryMonitor
import com.llminxsolver.platform.StorageManager
import com.llminxsolver.util.initializeFileExport

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        MemoryMonitor.initialize(this)
        StorageManager.initialize(this)
        initializeFileExport(this)
        initDataStore(this)

        val tablesDir = filesDir.resolve("pruning_tables")
        NativeLib.initialize(tablesDir.absolutePath)

        setContent {
            App()
        }
    }
}
