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

enum class AllowedFacesMode(val displayName: String) {
    R_U("R, U"),
    R_U_L("R, U, L"),
    R_U_F("R, U, F"),
    R_U_D("R, U, D"),
    R_U_bL("R, U, bL"),
    R_U_bR("R, U, bR"),
    R_U_L_F("R, U, L, F"),
    R_U_L_F_bL("R, U, L, F, bL")
}

enum class MetricType(val displayName: String) {
    FTM("FTM"),
    FFTM("FFTM")
}

data class SolverConfig(
    val allowedFaces: AllowedFacesMode = AllowedFacesMode.R_U,
    val metric: MetricType = MetricType.FTM,
    val limitDepth: Boolean = false,
    val maxDepth: Int = 12,
    val ignoreFlags: IgnoreFlags = IgnoreFlags()
)

data class SolverState(
    val isSearching: Boolean = false,
    val progress: Float = 0f,
    val status: String = "",
    val solutions: List<String> = emptyList()
)

data class ScoredSolution(
    val algorithm: String,
    val mcc: Float,
    val moveCount: Int
)

sealed class StickerType {
    data object Center : StickerType()
    data object Corner : StickerType()
    data object Edge : StickerType()
}

data class StickerInfo(
    val type: StickerType,
    val cubieIndex: Int,
    val orientationIndex: Int
)
