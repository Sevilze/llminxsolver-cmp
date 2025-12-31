package com.llminxsolver.ui.screens

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.viewmodel.SolverViewModel

data class MainScreenState(
    val megaminxState: MegaminxState,
    val solverConfig: SolverConfig,
    val solverState: SolverState,
    val scoredSolutions: List<ScoredSolution>,
    val memoryInfo: MemoryInfo?,
    val availableCpus: Int,
    val megaminxColorScheme: MegaminxColorScheme,
    val skipDeletionWarning: Boolean
)

data class MainScreenActions(
    val onSwapCorners: (Int, Int) -> Unit,
    val onRotateCorner: (Int, Int) -> Unit,
    val onSwapEdges: (Int, Int) -> Unit,
    val onFlipEdge: (Int) -> Unit,
    val onSelectedModesChange: (Set<GeneratorMode>) -> Unit,
    val onMetricChange: (MetricType) -> Unit,
    val onLimitDepthChange: (Boolean) -> Unit,
    val onMaxDepthChange: (Int) -> Unit,
    val onParallelConfigChange: (ParallelConfig) -> Unit,
    val onIgnoreFlagChange: (String, Boolean) -> Unit,
    val onMegaminxColorSchemeChange: (MegaminxColorScheme) -> Unit,
    val onSkipDeletionWarningChange: (Boolean) -> Unit,
    val onReset: () -> Unit,
    val onSolve: () -> Unit,
    val onCancel: () -> Unit
)

@Composable
expect fun MainScreen(viewModel: SolverViewModel = remember { SolverViewModel() })

@Composable
fun rememberMainScreenState(viewModel: SolverViewModel): MainScreenState {
    val megaminxState by viewModel.megaminxState.collectAsState()
    val solverConfig by viewModel.solverConfig.collectAsState()
    val solverState by viewModel.solverState.collectAsState()
    val scoredSolutions by viewModel.scoredSolutions.collectAsState()
    val memoryInfo by viewModel.memoryInfo.collectAsState()
    val availableCpus by viewModel.availableCpus.collectAsState()
    val megaminxColorScheme by viewModel.megaminxColorScheme.collectAsState()
    val skipDeletionWarning by viewModel.skipDeletionWarning.collectAsState()

    return MainScreenState(
        megaminxState = megaminxState,
        solverConfig = solverConfig,
        solverState = solverState,
        scoredSolutions = scoredSolutions,
        memoryInfo = memoryInfo,
        availableCpus = availableCpus,
        megaminxColorScheme = megaminxColorScheme,
        skipDeletionWarning = skipDeletionWarning
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
    onLimitDepthChange = viewModel::setLimitDepth,
    onMaxDepthChange = viewModel::setMaxDepth,
    onParallelConfigChange = viewModel::setParallelConfig,
    onIgnoreFlagChange = viewModel::setIgnoreFlag,
    onMegaminxColorSchemeChange = viewModel::setMegaminxColorScheme,
    onSkipDeletionWarningChange = viewModel::setSkipDeletionWarning,
    onReset = viewModel::reset,
    onSolve = viewModel::solve,
    onCancel = viewModel::cancelSolve
)

fun getMetricLabel(metric: MetricType): String = when (metric) {
    MetricType.FTM -> "FTM"
    MetricType.FFTM -> "FFTM"
}
