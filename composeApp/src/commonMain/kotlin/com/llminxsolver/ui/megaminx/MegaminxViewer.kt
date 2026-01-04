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
import androidx.compose.ui.input.pointer.pointerInput
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.StickerInfo
import com.llminxsolver.data.StickerType
import com.llminxsolver.theme.CornerColorMap
import com.llminxsolver.theme.EdgeColorMap
import com.llminxsolver.theme.HighlightColor
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.theme.SelectionColor
import com.llminxsolver.theme.StrokeColor
import com.llminxsolver.theme.StrokeWidth
import com.llminxsolver.theme.StrokeWidthSelected
import kotlin.math.PI
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sqrt

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
    colorScheme: MegaminxColorScheme = MegaminxColorScheme(),
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

    fun findStickerAt(position: Offset): StickerInfo? {
        val geo = geometry ?: return null
        val point = Point(position.x, position.y)

        for ((cubieIndex, corner) in geo.cornerStickers.withIndex()) {
            val sides = listOf(corner.top to 0, corner.rightSticker to 1, corner.leftSticker to 2)
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

            val indicatorReach = rectHeight + StrokeWidth * 3f
            val cos36 = cos(PI / 5.0).toFloat()

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
                        "right" -> corner.rightSticker
                        else -> corner.leftSticker
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
                            1 -> corner.rightSticker
                            else -> corner.leftSticker
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
                            1 -> startCorner.rightSticker
                            else -> startCorner.leftSticker
                        }
                    endPoints =
                        when (hovered.orientationIndex) {
                            0 -> endCorner.top
                            1 -> endCorner.rightSticker
                            else -> endCorner.leftSticker
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
                        end.y - arrowSize * kotlin.math.sin(angle - PI.toFloat() / 6)
                    )
                    lineTo(
                        end.x - arrowSize * cos(angle + PI.toFloat() / 6),
                        end.y - arrowSize * kotlin.math.sin(angle + PI.toFloat() / 6)
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
                            start.y + arrowSize * kotlin.math.sin(angle - PI.toFloat() / 6)
                        )
                        lineTo(
                            start.x + arrowSize * cos(angle + PI.toFloat() / 6),
                            start.y + arrowSize * kotlin.math.sin(angle + PI.toFloat() / 6)
                        )
                        close()
                    }
                drawPath(reverseArrowPath, SelectionColor)
            }
        }
    }
}
