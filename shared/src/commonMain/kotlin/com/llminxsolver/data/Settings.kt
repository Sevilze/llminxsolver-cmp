package com.llminxsolver.data

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import com.llminxsolver.theme.MegaminxColorScheme

enum class DynamicColorMode {
    BuiltIn,
    Matugen
}

enum class SchemeType {
    TonalSpot,
    Content,
    Expressive,
    Fidelity,
    FruitSalad,
    Monochrome,
    Neutral,
    Rainbow,
    Vibrant;

    fun toNative(): uniffi.llminxsolver.SchemeType = when (this) {
        TonalSpot -> uniffi.llminxsolver.SchemeType.TONAL_SPOT
        Content -> uniffi.llminxsolver.SchemeType.CONTENT
        Expressive -> uniffi.llminxsolver.SchemeType.EXPRESSIVE
        Fidelity -> uniffi.llminxsolver.SchemeType.FIDELITY
        FruitSalad -> uniffi.llminxsolver.SchemeType.FRUIT_SALAD
        Monochrome -> uniffi.llminxsolver.SchemeType.MONOCHROME
        Neutral -> uniffi.llminxsolver.SchemeType.NEUTRAL
        Rainbow -> uniffi.llminxsolver.SchemeType.RAINBOW
        Vibrant -> uniffi.llminxsolver.SchemeType.VIBRANT
    }
}

enum class ThemeMode {
    Dark,
    Light,
    System
}

data class AppSettings(
    val memoryBudgetMb: Int = 512,
    val tableGenThreads: Int = 4,
    val searchThreads: Int = 4,
    val defaultPruningDepth: Int = 12,
    val skipDeletionWarning: Boolean = false,
    val megaminxColorScheme: MegaminxColorScheme = MegaminxColorScheme(),
    val useDynamicColors: Boolean = true,
    val wallpaperPath: String? = null,
    val dynamicColorMode: DynamicColorMode = DynamicColorMode.BuiltIn,
    val schemeType: SchemeType = SchemeType.TonalSpot,
    val themeMode: ThemeMode = ThemeMode.System
) {
    companion object {
        val Default = AppSettings()
    }
}

fun Color.toHexString(): String = String.format("#%08X", toArgb())

fun hexStringToColor(hex: String): Color? = try {
    val cleanHex = hex.removePrefix("#")
    val colorInt = cleanHex.toLong(16)
    when (cleanHex.length) {
        6 -> Color(0xFF000000 or colorInt)
        8 -> Color(colorInt)
        else -> null
    }
} catch (_: Exception) {
    null
}
