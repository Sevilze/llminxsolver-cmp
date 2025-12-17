package com.llminxsolver.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.runtime.remember
import com.llminxsolver.viewmodel.SolverViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
    viewModel: SolverViewModel = remember { SolverViewModel() }
) {
    val megaminxState by viewModel.megaminxState.collectAsState()
    val solverConfig by viewModel.solverConfig.collectAsState()
    val solverState by viewModel.solverState.collectAsState()
    val scoredSolutions by viewModel.scoredSolutions.collectAsState()

    Scaffold(
        topBar = {
            CenterAlignedTopAppBar(
                title = {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            text = "LLMinx Solver",
                            fontWeight = FontWeight.Bold
                        )
                        Text(
                            text = "Last Layer Megaminx Algorithm Finder",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                },
                colors = TopAppBarDefaults.centerAlignedTopAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainer
                )
            )
        },
        bottomBar = {
            StatusBar(solverState = solverState)
        }
    ) { paddingValues ->
        BoxWithConstraints(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            val isCompact = maxWidth < 900.dp

            if (isCompact) {
                CompactLayout(
                    viewModel = viewModel,
                    megaminxState = megaminxState,
                    solverConfig = solverConfig,
                    solverState = solverState,
                    scoredSolutions = scoredSolutions
                )
            } else {
                ExpandedLayout(
                    viewModel = viewModel,
                    megaminxState = megaminxState,
                    solverConfig = solverConfig,
                    solverState = solverState,
                    scoredSolutions = scoredSolutions
                )
            }
        }
    }
}

@Composable
private fun CompactLayout(
    viewModel: SolverViewModel,
    megaminxState: com.llminxsolver.data.MegaminxState,
    solverConfig: com.llminxsolver.data.SolverConfig,
    solverState: com.llminxsolver.data.SolverState,
    scoredSolutions: List<com.llminxsolver.data.ScoredSolution>
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerLow
            )
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(
                    text = "Starting Position",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.height(8.dp))
                MegaminxViewer(
                    puzzleState = megaminxState,
                    ignoreFlags = solverConfig.ignoreFlags,
                    onSwapCorners = viewModel::swapCorners,
                    onRotateCorner = viewModel::rotateCorner,
                    onSwapEdges = viewModel::swapEdges,
                    onFlipEdge = viewModel::flipEdge,
                    enabled = !solverState.isSearching,
                    modifier = Modifier.fillMaxWidth(0.8f)
                )
                Spacer(modifier = Modifier.height(8.dp))
                IgnoreOptions(
                    flags = solverConfig.ignoreFlags,
                    onChange = viewModel::setIgnoreFlag,
                    enabled = !solverState.isSearching,
                    compact = true
                )
            }
        }

        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerLow
            )
        ) {
            ControlPanel(
                config = solverConfig,
                isSearching = solverState.isSearching,
                onAllowedFacesChange = viewModel::setAllowedFaces,
                onMetricChange = viewModel::setMetric,
                onLimitDepthChange = viewModel::setLimitDepth,
                onMaxDepthChange = viewModel::setMaxDepth,
                onIgnoreFlagChange = viewModel::setIgnoreFlag,
                onReset = viewModel::reset,
                onSolve = viewModel::solve,
                onCancel = viewModel::cancelSolve,
                modifier = Modifier.padding(16.dp)
            )
        }

        ScoredSolutionsPanel(
            scoredSolutions = scoredSolutions,
            metricLabel = if (solverConfig.metric == com.llminxsolver.data.MetricType.FTM) "FTM" else "FFTM",
            modifier = Modifier.height(300.dp)
        )

        SolutionsPanel(
            solverState = solverState,
            defaultCollapsed = true
        )
    }
}

@Composable
private fun ExpandedLayout(
    viewModel: SolverViewModel,
    megaminxState: com.llminxsolver.data.MegaminxState,
    solverConfig: com.llminxsolver.data.SolverConfig,
    solverState: com.llminxsolver.data.SolverState,
    scoredSolutions: List<com.llminxsolver.data.ScoredSolution>
) {
    Row(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Column(
            modifier = Modifier
                .width(320.dp)
                .fillMaxHeight()
                .verticalScroll(rememberScrollState()),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                )
            ) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        text = "Starting Position",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    MegaminxViewer(
                        puzzleState = megaminxState,
                        ignoreFlags = solverConfig.ignoreFlags,
                        onSwapCorners = viewModel::swapCorners,
                        onRotateCorner = viewModel::rotateCorner,
                        onSwapEdges = viewModel::swapEdges,
                        onFlipEdge = viewModel::flipEdge,
                        enabled = !solverState.isSearching,
                        modifier = Modifier.fillMaxWidth()
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    IgnoreOptions(
                        flags = solverConfig.ignoreFlags,
                        onChange = viewModel::setIgnoreFlag,
                        enabled = !solverState.isSearching,
                        compact = true
                    )
                }
            }

            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                )
            ) {
                ControlPanel(
                    config = solverConfig,
                    isSearching = solverState.isSearching,
                    onAllowedFacesChange = viewModel::setAllowedFaces,
                    onMetricChange = viewModel::setMetric,
                    onLimitDepthChange = viewModel::setLimitDepth,
                    onMaxDepthChange = viewModel::setMaxDepth,
                    onIgnoreFlagChange = viewModel::setIgnoreFlag,
                    onReset = viewModel::reset,
                    onSolve = viewModel::solve,
                    onCancel = viewModel::cancelSolve,
                    modifier = Modifier.padding(16.dp)
                )
            }
        }

        Column(
            modifier = Modifier
                .weight(1f)
                .fillMaxHeight(),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            ScoredSolutionsPanel(
                scoredSolutions = scoredSolutions,
                metricLabel = if (solverConfig.metric == com.llminxsolver.data.MetricType.FTM) "FTM" else "FFTM",
                modifier = Modifier.weight(1f)
            )

            SolutionsPanel(
                solverState = solverState,
                defaultCollapsed = true
            )
        }
    }
}
