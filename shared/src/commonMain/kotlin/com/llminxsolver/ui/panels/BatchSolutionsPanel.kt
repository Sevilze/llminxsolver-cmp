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
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboard
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.MetricType
import com.llminxsolver.data.ScoredSolution
import com.llminxsolver.util.setPlainText
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.llminxsolver.calculateMcc
import uniffi.llminxsolver.getMoveCount

private const val BATCH_PAGE_SIZE = 50

data class ScoredSolutionRef(
    val caseNumber: Int,
    val indexInCase: Int,
    val mcc: Float,
    val moveCount: Int
)

enum class BatchSortOption(val label: String) {
    MCC("MCC"),
    MOVE_COUNT("Movecount")
}

@Composable
fun BatchSolutionsPanel(
    caseNumbers: List<Int>,
    metric: MetricType,
    readCasePage: (caseNumber: Int, offset: Int, limit: Int) -> List<String>,
    getCaseCount: (caseNumber: Int) -> Int,
    solutionVersion: Int = 0,
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

    val casesToLoad = remember(caseNumbers, selectedCaseNumber) {
        if (selectedCaseNumber != null) {
            caseNumbers.filter { it == selectedCaseNumber }
        } else {
            caseNumbers
        }
    }

    var scoredRefs by remember { mutableStateOf<List<ScoredSolutionRef>>(emptyList()) }
    var totalToLoad by remember { mutableStateOf(0) }
    var isLoading by remember { mutableStateOf(false) }

    val loadedPerCase = remember(casesToLoad, metric) { mutableMapOf<Int, Int>() }
    val algorithmCache = remember(casesToLoad, metric) { mutableStateMapOf<String, String>() }

    LaunchedEffect(casesToLoad, metric, solutionVersion) {
        val isFullReload = loadedPerCase.isEmpty()
        if (isFullReload) {
            scoredRefs = emptyList()
            isLoading = true
        }

        val caseCounts = withContext(Dispatchers.IO) {
            casesToLoad.associateWith { getCaseCount(it) }
        }
        totalToLoad = caseCounts.values.sum()

        if (totalToLoad == 0) {
            scoredRefs = emptyList()
            isLoading = false
            return@LaunchedEffect
        }

        for (caseNum in casesToLoad) {
            val count = caseCounts[caseNum] ?: 0
            val alreadyLoaded = loadedPerCase[caseNum] ?: 0
            if (alreadyLoaded >= count) continue

            var offset = alreadyLoaded
            while (offset < count) {
                val limit = minOf(BATCH_PAGE_SIZE, count - offset)
                val currentRefCount = scoredRefs.size
                val pageRefs = withContext(Dispatchers.IO) {
                    val page = readCasePage(caseNum, offset, limit)
                    page.mapIndexed { i, solutionStr ->
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

                        if (currentRefCount + i < BATCH_PAGE_SIZE) {
                            algorithmCache["$caseNum:${offset + i}"] = cleanAlg
                        }

                        ScoredSolutionRef(
                            caseNumber = caseNum,
                            indexInCase = offset + i,
                            mcc = mcc,
                            moveCount = moveCount
                        )
                    }
                }
                loadedPerCase[caseNum] = offset + limit
                scoredRefs = scoredRefs + pageRefs
                isLoading = false
                offset += limit
            }
        }
        isLoading = false
    }

    val sortedRefs = remember(scoredRefs, sortOption, ascendingOrder) {
        val sorted = when (sortOption) {
            BatchSortOption.MCC -> scoredRefs.sortedBy { it.mcc }
            BatchSortOption.MOVE_COUNT -> scoredRefs.sortedBy { it.moveCount }
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
                        text = if (scoredRefs.size < totalToLoad && totalToLoad > 0) {
                            "(${sortedRefs.size}/$totalToLoad solutions)"
                        } else {
                            "(${sortedRefs.size} solutions)"
                        },
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }

            if (scoredRefs.isNotEmpty()) {
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

            if (scoredRefs.isEmpty() && !isLoading) {
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
            } else if (scoredRefs.isEmpty()) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = "Loading solutions...",
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
                    items(sortedRefs, key = { ref ->
                        "${ref.caseNumber}:${ref.indexInCase}"
                    }) { ref ->
                        val cacheKey = "${ref.caseNumber}:${ref.indexInCase}"
                        var algorithm by remember(cacheKey) {
                            mutableStateOf(algorithmCache[cacheKey])
                        }

                        LaunchedEffect(cacheKey) {
                            if (algorithm == null) {
                                val loaded = withContext(Dispatchers.IO) {
                                    readCasePage(ref.caseNumber, ref.indexInCase, 1)
                                        .firstOrNull()?.substringBefore("(")?.trim() ?: ""
                                }
                                algorithmCache[cacheKey] = loaded
                                algorithm = loaded
                            }
                        }

                        ScoredSolutionRow(
                            solution = ScoredSolution(
                                algorithm = algorithm ?: "",
                                mcc = ref.mcc,
                                moveCount = ref.moveCount
                            ),
                            metricLabel = metricLabel,
                            isCopied = copiedIndex == sortedRefs.indexOf(ref),
                            onCopy = {
                                val alg = algorithm ?: return@ScoredSolutionRow
                                scope.launch {
                                    clipboard.setPlainText(alg)
                                    copiedIndex = sortedRefs.indexOf(ref)
                                }
                            }
                        )
                    }
                }
            }
        }
    }
}
