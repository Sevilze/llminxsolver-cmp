package com.llminxsolver

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.theme.LLMinxTheme
import com.llminxsolver.viewmodel.SolverViewModel

@Composable
fun App() {
    val viewModel = remember { SolverViewModel() }
    val wallpaperPath by viewModel.wallpaperPath.collectAsState()
    val themeMode by viewModel.themeMode.collectAsState()
    val schemeType by viewModel.schemeType.collectAsState()
    val dynamicColorMode by viewModel.dynamicColorMode.collectAsState()
    val systemDarkTheme = isSystemInDarkTheme()

    val isDarkTheme = when (themeMode) {
        ThemeMode.Dark -> true
        ThemeMode.Light -> false
        ThemeMode.System -> systemDarkTheme
    }

    LLMinxTheme(
        darkTheme = isDarkTheme,
        wallpaperPath = wallpaperPath,
        schemeType = schemeType,
        dynamicColorMode = dynamicColorMode
    ) {
        Surface(modifier = Modifier.fillMaxSize()) {
            MainScreen(viewModel)
        }
    }
}
