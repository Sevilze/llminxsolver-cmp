package com.llminxsolver.ui.megaminx

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Fill
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.StickerInfo
import com.llminxsolver.data.StickerType
import com.llminxsolver.theme.CornerColorMap
import com.llminxsolver.theme.EdgeColorMap
import com.llminxsolver.theme.HighlightColor
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.theme.MegaminxColors
import com.llminxsolver.theme.SelectionColor
import com.llminxsolver.theme.StickerColors
import com.llminxsolver.theme.StrokeColor
import com.llminxsolver.theme.StrokeWidth
import com.llminxsolver.theme.StrokeWidthSelected
import kotlin.math.PI
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

data class Point(val x: Float, val y: Float)

data class EdgeSticker(val top: List<Point>, val bottom: List<Point>)

data class CornerSticker(
    val top: List<Point>,
    val leftSide: List<Point>,
    val rightSide: List<Point>
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

private fun pointAt(center: Point, distance: Float, angle: Float): Point = Point(
    x = center.x + distance * cos(angle),
    y = center.y + distance * sin(angle)
)

private fun lerp(p1: Point, p2: Point, fraction: Float): Point = Point(
    x = p1.x * (1 - fraction) + p2.x * fraction,
    y = p1.y * (1 - fraction) + p2.y * fraction
)

private fun distance(p1: Point, p2: Point): Float {
    val dx = p2.x - p1.x
    val dy = p2.y - p1.y
    return sqrt(dx * dx + dy * dy)
}

private fun lineIntersection(
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

private fun calculateGeometry(width: Float, height: Float, padding: Float = 10f): MegaminxGeometry {
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
                leftSide =
                    listOf(
                        middleCorners[i],
                        middleEdgesRight[i],
                        leftOuterCorner,
                        outerCorners[i]
                    ),
                rightSide =
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

private fun DrawScope.drawPolygon(
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

private fun DrawScope.drawRoundedPolygon(
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

private fun getCenterOfPoints(points: List<Point>): Point {
    if (points.isEmpty()) return Point(0f, 0f)
    val sumX = points.sumOf { it.x.toDouble() }.toFloat()
    val sumY = points.sumOf { it.y.toDouble() }.toFloat()
    return Point(sumX / points.size, sumY / points.size)
}

private fun pointInPolygon(point: Point, polygon: List<Point>): Boolean {
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

private fun faceColorIndex(cornerIndex: Int): Int = when (cornerIndex) {
    0 -> 4
    1 -> 5
    2 -> 1
    3 -> 2
    4 -> 3
    else -> 0
}

@Composable
fun MegaminxViewer(
    puzzleState: MegaminxState,
    ignoreFlags: IgnoreFlags,
    onSwapCorners: (Int, Int) -> Unit = { _, _ -> },
    onRotateCorner: (Int, Int) -> Unit = { _, _ -> },
    onSwapEdges: (Int, Int) -> Unit = { _, _ -> },
    onFlipEdge: (Int) -> Unit = { },
    colorScheme: MegaminxColorScheme = MegaminxColorScheme.Classic,
    enabled: Boolean = true,
    modifier: Modifier = Modifier
) {
    var canvasSize by remember { mutableStateOf(300f) }
    var geometry by remember { mutableStateOf<MegaminxGeometry?>(null) }
    var selectedSticker by remember { mutableStateOf<StickerInfo?>(null) }
    var hoveredSticker by remember { mutableStateOf<StickerInfo?>(null) }
    var isDragging by remember { mutableStateOf(false) }
    var dragPosition by remember { mutableStateOf(Offset.Zero) }

    fun getCornerColor(cubieIndex: Int, orientationIndex: Int): Color {
        val position = puzzleState.cornerPositions[cubieIndex]
        val orientation = puzzleState.cornerOrientations[cubieIndex]
        val effectiveOrientation = (orientationIndex - orientation + 3) % 3

        if (ignoreFlags.cornerPositions) {
            if (effectiveOrientation != 0 || ignoreFlags.cornerOrientations) {
                return MegaminxColors.Gray
            }
        }

        if (ignoreFlags.cornerOrientations) {
            return MegaminxColors.Gray
        }

        return colorScheme.stickerColors[CornerColorMap[position][effectiveOrientation]]
    }

    fun getEdgeColor(cubieIndex: Int, orientationIndex: Int): Color {
        val position = puzzleState.edgePositions[cubieIndex]
        val orientation = puzzleState.edgeOrientations[cubieIndex]
        val effectiveOrientation = kotlin.math.abs(orientationIndex - orientation)

        if (ignoreFlags.edgePositions) {
            if (effectiveOrientation != 0 || ignoreFlags.edgeOrientations) {
                return MegaminxColors.Gray
            }
        }

        if (ignoreFlags.edgeOrientations) {
            return MegaminxColors.Gray
        }

        return colorScheme.stickerColors[EdgeColorMap[position][effectiveOrientation]]
    }

    fun findStickerAt(position: Offset): StickerInfo? {
        val geo = geometry ?: return null
        val point = Point(position.x, position.y)

        for ((cubieIndex, corner) in geo.cornerStickers.withIndex()) {
            val sides = listOf(corner.top to 0, corner.rightSide to 1, corner.leftSide to 2)
            for ((polygon, orientationIndex) in sides) {
                if (pointInPolygon(point, polygon)) {
                    return StickerInfo(StickerType.Corner, cubieIndex, orientationIndex)
                }
            }
        }

        for ((cubieIndex, edge) in geo.edgeStickers.withIndex()) {
            val sides = listOf(edge.top to 0, edge.bottom to 1)
            for ((polygon, orientationIndex) in sides) {
                if (pointInPolygon(point, polygon)) {
                    return StickerInfo(StickerType.Edge, cubieIndex, orientationIndex)
                }
            }
        }

        return null
    }

    fun handleInteraction() {
        if (!isDragging || selectedSticker == null || hoveredSticker == null || !enabled) return

        val selected = selectedSticker!!
        val hovered = hoveredSticker!!

        if (selected.type == hovered.type) {
            when (selected.type) {
                StickerType.Corner -> {
                    if (selected.cubieIndex == hovered.cubieIndex) {
                        val direction = if ((
                                hovered.orientationIndex - selected.orientationIndex +
                                    3
                                ) %
                            3 ==
                            1
                        ) {
                            1
                        } else {
                            -1
                        }
                        onRotateCorner(selected.cubieIndex, direction)
                    } else {
                        onSwapCorners(selected.cubieIndex, hovered.cubieIndex)
                    }
                }

                StickerType.Edge -> {
                    if (selected.cubieIndex == hovered.cubieIndex) {
                        onFlipEdge(selected.cubieIndex)
                    } else {
                        onSwapEdges(selected.cubieIndex, hovered.cubieIndex)
                    }
                }

                else -> {}
            }
        }
    }

    Canvas(
        modifier =
            modifier
                .fillMaxWidth()
                .aspectRatio(1f)
                .pointerInput(enabled) {
                    if (!enabled) return@pointerInput
                    detectTapGestures(
                        onPress = { offset ->
                            val sticker = findStickerAt(offset)
                            if (sticker != null) {
                                selectedSticker = sticker
                                isDragging = true
                            }
                        }
                    )
                }.pointerInput(enabled) {
                    if (!enabled) return@pointerInput
                    detectDragGestures(
                        onDragStart = { offset ->
                            val sticker = findStickerAt(offset)
                            if (sticker != null) {
                                selectedSticker = sticker
                                isDragging = true
                                dragPosition = offset
                            }
                        },
                        onDrag = { change, _ ->
                            dragPosition = change.position
                            hoveredSticker = findStickerAt(change.position)
                        },
                        onDragEnd = {
                            handleInteraction()
                            selectedSticker = null
                            hoveredSticker = null
                            isDragging = false
                        },
                        onDragCancel = {
                            selectedSticker = null
                            hoveredSticker = null
                            isDragging = false
                        }
                    )
                }
    ) {
        val newSize = minOf(size.width, size.height)
        if (newSize != canvasSize || geometry == null) {
            canvasSize = newSize
            val halfSize = newSize / 2f
            val rectHeight = newSize * 0.06f
            val rectWidth = newSize * 0.15f
            // Distance from center to outer edge midpoint is ~ outerRadius * cos(36)
            // But we can just use the previous logic maxR.

            // The indicator center is at distance: outerEdgeMidpoint + normal * offset
            // offset = rectHeight/2 + strokeWidth * 2
            // reach = offset + rectHeight/2

            val indicatorReach = rectHeight + StrokeWidth * 3f
            val cos36 = cos(PI / 5.0).toFloat()

            // Ensure R * cos36 + indicatorReach <= halfSize
            val maxR = (halfSize - indicatorReach) / cos36
            val outerRadius = minOf(halfSize - 10f, maxR)
            val padding = halfSize - outerRadius

            geometry = calculateGeometry(newSize, newSize, padding)
        }

        val geo = geometry ?: return@Canvas

        drawPolygon(
            geo.centerPoints,
            colorScheme.uFace,
            StrokeColor,
            StrokeWidth
        )

        val edgeSides = listOf("top" to 0, "bottom" to 1)
        for ((cubieIndex, edge) in geo.edgeStickers.withIndex()) {
            for ((sideName, orientationIndex) in edgeSides) {
                val points = if (sideName == "top") edge.top else edge.bottom
                val info = StickerInfo(StickerType.Edge, cubieIndex, orientationIndex)

                val isSelected =
                    selectedSticker?.type == StickerType.Edge &&
                        selectedSticker?.cubieIndex == cubieIndex &&
                        selectedSticker?.orientationIndex == orientationIndex

                val isHovered =
                    hoveredSticker?.type == StickerType.Edge &&
                        hoveredSticker?.cubieIndex == cubieIndex &&
                        hoveredSticker?.orientationIndex == orientationIndex

                val isTarget =
                    isDragging && selectedSticker != null &&
                        selectedSticker!!.type == StickerType.Edge &&
                        isHovered

                if ((isTarget || (isSelected && isDragging)) && selectedSticker != null) {
                    val highlightPoints = if (selectedSticker!!.orientationIndex ==
                        0
                    ) {
                        edge.top
                    } else {
                        edge.bottom
                    }
                    drawPolygon(highlightPoints, HighlightColor, Color.Transparent, 0f)
                }

                val color = getEdgeColor(cubieIndex, orientationIndex)
                val strokeCol = if (isSelected) SelectionColor else StrokeColor
                val strokeW = if (isSelected) StrokeWidthSelected else StrokeWidth

                drawPolygon(points, color, strokeCol, strokeW)
            }
        }

        val cornerSides = listOf("top" to 0, "right" to 1, "left" to 2)
        for ((cubieIndex, corner) in geo.cornerStickers.withIndex()) {
            for ((sideName, orientationIndex) in cornerSides) {
                val points =
                    when (sideName) {
                        "top" -> corner.top
                        "right" -> corner.rightSide
                        else -> corner.leftSide
                    }

                val isSelected =
                    selectedSticker?.type == StickerType.Corner &&
                        selectedSticker?.cubieIndex == cubieIndex &&
                        selectedSticker?.orientationIndex == orientationIndex

                val isHovered =
                    hoveredSticker?.type == StickerType.Corner &&
                        hoveredSticker?.cubieIndex == cubieIndex &&
                        hoveredSticker?.orientationIndex == orientationIndex

                val isTarget =
                    isDragging && selectedSticker != null &&
                        selectedSticker!!.type == StickerType.Corner &&
                        isHovered

                if ((isTarget || (isSelected && isDragging)) && selectedSticker != null) {
                    val highlightPoints =
                        when (selectedSticker!!.orientationIndex) {
                            0 -> corner.top
                            1 -> corner.rightSide
                            else -> corner.leftSide
                        }
                    drawPolygon(highlightPoints, HighlightColor, Color.Transparent, 0f)
                }

                val color = getCornerColor(cubieIndex, orientationIndex)
                val strokeCol = if (isSelected) SelectionColor else StrokeColor
                val strokeW = if (isSelected) StrokeWidthSelected else StrokeWidth

                drawPolygon(points, color, strokeCol, strokeW)
            }
        }

        for (i in 0 until 5) {
            val faceColor = colorScheme.stickerColors[faceColorIndex(i)]
            val nextCornerIdx = (i + 1) % 5
            val outerEdgeMidpoint = Point(
                (geo.outerCorners[i].x + geo.outerCorners[nextCornerIdx].x) / 2f,
                (geo.outerCorners[i].y + geo.outerCorners[nextCornerIdx].y) / 2f
            )
            val edgeDx = geo.outerCorners[nextCornerIdx].x - geo.outerCorners[i].x
            val edgeDy = geo.outerCorners[nextCornerIdx].y - geo.outerCorners[i].y
            val edgeLength = sqrt(edgeDx * edgeDx + edgeDy * edgeDy)

            val normalX = edgeDy / edgeLength
            val normalY = -edgeDx / edgeLength

            val tangentX = edgeDx / edgeLength
            val tangentY = edgeDy / edgeLength

            val rectHeight = canvasSize * 0.06f
            val rectWidth = edgeLength * 0.4f

            val offsetDistance = rectHeight * 0.5f + StrokeWidth * 3f

            val rectCenter = Point(
                outerEdgeMidpoint.x + normalX * offsetDistance,
                outerEdgeMidpoint.y + normalY * offsetDistance
            )

            // Calculate 4 corners of the rotated rectangle
            // p1: center - width/2 * tangent - height/2 * normal
            // p2: center + width/2 * tangent - height/2 * normal
            // p3: center + width/2 * tangent + height/2 * normal
            // p4: center - width/2 * tangent + height/2 * normal
            // Wait, "normal" points OUT.
            // If we want "strip", maybe height is small dimension along normal?
            // yes.

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

            drawRoundedPolygon(
                listOf(p1, p2, p3, p4),
                faceColor,
                StrokeColor,
                StrokeWidth * 0.5f,
                cornerRadius = rectHeight * 0.3f
            )
        }

        if (isDragging && selectedSticker != null && hoveredSticker != null &&
            selectedSticker!!.type == hoveredSticker!!.type
        ) {
            val selected = selectedSticker!!
            val hovered = hoveredSticker!!

            val startPoints: List<Point>
            val endPoints: List<Point>

            when (selected.type) {
                StickerType.Corner -> {
                    val startCorner = geo.cornerStickers[selected.cubieIndex]
                    val endCorner = geo.cornerStickers[hovered.cubieIndex]
                    startPoints =
                        when (selected.orientationIndex) {
                            0 -> startCorner.top
                            1 -> startCorner.rightSide
                            else -> startCorner.leftSide
                        }
                    endPoints =
                        when (hovered.orientationIndex) {
                            0 -> endCorner.top
                            1 -> endCorner.rightSide
                            else -> endCorner.leftSide
                        }
                }

                StickerType.Edge -> {
                    val startEdge = geo.edgeStickers[selected.cubieIndex]
                    val endEdge = geo.edgeStickers[hovered.cubieIndex]
                    startPoints =
                        if (selected.orientationIndex == 0) startEdge.top else startEdge.bottom
                    endPoints = if (hovered.orientationIndex == 0) endEdge.top else endEdge.bottom
                }

                else -> {
                    return@Canvas
                }
            }

            val start = getCenterOfPoints(startPoints)
            val end = getCenterOfPoints(endPoints)
            val angle = atan2((end.y - start.y).toDouble(), (end.x - start.x).toDouble()).toFloat()
            val arrowSize = 10f

            drawLine(
                color = SelectionColor,
                start = Offset(start.x, start.y),
                end = Offset(end.x, end.y),
                strokeWidth = 3f
            )

            val arrowPath =
                Path().apply {
                    moveTo(end.x, end.y)
                    lineTo(
                        end.x - arrowSize * cos(angle - PI.toFloat() / 6),
                        end.y - arrowSize * sin(angle - PI.toFloat() / 6)
                    )
                    lineTo(
                        end.x - arrowSize * cos(angle + PI.toFloat() / 6),
                        end.y - arrowSize * sin(angle + PI.toFloat() / 6)
                    )
                    close()
                }
            drawPath(arrowPath, SelectionColor)

            if (selected.cubieIndex != hovered.cubieIndex) {
                val reverseArrowPath =
                    Path().apply {
                        moveTo(start.x, start.y)
                        lineTo(
                            start.x + arrowSize * cos(angle - PI.toFloat() / 6),
                            start.y + arrowSize * sin(angle - PI.toFloat() / 6)
                        )
                        lineTo(
                            start.x + arrowSize * cos(angle + PI.toFloat() / 6),
                            start.y + arrowSize * sin(angle + PI.toFloat() / 6)
                        )
                        close()
                    }
                drawPath(reverseArrowPath, SelectionColor)
            }
        }
    }
}
