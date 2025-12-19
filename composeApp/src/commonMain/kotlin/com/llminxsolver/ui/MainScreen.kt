package com.llminxsolver.ui

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import com.llminxsolver.viewmodel.SolverViewModel

data class MainScreenState(
    val megaminxState: MegaminxState,
    val solverConfig: SolverConfig,
    val solverState: SolverState,
    val scoredSolutions: List<ScoredSolution>
)

data class MainScreenActions(
    val onSwapCorners: (Int, Int) -> Unit,
    val onRotateCorner: (Int, Int) -> Unit,
    val onSwapEdges: (Int, Int) -> Unit,
    val onFlipEdge: (Int) -> Unit,
    val onAllowedFacesChange: (com.llminxsolver.data.AllowedFacesMode) -> Unit,
    val onMetricChange: (MetricType) -> Unit,
    val onLimitDepthChange: (Boolean) -> Unit,
    val onMaxDepthChange: (Int) -> Unit,
    val onIgnoreFlagChange: (String, Boolean) -> Unit,
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

    return MainScreenState(
        megaminxState = megaminxState,
        solverConfig = solverConfig,
        solverState = solverState,
        scoredSolutions = scoredSolutions
    )
}

@Composable
fun rememberMainScreenActions(viewModel: SolverViewModel): MainScreenActions = MainScreenActions(
    onSwapCorners = viewModel::swapCorners,
    onRotateCorner = viewModel::rotateCorner,
    onSwapEdges = viewModel::swapEdges,
    onFlipEdge = viewModel::flipEdge,
    onAllowedFacesChange = viewModel::setAllowedFaces,
    onMetricChange = viewModel::setMetric,
    onLimitDepthChange = viewModel::setLimitDepth,
    onMaxDepthChange = viewModel::setMaxDepth,
    onIgnoreFlagChange = viewModel::setIgnoreFlag,
    onReset = viewModel::reset,
    onSolve = viewModel::solve,
    onCancel = viewModel::cancelSolve
)

fun getMetricLabel(metric: MetricType): String = when (metric) {
    MetricType.FTM -> "FTM"
    MetricType.FFTM -> "FFTM"
}
