package com.llminxsolver.data

data class MegaminxState(
    val cornerPositions: List<Int> = listOf(0, 1, 2, 3, 4),
    val cornerOrientations: List<Int> = listOf(0, 0, 0, 0, 0),
    val edgePositions: List<Int> = listOf(0, 1, 2, 3, 4),
    val edgeOrientations: List<Int> = listOf(0, 0, 0, 0, 0)
)

data class IgnoreFlags(
    val cornerPositions: Boolean = false,
    val edgePositions: Boolean = false,
    val cornerOrientations: Boolean = false,
    val edgeOrientations: Boolean = false
)

enum class GeneratorMode(val displayName: String) {
    R_U("R, U"),
    R_U_L("R, U, L"),
    R_U_F("R, U, F"),
    R_U_D("R, U, D"),
    R_U_BL("R, U, bL"),
    R_U_BR("R, U, bR"),
    R_U_L_F("R, U, L, F"),
    R_U_L_F_BL("R, U, L, F, bL")
}

enum class MetricType(val displayName: String) {
    FTM("FTM"),
    FFTM("FFTM")
}

data class ParallelConfig(
    val memoryBudgetMb: Int = 256,
    val tableGenThreads: Int = 2,
    val searchThreads: Int = 4
) {
    companion object {
        fun forMobile(): ParallelConfig = ParallelConfig(
            memoryBudgetMb = 1024,
            tableGenThreads = 2,
            searchThreads = 2
        )

        fun forDesktop(availableCpus: Int, availableMemoryMb: Int): ParallelConfig = ParallelConfig(
            memoryBudgetMb = 8192.coerceAtMost(availableMemoryMb).coerceAtLeast(256),
            tableGenThreads = 4.coerceAtMost(availableCpus),
            searchThreads = 4.coerceAtMost(availableCpus)
        )
    }
}

data class SolverConfig(
    val generatorMode: GeneratorMode = GeneratorMode.R_U,
    val selectedModes: Set<GeneratorMode> = setOf(GeneratorMode.R_U),
    val metric: MetricType = MetricType.FTM,
    val limitDepth: Boolean = false,
    val maxDepth: Int = 12,
    val ignoreFlags: IgnoreFlags = IgnoreFlags(),
    val parallelConfig: ParallelConfig = ParallelConfig()
) {
    val isMultiMode: Boolean
        get() = selectedModes.size > 1
}

data class SolverState(
    val isSearching: Boolean = false,
    val progress: Float = 0f,
    val status: String = "",
    val solutions: List<String> = emptyList()
)

data class ScoredSolution(val algorithm: String, val mcc: Float, val moveCount: Int)

sealed class StickerType {
    data object Center : StickerType()

    data object Corner : StickerType()

    data object Edge : StickerType()
}

data class StickerInfo(val type: StickerType, val cubieIndex: Int, val orientationIndex: Int)
