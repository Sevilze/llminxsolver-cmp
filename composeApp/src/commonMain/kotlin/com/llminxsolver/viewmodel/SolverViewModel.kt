package com.llminxsolver.viewmodel

import com.llminxsolver.data.AllowedFacesMode
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.data.SolverState
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

    private var solveJob: Job? = null

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

    fun solve() {
        if (_solverState.value.isSearching) return

        solveJob = scope.launch {
            _solverState.update { it.copy(isSearching = true, status = "Starting search...", progress = 0f, solutions = emptyList()) }
            _scoredSolutions.value = emptyList()

            try {
                for (depth in 1..12) {
                    if (!_solverState.value.isSearching) break

                    _solverState.update {
                        it.copy(
                            status = "Searching depth $depth...",
                            progress = depth / 12f
                        )
                    }

                    delay(500)
                }

                val mockSolutions = listOf(
                    "R U R' U R U2 R'",
                    "R U2 R' U' R U' R'",
                    "R' U' R U' R' U2 R"
                )

                _solverState.update {
                    it.copy(
                        solutions = mockSolutions,
                        status = "Search completed. Found ${mockSolutions.size} solutions.",
                        isSearching = false,
                        progress = 1f
                    )
                }

                val scored = mockSolutions.mapIndexed { index, alg ->
                    ScoredSolution(
                        algorithm = alg,
                        mcc = 5.0f + index * 0.5f,
                        moveCount = alg.split(" ").size
                    )
                }.sortedBy { it.mcc }

                _scoredSolutions.value = scored

            } catch (e: Exception) {
                _solverState.update {
                    it.copy(
                        isSearching = false,
                        status = "Error: ${e.message}"
                    )
                }
            }
        }
    }

    fun cancelSolve() {
        solveJob?.cancel()
        _solverState.update {
            it.copy(
                isSearching = false,
                status = "Search cancelled"
            )
        }
    }

    fun onCleared() {
        solveJob?.cancel()
    }
}
