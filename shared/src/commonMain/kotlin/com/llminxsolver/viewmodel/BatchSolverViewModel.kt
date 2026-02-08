package com.llminxsolver.viewmodel

import com.llminxsolver.NativeLib
import com.llminxsolver.data.BatchCaseResult
import com.llminxsolver.data.BatchSolveResults
import com.llminxsolver.data.BatchSolverConfig
import com.llminxsolver.data.BatchSolverState
import com.llminxsolver.data.BatchState
import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.SortingCriterion
import com.llminxsolver.data.SortingType
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.platform.MemoryMonitor
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import uniffi.llminxsolver.BatchSolverHandle
import uniffi.llminxsolver.getAvailableCpus
import uniffi.llminxsolver.getAvailableMemoryMb

class BatchSolverViewModel(private val settingsViewModel: SettingsViewModel) {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)
    private val memoryMonitor = MemoryMonitor()

    private var solverHandle: BatchSolverHandle? = null
    private var timerJob: Job? = null

    private val _config = MutableStateFlow(BatchSolverConfig())
    val config: StateFlow<BatchSolverConfig> = _config.asStateFlow()

    private val _state = MutableStateFlow(BatchSolverState())
    val state: StateFlow<BatchSolverState> = _state.asStateFlow()

    private val _memoryInfo = MutableStateFlow<MemoryInfo?>(null)
    val memoryInfo: StateFlow<MemoryInfo?> = _memoryInfo.asStateFlow()

    private val _availableCpus = MutableStateFlow(4)
    val availableCpus: StateFlow<Int> = _availableCpus.asStateFlow()

    val megaminxColorScheme: StateFlow<MegaminxColorScheme> = settingsViewModel.megaminxColorScheme

    private var lastKnownDefaultPruningDepth: Int? = null

    init {
        NativeLib.ensureLoaded()
        initializePlatformInfo()
        startMemoryMonitoring()
        settingsViewModel.collectSettings {
                memoryBudgetMb,
                tableGenThreads,
                searchThreads,
                defaultPruningDepth
            ->
            _config.update { config ->
                val newPruningDepth = if (defaultPruningDepth != lastKnownDefaultPruningDepth) {
                    defaultPruningDepth
                } else {
                    config.pruningDepth
                }

                config.copy(
                    parallelConfig = config.parallelConfig.copy(
                        memoryBudgetMb = memoryBudgetMb,
                        tableGenThreads = tableGenThreads,
                        searchThreads = searchThreads
                    ),
                    pruningDepth = newPruningDepth
                )
            }
            lastKnownDefaultPruningDepth = defaultPruningDepth
        }
    }

    private fun initializePlatformInfo() {
        try {
            _availableCpus.value = getAvailableCpus().toInt()
            val availableMemory = getAvailableMemoryMb().toInt()
            val defaultConfig = ParallelConfig.forDesktop(_availableCpus.value, availableMemory)
            _config.update { it.copy(parallelConfig = defaultConfig) }
        } catch (e: Exception) {
            _availableCpus.value = 4
        }
    }

    private fun startMemoryMonitoring() {
        memoryMonitor.startMonitoring(2000L) { info ->
            _memoryInfo.value = info
        }
    }

    fun setScramble(scramble: String) {
        _config.update { it.copy(scramble = scramble) }
    }

    fun setEquivalences(equivalences: String) {
        _config.update { it.copy(equivalences = equivalences) }
    }

    fun setPreAdjust(preAdjust: String) {
        _config.update { it.copy(preAdjust = preAdjust) }
    }

    fun setPostAdjust(postAdjust: String) {
        _config.update { it.copy(postAdjust = postAdjust) }
    }

    fun setSortingCriteria(criteria: List<SortingCriterion>) {
        _config.update { it.copy(sortingCriteria = criteria) }
    }

    fun setSearchMode(mode: GeneratorMode) {
        _config.update { it.copy(searchMode = mode) }
    }

    fun setMetric(metric: MetricType) {
        _config.update { it.copy(metric = metric) }
    }

    fun setPruningDepth(depth: Int) {
        val coercedDepth = depth.coerceIn(6, 18)
        _config.update { it.copy(pruningDepth = coercedDepth) }
        settingsViewModel.savePruningDepth(coercedDepth)
    }

    fun setSearchDepth(depth: Int) {
        val coercedDepth = depth.coerceIn(1, 50)
        _config.update { it.copy(searchDepth = coercedDepth) }
    }

    fun setStopAfterFirst(stop: Boolean) {
        _config.update { it.copy(stopAfterFirst = stop) }
    }

    fun setIgnoreCornerPermutation(ignore: Boolean) {
        _config.update { it.copy(ignoreCornerPermutation = ignore) }
    }

    fun setIgnoreEdgePermutation(ignore: Boolean) {
        _config.update { it.copy(ignoreEdgePermutation = ignore) }
    }

    fun setIgnoreCornerOrientation(ignore: Boolean) {
        _config.update { it.copy(ignoreCornerOrientation = ignore) }
    }

    fun setIgnoreEdgeOrientation(ignore: Boolean) {
        _config.update { it.copy(ignoreEdgeOrientation = ignore) }
    }

    fun setIgnoreFlag(flag: String, value: Boolean) {
        _config.update { config ->
            when (flag) {
                "cornerPositions" -> config.copy(ignoreCornerPermutation = value)
                "edgePositions" -> config.copy(ignoreEdgePermutation = value)
                "cornerOrientations" -> config.copy(ignoreCornerOrientation = value)
                "edgeOrientations" -> config.copy(ignoreEdgeOrientation = value)
                else -> config
            }
        }
    }

    fun setParallelConfig(config: ParallelConfig) {
        _config.update { it.copy(parallelConfig = config) }
    }

    fun setCurrentStateIndex(index: Int) {
        val maxIndex = _state.value.generatedStates.size - 1
        _state.update { it.copy(currentStateIndex = index.coerceIn(0, maxIndex.coerceAtLeast(0))) }
    }

    fun nextState() {
        val currentIndex = _state.value.currentStateIndex
        val maxIndex = _state.value.generatedStates.size - 1
        if (currentIndex < maxIndex) {
            _state.update { it.copy(currentStateIndex = currentIndex + 1) }
        }
    }

    fun previousState() {
        val currentIndex = _state.value.currentStateIndex
        if (currentIndex > 0) {
            _state.update { it.copy(currentStateIndex = currentIndex - 1) }
        }
    }

    fun generateStates() {
        val currentConfig = _config.value
        if (currentConfig.scramble.isBlank()) {
            _state.update { it.copy(statusMessage = "Scramble is required") }
            return
        }

        _state.update {
            it.copy(
                isGenerating = true,
                statusMessage = "Generating states...",
                generatedStates = emptyList(),
                currentStateIndex = 0
            )
        }

        scope.launch(Dispatchers.IO) {
            try {
                val handle = createSolverHandle(currentConfig)
                if (handle == null) {
                    _state.update {
                        it.copy(isGenerating = false, statusMessage = "Failed to create solver")
                    }
                    return@launch
                }

                handle.setCallback(object : uniffi.llminxsolver.BatchSolverCallback {
                    override fun onProgress(event: uniffi.llminxsolver.ProgressEvent) {
                        if (event.eventType == "GeneratingStates") {
                            _state.update {
                                it.copy(statusMessage = event.message)
                            }
                        }
                    }

                    override fun onCaseSolved(result: uniffi.llminxsolver.BatchCaseResult) {}
                    override fun onComplete(results: uniffi.llminxsolver.BatchSolveResults) {}
                })

                val generatedStates = handle.generateStates()
                val batchStates = generatedStates.map { gs ->
                    BatchState(
                        caseNumber = gs.caseNumber.toInt(),
                        setupMoves = gs.setupMoves,
                        megaminxState = MegaminxState(
                            cornerPositions = gs.cornerPositions.map { it.toInt() },
                            cornerOrientations = gs.cornerOrientations.map { it.toInt() },
                            edgePositions = gs.edgePositions.map { it.toInt() },
                            edgeOrientations = gs.edgeOrientations.map { it.toInt() }
                        )
                    )
                }

                solverHandle = handle
                _state.update {
                    it.copy(
                        isGenerating = false,
                        generatedStates = batchStates,
                        totalCases = batchStates.size,
                        statusMessage = "Generated ${batchStates.size} cases"
                    )
                }
            } catch (e: Exception) {
                _state.update {
                    it.copy(isGenerating = false, statusMessage = "Error: ${e.message}")
                }
            }
        }
    }

    fun startSolve() {
        val handle = solverHandle ?: run {
            _state.update { it.copy(statusMessage = "Generate states first") }
            return
        }

        scope.launch(Dispatchers.IO) {
            try {
                val uniffiConfig = buildUniffiConfig(_config.value)
                handle.updateConfig(uniffiConfig)

                _state.update {
                    it.copy(
                        isSearching = true,
                        statusMessage = "Starting batch solve...",
                        progress = 0f,
                        currentCase = 0,
                        elapsedTime = 0.0,
                        results = BatchSolveResults(
                            totalCases = handle.getTotalCases().toInt(),
                            solvedCases = 0,
                            failedCases = emptyList(),
                            caseResults = emptyList(),
                            totalTime = 0.0,
                            averageTimePerCase = 0.0
                        )
                    )
                }

                val startTime = System.currentTimeMillis()
                timerJob?.cancel()
                timerJob = scope.launch {
                    while (true) {
                        delay(100)
                        val elapsed = (System.currentTimeMillis() - startTime) / 1000.0
                        _state.update { it.copy(elapsedTime = elapsed) }
                    }
                }

                handle.setCallback(object : uniffi.llminxsolver.BatchSolverCallback {
                    override fun onProgress(event: uniffi.llminxsolver.ProgressEvent) {
                        // Filter out SolutionFound events to prevent status flickering
                        if (event.eventType == "SolutionFound") return

                        _state.update {
                            it.copy(
                                statusMessage = event.message,
                                progress = event.progress.toFloat()
                            )
                        }
                    }

                    override fun onCaseSolved(result: uniffi.llminxsolver.BatchCaseResult) {
                        val convertedResult = BatchCaseResult(
                            caseNumber = result.caseNumber.toInt(),
                            setupMoves = result.setupMoves,
                            solutions = result.solutions,
                            bestSolution = result.bestSolution,
                            solveTime = result.solveTime
                        )
                        _state.update { currentState ->
                            val existingResults = currentState.results
                            val updatedCaseResults =
                                (existingResults?.caseResults ?: emptyList()) + convertedResult
                            val solvedCount = updatedCaseResults.count { it.solutions.isNotEmpty() }
                            val failedCases = updatedCaseResults.filter {
                                it.solutions.isEmpty()
                            }.map { it.caseNumber }
                            val totalTime = (System.currentTimeMillis() - startTime) / 1000.0

                            currentState.copy(
                                currentCase = result.caseNumber.toInt(),
                                results = BatchSolveResults(
                                    totalCases = currentState.totalCases,
                                    solvedCases = solvedCount,
                                    failedCases = failedCases,
                                    caseResults = updatedCaseResults,
                                    totalTime = totalTime,
                                    averageTimePerCase = if (solvedCount > 0) {
                                        totalTime / solvedCount
                                    } else {
                                        0.0
                                    }
                                )
                            )
                        }
                    }

                    override fun onComplete(results: uniffi.llminxsolver.BatchSolveResults) {
                        val convertedResults = BatchSolveResults(
                            totalCases = results.totalCases.toInt(),
                            solvedCases = results.solvedCases.toInt(),
                            failedCases = results.failedCases.map { it.toInt() },
                            caseResults = results.caseResults.map { cr ->
                                BatchCaseResult(
                                    caseNumber = cr.caseNumber.toInt(),
                                    setupMoves = cr.setupMoves,
                                    solutions = cr.solutions,
                                    bestSolution = cr.bestSolution,
                                    solveTime = cr.solveTime
                                )
                            },
                            totalTime = results.totalTime,
                            averageTimePerCase = results.averageTimePerCase
                        )
                        timerJob?.cancel()
                        _state.update {
                            it.copy(
                                isSearching = false,
                                results = convertedResults,
                                statusMessage =
                                    "Solved ${results.solvedCases}/${results.totalCases} cases"
                            )
                        }
                    }
                })

                handle.start()
            } catch (e: Exception) {
                _state.update {
                    it.copy(
                        isSearching = false,
                        statusMessage = "Error starting solve: ${e.message}",
                        isError = true
                    )
                }
            }
        }
    }

    fun cancelSolve() {
        solverHandle?.cancel()
        timerJob?.cancel()
        _state.update {
            it.copy(isSearching = false, statusMessage = "Cancelled")
        }
    }

    fun reset() {
        cancelSolve()
        solverHandle = null
        _state.value = BatchSolverState()
    }

    private fun createSolverHandle(config: BatchSolverConfig): BatchSolverHandle? = try {
        val batchConfig = buildUniffiConfig(config)
        BatchSolverHandle(batchConfig)
    } catch (e: Exception) {
        null
    }

    private fun buildUniffiConfig(
        config: BatchSolverConfig
    ): uniffi.llminxsolver.BatchSolverConfig {
        val searchMode = when (config.searchMode) {
            GeneratorMode.R_U -> uniffi.llminxsolver.SearchMode.RU
            GeneratorMode.R_U_L -> uniffi.llminxsolver.SearchMode.RUL
            GeneratorMode.R_U_F -> uniffi.llminxsolver.SearchMode.RUF
            GeneratorMode.R_U_D -> uniffi.llminxsolver.SearchMode.RUD
            GeneratorMode.R_U_BL -> uniffi.llminxsolver.SearchMode.R_UB_L
            GeneratorMode.R_U_BR -> uniffi.llminxsolver.SearchMode.R_UB_R
            GeneratorMode.R_U_L_F -> uniffi.llminxsolver.SearchMode.RUFL
            GeneratorMode.R_U_L_F_BL -> uniffi.llminxsolver.SearchMode.RUF_LB_L
        }

        val metric = when (config.metric) {
            MetricType.FTM -> uniffi.llminxsolver.Metric.FACE
            MetricType.FFTM -> uniffi.llminxsolver.Metric.FIFTH
        }

        val sortingCriteria = config.sortingCriteria.map { sc ->
            uniffi.llminxsolver.SortingCriterion(
                sortingType = when (sc.type) {
                    SortingType.SET_PRIORITY -> uniffi.llminxsolver.SortingType.SET_PRIORITY
                    SortingType.ORIENTATION_OF -> uniffi.llminxsolver.SortingType.ORIENTATION_OF
                    SortingType.ORIENTATION_AT -> uniffi.llminxsolver.SortingType.ORIENTATION_AT
                    SortingType.PERMUTATION_OF -> uniffi.llminxsolver.SortingType.PERMUTATION_OF
                    SortingType.PERMUTATION_AT -> uniffi.llminxsolver.SortingType.PERMUTATION_AT
                },
                pieces = sc.pieces
            )
        }

        val parallelConfig = uniffi.llminxsolver.ParallelConfig(
            memoryBudgetMb = config.parallelConfig.memoryBudgetMb.toUInt(),
            tableGenThreads = config.parallelConfig.tableGenThreads.toUInt(),
            searchThreads = config.parallelConfig.searchThreads.toUInt()
        )

        return uniffi.llminxsolver.BatchSolverConfig(
            scramble = config.scramble,
            equivalences = config.equivalences,
            preAdjust = config.preAdjust,
            postAdjust = config.postAdjust,
            sortingCriteria = sortingCriteria,
            searchMode = searchMode,
            metric = metric,
            pruningDepth = config.pruningDepth.toUByte(),
            searchDepth = config.searchDepth.toUInt(),
            stopAfterFirst = config.stopAfterFirst,
            parallelConfig = parallelConfig,
            ignoreCornerPermutation = config.ignoreCornerPermutation,
            ignoreEdgePermutation = config.ignoreEdgePermutation,
            ignoreCornerOrientation = config.ignoreCornerOrientation,
            ignoreEdgeOrientation = config.ignoreEdgeOrientation
        )
    }

    fun onCleared() {
        cancelSolve()
        memoryMonitor.stopMonitoring()
    }
}
