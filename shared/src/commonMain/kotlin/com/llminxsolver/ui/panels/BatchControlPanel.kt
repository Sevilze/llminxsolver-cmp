package com.llminxsolver.ui.panels

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.Checkbox
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.BatchSolverConfig
import com.llminxsolver.data.BatchSolverState
import com.llminxsolver.data.toIgnoreFlags
import com.llminxsolver.ui.components.AdjustmentInputs
import com.llminxsolver.ui.components.DepthSliders
import com.llminxsolver.ui.components.EquivalencesInput
import com.llminxsolver.ui.components.IgnoreOptions
import com.llminxsolver.ui.components.ScrambleInput
import com.llminxsolver.ui.components.SearchModeSelector

@Composable
fun BatchControlPanel(
    config: BatchSolverConfig,
    state: BatchSolverState,
    onScrambleChange: (String) -> Unit,
    onEquivalencesChange: (String) -> Unit,
    onPreAdjustChange: (String) -> Unit,
    onPostAdjustChange: (String) -> Unit,
    onSearchModeChange: (com.llminxsolver.data.GeneratorMode) -> Unit,
    onPruningDepthChange: (Int) -> Unit,
    onSearchDepthChange: (Int) -> Unit,
    onStopAfterFirstChange: (Boolean) -> Unit,
    onIgnoreCornerPermutationChange: (Boolean) -> Unit,
    onIgnoreEdgePermutationChange: (Boolean) -> Unit,
    onIgnoreCornerOrientationChange: (Boolean) -> Unit,
    onIgnoreEdgeOrientationChange: (Boolean) -> Unit,
    onGenerateStates: () -> Unit,
    onStartSolve: () -> Unit,
    onCancelSolve: () -> Unit,
    onReset: () -> Unit,
    modifier: Modifier = Modifier
) {
    val isWorking = state.isGenerating || state.isSearching
    val hasStates = state.generatedStates.isNotEmpty()

    Column(
        modifier = modifier
            .fillMaxWidth()
            .verticalScroll(rememberScrollState()),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        ScrambleInput(
            value = config.scramble,
            onValueChange = onScrambleChange,
            enabled = !isWorking
        )

        EquivalencesInput(
            value = config.equivalences,
            onValueChange = onEquivalencesChange,
            enabled = !isWorking
        )

        AdjustmentInputs(
            preAdjust = config.preAdjust,
            postAdjust = config.postAdjust,
            onPreAdjustChange = onPreAdjustChange,
            onPostAdjustChange = onPostAdjustChange,
            enabled = !isWorking
        )

        HorizontalDivider()

        SearchModeSelector(
            selectedMode = config.searchMode,
            onModeChange = onSearchModeChange,
            enabled = !isWorking
        )

        DepthSliders(
            pruningDepth = config.pruningDepth,
            searchDepth = config.searchDepth,
            onPruningDepthChange = onPruningDepthChange,
            onSearchDepthChange = onSearchDepthChange,
            enabled = !isWorking
        )

        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Checkbox(
                checked = config.stopAfterFirst,
                onCheckedChange = onStopAfterFirstChange,
                enabled = !isWorking
            )
            Text(
                text = "Stop after first solution",
                style = MaterialTheme.typography.bodyMedium
            )
        }

        HorizontalDivider()

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            if (!isWorking) {
                Button(
                    onClick = onGenerateStates,
                    modifier = Modifier.weight(1f),
                    enabled = config.scramble.isNotBlank()
                ) {
                    Text("Generate")
                }

                Button(
                    onClick = onStartSolve,
                    modifier = Modifier.weight(1f),
                    enabled = hasStates
                ) {
                    Text("Solve")
                }

                OutlinedButton(
                    onClick = onReset,
                    modifier = Modifier.weight(1f)
                ) {
                    Text("Reset")
                }
            } else {
                OutlinedButton(
                    onClick = onCancelSolve,
                    modifier = Modifier.weight(1f)
                ) {
                    Text("Cancel")
                }
            }
        }
    }
}
