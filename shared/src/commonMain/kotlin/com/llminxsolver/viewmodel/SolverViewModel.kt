package com.llminxsolver.viewmodel

import com.llminxsolver.NativeLib
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
import com.llminxsolver.platform.MemoryMonitor
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import uniffi.llminxsolver.getAvailableCpus
import uniffi.llminxsolver.getAvailableMemoryMb

class SolverViewModel {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val memoryMonitor = MemoryMonitor()

    private val settingsViewModel = SettingsViewModel(scope)
    private val megaminxViewModel = MegaminxViewModel()
    private val solverOperations = SolverOperations(scope)

    private val _solverConfig = MutableStateFlow(SolverConfig())
    val solverConfig: StateFlow<SolverConfig> = _solverConfig.asStateFlow()

    private val _memoryInfo = MutableStateFlow<MemoryInfo?>(null)
    val memoryInfo: StateFlow<MemoryInfo?> = _memoryInfo.asStateFlow()

    private val _availableCpus = MutableStateFlow(4)
    val availableCpus: StateFlow<Int> = _availableCpus.asStateFlow()

    val megaminxState: StateFlow<MegaminxState> = megaminxViewModel.megaminxState
    val solverState: StateFlow<SolverState> = solverOperations.solverState
    val scoredSolutions: StateFlow<List<ScoredSolution>> = solverOperations.scoredSolutions
    val tempFilePath: StateFlow<String?> = solverOperations.tempFilePath
    val megaminxColorScheme: StateFlow<MegaminxColorScheme> = settingsViewModel.megaminxColorScheme
    val skipDeletionWarning: StateFlow<Boolean> = settingsViewModel.skipDeletionWarning
    val wallpaperPath: StateFlow<String?> = settingsViewModel.wallpaperPath
    val dynamicColorMode: StateFlow<DynamicColorMode> = settingsViewModel.dynamicColorMode
    val schemeType: StateFlow<SchemeType> = settingsViewModel.schemeType
    val themeMode: StateFlow<ThemeMode> = settingsViewModel.themeMode
    val settingsLoaded: StateFlow<Boolean> = settingsViewModel.isLoaded

    init {
        NativeLib.ensureLoaded()
        uniffi.llminxsolver.cleanupStaleTempFiles()
        initializePlatformInfo()
        startMemoryMonitoring()
        settingsViewModel.collectSettings { memoryBudgetMb, tableGenThreads, searchThreads ->
            _solverConfig.update { config ->
                config.copy(
                    parallelConfig = config.parallelConfig.copy(
                        memoryBudgetMb = memoryBudgetMb,
                        tableGenThreads = tableGenThreads,
                        searchThreads = searchThreads
                    )
                )
            }
        }
    }

    private fun initializePlatformInfo() {
        try {
            _availableCpus.value = getAvailableCpus().toInt()
            val availableMemory = getAvailableMemoryMb().toInt()
            val defaultConfig = ParallelConfig.forDesktop(_availableCpus.value, availableMemory)
            _solverConfig.update { it.copy(parallelConfig = defaultConfig) }
        } catch (e: Exception) {
            _availableCpus.value = 4
        }
    }

    private fun startMemoryMonitoring() {
        memoryMonitor.startMonitoring(2000L) { info ->
            _memoryInfo.value = info
        }
    }

    fun setSelectedModes(modes: Set<GeneratorMode>) {
        _solverConfig.update {
            it.copy(
                selectedModes = modes,
                generatorMode = modes.firstOrNull() ?: GeneratorMode.R_U
            )
        }
    }

    fun setGeneratorModes(mode: GeneratorMode) {
        _solverConfig.update {
            it.copy(
                generatorMode = mode,
                selectedModes = setOf(mode)
            )
        }
    }

    fun setMetric(metric: MetricType) {
        _solverConfig.update { it.copy(metric = metric) }
    }

    fun setLimitDepth(limit: Boolean) {
        _solverConfig.update { it.copy(limitDepth = limit) }
    }

    fun setMaxDepth(depth: Int) {
        _solverConfig.update { it.copy(maxDepth = depth) }
    }

    fun setParallelConfig(config: ParallelConfig) {
        _solverConfig.update { it.copy(parallelConfig = config) }
        settingsViewModel.saveParallelConfig(config)
    }

    fun setMegaminxColorScheme(scheme: MegaminxColorScheme) {
        settingsViewModel.setMegaminxColorScheme(scheme)
    }

    fun setSkipDeletionWarning(skip: Boolean) {
        settingsViewModel.setSkipDeletionWarning(skip)
    }

    fun setWallpaperPath(path: String?) {
        settingsViewModel.setWallpaperPath(path)
    }

    fun setDynamicColorMode(mode: DynamicColorMode) {
        settingsViewModel.setDynamicColorMode(mode)
    }

    fun setSchemeType(type: SchemeType) {
        settingsViewModel.setSchemeType(type)
    }

    fun setThemeMode(mode: ThemeMode) {
        settingsViewModel.setThemeMode(mode)
    }

    fun setIgnoreFlag(flag: String, value: Boolean) {
        _solverConfig.update { config ->
            val newFlags = when (flag) {
                "cornerPositions" -> config.ignoreFlags.copy(cornerPositions = value)
                "edgePositions" -> config.ignoreFlags.copy(edgePositions = value)
                "cornerOrientations" -> config.ignoreFlags.copy(cornerOrientations = value)
                "edgeOrientations" -> config.ignoreFlags.copy(edgeOrientations = value)
                else -> config.ignoreFlags
            }
            config.copy(ignoreFlags = newFlags)
        }
    }

    fun swapCorners(i: Int, j: Int) {
        megaminxViewModel.swapCorners(i, j)
    }

    fun rotateCorner(index: Int, direction: Int) {
        megaminxViewModel.rotateCorner(index, direction)
    }

    fun swapEdges(i: Int, j: Int) {
        megaminxViewModel.swapEdges(i, j)
    }

    fun flipEdge(index: Int) {
        megaminxViewModel.flipEdge(index)
    }

    fun reset() {
        megaminxViewModel.reset()
        solverOperations.reset()
    }

    fun solve() {
        solverOperations.solve(megaminxViewModel.currentState(), _solverConfig.value)
    }

    fun cancelSolve() = solverOperations.cancelSolve()

    fun flushTempFile() = solverOperations.flushTempFile()

    fun readSolutionsPage(offset: Int, limit: Int): List<String> =
        solverOperations.readSolutionsPage(offset, limit)

    fun onCleared() {
        cancelSolve()
        memoryMonitor.stopMonitoring()
    }
}
