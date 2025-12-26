package com.llminxsolver.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.llminxsolver.viewmodel.SolverViewModel

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
actual fun MainScreen(viewModel: SolverViewModel) {
    val state = rememberMainScreenState(viewModel)
    val actions = rememberMainScreenActions(viewModel)
    var showStorageSettings by remember { mutableStateOf(false) }

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
                actions = {
                    IconButton(onClick = { showStorageSettings = true }) {
                        Icon(
                            imageVector = Icons.Default.Settings,
                            contentDescription = "Settings"
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainer
                )
            )
        },
        bottomBar = {
            StatusBar(
                solverState = state.solverState,
                modifier = Modifier.navigationBarsPadding()
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(rememberScrollState())
                .padding(paddingValues)
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
                        puzzleState = state.megaminxState,
                        ignoreFlags = state.solverConfig.ignoreFlags,
                        onSwapCorners = actions.onSwapCorners,
                        onRotateCorner = actions.onRotateCorner,
                        onSwapEdges = actions.onSwapEdges,
                        onFlipEdge = actions.onFlipEdge,
                        enabled = !state.solverState.isSearching,
                        modifier = Modifier.fillMaxWidth(0.8f)
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    IgnoreOptions(
                        flags = state.solverConfig.ignoreFlags,
                        onChange = actions.onIgnoreFlagChange,
                        enabled = !state.solverState.isSearching,
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
                    config = state.solverConfig,
                    isSearching = state.solverState.isSearching,
                    onSelectedModesChange = actions.onSelectedModesChange,
                    onMetricChange = actions.onMetricChange,
                    onLimitDepthChange = actions.onLimitDepthChange,
                    onMaxDepthChange = actions.onMaxDepthChange,
                    onIgnoreFlagChange = actions.onIgnoreFlagChange,
                    onReset = actions.onReset,
                    onSolve = actions.onSolve,
                    onCancel = actions.onCancel,
                    modifier = Modifier.padding(16.dp)
                )
            }

            ScoredSolutionsPanel(
                scoredSolutions = state.scoredSolutions,
                metricLabel = getMetricLabel(state.solverConfig.metric),
                modifier = Modifier.height(300.dp)
            )

            SolutionsPanel(
                solverState = state.solverState,
                defaultCollapsed = true
            )
        }
    }

    if (showStorageSettings) {
        StorageSettingsDialog(
            onDismiss = { showStorageSettings = false },
            parallelConfig = state.solverConfig.parallelConfig,
            memoryInfo = state.memoryInfo,
            availableCpus = state.availableCpus,
            onParallelConfigChange = actions.onParallelConfigChange
        )
    }
}
