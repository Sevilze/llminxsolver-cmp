package com.llminxsolver.data

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import com.llminxsolver.theme.MegaminxColorScheme

data class AppSettings(
    val memoryBudgetMb: Int = 512,
    val tableGenThreads: Int = 4,
    val searchThreads: Int = 4,
    val skipDeletionWarning: Boolean = false,
    val megaminxColorScheme: MegaminxColorScheme = MegaminxColorScheme.Classic,
    val useDynamicColors: Boolean = true
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
