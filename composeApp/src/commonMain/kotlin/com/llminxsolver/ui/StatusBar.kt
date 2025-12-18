package com.llminxsolver.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.scaleIn
import androidx.compose.animation.scaleOut
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.LinearWavyProgressIndicator
import androidx.compose.material3.LoadingIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.SolverState

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun StatusBar(solverState: SolverState, modifier: Modifier = Modifier) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        color = MaterialTheme.colorScheme.surfaceContainer,
        tonalElevation = 1.dp
    ) {
        Column(
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            AnimatedVisibility(
                visible = solverState.isSearching,
                enter = fadeIn(spring(stiffness = Spring.StiffnessLow)),
                exit = fadeOut(spring(stiffness = Spring.StiffnessLow))
            ) {
                if (solverState.progress > 0f && solverState.progress < 1f) {
                    LinearWavyProgressIndicator(
                        progress = { solverState.progress },
                        modifier = Modifier.fillMaxWidth()
                    )
                } else {
                    LinearWavyProgressIndicator(
                        modifier = Modifier.fillMaxWidth()
                    )
                }
            }

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                AnimatedContent(
                    targetState = solverState.isSearching,
                    transitionSpec = {
                        (scaleIn(spring(stiffness = Spring.StiffnessMediumLow)) + fadeIn())
                            .togetherWith(
                                scaleOut(spring(stiffness = Spring.StiffnessMediumLow)) + fadeOut()
                            )
                    }
                ) { isSearching ->
                    if (isSearching) {
                        LoadingIndicator(
                            modifier = Modifier.size(24.dp),
                            color = MaterialTheme.colorScheme.primary
                        )
                    } else {
                        Box(modifier = Modifier.size(24.dp))
                    }
                }

                Text(
                    text = solverState.status.ifEmpty { "Ready" },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
    }
}
