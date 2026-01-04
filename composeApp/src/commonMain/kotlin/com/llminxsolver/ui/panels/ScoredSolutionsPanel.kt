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
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Done
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
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.ScoredSolution
import kotlin.math.roundToInt
import kotlinx.coroutines.delay

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun ScoredSolutionsPanel(
    scoredSolutions: List<ScoredSolution>,
    metricLabel: String = "Moves",
    maxSolutions: Int = 20,
    listHeight: Int? = null,
    modifier: Modifier = Modifier
) {
    var displayCount by remember { mutableIntStateOf(maxSolutions) }
    var copiedIndex by remember { mutableStateOf<Int?>(null) }
    var displayMetricLabel by remember { mutableStateOf(metricLabel) }

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

    @Suppress("DEPRECATION")
    val clipboardManager = LocalClipboardManager.current

    val displayedSolutions = scoredSolutions.take(displayCount)

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
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Top",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Slider(
                        value = displayCount.toFloat(),
                        onValueChange = { displayCount = it.roundToInt() },
                        valueRange = 5f..100f,
                        modifier = Modifier.width(100.dp)
                    )
                    Text(
                        text = displayCount.toString(),
                        style = MaterialTheme.typography.bodySmall,
                        fontWeight = FontWeight.Medium,
                        color = MaterialTheme.colorScheme.primary
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
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "MCC",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.width(48.dp)
                    )
                    Text(
                        text = displayMetricLabel,
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.width(48.dp)
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = "Algorithm",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.weight(1f)
                    )
                    Box(modifier = Modifier.width(40.dp))
                }

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
                                clipboardManager.setText(AnnotatedString(algorithmOnly))
                                copiedIndex = index
                            }
                        )
                    }
                }
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
        Text(
            text = String.format("%.1f", solution.mcc),
            style = MaterialTheme.typography.bodySmall,
            fontWeight = FontWeight.Medium,
            color = MaterialTheme.colorScheme.tertiary,
            fontFamily = FontFamily.Monospace,
            modifier = Modifier.width(48.dp)
        )

        Text(
            text = solution.moveCount.toString(),
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            fontFamily = FontFamily.Monospace,
            modifier = Modifier.width(48.dp)
        )
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
            modifier = Modifier.width(40.dp)
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
