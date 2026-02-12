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
import com.llminxsolver.screens.BatchSolverScreen
import com.llminxsolver.screens.SolverScreen
import com.llminxsolver.screens.SplashScreen
import com.llminxsolver.theme.LLMinxTheme
import com.llminxsolver.viewmodel.BatchSolverViewModel
import com.llminxsolver.viewmodel.SettingsViewModel
import com.llminxsolver.viewmodel.SolverViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay

private const val MIN_SPLASH_DURATION_MS = 1500L

enum class AppScreen {
    DEDICATED_SOLVER,
    BATCH_SOLVER
}

@Composable
fun App() {
    val viewModel = remember { SolverViewModel() }
    val sharedScope = remember { CoroutineScope(SupervisorJob() + Dispatchers.Default) }
    val sharedSettingsViewModel = remember { SettingsViewModel(sharedScope) }
    val batchViewModel = remember { BatchSolverViewModel(sharedSettingsViewModel) }

    val wallpaperPath by viewModel.wallpaperPath.collectAsState()
    val themeMode by viewModel.themeMode.collectAsState()
    val schemeType by viewModel.schemeType.collectAsState()
    val dynamicColorMode by viewModel.dynamicColorMode.collectAsState()
    val settingsLoaded by viewModel.settingsLoaded.collectAsState()
    val systemDarkTheme = isSystemInDarkTheme()

    var minDurationElapsed by remember { mutableStateOf(false) }
    var currentScreen by remember { mutableStateOf(AppScreen.DEDICATED_SOLVER) }

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
                targetState = if (showSplash) null else currentScreen,
                transitionSpec = {
                    fadeIn() togetherWith fadeOut()
                },
                label = "screenTransition"
            ) { screen ->
                when (screen) {
                    null -> SplashScreen()

                    AppScreen.DEDICATED_SOLVER -> SolverScreen(
                        viewModel = viewModel,
                        onNavigateToBatchSolver = { currentScreen = AppScreen.BATCH_SOLVER }
                    )

                    AppScreen.BATCH_SOLVER -> BatchSolverScreen(
                        viewModel = batchViewModel,
                        onNavigateBack = { currentScreen = AppScreen.DEDICATED_SOLVER }
                    )
                }
            }
        }
    }
}
