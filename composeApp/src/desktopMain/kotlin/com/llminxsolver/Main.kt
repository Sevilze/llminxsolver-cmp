package com.llminxsolver

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.llminxsolver.ui.App

fun main() = application {
    Window(
        onCloseRequest = ::exitApplication,
        title = "LLMinx Solver",
    ) {
        App()
    }
}
