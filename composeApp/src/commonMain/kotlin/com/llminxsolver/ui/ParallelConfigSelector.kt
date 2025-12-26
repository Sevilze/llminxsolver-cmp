package com.llminxsolver.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.platform.MemoryInfo

@Composable
fun ParallelConfigSelector(
    config: ParallelConfig,
    memoryInfo: MemoryInfo?,
    availableCpus: Int,
    onConfigChange: (ParallelConfig) -> Unit,
    enabled: Boolean = true,
    modifier: Modifier = Modifier
) {
    var expanded by remember { mutableStateOf(false) }

    Column(modifier = modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Parallel Settings",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )

            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(
                    text = "${config.searchThreads} threads",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                IconButton(onClick = { expanded = !expanded }) {
                    Icon(
                        imageVector = if (expanded) {
                            Icons.Default.KeyboardArrowUp
                        } else {
                            Icons.Default.KeyboardArrowDown
                        },
                        contentDescription = if (expanded) "Collapse" else "Expand"
                    )
                }
            }
        }

        AnimatedVisibility(visible = expanded) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = 8.dp),
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                Column {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text(
                            text = "Memory Budget",
                            style = MaterialTheme.typography.bodyMedium
                        )
                        Text(
                            text = "${config.memoryBudgetMb} MB",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.primary
                        )
                    }
                    val maxMemory = (memoryInfo?.totalMB ?: 4096L).toFloat()
                    Slider(
                        value = config.memoryBudgetMb.toFloat(),
                        onValueChange = { newValue ->
                            onConfigChange(config.copy(memoryBudgetMb = newValue.toInt()))
                        },
                        valueRange = 64f..maxMemory.coerceAtMost(8192f),
                        steps = 15,
                        enabled = enabled
                    )
                }

                Column {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text(
                            text = "Table Generation Threads",
                            style = MaterialTheme.typography.bodyMedium
                        )
                        Text(
                            text = "${config.tableGenThreads}",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.primary
                        )
                    }
                    Slider(
                        value = config.tableGenThreads.toFloat(),
                        onValueChange = { newValue ->
                            onConfigChange(config.copy(tableGenThreads = newValue.toInt()))
                        },
                        valueRange = 1f..availableCpus.toFloat().coerceAtLeast(2f),
                        steps = (availableCpus - 2).coerceAtLeast(0),
                        enabled = enabled
                    )
                }

                Column {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween
                    ) {
                        Text(
                            text = "Search Threads",
                            style = MaterialTheme.typography.bodyMedium
                        )
                        Text(
                            text = "${config.searchThreads}",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.primary
                        )
                    }
                    Slider(
                        value = config.searchThreads.toFloat(),
                        onValueChange = { newValue ->
                            onConfigChange(config.copy(searchThreads = newValue.toInt()))
                        },
                        valueRange = 1f..availableCpus.toFloat().coerceAtLeast(2f),
                        steps = (availableCpus - 2).coerceAtLeast(0),
                        enabled = enabled
                    )
                }

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    TextButton(
                        onClick = {
                            val desktopConfig = ParallelConfig.forDesktop(
                                availableCpus,
                                memoryInfo?.totalMB?.toInt() ?: 4096
                            )
                            onConfigChange(desktopConfig)
                        },
                        enabled = enabled
                    ) {
                        Text("Desktop Preset")
                    }
                    TextButton(
                        onClick = {
                            onConfigChange(ParallelConfig.forMobile())
                        },
                        enabled = enabled
                    ) {
                        Text("Mobile Preset")
                    }
                }
            }
        }
    }
}
