package com.llminxsolver.theme

import androidx.compose.ui.graphics.Color

object MegaminxColors {
    val Yellow = Color(0xFFE1E100)
    val Red = Color(0xFFC80000)
    val Orange = Color(0xFFE16400)
    val Green = Color(0xFF00C800)
    val Pink = Color(0xFFFF9696)
    val Blue = Color(0xFF000096)
    val Gray = Color(0xFF808080)
}

val StickerColors =
    listOf(
        MegaminxColors.Yellow,
        MegaminxColors.Red,
        MegaminxColors.Orange,
        MegaminxColors.Green,
        MegaminxColors.Pink,
        MegaminxColors.Blue
    )

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
val StrokeWidth = 1.5f
val StrokeWidthSelected = 3f
