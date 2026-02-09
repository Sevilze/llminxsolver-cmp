package com.llminxsolver.data

enum class SortingType(val displayName: String) {
    SET_PRIORITY("Set Priority"),
    ORIENTATION_OF("Orientation Of"),
    ORIENTATION_AT("Orientation At"),
    PERMUTATION_OF("Permutation Of"),
    PERMUTATION_AT("Permutation At")
}

data class SortingCriterion(
    val type: SortingType = SortingType.SET_PRIORITY,
    val pieces: String = ""
)

data class BatchSolverConfig(
    val scramble: String = "",
    val equivalences: String = "",
    val preAdjust: String = "U",
    val postAdjust: String = "U",
    val sortingCriteria: List<SortingCriterion> = emptyList(),
    val searchMode: GeneratorMode = GeneratorMode.R_U,
    val metric: MetricType = MetricType.FTM,
    val pruningDepth: Int = 12,
    val searchDepth: Int = 14,
    val stopAfterFirst: Boolean = true,
    val parallelConfig: ParallelConfig = ParallelConfig(),
    val ignoreCornerPermutation: Boolean = false,
    val ignoreEdgePermutation: Boolean = false,
    val ignoreCornerOrientation: Boolean = false,
    val ignoreEdgeOrientation: Boolean = false
)

data class BatchState(val caseNumber: Int, val setupMoves: String, val megaminxState: MegaminxState)

data class BatchCaseResult(
    val caseNumber: Int,
    val setupMoves: String,
    val solutions: List<String>,
    val bestSolution: String?,
    val solveTime: Double
)

data class BatchSolveResults(
    val totalCases: Int,
    val solvedCases: Int,
    val failedCases: List<Int>,
    val caseResults: List<BatchCaseResult>,
    val totalTime: Double,
    val averageTimePerCase: Double
)

data class BatchSolverState(
    val isGenerating: Boolean = false,
    val isSearching: Boolean = false,
    val isError: Boolean = false,
    val generatedStates: List<BatchState> = emptyList(),
    val currentStateIndex: Int = 0,
    val statusMessage: String = "",
    val progress: Float = 0f,
    val currentCase: Int = 0,
    val totalCases: Int = 0,
    val elapsedTime: Double = 0.0,
    val results: BatchSolveResults? = null
)

fun BatchSolverConfig.toIgnoreFlags() = IgnoreFlags(
    cornerPositions = ignoreCornerPermutation,
    edgePositions = ignoreEdgePermutation,
    cornerOrientations = ignoreCornerOrientation,
    edgeOrientations = ignoreEdgeOrientation
)
fun BatchSolverState.toSolverState() = SolverState(
    isSearching = isSearching || isGenerating,
    progress = progress,
    status = statusMessage
)
