package com.llminxsolver.ui.components

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.SizeTransform
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.LinearWavyProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.ModeProgress
import com.llminxsolver.data.SolverState

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun StatusBar(
    solverState: SolverState,
    onMultiModeExpandChange: ((Boolean) -> Unit)? = null,
    expandUp: Boolean = false,
    modifier: Modifier = Modifier
) {
    var multiModeExpanded by remember { mutableStateOf(false) }

    Surface(
        modifier = modifier
            .fillMaxWidth()
            .padding(8.dp),
        color = MaterialTheme.colorScheme.surfaceContainer,
        tonalElevation = 1.dp,
        shape = MaterialTheme.shapes.medium
    ) {
        Column(
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(6.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            AnimatedContent(
                targetState = solverState.isSearching,
                transitionSpec = {
                    fadeIn(MotionScheme.expressive().defaultSpatialSpec()) togetherWith
                        fadeOut(MotionScheme.expressive().defaultSpatialSpec()) using
                        SizeTransform(clip = false)
                },
                label = "search_content"
            ) { isSearching ->
                if (isSearching && solverState.modeProgresses.size > 1) {
                    MultiModeProgressSection(
                        modeProgresses = solverState.modeProgresses,
                        overallProgress = solverState.progress,
                        expanded = multiModeExpanded,
                        onExpandToggle = {
                            val newState = !multiModeExpanded
                            multiModeExpanded = newState
                            onMultiModeExpandChange?.invoke(newState)
                        },
                        expandUp = expandUp
                    )
                } else if (isSearching) {
                    SingleModeProgressSection(
                        status = solverState.status,
                        progress = solverState.progress
                    )
                } else {
                    Text(
                        text = solverState.status.ifEmpty { "Ready" },
                        textAlign = TextAlign.Center,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis
                    )
                }
            }

            if (!solverState.isSearching) {
                Spacer(modifier = Modifier.height(8.dp))
            }

            Box(
                modifier = Modifier.fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                MemoryIndicator()
            }
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun SingleModeProgressSection(status: String, progress: Float) {
    Column(
        verticalArrangement = Arrangement.spacedBy(6.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = status.ifEmpty { "Searching..." },
            textAlign = TextAlign.Center,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis
        )

        val indicatorModifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp)

        if (progress > 0f && progress < 1f) {
            LinearWavyProgressIndicator(
                progress = { progress },
                modifier = indicatorModifier
            )
        } else {
            LinearWavyProgressIndicator(
                modifier = indicatorModifier
            )
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun MultiModeProgressSection(
    modeProgresses: Map<String, ModeProgress>,
    overallProgress: Float,
    expanded: Boolean,
    onExpandToggle: () -> Unit,
    expandUp: Boolean
) {
    Column(
        verticalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        if (expandUp) {
            AnimatedVisibility(
                visible = expanded,
                enter = expandVertically(expandFrom = Alignment.Bottom) + fadeIn(),
                exit = shrinkVertically(shrinkTowards = Alignment.Bottom) + fadeOut()
            ) {
                Column(
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                    modifier = Modifier.padding(bottom = 4.dp)
                ) {
                    modeProgresses.forEach { (mode, progress) ->
                        CompactModeProgressItem(mode, progress)
                    }
                }
            }
        }

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable(onClick = onExpandToggle)
                .padding(vertical = 4.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "${modeProgresses.size} modes",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )

            Text(
                text = modeProgresses.values
                    .map { "D${it.currentDepth}" }
                    .joinToString(" "),
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.primary
            )

            Icon(
                imageVector = if (expanded) {
                    if (expandUp) Icons.Filled.KeyboardArrowDown else Icons.Filled.KeyboardArrowUp
                } else {
                    if (expandUp) Icons.Filled.KeyboardArrowUp else Icons.Filled.KeyboardArrowDown
                },
                contentDescription = if (expanded) "Collapse" else "Expand",
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        if (!expandUp) {
            AnimatedVisibility(
                visible = expanded,
                enter = expandVertically() + fadeIn(),
                exit = shrinkVertically() + fadeOut()
            ) {
                Column(
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                    modifier = Modifier.padding(top = 4.dp)
                ) {
                    modeProgresses.forEach { (mode, progress) ->
                        CompactModeProgressItem(mode, progress)
                    }
                }
            }
        }
    }
}

@Composable
private fun CompactModeProgressItem(mode: String, modeProgress: ModeProgress) {
    val etrMatch = Regex("""\(ETR: ([^)]+)\)""").find(modeProgress.message)
    val etrDisplay = etrMatch?.groupValues?.get(1) ?: "--"

    Column(
        verticalArrangement = Arrangement.spacedBy(2.dp)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = mode,
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.weight(1f)
            )
            Text(
                text = "D${modeProgress.currentDepth} • $etrDisplay",
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.primary
            )
        }
        LinearProgressIndicator(
            progress = { modeProgress.progress.coerceIn(0f, 1f) },
            modifier = Modifier
                .fillMaxWidth()
                .height(4.dp)
        )
    }
}
