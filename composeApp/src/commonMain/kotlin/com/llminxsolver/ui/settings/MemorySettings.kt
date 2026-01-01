package com.llminxsolver.ui.settings

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.platform.MemoryInfo

@Composable
internal fun MemoryTabContent(
    parallelConfig: ParallelConfig,
    memoryInfo: MemoryInfo?,
    availableCpus: Int,
    onParallelConfigChange: ((ParallelConfig) -> Unit)?
) {
    if (onParallelConfigChange == null) {
        Box(
            modifier = Modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = "Memory settings not available",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        return
    }

    Column(
        modifier = Modifier.fillMaxSize(),
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
                    text = "${parallelConfig.memoryBudgetMb} MB",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary
                )
            }
            val maxMemory = ((memoryInfo?.totalMB ?: 4096L) * 0.5f).coerceAtLeast(256f)
            Slider(
                value = parallelConfig.memoryBudgetMb.toFloat(),
                onValueChange = { newValue ->
                    onParallelConfigChange(
                        parallelConfig.copy(memoryBudgetMb = newValue.toInt())
                    )
                },
                valueRange = 64f..maxMemory,
                steps = 15
            )
        }

        Column {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = "Table Gen Threads",
                    style = MaterialTheme.typography.bodyMedium
                )
                Text(
                    text = "${parallelConfig.tableGenThreads}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary
                )
            }
            Slider(
                value = parallelConfig.tableGenThreads.toFloat(),
                onValueChange = { newValue ->
                    onParallelConfigChange(
                        parallelConfig.copy(tableGenThreads = newValue.toInt())
                    )
                },
                valueRange = 1f..availableCpus.toFloat().coerceAtLeast(2f),
                steps = (availableCpus - 2).coerceAtLeast(0)
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
                    text = "${parallelConfig.searchThreads}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.primary
                )
            }
            Slider(
                value = parallelConfig.searchThreads.toFloat(),
                onValueChange = { newValue ->
                    onParallelConfigChange(
                        parallelConfig.copy(searchThreads = newValue.toInt())
                    )
                },
                valueRange = 1f..availableCpus.toFloat().coerceAtLeast(2f),
                steps = (availableCpus - 2).coerceAtLeast(0)
            )
        }

        HorizontalDivider()

        Text(
            text = "Presets",
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            TextButton(
                onClick = {
                    val desktop = ParallelConfig.forDesktop(
                        availableCpus,
                        memoryInfo?.totalMB?.toInt() ?: 4096
                    )
                    onParallelConfigChange(desktop)
                }
            ) {
                Text("Desktop")
            }
            TextButton(
                onClick = {
                    onParallelConfigChange(ParallelConfig.forMobile())
                }
            ) {
                Text("Mobile")
            }
        }
    }
}
