package com.llminxsolver

import android.content.res.Configuration
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.foundation.ScrollState
import androidx.compose.foundation.gestures.awaitEachGesture
import androidx.compose.foundation.gestures.awaitFirstDown
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
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
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.platform.LocalDensity
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
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

private const val SOLUTIONS_LIST_HEIGHT = 400
private const val TABLET_MIN_WIDTH_DP = 600

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
actual fun MainScreen(viewModel: SolverViewModel) {
    val state = rememberMainScreenState(viewModel)
    val actions = rememberMainScreenActions(viewModel)
    var showStorageSettings by remember { mutableStateOf(false) }
    val snackbarHostState = remember { SnackbarHostState() }
    val scope = rememberCoroutineScope()

    val scrollState = rememberScrollState()
    var solutionsPanelExpanded by remember { mutableStateOf(false) }
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
    val density = LocalDensity.current

    val configuration = LocalConfiguration.current
    val isTablet = (
        configuration.screenWidthDp >= TABLET_MIN_WIDTH_DP ||
            configuration.orientation == Configuration.ORIENTATION_LANDSCAPE
        ) &&
        (configuration.screenWidthDp >= 480)

    LaunchedEffect(solutionsPanelExpanded) {
        if (solutionsPanelExpanded && !isTablet) {
            delay(100)
            val panelHeightPx = with(density) { 460.dp.toPx() }.toInt()
            val targetScroll = scrollState.maxValue + panelHeightPx
            scrollState.animateScrollTo(
                value = targetScroll,
                animationSpec = spring(
                    dampingRatio = Spring.DampingRatioNoBouncy,
                    stiffness = Spring.StiffnessMedium
                )
            )
        }
    }

    Scaffold(
        modifier = Modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
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
                ),
                scrollBehavior = scrollBehavior
            )
        },
        bottomBar = {
            if (!isTablet) {
                StatusBar(
                    solverState = state.solverState,
                    modifier = Modifier.navigationBarsPadding()
                )
            }
        }
    ) { paddingValues ->
        if (isTablet) {
            // Tablet/Desktop layout - side by side
            TabletLayout(
                state = state,
                actions = actions,
                paddingValues = paddingValues,
                snackbarHostState = snackbarHostState,
                scope = scope,
                onSolutionsExpandChange = { solutionsPanelExpanded = it }
            )
        } else {
            // Phone layout - vertical scrolling
            PhoneLayout(
                state = state,
                actions = actions,
                paddingValues = paddingValues,
                scrollState = scrollState,
                snackbarHostState = snackbarHostState,
                scope = scope,
                onSolutionsExpandChange = { solutionsPanelExpanded = it }
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

@Composable
private fun PhoneLayout(
    state: MainScreenState,
    actions: MainScreenActions,
    paddingValues: PaddingValues,
    scrollState: ScrollState,
    snackbarHostState: SnackbarHostState,
    scope: CoroutineScope,
    onSolutionsExpandChange: (Boolean) -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(paddingValues)
            .padding(horizontal = 16.dp)
            .verticalScroll(scrollState),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Spacer(modifier = Modifier.padding(top = 8.dp))

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
                Box(
                    modifier = Modifier
                        .fillMaxWidth(0.8f)
                        .pointerInput(Unit) {
                            awaitEachGesture {
                                awaitFirstDown()
                            }
                        }
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
                        modifier = Modifier.fillMaxWidth()
                    )
                }
                Spacer(modifier = Modifier.padding(vertical = 8.dp))
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
            listHeight = SOLUTIONS_LIST_HEIGHT,
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
            defaultCollapsed = true,
            listHeight = SOLUTIONS_LIST_HEIGHT,
            onExpandChange = onSolutionsExpandChange
        )

        Spacer(modifier = Modifier.padding(bottom = 8.dp))
    }
}

@Composable
private fun TabletLayout(
    state: MainScreenState,
    actions: MainScreenActions,
    paddingValues: PaddingValues,
    snackbarHostState: SnackbarHostState,
    scope: CoroutineScope,
    onSolutionsExpandChange: (Boolean) -> Unit
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
                .width(420.dp)
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
                    Column(
                        modifier = Modifier.padding(16.dp),
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {
                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .pointerInput(Unit) {
                                    awaitEachGesture {
                                        awaitFirstDown()
                                    }
                                }
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
                                modifier = Modifier.fillMaxWidth()
                            )
                        }
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
            }

            StatusBar(
                solverState = state.solverState,
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
            ScoredSolutionsPanel(
                scoredSolutions = state.scoredSolutions,
                tempFilePath = state.tempFilePath,
                megaminxState = state.megaminxState,
                colorScheme = state.megaminxColorScheme,
                ignoreFlags = state.solverConfig.ignoreFlags,
                metricLabel = getMetricLabel(state.solverConfig.metric),
                onFlushTempFile = actions.flushTempFile,
                onExportSuccess = { filename ->
                    scope.launch {
                        snackbarHostState.showSnackbar("Exported $filename to Downloads")
                    }
                },
                modifier = Modifier.weight(1f)
            )

            SolutionsPanel(
                solverState = state.solverState,
                readSolutionsPage = actions.readSolutionsPage,
                tempFilePath = state.tempFilePath,
                defaultCollapsed = true,
                onExpandChange = onSolutionsExpandChange
            )
        }
    }
}
