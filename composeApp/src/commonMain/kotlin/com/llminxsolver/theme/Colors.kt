package com.llminxsolver.theme

import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.graphics.Color

@Immutable
data class MegaminxColorScheme(
    val uFace: Color = Color(0xFF1E1E1C),
    val fFace: Color = Color(0xFF7EFA03),
    val lFace: Color = Color(0xFFFEA504),
    val blFace: Color = Color(0xFF04FDFE),
    val brFace: Color = Color(0xFFFEF8CD),
    val rFace: Color = Color(0xFFFFC0CB),
    val blank: Color = Color(0xFF808080)
) {
    val stickerColors: List<Color>
        get() = listOf(uFace, fFace, lFace, blFace, brFace, rFace)
}

val LocalMegaminxColorScheme = staticCompositionLocalOf { MegaminxColorScheme() }

@Composable
fun megaminxColorScheme(): MegaminxColorScheme = LocalMegaminxColorScheme.current

val CornerColorMap =
    listOf(
        listOf(0, 3, 4),
        listOf(0, 4, 5),
        listOf(0, 5, 1),
        listOf(0, 1, 2),
        listOf(0, 2, 3)
    )

val EdgeColorMap =
    listOf(
        listOf(0, 1),
        listOf(0, 2),
        listOf(0, 3),
        listOf(0, 4),
        listOf(0, 5)
    )

val SelectionColor = Color(0xE6800000)
val HighlightColor = Color(0x26FF0000)
val StrokeColor = Color.Black
val StrokeWidth = 2f
val StrokeWidthSelected = 3f
