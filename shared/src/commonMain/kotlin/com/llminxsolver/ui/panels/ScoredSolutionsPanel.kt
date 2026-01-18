package com.llminxsolver.ui.panels

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.scaleIn
import androidx.compose.animation.scaleOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.clickable
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Done
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalClipboard
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.components.SplitExportButton
import com.llminxsolver.ui.megaminx.renderMegaminxToImageBitmap
import com.llminxsolver.util.getDownloadDirectory
import com.llminxsolver.util.imageBitmapToPng
import com.llminxsolver.util.setPlainText
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import kotlin.math.roundToInt
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uniffi.llminxsolver.ScoredSolutionExport
import uniffi.llminxsolver.exportRawXlsxFromFile
import uniffi.llminxsolver.exportScoredXlsx

enum class SortOption(val label: String) {
    MCC("MCC"),
    MOVE_COUNT("Moves")
}

enum class ExportOption(val label: String, val filenamePrefix: String) {
    SCORED("Scored", "scored_solutions"),
    RAW("Raw", "raw_solutions")
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun ScoredSolutionsPanel(
    scoredSolutions: List<ScoredSolution>,
    tempFilePath: String? = null,
    megaminxState: MegaminxState? = null,
    colorScheme: MegaminxColorScheme = MegaminxColorScheme(),
    ignoreFlags: IgnoreFlags = IgnoreFlags(),
    imageResolution: Int = 200,
    metricLabel: String = "Moves",
    maxSolutions: Int = 5,
    listHeight: Int? = null,
    onFlushTempFile: (() -> Unit)? = null,
    onExportSuccess: ((String) -> Unit)? = null,
    modifier: Modifier = Modifier
) {
    var displayCount by remember { mutableIntStateOf(maxSolutions) }
    var copiedIndex by remember { mutableStateOf<Int?>(null) }
    var displayMetricLabel by remember { mutableStateOf(metricLabel) }
    var sortOption by remember { mutableStateOf(SortOption.MCC) }
    var exportOption by remember { mutableStateOf(ExportOption.SCORED) }

    LaunchedEffect(scoredSolutions.isEmpty()) {
        if (scoredSolutions.isEmpty()) {
            displayMetricLabel = metricLabel
        }
    }

    LaunchedEffect(copiedIndex) {
        if (copiedIndex != null) {
            delay(1000)
            copiedIndex = null
        }
    }

    val clipboard = LocalClipboard.current
    val scope = rememberCoroutineScope()

    val sortedSolutions = remember(scoredSolutions, sortOption) {
        when (sortOption) {
            SortOption.MCC -> scoredSolutions.sortedBy { it.mcc }
            SortOption.MOVE_COUNT -> scoredSolutions.sortedBy { it.moveCount }
        }
    }
    val displayedSolutions = sortedSolutions.take(displayCount)

    fun generateTimestamp(): String {
        val formatter = DateTimeFormatter.ofPattern("yyyyMMdd_HHmmss")
        return LocalDateTime.now().format(formatter)
    }

    fun performExport() {
        val timestamp = generateTimestamp()
        val xlsxFilename = "${exportOption.filenamePrefix}_$timestamp.xlsx"
        val outputPath = "${getDownloadDirectory()}/$xlsxFilename"

        val imagePngBytes = megaminxState?.let {
            val bitmap = renderMegaminxToImageBitmap(
                puzzleState = it,
                colorScheme = colorScheme,
                ignoreFlags = ignoreFlags,
                size = imageResolution
            )
            imageBitmapToPng(bitmap).map { b -> b.toUByte() }
        }

        val error = when (exportOption) {
            ExportOption.SCORED -> {
                val exports = displayedSolutions.map { solution ->
                    ScoredSolutionExport(
                        mcc = solution.mcc.toDouble(),
                        moveCount = solution.moveCount.toUInt(),
                        algorithm = solution.algorithm.substringBefore("(").trim()
                    )
                }
                exportScoredXlsx(outputPath, exports, imagePngBytes, imageResolution.toUInt())
            }

            ExportOption.RAW -> {
                if (tempFilePath != null) {
                    onFlushTempFile?.invoke()
                    exportRawXlsxFromFile(
                        outputPath,
                        tempFilePath,
                        imagePngBytes,
                        imageResolution.toUInt()
                    )
                } else {
                    "No solutions file available"
                }
            }
        }

        if (error == null) {
            onExportSuccess?.invoke(xlsxFilename)
        }
    }

    Card(
        modifier = modifier.fillMaxSize(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Column(modifier = Modifier.fillMaxSize()) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp, vertical = 12.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(4.dp)
                ) {
                    Text(
                        text = "Scored Algorithms",
                        style = MaterialTheme.typography.titleSmall
                    )
                    if (scoredSolutions.isNotEmpty()) {
                        Text(
                            text = "(${minOf(displayCount, scoredSolutions.size)}" +
                                " of ${scoredSolutions.size})",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.Center
                ) {
                    Text(
                        text = "Top",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Slider(
                        value = displayCount.toFloat(),
                        onValueChange = { displayCount = it.roundToInt() },
                        valueRange = 5f..100f,
                        modifier = Modifier.width(100.dp)
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = displayCount.toString(),
                        style = MaterialTheme.typography.bodySmall,
                        fontWeight = FontWeight.Medium,
                        color = MaterialTheme.colorScheme.primary
                    )
                }
            }

            val showHeadersAndButton = scoredSolutions.isNotEmpty() || tempFilePath != null

            if (showHeadersAndButton) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Box(
                        modifier = Modifier.width(60.dp),
                        contentAlignment = Alignment.CenterStart
                    ) {
                        SortableHeaderCell(
                            text = "MCC",
                            isSelected = sortOption == SortOption.MCC,
                            onClick = { sortOption = SortOption.MCC },
                            modifier = Modifier.wrapContentWidth()
                        )
                    }

                    Box(
                        modifier = Modifier.width(60.dp).padding(horizontal = 4.dp),
                        contentAlignment = Alignment.CenterStart
                    ) {
                        SortableHeaderCell(
                            text = displayMetricLabel,
                            isSelected = sortOption == SortOption.MOVE_COUNT,
                            onClick = { sortOption = SortOption.MOVE_COUNT },
                            modifier = Modifier.wrapContentWidth()
                        )
                    }

                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = "Algorithm",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.weight(1f)
                    )

                    SplitExportButton(
                        options = ExportOption.entries.toList(),
                        selectedOption = exportOption,
                        onOptionSelected = { exportOption = it },
                        onExport = { performExport() },
                        optionLabel = { it.label },
                        enabled = true
                    )
                }
            }

            if (scoredSolutions.isEmpty()) {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(32.dp),
                    contentAlignment = Alignment.Center
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        Text(
                            text = "No solutions to score yet.",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Text(
                            text = "Run the solver to generate algorithms.",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.outline
                        )
                    }
                }
            } else {
                LazyColumn(
                    modifier = Modifier
                        .fillMaxWidth()
                        .let {
                            if (listHeight != null) {
                                it.height(listHeight.dp)
                            } else {
                                it.weight(1f)
                            }
                        },
                    contentPadding = PaddingValues(bottom = 8.dp)
                ) {
                    itemsIndexed(displayedSolutions) { index, solution ->
                        ScoredSolutionRow(
                            solution = solution,
                            metricLabel = displayMetricLabel,
                            isCopied = copiedIndex == index,
                            onCopy = {
                                val algorithmOnly = solution.algorithm.substringBefore("(").trim()
                                scope.launch { clipboard.setPlainText(algorithmOnly) }
                                copiedIndex = index
                            }
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun SortableHeaderCell(
    text: String,
    isSelected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier
            .clip(MaterialTheme.shapes.small)
            .clickable(onClick = onClick)
            .padding(vertical = 4.dp, horizontal = 4.dp),
        contentAlignment = Alignment.CenterStart
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                text = text,
                style = MaterialTheme.typography.labelSmall,
                fontWeight = if (isSelected) FontWeight.Bold else FontWeight.Normal,
                color = if (isSelected) {
                    MaterialTheme.colorScheme.primary
                } else {
                    MaterialTheme.colorScheme.onSurfaceVariant
                },
                textAlign = TextAlign.Start
            )

            if (isSelected) {
                Spacer(modifier = Modifier.width(4.dp))
                Icon(
                    imageVector = Icons.Filled.KeyboardArrowDown,
                    contentDescription = null,
                    modifier = Modifier.size(14.dp),
                    tint = MaterialTheme.colorScheme.primary
                )
            }
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun ScoredSolutionRow(
    solution: ScoredSolution,
    metricLabel: String,
    isCopied: Boolean,
    onCopy: () -> Unit,
    modifier: Modifier = Modifier
) {
    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    Row(
        modifier = modifier
            .fillMaxWidth()
            .hoverable(interactionSource)
            .clickable(onClick = onCopy)
            .padding(horizontal = 16.dp, vertical = 6.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Box(
            modifier = Modifier.width(60.dp),
            contentAlignment = Alignment.CenterStart
        ) {
            Text(
                text = String.format("%.1f", solution.mcc),
                style = MaterialTheme.typography.bodySmall,
                fontWeight = FontWeight.Medium,
                color = MaterialTheme.colorScheme.tertiary,
                fontFamily = FontFamily.Monospace,
                modifier = Modifier.padding(start = 4.dp)
            )
        }

        Box(
            modifier = Modifier.width(60.dp).padding(horizontal = 12.dp)
        ) {
            Text(
                text = solution.moveCount.toString(),
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                fontFamily = FontFamily.Monospace,
                textAlign = TextAlign.Center
            )
        }

        Spacer(modifier = Modifier.width(8.dp))

        Text(
            text = solution.algorithm,
            style = MaterialTheme.typography.bodySmall,
            fontFamily = FontFamily.Monospace,
            modifier = Modifier.weight(1f)
        )

        AnimatedContent(
            targetState = isCopied,
            transitionSpec = {
                (scaleIn(spring(stiffness = Spring.StiffnessMediumLow)) + fadeIn())
                    .togetherWith(
                        scaleOut(spring(stiffness = Spring.StiffnessMediumLow)) + fadeOut()
                    )
            },
            modifier = Modifier.width(40.dp),
            contentAlignment = Alignment.CenterEnd
        ) { copied ->
            if (copied) {
                Icon(
                    imageVector = Icons.Filled.Done,
                    contentDescription = "Copied",
                    modifier = Modifier.size(18.dp),
                    tint = MaterialTheme.colorScheme.primary
                )
            } else {
                IconButton(
                    onClick = onCopy,
                    modifier = Modifier.size(32.dp)
                ) {
                    Icon(
                        imageVector = Icons.Filled.ContentCopy,
                        contentDescription = "Copy",
                        modifier = Modifier.size(16.dp),
                        tint = if (isHovered) {
                            MaterialTheme.colorScheme.primary
                        } else {
                            MaterialTheme.colorScheme.outline
                        }
                    )
                }
            }
        }
    }
}
