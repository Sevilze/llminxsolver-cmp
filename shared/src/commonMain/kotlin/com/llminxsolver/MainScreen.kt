package com.llminxsolver

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import com.llminxsolver.data.DynamicColorMode
import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.SchemeType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.viewmodel.SolverViewModel

data class MainScreenState(
    val megaminxState: MegaminxState,
    val solverConfig: SolverConfig,
    val solverState: SolverState,
    val scoredSolutions: List<ScoredSolution>,
    val tempFilePath: String?,
    val memoryInfo: MemoryInfo?,
    val availableCpus: Int,
    val megaminxColorScheme: MegaminxColorScheme,
    val skipDeletionWarning: Boolean,
    val wallpaperPath: String?,
    val dynamicColorMode: DynamicColorMode,
    val themeMode: ThemeMode,
    val schemeType: SchemeType
)

data class MainScreenActions(
    val onSwapCorners: (Int, Int) -> Unit,
    val onRotateCorner: (Int, Int) -> Unit,
    val onSwapEdges: (Int, Int) -> Unit,
    val onFlipEdge: (Int) -> Unit,
    val onSelectedModesChange: (Set<GeneratorMode>) -> Unit,
    val onMetricChange: (MetricType) -> Unit,
    val onLimitSearchDepthChange: (Boolean) -> Unit,
    val onMaxSearchDepthChange: (Int) -> Unit,
    val onPruningDepthChange: (Int) -> Unit,
    val onModePruningDepthChange: (GeneratorMode, Int?) -> Unit,
    val onParallelConfigChange: (ParallelConfig) -> Unit,
    val onIgnoreFlagChange: (String, Boolean) -> Unit,
    val onMegaminxColorSchemeChange: (MegaminxColorScheme) -> Unit,
    val onSkipDeletionWarningChange: (Boolean) -> Unit,
    val onWallpaperPathChange: (String?) -> Unit,
    val onDynamicColorModeChange: (DynamicColorMode) -> Unit,
    val onSchemeTypeChange: (SchemeType) -> Unit,
    val onThemeModeChange: (ThemeMode) -> Unit,
    val onReset: () -> Unit,
    val onSolve: () -> Unit,
    val onCancel: () -> Unit,
    val readSolutionsPage: (Int, Int) -> List<String>,
    val flushTempFile: () -> Unit
)

@Composable
expect fun MainScreen(viewModel: SolverViewModel = remember { SolverViewModel() })

@Composable
fun rememberMainScreenState(viewModel: SolverViewModel): MainScreenState {
    val megaminxState by viewModel.megaminxState.collectAsState()
    val solverConfig by viewModel.solverConfig.collectAsState()
    val solverState by viewModel.solverState.collectAsState()
    val scoredSolutions by viewModel.scoredSolutions.collectAsState()
    val tempFilePath by viewModel.tempFilePath.collectAsState()
    val memoryInfo by viewModel.memoryInfo.collectAsState()
    val availableCpus by viewModel.availableCpus.collectAsState()
    val megaminxColorScheme by viewModel.megaminxColorScheme.collectAsState()
    val skipDeletionWarning by viewModel.skipDeletionWarning.collectAsState()
    val wallpaperPath by viewModel.wallpaperPath.collectAsState()
    val dynamicColorMode by viewModel.dynamicColorMode.collectAsState()
    val schemeType by viewModel.schemeType.collectAsState()
    val themeMode by viewModel.themeMode.collectAsState()

    return MainScreenState(
        megaminxState = megaminxState,
        solverConfig = solverConfig,
        solverState = solverState,
        scoredSolutions = scoredSolutions,
        tempFilePath = tempFilePath,
        memoryInfo = memoryInfo,
        availableCpus = availableCpus,
        megaminxColorScheme = megaminxColorScheme,
        skipDeletionWarning = skipDeletionWarning,
        wallpaperPath = wallpaperPath,
        dynamicColorMode = dynamicColorMode,
        schemeType = schemeType,
        themeMode = themeMode
    )
}

@Composable
fun rememberMainScreenActions(viewModel: SolverViewModel): MainScreenActions = MainScreenActions(
    onSwapCorners = viewModel::swapCorners,
    onRotateCorner = viewModel::rotateCorner,
    onSwapEdges = viewModel::swapEdges,
    onFlipEdge = viewModel::flipEdge,
    onSelectedModesChange = viewModel::setSelectedModes,
    onMetricChange = viewModel::setMetric,
    onLimitSearchDepthChange = viewModel::setLimitSearchDepth,
    onMaxSearchDepthChange = viewModel::setMaxSearchDepth,
    onPruningDepthChange = viewModel::setPruningDepth,
    onModePruningDepthChange = viewModel::setModePruningDepth,
    onParallelConfigChange = viewModel::setParallelConfig,
    onIgnoreFlagChange = viewModel::setIgnoreFlag,
    onMegaminxColorSchemeChange = viewModel::setMegaminxColorScheme,
    onSkipDeletionWarningChange = viewModel::setSkipDeletionWarning,
    onWallpaperPathChange = viewModel::setWallpaperPath,
    onDynamicColorModeChange = viewModel::setDynamicColorMode,
    onSchemeTypeChange = viewModel::setSchemeType,
    onThemeModeChange = viewModel::setThemeMode,
    onReset = viewModel::reset,
    onSolve = viewModel::solve,
    onCancel = viewModel::cancelSolve,
    readSolutionsPage = viewModel::readSolutionsPage,
    flushTempFile = viewModel::flushTempFile
)

fun getMetricLabel(metric: MetricType): String = when (metric) {
    MetricType.FTM -> "FTM"
    MetricType.FFTM -> "FFTM"
}
