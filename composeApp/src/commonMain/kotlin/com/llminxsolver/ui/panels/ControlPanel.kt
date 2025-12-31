package com.llminxsolver.ui.panels

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.GeneratorMode
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.SolverConfig
import com.llminxsolver.ui.components.MetricSelector
import com.llminxsolver.ui.components.SearchDepthSelector
import com.llminxsolver.ui.components.SearchModeSelector

@Composable
fun ControlPanel(
    config: SolverConfig,
    isSearching: Boolean,
    onSelectedModesChange: (Set<GeneratorMode>) -> Unit,
    onMetricChange: (MetricType) -> Unit,
    onLimitDepthChange: (Boolean) -> Unit,
    onMaxDepthChange: (Int) -> Unit,
    onIgnoreFlagChange: (String, Boolean) -> Unit,
    onReset: () -> Unit,
    onSolve: () -> Unit,
    onCancel: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(20.dp)
    ) {
        SearchModeSelector(
            selectedModes = config.selectedModes,
            onModesChange = onSelectedModesChange,
            enabled = !isSearching
        )

        MetricSelector(
            value = config.metric,
            onChange = onMetricChange,
            enabled = !isSearching
        )

        SearchDepthSelector(
            limitDepth = config.limitDepth,
            maxDepth = config.maxDepth,
            onLimitChange = onLimitDepthChange,
            onDepthChange = onMaxDepthChange,
            enabled = !isSearching
        )

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            OutlinedButton(
                onClick = onReset,
                enabled = !isSearching,
                modifier = Modifier.weight(1f)
            ) {
                Text("Reset")
            }

            if (isSearching) {
                Button(
                    onClick = onCancel,
                    modifier = Modifier.weight(1f)
                ) {
                    Text("Cancel")
                }
            } else {
                Button(
                    onClick = onSolve,
                    modifier = Modifier.weight(1f)
                ) {
                    Text("Solve")
                }
            }
        }
    }
}
