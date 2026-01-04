package com.llminxsolver.ui.megaminx

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Fill
import androidx.compose.ui.graphics.drawscope.Stroke
import kotlin.math.sqrt

internal fun DrawScope.drawPolygon(
    points: List<Point>,
    fillColor: Color,
    strokeColor: Color,
    strokeWidth: Float
) {
    if (points.isEmpty()) return

    val path =
        Path().apply {
            moveTo(points[0].x, points[0].y)
            for (i in 1 until points.size) {
                lineTo(points[i].x, points[i].y)
            }
            close()
        }

    drawPath(path, fillColor, style = Fill)
    drawPath(path, strokeColor, style = Stroke(width = strokeWidth))
}

internal fun DrawScope.drawRoundedPolygon(
    points: List<Point>,
    fillColor: Color,
    strokeColor: Color,
    strokeWidth: Float,
    cornerRadius: Float
) {
    if (points.isEmpty()) return

    val path = Path()
    for (i in points.indices) {
        val curr = points[i]
        val prev = points[(i - 1 + points.size) % points.size]
        val next = points[(i + 1) % points.size]

        val dx1 = prev.x - curr.x
        val dy1 = prev.y - curr.y
        val len1 = sqrt(dx1 * dx1 + dy1 * dy1)
        val p1 = Point(curr.x + dx1 / len1 * cornerRadius, curr.y + dy1 / len1 * cornerRadius)

        val dx2 = next.x - curr.x
        val dy2 = next.y - curr.y
        val len2 = sqrt(dx2 * dx2 + dy2 * dy2)
        val p2 = Point(curr.x + dx2 / len2 * cornerRadius, curr.y + dy2 / len2 * cornerRadius)

        if (i == 0) {
            path.moveTo(p1.x, p1.y)
        } else {
            path.lineTo(p1.x, p1.y)
        }
        path.quadraticTo(curr.x, curr.y, p2.x, p2.y)
    }
    path.close()

    drawPath(path, fillColor, style = Fill)
    drawPath(path, strokeColor, style = Stroke(width = strokeWidth))
}

internal fun getCenterOfPoints(points: List<Point>): Point {
    if (points.isEmpty()) return Point(0f, 0f)
    val sumX = points.sumOf { it.x.toDouble() }.toFloat()
    val sumY = points.sumOf { it.y.toDouble() }.toFloat()
    return Point(sumX / points.size, sumY / points.size)
}

internal fun pointInPolygon(point: Point, polygon: List<Point>): Boolean {
    if (polygon.size < 3) return false

    var inside = false
    var j = polygon.size - 1

    for (i in polygon.indices) {
        val xi = polygon[i].x
        val yi = polygon[i].y
        val xj = polygon[j].x
        val yj = polygon[j].y

        val intersect =
            ((yi > point.y) != (yj > point.y)) &&
                (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi)

        if (intersect) inside = !inside
        j = i
    }

    return inside
}
