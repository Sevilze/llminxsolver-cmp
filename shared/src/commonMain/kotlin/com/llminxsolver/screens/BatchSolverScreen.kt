package com.llminxsolver.screens

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.BatchSolverConfig
import com.llminxsolver.data.BatchSolverState
import com.llminxsolver.data.toIgnoreFlags
import com.llminxsolver.data.toSolverState
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.components.BatchStateNavigator
import com.llminxsolver.ui.components.StatusBar
import com.llminxsolver.ui.dialogs.SettingsDialog
import com.llminxsolver.ui.panels.BatchControlPanel
import com.llminxsolver.ui.panels.BatchResultsPanel
import com.llminxsolver.ui.panels.BatchSolutionsPanel
import com.llminxsolver.viewmodel.BatchSolverViewModel

private const val COMPACT_MAX_WIDTH_DP = 600
private const val SIDEBAR_WIDTH_DP = 420

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun BatchSolverScreen(viewModel: BatchSolverViewModel, onNavigateBack: () -> Unit) {
    val config by viewModel.config.collectAsState()
    val state by viewModel.state.collectAsState()
    val colorScheme by viewModel.megaminxColorScheme.collectAsState()

    var showSettings by remember { mutableStateOf(false) }
    val snackbarHostState = remember { SnackbarHostState() }
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()

    Scaffold(
        modifier = Modifier.fillMaxSize(),
        snackbarHost = { SnackbarHost(snackbarHostState) },
        topBar = {
            CenterAlignedTopAppBar(
                title = {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            text = "Batch Solver",
                            fontWeight = FontWeight.Bold
                        )
                        Text(
                            text = "Generate and solve multiple cases",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = "Back"
                        )
                    }
                },
                actions = {
                    IconButton(onClick = { showSettings = true }) {
                        Icon(
                            imageVector = Icons.Default.Settings,
                            contentDescription = "Settings"
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainer
                ),
                scrollBehavior = scrollBehavior
            )
        }
    ) { paddingValues ->
        BoxWithConstraints(
            modifier = Modifier.fillMaxSize()
        ) {
            val isCompact = maxWidth < COMPACT_MAX_WIDTH_DP.dp

            if (isCompact) {
                CompactBatchLayout(
                    viewModel = viewModel,
                    config = config,
                    state = state,
                    colorScheme = colorScheme,
                    paddingValues = paddingValues
                )
            } else {
                ExpandedBatchLayout(
                    viewModel = viewModel,
                    config = config,
                    state = state,
                    colorScheme = colorScheme,
                    paddingValues = paddingValues
                )
            }
        }
    }

    if (showSettings) {
        SettingsDialog(
            onDismiss = { showSettings = false },
            parallelConfig = config.parallelConfig,
            pruningDepth = config.pruningDepth,
            memoryInfo = viewModel.memoryInfo.collectAsState().value,
            availableCpus = viewModel.availableCpus.collectAsState().value,
            onParallelConfigChange = viewModel::setParallelConfig,
            onPruningDepthChange = viewModel::setPruningDepth,
            megaminxColorScheme = colorScheme,
            onMegaminxColorSchemeChange = {},
            skipDeletionWarning = false,
            onSkipDeletionWarningChange = {}
        )
    }
}

@Composable
private fun CompactBatchLayout(
    viewModel: BatchSolverViewModel,
    config: BatchSolverConfig,
    state: BatchSolverState,
    colorScheme: MegaminxColorScheme,
    paddingValues: PaddingValues
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(paddingValues)
            .padding(horizontal = 16.dp)
            .verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Spacer(modifier = Modifier.height(8.dp))

        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerLow
            )
        ) {
            BatchStateNavigator(
                states = state.generatedStates,
                currentIndex = state.currentStateIndex,
                onIndexChange = viewModel::setCurrentStateIndex,
                colorScheme = colorScheme,
                ignoreFlags = config.toIgnoreFlags(),
                onIgnoreFlagChange = viewModel::setIgnoreFlag,
                enabled = !state.isSearching && !state.isGenerating,
                modifier = Modifier.padding(16.dp)
            )
        }

        Card(
            modifier = Modifier.fillMaxWidth().heightIn(max = 500.dp),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerLow
            )
        ) {
            BatchControlPanel(
                config = config,
                state = state,
                onScrambleChange = viewModel::setScramble,
                onEquivalencesChange = viewModel::setEquivalences,
                onPreAdjustChange = viewModel::setPreAdjust,
                onPostAdjustChange = viewModel::setPostAdjust,
                onSearchModeChange = viewModel::setSearchMode,
                onPruningDepthChange = viewModel::setPruningDepth,
                onSearchDepthChange = viewModel::setSearchDepth,
                onStopAfterFirstChange = viewModel::setStopAfterFirst,
                onIgnoreCornerPermutationChange = viewModel::setIgnoreCornerPermutation,
                onIgnoreEdgePermutationChange = viewModel::setIgnoreEdgePermutation,
                onIgnoreCornerOrientationChange = viewModel::setIgnoreCornerOrientation,
                onIgnoreEdgeOrientationChange = viewModel::setIgnoreEdgeOrientation,
                onGenerateStates = viewModel::generateStates,
                onStartSolve = viewModel::startSolve,
                onCancelSolve = viewModel::cancelSolve,
                onReset = viewModel::reset,
                modifier = Modifier.padding(16.dp)
            )
        }

        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerLow
            )
        ) {
            BatchResultsPanel(
                results = state.results,
                currentCaseIndex = state.currentStateIndex,
                modifier = Modifier.padding(16.dp),
                listHeight = 300
            )
        }

        Spacer(modifier = Modifier.height(8.dp))
    }
}

@Composable
private fun ExpandedBatchLayout(
    viewModel: BatchSolverViewModel,
    config: BatchSolverConfig,
    state: BatchSolverState,
    colorScheme: MegaminxColorScheme,
    paddingValues: PaddingValues
) {
    Row(
        modifier = Modifier
            .fillMaxSize()
            .padding(paddingValues)
            .padding(16.dp),
        horizontalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Column(
            modifier = Modifier
                .width(SIDEBAR_WIDTH_DP.dp)
                .fillMaxHeight()
        ) {
            Column(
                modifier = Modifier
                    .weight(1f)
                    .verticalScroll(rememberScrollState()),
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                    )
                ) {
                    BatchStateNavigator(
                        states = state.generatedStates,
                        currentIndex = state.currentStateIndex,
                        onIndexChange = viewModel::setCurrentStateIndex,
                        colorScheme = colorScheme,
                        ignoreFlags = config.toIgnoreFlags(),
                        onIgnoreFlagChange = viewModel::setIgnoreFlag,
                        enabled = !state.isSearching && !state.isGenerating,
                        modifier = Modifier.padding(16.dp)
                    )
                }

                Card(
                    modifier = Modifier.fillMaxWidth().heightIn(max = 600.dp),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                    )
                ) {
                    BatchControlPanel(
                        config = config,
                        state = state,
                        onScrambleChange = viewModel::setScramble,
                        onEquivalencesChange = viewModel::setEquivalences,
                        onPreAdjustChange = viewModel::setPreAdjust,
                        onPostAdjustChange = viewModel::setPostAdjust,
                        onSearchModeChange = viewModel::setSearchMode,
                        onPruningDepthChange = viewModel::setPruningDepth,
                        onSearchDepthChange = viewModel::setSearchDepth,
                        onStopAfterFirstChange = viewModel::setStopAfterFirst,
                        onIgnoreCornerPermutationChange = viewModel::setIgnoreCornerPermutation,
                        onIgnoreEdgePermutationChange = viewModel::setIgnoreEdgePermutation,
                        onIgnoreCornerOrientationChange = viewModel::setIgnoreCornerOrientation,
                        onIgnoreEdgeOrientationChange = viewModel::setIgnoreEdgeOrientation,
                        onGenerateStates = viewModel::generateStates,
                        onStartSolve = viewModel::startSolve,
                        onCancelSolve = viewModel::cancelSolve,
                        onReset = viewModel::reset,
                        modifier = Modifier.padding(16.dp)
                    )
                }
            }

            StatusBar(
                solverState = state.toSolverState(),
                expandUp = true,
                modifier = Modifier.padding(top = 16.dp)
            )
        }

        Column(
            modifier = Modifier
                .weight(1f)
                .fillMaxHeight(),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Card(
                modifier = Modifier.weight(1f).fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                )
            ) {
                BatchResultsPanel(
                    results = state.results,
                    currentCaseIndex = state.currentStateIndex,
                    modifier = Modifier.padding(16.dp)
                )
            }
            Card(
                modifier = Modifier.weight(1f).fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceContainerLow
                )
            ) {
                BatchSolutionsPanel(
                    caseResults = state.results?.caseResults ?: emptyList(),
                    metric = config.metric,
                    selectedCaseNumber = if (state.generatedStates.isNotEmpty()) {
                        state.currentStateIndex + 1
                    } else {
                        null
                    },
                    modifier = Modifier.padding(vertical = 8.dp)
                )
            }
        }
    }
}
