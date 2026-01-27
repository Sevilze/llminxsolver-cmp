package com.llminxsolver

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
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.llminxsolver.ui.components.IgnoreOptions
import com.llminxsolver.ui.components.StatusBar
import com.llminxsolver.ui.dialogs.SettingsDialog
import com.llminxsolver.ui.megaminx.MegaminxViewer
import com.llminxsolver.ui.panels.ControlPanel
import com.llminxsolver.ui.panels.ScoredSolutionsPanel
import com.llminxsolver.ui.panels.SolutionsPanel
import com.llminxsolver.viewmodel.SolverViewModel
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
actual fun MainScreen(viewModel: SolverViewModel) {
    val state = rememberMainScreenState(viewModel)
    val actions = rememberMainScreenActions(viewModel)
    var showStorageSettings by remember { mutableStateOf(false) }
    val snackbarHostState = remember { SnackbarHostState() }
    val scope = rememberCoroutineScope()

    Scaffold(
        snackbarHost = { SnackbarHost(snackbarHostState) },
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
                .padding(paddingValues)
                .padding(16.dp)
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
                    MegaminxViewer(
                        puzzleState = state.megaminxState,
                        ignoreFlags = state.solverConfig.ignoreFlags,
                        colorScheme = state.megaminxColorScheme,
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
                    onModePruningDepthChange = actions.onModePruningDepthChange,
                    onMetricChange = actions.onMetricChange,
                    onLimitSearchDepthChange = actions.onLimitSearchDepthChange,
                    onMaxSearchDepthChange = actions.onMaxSearchDepthChange,
                    onIgnoreFlagChange = actions.onIgnoreFlagChange,
                    onReset = actions.onReset,
                    onSolve = actions.onSolve,
                    onCancel = actions.onCancel,
                    modifier = Modifier.padding(16.dp)
                )
            }

            ScoredSolutionsPanel(
                scoredSolutions = state.scoredSolutions,
                tempFilePath = state.tempFilePath,
                megaminxState = state.megaminxState,
                colorScheme = state.megaminxColorScheme,
                ignoreFlags = state.solverConfig.ignoreFlags,
                metricLabel = getMetricLabel(state.solverConfig.metric),
                listHeight = 400,
                onFlushTempFile = actions.flushTempFile,
                onExportSuccess = { filename ->
                    scope.launch {
                        snackbarHostState.showSnackbar("Exported $filename to Downloads")
                    }
                }
            )

            SolutionsPanel(
                solverState = state.solverState,
                readSolutionsPage = actions.readSolutionsPage,
                tempFilePath = state.tempFilePath,
                defaultCollapsed = true
            )
        }
    }

    if (showStorageSettings) {
        SettingsDialog(
            onDismiss = { showStorageSettings = false },
            parallelConfig = state.solverConfig.parallelConfig,
            pruningDepth = state.solverConfig.pruningDepth,
            memoryInfo = state.memoryInfo,
            availableCpus = state.availableCpus,
            onParallelConfigChange = actions.onParallelConfigChange,
            onPruningDepthChange = actions.onPruningDepthChange,
            megaminxColorScheme = state.megaminxColorScheme,
            onMegaminxColorSchemeChange = actions.onMegaminxColorSchemeChange,
            skipDeletionWarning = state.skipDeletionWarning,
            onSkipDeletionWarningChange = actions.onSkipDeletionWarningChange,
            showDynamicColorModeConfig = true,
            dynamicColorMode = state.dynamicColorMode,
            onDynamicColorModeChange = actions.onDynamicColorModeChange,
            schemeType = state.schemeType,
            onSchemeTypeChange = actions.onSchemeTypeChange,
            themeMode = state.themeMode,
            onThemeModeChange = actions.onThemeModeChange
        )
    }
}
