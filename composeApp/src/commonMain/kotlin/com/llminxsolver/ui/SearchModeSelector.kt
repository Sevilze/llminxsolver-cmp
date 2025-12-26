package com.llminxsolver.ui

import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.GeneratorMode

@Composable
fun SearchModeSelector(
    selectedModes: Set<GeneratorMode>,
    onModesChange: (Set<GeneratorMode>) -> Unit,
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

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
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
    }
}
