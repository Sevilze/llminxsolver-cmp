package com.llminxsolver

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.WindowPlacement
import androidx.compose.ui.window.application
import androidx.compose.ui.window.rememberWindowState
import java.io.File

fun main() {
    val userHome = System.getProperty("user.home")
    val tablesDir = File(userHome, ".llminxsolver/tables")
    NativeLib.initialize(tablesDir.absolutePath)

    application {
        val windowState = rememberWindowState(placement = WindowPlacement.Maximized)
        Window(
            onCloseRequest = ::exitApplication,
            title = "LLMinx Solver",
            state = windowState
        ) {
            App()
        }
    }
}
