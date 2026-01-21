package com.llminxsolver

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.theme.LLMinxTheme
import com.llminxsolver.ui.SplashScreen
import com.llminxsolver.viewmodel.SolverViewModel
import kotlinx.coroutines.delay

private const val MIN_SPLASH_DURATION_MS = 1500L

@Composable
fun App() {
    val viewModel = remember { SolverViewModel() }
    val wallpaperPath by viewModel.wallpaperPath.collectAsState()
    val themeMode by viewModel.themeMode.collectAsState()
    val schemeType by viewModel.schemeType.collectAsState()
    val dynamicColorMode by viewModel.dynamicColorMode.collectAsState()
    val settingsLoaded by viewModel.settingsLoaded.collectAsState()
    val systemDarkTheme = isSystemInDarkTheme()

    var minDurationElapsed by remember { mutableStateOf(false) }
    LaunchedEffect(Unit) {
        delay(MIN_SPLASH_DURATION_MS)
        minDurationElapsed = true
    }

    val showSplash = !settingsLoaded || !minDurationElapsed

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
            AnimatedContent(
                targetState = showSplash,
                transitionSpec = {
                    fadeIn() togetherWith fadeOut()
                },
                label = "splashTransition"
            ) { isSplash ->
                if (isSplash) {
                    SplashScreen()
                } else {
                    MainScreen(viewModel)
                }
            }
        }
    }
}
