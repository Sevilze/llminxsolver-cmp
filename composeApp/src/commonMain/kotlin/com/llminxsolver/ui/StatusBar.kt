package com.llminxsolver.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.SizeTransform
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.scaleIn
import androidx.compose.animation.scaleOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.LinearWavyProgressIndicator
import androidx.compose.material3.LoadingIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.SolverState

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun StatusBar(solverState: SolverState, modifier: Modifier = Modifier) {
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
            verticalArrangement = Arrangement.spacedBy(6.dp)
        ) {
            Box(
                modifier = Modifier.fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                Box(
                    modifier = Modifier.fillMaxWidth(),
                    contentAlignment = Alignment.CenterStart
                ) {
                    AnimatedContent(
                        targetState = solverState.isSearching,
                        transitionSpec = {
                            val enterAnim = scaleIn(
                                MotionScheme.expressive().defaultSpatialSpec()
                            ) + fadeIn()
                            val exitAnim = scaleOut(
                                MotionScheme.expressive().defaultSpatialSpec()
                            ) + fadeOut()
                            enterAnim.togetherWith(exitAnim)
                        },
                        label = "loading_icon"
                    ) { isSearching ->
                        if (isSearching) {
                            LoadingIndicator(
                                modifier = Modifier.size(24.dp),
                                color = MaterialTheme.colorScheme.primary
                            )
                        }
                    }
                }

                Text(
                    text = solverState.status.ifEmpty { "Ready" },
                    textAlign = TextAlign.Center,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }

            AnimatedContent(
                targetState = solverState.isSearching,
                transitionSpec = {
                    fadeIn(MotionScheme.expressive().defaultSpatialSpec()) togetherWith
                        fadeOut(MotionScheme.expressive().defaultSpatialSpec()) using
                        SizeTransform(clip = false)
                },
                label = "middle_section"
            ) { isSearching ->
                if (isSearching) {
                    val indicatorModifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 12.dp)

                    if (solverState.progress > 0f && solverState.progress < 1f) {
                        LinearWavyProgressIndicator(
                            progress = { solverState.progress },
                            modifier = indicatorModifier
                        )
                    } else {
                        LinearWavyProgressIndicator(
                            modifier = indicatorModifier
                        )
                    }
                } else {
                    Spacer(modifier = Modifier.height(16.dp))
                }
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
