package com.llminxsolver

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.llminxsolver.data.initDataStore
import com.llminxsolver.platform.MemoryMonitor
import com.llminxsolver.platform.StorageManager

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        MemoryMonitor.initialize(this)
        StorageManager.initialize(this)
        initDataStore(this)

        val tablesDir = filesDir.resolve("pruning_tables")
        NativeLib.initialize(tablesDir.absolutePath)

        setContent {
            App()
        }
    }
}

@Preview
@Composable
fun AppAndroidPreview() {
    App()
}
