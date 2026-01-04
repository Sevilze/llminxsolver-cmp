package com.llminxsolver.viewmodel

import com.llminxsolver.NativeLib
import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import com.llminxsolver.data.createPlatformSettingsRepository
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.platform.MemoryMonitor
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import uniffi.llminxsolver.Metric
import uniffi.llminxsolver.ParallelSolverConfig
import uniffi.llminxsolver.ParallelSolverHandle
import uniffi.llminxsolver.ProgressEvent
import uniffi.llminxsolver.SearchMode
import uniffi.llminxsolver.SolverCallback
import uniffi.llminxsolver.SolverHandle
import uniffi.llminxsolver.calculateMcc
import uniffi.llminxsolver.getAvailableCpus
import uniffi.llminxsolver.getAvailableMemoryMb
import uniffi.llminxsolver.getMoveCount

class SolverViewModel {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val memoryMonitor = MemoryMonitor()
    private val settingsRepository = createPlatformSettingsRepository()

    private val _megaminxState = MutableStateFlow(MegaminxState())
    val megaminxState: StateFlow<MegaminxState> = _megaminxState.asStateFlow()

    private val _solverConfig = MutableStateFlow(SolverConfig())
    val solverConfig: StateFlow<SolverConfig> = _solverConfig.asStateFlow()

    private val _solverState = MutableStateFlow(SolverState())
    val solverState: StateFlow<SolverState> = _solverState.asStateFlow()

    private val _scoredSolutions = MutableStateFlow<List<ScoredSolution>>(emptyList())
    val scoredSolutions: StateFlow<List<ScoredSolution>> = _scoredSolutions.asStateFlow()

    private val _memoryInfo = MutableStateFlow<MemoryInfo?>(null)
    val memoryInfo: StateFlow<MemoryInfo?> = _memoryInfo.asStateFlow()

    private val _availableCpus = MutableStateFlow(4)
    val availableCpus: StateFlow<Int> = _availableCpus.asStateFlow()

    private val _megaminxColorScheme = MutableStateFlow(MegaminxColorScheme())
    val megaminxColorScheme: StateFlow<MegaminxColorScheme> = _megaminxColorScheme.asStateFlow()

    private val _skipDeletionWarning = MutableStateFlow(false)
    val skipDeletionWarning: StateFlow<Boolean> = _skipDeletionWarning.asStateFlow()

    private var solverHandle: SolverHandle? = null
    private var parallelSolverHandle: ParallelSolverHandle? = null
    private var solveJob: Job? = null

    @Volatile
    private var isSolveCancelled = false

    init {
        NativeLib.ensureLoaded()
        initializePlatformInfo()
        startMemoryMonitoring()
        collectSettings()
    }

    private fun collectSettings() {
        settingsRepository.settings
            .onEach { settings ->
                _megaminxColorScheme.value = settings.megaminxColorScheme
                _skipDeletionWarning.value = settings.skipDeletionWarning
                _solverConfig.update { config ->
                    config.copy(
                        parallelConfig = config.parallelConfig.copy(
                            memoryBudgetMb = settings.memoryBudgetMb,
                            tableGenThreads = settings.tableGenThreads,
                            searchThreads = settings.searchThreads
                        )
                    )
                }
            }
            .launchIn(scope)
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
        scope.launch {
            settingsRepository.updateSettings {
                it.copy(
                    memoryBudgetMb = config.memoryBudgetMb,
                    tableGenThreads = config.tableGenThreads,
                    searchThreads = config.searchThreads
                )
            }
        }
    }

    fun setMegaminxColorScheme(scheme: MegaminxColorScheme) {
        _megaminxColorScheme.value = scheme
        scope.launch {
            settingsRepository.updateSettings { it.copy(megaminxColorScheme = scheme) }
        }
    }

    fun setSkipDeletionWarning(skip: Boolean) {
        _skipDeletionWarning.value = skip
        scope.launch {
            settingsRepository.updateSettings { it.copy(skipDeletionWarning = skip) }
        }
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
        _megaminxState.update { state ->
            val newPositions = state.cornerPositions.toMutableList()
            val temp = newPositions[i]
            newPositions[i] = newPositions[j]
            newPositions[j] = temp
            state.copy(cornerPositions = newPositions)
        }
    }

    fun rotateCorner(index: Int, direction: Int) {
        _megaminxState.update { state ->
            val newOrientations = state.cornerOrientations.toMutableList()
            newOrientations[index] = (newOrientations[index] + direction + 3) % 3
            state.copy(cornerOrientations = newOrientations)
        }
    }

    fun swapEdges(i: Int, j: Int) {
        _megaminxState.update { state ->
            val newPositions = state.edgePositions.toMutableList()
            val temp = newPositions[i]
            newPositions[i] = newPositions[j]
            newPositions[j] = temp
            state.copy(edgePositions = newPositions)
        }
    }

    fun flipEdge(index: Int) {
        _megaminxState.update { state ->
            val newOrientations = state.edgeOrientations.toMutableList()
            newOrientations[index] = (newOrientations[index] + 1) % 2
            state.copy(edgeOrientations = newOrientations)
        }
    }

    fun reset() {
        _megaminxState.value = MegaminxState()
        _solverState.value = SolverState()
        _scoredSolutions.value = emptyList()
    }

    private fun mapGeneratorModesToSearchMode(mode: GeneratorMode): SearchMode = when (mode) {
        GeneratorMode.R_U -> SearchMode.RU
        GeneratorMode.R_U_L -> SearchMode.RUL
        GeneratorMode.R_U_F -> SearchMode.RUF
        GeneratorMode.R_U_D -> SearchMode.RUD
        GeneratorMode.R_U_BL -> SearchMode.R_UB_L
        GeneratorMode.R_U_BR -> SearchMode.R_UB_R
        GeneratorMode.R_U_L_F -> SearchMode.RUFL
        GeneratorMode.R_U_L_F_BL -> SearchMode.RUF_LB_L
    }

    private fun mapMetricType(metric: MetricType): Metric = when (metric) {
        MetricType.FTM -> Metric.FACE
        MetricType.FFTM -> Metric.FIFTH
    }

    private fun buildUniffiMegaminxState(): uniffi.llminxsolver.MegaminxState {
        val state = _megaminxState.value
        return uniffi.llminxsolver.MegaminxState(
            cornerPositions = state.cornerPositions.map { it.toUByte() },
            cornerOrientations = state.cornerOrientations.map { it.toUByte() },
            edgePositions = state.edgePositions.map { it.toUByte() },
            edgeOrientations = state.edgeOrientations.map { it.toUByte() }
        )
    }

    private fun buildUniffiParallelConfig(): uniffi.llminxsolver.ParallelConfig {
        val config = _solverConfig.value.parallelConfig
        return uniffi.llminxsolver.ParallelConfig(
            memoryBudgetMb = config.memoryBudgetMb.toUInt(),
            tableGenThreads = config.tableGenThreads.toUInt(),
            searchThreads = config.searchThreads.toUInt()
        )
    }

    private fun buildUniffiSolverConfig(): uniffi.llminxsolver.SolverConfig {
        val config = _solverConfig.value
        return uniffi.llminxsolver.SolverConfig(
            searchMode = mapGeneratorModesToSearchMode(config.generatorMode),
            metric = mapMetricType(config.metric),
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth.toUInt(),
            ignoreCornerPositions = config.ignoreFlags.cornerPositions,
            ignoreEdgePositions = config.ignoreFlags.edgePositions,
            ignoreCornerOrientations = config.ignoreFlags.cornerOrientations,
            ignoreEdgeOrientations = config.ignoreFlags.edgeOrientations,
            parallelConfig = buildUniffiParallelConfig()
        )
    }

    private fun buildUniffiParallelSolverConfig(): ParallelSolverConfig {
        val config = _solverConfig.value
        return ParallelSolverConfig(
            searchModes = config.selectedModes.map { mapGeneratorModesToSearchMode(it) },
            metric = mapMetricType(config.metric),
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth.toUInt(),
            ignoreCornerPositions = config.ignoreFlags.cornerPositions,
            ignoreEdgePositions = config.ignoreFlags.edgePositions,
            ignoreCornerOrientations = config.ignoreFlags.cornerOrientations,
            ignoreEdgeOrientations = config.ignoreFlags.edgeOrientations,
            parallelConfig = buildUniffiParallelConfig()
        )
    }

    fun solve() {
        if (_solverState.value.isSearching) return

        val config = _solverConfig.value
        if (config.isMultiMode) {
            solveMultiMode()
        } else {
            solveSingleMode()
        }
    }

    private fun solveSingleMode() {
        isSolveCancelled = false
        solveJob = scope.launch {
            _solverState.update {
                it.copy(
                    isSearching = true,
                    status = "Starting search...",
                    progress = 0f,
                    solutions = emptyList()
                )
            }
            _scoredSolutions.value = emptyList()

            try {
                val uniffiConfig = buildUniffiSolverConfig()
                val uniffiState = buildUniffiMegaminxState()

                solverHandle = SolverHandle(uniffiConfig, uniffiState)

                val callback = object : SolverCallback {
                    override fun onProgress(event: ProgressEvent) {
                        _solverState.update {
                            it.copy(
                                status = event.message,
                                progress = event.progress.toFloat()
                            )
                        }
                    }

                    override fun onSolutionFound(solution: String) {
                        if (isSolveCancelled) return
                        _solverState.update {
                            it.copy(solutions = it.solutions + solution)
                        }
                        updateScoredSolutions()
                    }

                    override fun onComplete() {
                        val solutionCount = _solverState.value.solutions.size
                        _solverState.update {
                            it.copy(
                                isSearching = false,
                                status = "Found $solutionCount solutions.",
                                progress = 1f
                            )
                        }
                        solverHandle?.close()
                        solverHandle = null
                    }
                }

                solverHandle?.setCallback(callback)
                solverHandle?.start()
            } catch (e: Exception) {
                _solverState.update {
                    it.copy(
                        isSearching = false,
                        status = "Error: ${e.message}"
                    )
                }
                solverHandle?.close()
                solverHandle = null
            }
        }
    }

    private fun solveMultiMode() {
        val config = _solverConfig.value
        isSolveCancelled = false
        solveJob = scope.launch {
            _solverState.update {
                it.copy(
                    isSearching = true,
                    status = "Starting multi-mode search (${config.selectedModes.size} modes)...",
                    progress = 0f,
                    solutions = emptyList()
                )
            }
            _scoredSolutions.value = emptyList()

            try {
                val uniffiConfig = buildUniffiParallelSolverConfig()
                val uniffiState = buildUniffiMegaminxState()

                parallelSolverHandle = ParallelSolverHandle(uniffiConfig, uniffiState)

                val callback = object : SolverCallback {
                    override fun onProgress(event: ProgressEvent) {
                        _solverState.update {
                            it.copy(
                                status = event.message,
                                progress = event.progress.toFloat()
                            )
                        }
                    }

                    override fun onSolutionFound(solution: String) {
                        if (isSolveCancelled) return
                        _solverState.update {
                            it.copy(solutions = it.solutions + solution)
                        }
                        updateScoredSolutions()
                    }

                    override fun onComplete() {
                        val solutionCount = _solverState.value.solutions.size
                        _solverState.update {
                            it.copy(
                                isSearching = false,
                                status = "Found $solutionCount solutions (multi-mode).",
                                progress = 1f
                            )
                        }
                        parallelSolverHandle?.close()
                        parallelSolverHandle = null
                    }
                }

                parallelSolverHandle?.setCallback(callback)
                parallelSolverHandle?.start()
            } catch (e: Exception) {
                _solverState.update {
                    it.copy(
                        isSearching = false,
                        status = "Error: ${e.message}"
                    )
                }
                parallelSolverHandle?.close()
                parallelSolverHandle = null
            }
        }
    }

    private fun updateScoredSolutions() {
        val solutions = _solverState.value.solutions
        val metricStr = when (_solverConfig.value.metric) {
            MetricType.FTM -> "FTM"
            MetricType.FFTM -> "FFTM"
        }

        val scored = solutions.mapNotNull { alg ->
            try {
                val mcc = calculateMcc(alg).toFloat()
                val moveCount = getMoveCount(alg, metricStr).toInt()
                ScoredSolution(
                    algorithm = alg,
                    mcc = mcc,
                    moveCount = moveCount
                )
            } catch (e: Exception) {
                null
            }
        }.sortedBy { it.mcc }

        _scoredSolutions.value = scored
    }

    fun cancelSolve() {
        isSolveCancelled = true
        solverHandle?.cancel()
        solverHandle?.close()
        solverHandle = null
        parallelSolverHandle?.cancel()
        parallelSolverHandle?.close()
        parallelSolverHandle = null
        solveJob?.cancel()
        _solverState.update {
            it.copy(
                isSearching = false,
                status = "Search cancelled"
            )
        }
    }

    fun onCleared() {
        cancelSolve()
        memoryMonitor.stopMonitoring()
    }
}
