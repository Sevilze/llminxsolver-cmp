package com.llminxsolver.ui.megaminx

data class Point(val x: Float, val y: Float)

data class EdgeSticker(val top: List<Point>, val bottom: List<Point>)

data class CornerSticker(
    val top: List<Point>,
    val leftSticker: List<Point>,
    val rightSticker: List<Point>
)

data class MegaminxGeometry(
    val centerPoints: List<Point>,
    val innerCorners: List<Point>,
    val middleCorners: List<Point>,
    val outerCorners: List<Point>,
    val middleEdgesLeft: List<Point>,
    val middleEdgesRight: List<Point>,
    val edgeStickers: List<EdgeSticker>,
    val cornerStickers: List<CornerSticker>
)
