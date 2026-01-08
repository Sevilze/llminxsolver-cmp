package com.llminxsolver.ui.megaminx

import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.unit.IntSize
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.theme.CornerColorMap
import com.llminxsolver.theme.EdgeColorMap
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.theme.StrokeColor
import com.llminxsolver.theme.StrokeWidth
import kotlin.math.sqrt

private fun faceColorIndexForRender(cornerIndex: Int): Int = when (cornerIndex) {
    0 -> 4
    1 -> 5
    2 -> 1
    3 -> 2
    4 -> 3
    else -> 0
}

fun renderMegaminxToImageBitmap(
    puzzleState: MegaminxState,
    colorScheme: MegaminxColorScheme = MegaminxColorScheme(),
    ignoreFlags: IgnoreFlags = IgnoreFlags(),
    size: Int = 400
): ImageBitmap {
    val bitmap = ImageBitmap(size, size)
    val canvas = Canvas(bitmap)

    val geometry = calculateGeometry(size.toFloat(), size.toFloat(), 10f)

    fun getCornerColor(cubieIndex: Int, orientationIndex: Int): Color {
        val position = puzzleState.cornerPositions[cubieIndex]
        val orientation = puzzleState.cornerOrientations[cubieIndex]
        val effectiveOrientation = (orientationIndex - orientation + 3) % 3

        if (ignoreFlags.cornerPositions) {
            if (effectiveOrientation != 0 || ignoreFlags.cornerOrientations) {
                return colorScheme.blank
            }
        }

        if (ignoreFlags.cornerOrientations) {
            return colorScheme.blank
        }

        return colorScheme.stickerColors[CornerColorMap[position][effectiveOrientation]]
    }

    fun getEdgeColor(cubieIndex: Int, orientationIndex: Int): Color {
        val position = puzzleState.edgePositions[cubieIndex]
        val orientation = puzzleState.edgeOrientations[cubieIndex]
        val effectiveOrientation = kotlin.math.abs(orientationIndex - orientation)

        if (ignoreFlags.edgePositions) {
            if (effectiveOrientation != 0 || ignoreFlags.edgeOrientations) {
                return colorScheme.blank
            }
        }

        if (ignoreFlags.edgeOrientations) {
            return colorScheme.blank
        }

        return colorScheme.stickerColors[EdgeColorMap[position][effectiveOrientation]]
    }

    fun drawPolygonOnCanvas(
        points: List<Point>,
        fillColor: Color,
        strokeColor: Color,
        strokeWidth: Float
    ) {
        if (points.isEmpty()) return

        val path = Path().apply {
            moveTo(points[0].x, points[0].y)
            for (i in 1 until points.size) {
                lineTo(points[i].x, points[i].y)
            }
            close()
        }

        val fillPaint = androidx.compose.ui.graphics.Paint().apply {
            color = fillColor
            style = androidx.compose.ui.graphics.PaintingStyle.Fill
        }
        canvas.drawPath(path, fillPaint)

        if (strokeWidth > 0f) {
            val strokePaint = androidx.compose.ui.graphics.Paint().apply {
                color = strokeColor
                style = androidx.compose.ui.graphics.PaintingStyle.Stroke
                this.strokeWidth = strokeWidth
            }
            canvas.drawPath(path, strokePaint)
        }
    }

    drawPolygonOnCanvas(geometry.centerPoints, colorScheme.uFace, StrokeColor, StrokeWidth)

    for ((cubieIndex, edge) in geometry.edgeStickers.withIndex()) {
        drawPolygonOnCanvas(edge.top, getEdgeColor(cubieIndex, 0), StrokeColor, StrokeWidth)
        drawPolygonOnCanvas(edge.bottom, getEdgeColor(cubieIndex, 1), StrokeColor, StrokeWidth)
    }

    for ((cubieIndex, corner) in geometry.cornerStickers.withIndex()) {
        drawPolygonOnCanvas(corner.top, getCornerColor(cubieIndex, 0), StrokeColor, StrokeWidth)
        drawPolygonOnCanvas(
            corner.rightSticker,
            getCornerColor(cubieIndex, 1),
            StrokeColor,
            StrokeWidth
        )
        drawPolygonOnCanvas(
            corner.leftSticker,
            getCornerColor(cubieIndex, 2),
            StrokeColor,
            StrokeWidth
        )
    }

    for (i in 0 until 5) {
        val faceColor = colorScheme.stickerColors[faceColorIndexForRender(i)]
        val nextCornerIdx = (i + 1) % 5
        val outerEdgeMidpoint = Point(
            (geometry.outerCorners[i].x + geometry.outerCorners[nextCornerIdx].x) / 2f,
            (geometry.outerCorners[i].y + geometry.outerCorners[nextCornerIdx].y) / 2f
        )
        val edgeDx = geometry.outerCorners[nextCornerIdx].x - geometry.outerCorners[i].x
        val edgeDy = geometry.outerCorners[nextCornerIdx].y - geometry.outerCorners[i].y
        val edgeLength = sqrt(edgeDx * edgeDx + edgeDy * edgeDy)

        val normalX = edgeDy / edgeLength
        val normalY = -edgeDx / edgeLength

        val tangentX = edgeDx / edgeLength
        val tangentY = edgeDy / edgeLength

        val rectHeight = size * 0.06f
        val rectWidth = edgeLength * 0.4f

        val offsetDistance = rectHeight * 0.5f + StrokeWidth * 3f

        val rectCenter = Point(
            outerEdgeMidpoint.x + normalX * offsetDistance,
            outerEdgeMidpoint.y + normalY * offsetDistance
        )

        val hw = rectWidth / 2f
        val hh = rectHeight / 2f

        val p1 = Point(
            rectCenter.x - tangentX * hw - normalX * hh,
            rectCenter.y - tangentY * hw - normalY * hh
        )
        val p2 = Point(
            rectCenter.x + tangentX * hw - normalX * hh,
            rectCenter.y + tangentY * hw - normalY * hh
        )
        val p3 = Point(
            rectCenter.x + tangentX * hw + normalX * hh,
            rectCenter.y + tangentY * hw + normalY * hh
        )
        val p4 = Point(
            rectCenter.x - tangentX * hw + normalX * hh,
            rectCenter.y - tangentY * hw + normalY * hh
        )

        drawPolygonOnCanvas(listOf(p1, p2, p3, p4), faceColor, StrokeColor, StrokeWidth * 0.5f)
    }

    return bitmap
}
