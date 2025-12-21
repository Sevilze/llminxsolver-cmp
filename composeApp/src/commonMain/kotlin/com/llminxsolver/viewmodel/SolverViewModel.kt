package com.llminxsolver.viewmodel

import com.llminxsolver.NativeLib
import com.llminxsolver.data.AllowedFacesMode
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import uniffi.llminxsolver.Metric
import uniffi.llminxsolver.ProgressEvent
import uniffi.llminxsolver.SearchMode
import uniffi.llminxsolver.SolverCallback
import uniffi.llminxsolver.SolverHandle
import uniffi.llminxsolver.calculateMcc
import uniffi.llminxsolver.getMoveCount

class SolverViewModel {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Default)

    private val _megaminxState = MutableStateFlow(MegaminxState())
    val megaminxState: StateFlow<MegaminxState> = _megaminxState.asStateFlow()

    private val _solverConfig = MutableStateFlow(SolverConfig())
    val solverConfig: StateFlow<SolverConfig> = _solverConfig.asStateFlow()

    private val _solverState = MutableStateFlow(SolverState())
    val solverState: StateFlow<SolverState> = _solverState.asStateFlow()

    private val _scoredSolutions = MutableStateFlow<List<ScoredSolution>>(emptyList())
    val scoredSolutions: StateFlow<List<ScoredSolution>> = _scoredSolutions.asStateFlow()

    private var solverHandle: SolverHandle? = null
    private var solveJob: Job? = null

    init {
        NativeLib.ensureLoaded()
    }

    fun setAllowedFaces(mode: AllowedFacesMode) {
        _solverConfig.update { it.copy(allowedFaces = mode) }
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

    private fun mapAllowedFacesToSearchMode(mode: AllowedFacesMode): SearchMode = when (mode) {
        AllowedFacesMode.R_U -> SearchMode.RU
        AllowedFacesMode.R_U_L -> SearchMode.RUL
        AllowedFacesMode.R_U_F -> SearchMode.RUF
        AllowedFacesMode.R_U_D -> SearchMode.RUD
        AllowedFacesMode.R_U_BL -> SearchMode.R_UB_L
        AllowedFacesMode.R_U_BR -> SearchMode.R_UB_R
        AllowedFacesMode.R_U_L_F -> SearchMode.RUFL
        AllowedFacesMode.R_U_L_F_BL -> SearchMode.RUF_LB_L
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

    private fun buildUniffiSolverConfig(): uniffi.llminxsolver.SolverConfig {
        val config = _solverConfig.value
        return uniffi.llminxsolver.SolverConfig(
            searchMode = mapAllowedFacesToSearchMode(config.allowedFaces),
            metric = mapMetricType(config.metric),
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth.toUInt(),
            ignoreCornerPositions = config.ignoreFlags.cornerPositions,
            ignoreEdgePositions = config.ignoreFlags.edgePositions,
            ignoreCornerOrientations = config.ignoreFlags.cornerOrientations,
            ignoreEdgeOrientations = config.ignoreFlags.edgeOrientations
        )
    }

    fun solve() {
        if (_solverState.value.isSearching) return

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
        solverHandle?.cancel()
        solverHandle?.close()
        solverHandle = null
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
    }
}
