package com.llminxsolver.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.GeneratorMode

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun SearchModeSelector(
    selectedModes: Set<GeneratorMode>,
    defaultPruningDepth: Int = 12,
    modePruningDepths: Map<GeneratorMode, Int> = emptyMap(),
    onModesChange: (Set<GeneratorMode>) -> Unit,
    onModePruningDepthChange: ((GeneratorMode, Int?) -> Unit)? = null,
    enabled: Boolean = true,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxWidth()) {
        Text(
            text = "Search Modes",
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.padding(bottom = 8.dp)
        )

        FlowRow(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            GeneratorMode.entries.forEach { mode ->
                val isSelected = selectedModes.contains(mode)
                FilterChip(
                    selected = isSelected,
                    onClick = {
                        if (enabled) {
                            val newModes = if (isSelected) {
                                if (selectedModes.size > 1) {
                                    selectedModes - mode
                                } else {
                                    selectedModes
                                }
                            } else {
                                selectedModes + mode
                            }
                            onModesChange(newModes)
                        }
                    },
                    label = { Text(mode.displayName) },
                    enabled = enabled,
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = MaterialTheme.colorScheme.primary,
                        selectedLabelColor = MaterialTheme.colorScheme.onPrimary
                    )
                )
            }
        }

        AnimatedVisibility(
            visible = selectedModes.size > 1 && onModePruningDepthChange != null
        ) {
            Column(
                modifier = Modifier.padding(top = 16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Text(
                    text = "Pruning Depth per Mode",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )

                selectedModes.forEach { mode ->
                    val currentDepth = modePruningDepths[mode] ?: defaultPruningDepth
                    Column {
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            horizontalArrangement = Arrangement.SpaceBetween
                        ) {
                            Text(
                                text = mode.displayName,
                                style = MaterialTheme.typography.bodySmall
                            )
                            Text(
                                text = "$currentDepth",
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.primary
                            )
                        }
                        Slider(
                            value = currentDepth.toFloat(),
                            onValueChange = { newValue ->
                                onModePruningDepthChange?.invoke(mode, newValue.toInt())
                            },
                            valueRange = 8f..18f,
                            steps = 9,
                            enabled = enabled
                        )
                    }
                }
            }
        }
    }
}
