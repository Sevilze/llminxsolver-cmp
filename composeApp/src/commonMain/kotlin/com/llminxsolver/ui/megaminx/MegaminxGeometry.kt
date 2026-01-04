package com.llminxsolver.ui.megaminx

import kotlin.math.PI
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

internal fun pointAt(center: Point, distance: Float, angle: Float): Point = Point(
    x = center.x + distance * cos(angle),
    y = center.y + distance * sin(angle)
)

internal fun lerp(p1: Point, p2: Point, fraction: Float): Point = Point(
    x = p1.x * (1 - fraction) + p2.x * fraction,
    y = p1.y * (1 - fraction) + p2.y * fraction
)

internal fun distance(p1: Point, p2: Point): Float {
    val dx = p2.x - p1.x
    val dy = p2.y - p1.y
    return sqrt(dx * dx + dy * dy)
}

internal fun lineIntersection(
    x1: Float,
    y1: Float,
    x2: Float,
    y2: Float,
    x3: Float,
    y3: Float,
    x4: Float,
    y4: Float
): Point {
    fun det(a: Float, b: Float, c: Float, d: Float) = a * d - b * c
    val denom = det(x1 - x2, y1 - y2, x3 - x4, y3 - y4)
    val det12 = det(x1, y1, x2, y2)
    val det34 = det(x3, y3, x4, y4)
    return Point(
        x = det(det12, x1 - x2, det34, x3 - x4) / denom,
        y = det(det12, y1 - y2, det34, y3 - y4) / denom
    )
}

internal fun calculateGeometry(
    width: Float,
    height: Float,
    padding: Float = 10f
): MegaminxGeometry {
    val halfWidth = width / 2f
    val halfHeight = height / 2f
    val outerRadius = minOf(halfHeight, halfWidth) - padding
    val middleRadius = (3f * outerRadius) / 4f
    val innerRadius = outerRadius / 3f
    val center = Point(halfWidth, halfHeight)

    val innerCorners = mutableListOf<Point>()
    val middleCorners = mutableListOf<Point>()
    val outerCorners = mutableListOf<Point>()
    val centerPoints = mutableListOf<Point>()

    for (i in 0 until 5) {
        val angle = (-PI / 2.0 + (i.toDouble() / 5.0) * PI * 2.0).toFloat()
        innerCorners.add(pointAt(center, innerRadius, angle))
        middleCorners.add(pointAt(center, middleRadius, angle))
        outerCorners.add(pointAt(center, outerRadius, angle))
        centerPoints.add(pointAt(center, innerRadius, angle))
    }

    val middleEdgesLeft = MutableList(5) { Point(0f, 0f) }
    val middleEdgesRight = MutableList(5) { Point(0f, 0f) }

    for (i in 0 until 5) {
        val prevCorner = (i + 4) % 5
        val nextCorner = (i + 1) % 5

        val intersectionRight =
            lineIntersection(
                innerCorners[prevCorner].x,
                innerCorners[prevCorner].y,
                innerCorners[i].x,
                innerCorners[i].y,
                middleCorners[i].x,
                middleCorners[i].y,
                middleCorners[nextCorner].x,
                middleCorners[nextCorner].y
            )
        middleEdgesRight[i] = intersectionRight

        val intersectionLeft =
            lineIntersection(
                innerCorners[i].x,
                innerCorners[i].y,
                innerCorners[nextCorner].x,
                innerCorners[nextCorner].y,
                middleCorners[prevCorner].x,
                middleCorners[prevCorner].y,
                middleCorners[i].x,
                middleCorners[i].y
            )
        middleEdgesLeft[prevCorner] = intersectionLeft
    }

    val edgeStickers = MutableList(5) { EdgeSticker(emptyList(), emptyList()) }
    val cornerStickers = mutableListOf<CornerSticker>()

    for (i in 0 until 5) {
        val prevCorner = (i + 4) % 5
        val nextCorner = (i + 1) % 5

        val fraction =
            distance(middleEdgesLeft[prevCorner], innerCorners[i]) /
                distance(middleEdgesLeft[prevCorner], middleEdgesRight[nextCorner])

        val leftOuterCorner = lerp(outerCorners[i], outerCorners[nextCorner], fraction)
        val rightOuterCorner = lerp(outerCorners[i], outerCorners[prevCorner], fraction)
        val leftOuterEdge = lerp(outerCorners[nextCorner], outerCorners[i], fraction)

        val edgeIndex = (i + 3) % 5
        edgeStickers[edgeIndex] =
            EdgeSticker(
                top =
                    listOf(
                        innerCorners[i],
                        innerCorners[nextCorner],
                        middleEdgesLeft[i],
                        middleEdgesRight[i]
                    ),
                bottom =
                    listOf(
                        leftOuterCorner,
                        middleEdgesRight[i],
                        middleEdgesLeft[i],
                        leftOuterEdge
                    )
            )

        cornerStickers.add(
            CornerSticker(
                top =
                    listOf(
                        innerCorners[i],
                        middleEdgesLeft[prevCorner],
                        middleCorners[i],
                        middleEdgesRight[i]
                    ),
                leftSticker =
                    listOf(
                        middleCorners[i],
                        middleEdgesRight[i],
                        leftOuterCorner,
                        outerCorners[i]
                    ),
                rightSticker =
                    listOf(
                        middleCorners[i],
                        middleEdgesLeft[prevCorner],
                        rightOuterCorner,
                        outerCorners[i]
                    )
            )
        )
    }

    return MegaminxGeometry(
        centerPoints = centerPoints,
        innerCorners = innerCorners,
        middleCorners = middleCorners,
        outerCorners = outerCorners,
        middleEdgesLeft = middleEdgesLeft,
        middleEdgesRight = middleEdgesRight,
        edgeStickers = edgeStickers,
        cornerStickers = cornerStickers
    )
}
