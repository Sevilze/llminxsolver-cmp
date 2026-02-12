package com.llminxsolver.ui.panels

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
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboard
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.BatchCaseResult
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.util.setPlainText
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uniffi.llminxsolver.calculateMcc
import uniffi.llminxsolver.getMoveCount

data class CaseScoredSolution(val caseNumber: Int, val solution: ScoredSolution)

enum class BatchSortOption(val label: String) {
    MCC("MCC"),
    MOVE_COUNT("Movecount")
}

@Composable
fun BatchSolutionsPanel(
    caseResults: List<BatchCaseResult>,
    metric: MetricType,
    selectedCaseNumber: Int? = null,
    listHeight: Int? = null,
    modifier: Modifier = Modifier
) {
    var copiedIndex by remember { mutableStateOf<Int?>(null) }
    var sortOption by remember { mutableStateOf(BatchSortOption.MCC) }
    var ascendingOrder by remember { mutableStateOf(true) }

    val metricLabel = remember(metric) {
        when (metric) {
            MetricType.FTM -> "FTM"
            MetricType.FFTM -> "FFTM"
        }
    }

    LaunchedEffect(copiedIndex) {
        if (copiedIndex != null) {
            delay(1000)
            copiedIndex = null
        }
    }

    val clipboard = LocalClipboard.current
    val scope = rememberCoroutineScope()

    val filteredResults = remember(caseResults, selectedCaseNumber) {
        if (selectedCaseNumber != null) {
            caseResults.filter { it.caseNumber == selectedCaseNumber }
        } else {
            caseResults
        }
    }

    val allSolutions = remember(filteredResults, metric) {
        filteredResults.flatMap { result ->
            result.solutions.map { solutionStr ->
                val cleanAlg = solutionStr.substringBefore("(").trim()

                val mcc = try {
                    calculateMcc(cleanAlg).toFloat()
                } catch (e: Exception) {
                    0f
                }

                val moveCount = try {
                    getMoveCount(cleanAlg, metricLabel).toInt()
                } catch (e: Exception) {
                    0
                }

                CaseScoredSolution(
                    caseNumber = result.caseNumber,
                    solution = ScoredSolution(
                        algorithm = cleanAlg,
                        mcc = mcc,
                        moveCount = moveCount
                    )
                )
            }
        }
    }

    val sortedSolutions = remember(allSolutions, sortOption, ascendingOrder) {
        val sorted = when (sortOption) {
            BatchSortOption.MCC -> allSolutions.sortedBy { it.solution.mcc }
            BatchSortOption.MOVE_COUNT -> allSolutions.sortedBy { it.solution.moveCount }
        }
        if (ascendingOrder) sorted else sorted.reversed()
    }

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
                        text = "Batch Solutions",
                        style = MaterialTheme.typography.titleSmall
                    )
                    Text(
                        text = "(${sortedSolutions.size} solutions)",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }

            if (allSolutions.isNotEmpty()) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Box(
                        modifier = Modifier.width(60.dp),
                        contentAlignment = Alignment.CenterStart
                    ) {
                        SortableHeaderCell(
                            text = "MCC",
                            isSelected = sortOption == BatchSortOption.MCC,
                            onClick = {
                                if (sortOption == BatchSortOption.MCC) {
                                    ascendingOrder = !ascendingOrder
                                } else {
                                    sortOption = BatchSortOption.MCC
                                    ascendingOrder = true
                                }
                            },
                            modifier = Modifier.wrapContentWidth()
                        )
                    }

                    Box(
                        modifier = Modifier.width(60.dp).padding(horizontal = 4.dp),
                        contentAlignment = Alignment.CenterStart
                    ) {
                        SortableHeaderCell(
                            text = metricLabel,
                            isSelected = sortOption == BatchSortOption.MOVE_COUNT,
                            onClick = {
                                if (sortOption == BatchSortOption.MOVE_COUNT) {
                                    ascendingOrder = !ascendingOrder
                                } else {
                                    sortOption = BatchSortOption.MOVE_COUNT
                                    ascendingOrder = true
                                }
                            },
                            modifier = Modifier.wrapContentWidth()
                        )
                    }

                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = "Algorithm",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.weight(1f)
                    )
                }
            }

            if (allSolutions.isEmpty()) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = "No solutions found yet",
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.6f)
                    )
                }
            } else {
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
                    itemsIndexed(sortedSolutions, key = { index, sol ->
                        "${sol.caseNumber}_${sol.solution.algorithm}_$index"
                    }) { index, caseSol ->
                        ScoredSolutionRow(
                            solution = caseSol.solution,
                            metricLabel = metricLabel,
                            isCopied = copiedIndex == index,
                            onCopy = {
                                scope.launch {
                                    clipboard.setPlainText(caseSol.solution.algorithm)
                                    copiedIndex = index
                                }
                            }
                        )
                    }
                }
            }
        }
    }
}
