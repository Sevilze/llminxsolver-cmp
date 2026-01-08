package com.llminxsolver.viewmodel

import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import uniffi.llminxsolver.Metric
import uniffi.llminxsolver.ParallelSolverConfig
import uniffi.llminxsolver.ParallelSolverHandle
import uniffi.llminxsolver.ProgressEvent
import uniffi.llminxsolver.SearchMode
import uniffi.llminxsolver.SolverCallback
import uniffi.llminxsolver.SolverHandle
import uniffi.llminxsolver.TempFile
import uniffi.llminxsolver.calculateMcc
import uniffi.llminxsolver.getMoveCount

class SolverOperations(private val scope: CoroutineScope) {
    private val _solverState = MutableStateFlow(SolverState())
    val solverState: StateFlow<SolverState> = _solverState.asStateFlow()

    private val _scoredSolutions = MutableStateFlow<List<ScoredSolution>>(emptyList())
    val scoredSolutions: StateFlow<List<ScoredSolution>> = _scoredSolutions.asStateFlow()

    private val _tempFilePath = MutableStateFlow<String?>(null)
    val tempFilePath: StateFlow<String?> = _tempFilePath.asStateFlow()

    private var solverHandle: SolverHandle? = null
    private var parallelSolverHandle: ParallelSolverHandle? = null
    private var solveJob: Job? = null
    private var tempFile: TempFile? = null

    @Volatile
    private var isSolveCancelled = false

    fun solve(megaminxState: MegaminxState, solverConfig: SolverConfig) {
        if (_solverState.value.isSearching) return

        if (solverConfig.isMultiMode) {
            solveMultiMode(megaminxState, solverConfig)
        } else {
            solveSingleMode(megaminxState, solverConfig)
        }
    }

    private fun initTempFile() {
        tempFile?.deleteFile()
        tempFile = TempFile()
        _tempFilePath.value = tempFile?.getPath()
    }

    private fun solveSingleMode(megaminxState: MegaminxState, config: SolverConfig) {
        isSolveCancelled = false
        solveJob = scope.launch {
            initTempFile()

            _solverState.update {
                it.copy(
                    isSearching = true,
                    status = "Starting search...",
                    progress = 0f,
                    solutionCount = 0
                )
            }
            _scoredSolutions.value = emptyList()

            try {
                val uniffiConfig = buildUniffiSolverConfig(config)
                val uniffiState = buildUniffiMegaminxState(megaminxState)

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
                        tempFile?.append(solution)
                        _solverState.update {
                            it.copy(solutionCount = it.solutionCount + 1)
                        }
                        addScoredSolution(solution, config.metric)
                    }

                    override fun onComplete() {
                        val solutionCount = _solverState.value.solutionCount
                        tempFile?.flushFile()
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

    private fun solveMultiMode(megaminxState: MegaminxState, config: SolverConfig) {
        isSolveCancelled = false
        solveJob = scope.launch {
            initTempFile()

            _solverState.update {
                it.copy(
                    isSearching = true,
                    status = "Starting multi-mode search (${config.selectedModes.size} modes)...",
                    progress = 0f,
                    solutionCount = 0
                )
            }
            _scoredSolutions.value = emptyList()

            try {
                val uniffiConfig = buildUniffiParallelSolverConfig(config)
                val uniffiState = buildUniffiMegaminxState(megaminxState)

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
                        tempFile?.append(solution)
                        _solverState.update {
                            it.copy(solutionCount = it.solutionCount + 1)
                        }
                        addScoredSolution(solution, config.metric)
                    }

                    override fun onComplete() {
                        val solutionCount = _solverState.value.solutionCount
                        tempFile?.flushFile()
                        _solverState.update {
                            it.copy(
                                isSearching = false,
                                status = "Found $solutionCount solutions.",
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

    private fun addScoredSolution(solution: String, metric: MetricType) {
        val metricStr = when (metric) {
            MetricType.FTM -> "FTM"
            MetricType.FFTM -> "FFTM"
        }

        try {
            val mcc = calculateMcc(solution).toFloat()
            val moveCount = getMoveCount(solution, metricStr).toInt()
            val scored = ScoredSolution(
                algorithm = solution,
                mcc = mcc,
                moveCount = moveCount
            )
            _scoredSolutions.update { current ->
                (current + scored).sortedBy { it.mcc }
            }
        } catch (_: Exception) { }
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
        tempFile?.flushFile()
        _solverState.update {
            it.copy(
                isSearching = false,
                status = "Search cancelled"
            )
        }
    }

    fun reset() {
        tempFile?.deleteFile()
        tempFile = null
        _tempFilePath.value = null
        _solverState.value = SolverState()
        _scoredSolutions.value = emptyList()
    }

    fun readSolutionsPage(offset: Int, limit: Int): List<String> =
        tempFile?.readPage(offset.toULong(), limit.toULong()) ?: emptyList()

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

    private fun buildUniffiMegaminxState(state: MegaminxState): uniffi.llminxsolver.MegaminxState =
        uniffi.llminxsolver.MegaminxState(
            cornerPositions = state.cornerPositions.map { it.toUByte() },
            cornerOrientations = state.cornerOrientations.map { it.toUByte() },
            edgePositions = state.edgePositions.map { it.toUByte() },
            edgeOrientations = state.edgeOrientations.map { it.toUByte() }
        )

    private fun buildUniffiParallelConfig(
        config: SolverConfig
    ): uniffi.llminxsolver.ParallelConfig = uniffi.llminxsolver.ParallelConfig(
        memoryBudgetMb = config.parallelConfig.memoryBudgetMb.toUInt(),
        tableGenThreads = config.parallelConfig.tableGenThreads.toUInt(),
        searchThreads = config.parallelConfig.searchThreads.toUInt()
    )

    private fun buildUniffiSolverConfig(config: SolverConfig): uniffi.llminxsolver.SolverConfig =
        uniffi.llminxsolver.SolverConfig(
            searchMode = mapGeneratorModesToSearchMode(config.generatorMode),
            metric = mapMetricType(config.metric),
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth.toUInt(),
            ignoreCornerPositions = config.ignoreFlags.cornerPositions,
            ignoreEdgePositions = config.ignoreFlags.edgePositions,
            ignoreCornerOrientations = config.ignoreFlags.cornerOrientations,
            ignoreEdgeOrientations = config.ignoreFlags.edgeOrientations,
            parallelConfig = buildUniffiParallelConfig(config)
        )

    private fun buildUniffiParallelSolverConfig(config: SolverConfig): ParallelSolverConfig =
        ParallelSolverConfig(
            searchModes = config.selectedModes.map { mapGeneratorModesToSearchMode(it) },
            metric = mapMetricType(config.metric),
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth.toUInt(),
            ignoreCornerPositions = config.ignoreFlags.cornerPositions,
            ignoreEdgePositions = config.ignoreFlags.edgePositions,
            ignoreCornerOrientations = config.ignoreFlags.cornerOrientations,
            ignoreEdgeOrientations = config.ignoreFlags.edgeOrientations,
            parallelConfig = buildUniffiParallelConfig(config)
        )
}
