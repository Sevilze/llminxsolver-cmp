package com.llminxsolver.ui.panels

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.scaleIn
import androidx.compose.animation.scaleOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.clickable
import androidx.compose.foundation.hoverable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsHoveredAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ContentCopy
import androidx.compose.material.icons.filled.Done
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalClipboard
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.SolverState
import com.llminxsolver.util.setPlainText
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

private const val PAGE_SIZE = 50
private const val BUFFER_PAGES = 2

/**
 * Displays raw solutions with paginated lazy loading from a temp file.
 *
 * Cache is keyed on [tempFilePath] - when a new search starts (creating a new temp file),
 * the entire cache is automatically recreated as fresh objects, preventing stale data issues.
 */
@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun SolutionsPanel(
    solverState: SolverState,
    readSolutionsPage: (Int, Int) -> List<String> = { _, _ -> emptyList() },
    tempFilePath: String? = null,
    defaultCollapsed: Boolean = false,
    onExpandChange: ((Boolean) -> Unit)? = null,
    listHeight: Int? = 400,
    modifier: Modifier = Modifier
) {
    var isExpanded by remember { mutableStateOf(!defaultCollapsed) }
    val listState = rememberLazyListState()
    val scope = rememberCoroutineScope()

    val clipboard = LocalClipboard.current
    val copiedSolutions = remember { mutableStateMapOf<String, Boolean>() }

    val totalCount = solverState.solutionCount

    // Cache is recreated when a new search starts
    val pageCache = remember(tempFilePath) { mutableStateMapOf<Int, List<String>>() }
    val loadingPages = remember(tempFilePath) { mutableStateOf(setOf<Int>()) }

    // Loads a page asynchronously, checking if page is already loading or cached
    fun loadPage(pageIndex: Int, currentCount: Int) {
        val offset = pageIndex * PAGE_SIZE
        if (offset >= currentCount || currentCount <= 0) return
        if (loadingPages.value.contains(pageIndex)) return

        val cachedPage = pageCache[pageIndex]
        val expectedSize = minOf(PAGE_SIZE, currentCount - offset)
        if (cachedPage != null && cachedPage.size >= expectedSize) return

        scope.launch {
            loadingPages.value += pageIndex
            try {
                val page = readSolutionsPage(offset, PAGE_SIZE)
                pageCache[pageIndex] = page
            } finally {
                loadingPages.value -= pageIndex
            }
        }
    }

    // Preloads pages around the center to provide smooth scrolling
    fun loadPagesAround(centerPage: Int, currentCount: Int) {
        if (currentCount <= 0) return
        val maxPage = (currentCount - 1) / PAGE_SIZE
        val startPage = maxOf(0, centerPage - BUFFER_PAGES)
        val endPage = minOf(maxPage, centerPage + BUFFER_PAGES)

        for (page in startPage..endPage) {
            loadPage(page, currentCount)
        }
    }

    val visiblePageIndices by remember {
        derivedStateOf {
            val items = listState.layoutInfo.visibleItemsInfo
            if (items.isEmpty()) {
                setOf(0)
            } else {
                val first = items.first().index / PAGE_SIZE
                val last = items.last().index / PAGE_SIZE
                (first..last).toSet()
            }
        }
    }

    // Fallback trigger for items that render as "Loading..."
    fun ensureItemLoaded(index: Int, currentCount: Int) {
        val pageIndex = index / PAGE_SIZE
        loadPage(pageIndex, currentCount)
    }

    // Load pages when panel expands or new solutions arrive
    LaunchedEffect(isExpanded, totalCount, tempFilePath) {
        if (isExpanded && totalCount > 0) {
            val items = listState.layoutInfo.visibleItemsInfo
            val centerPage = if (items.isEmpty()) 0 else items.first().index / PAGE_SIZE
            loadPagesAround(centerPage, totalCount)
        }
    }

    // Load visible pages when user scrolls
    LaunchedEffect(visiblePageIndices, tempFilePath) {
        if (!isExpanded || totalCount <= 0) return@LaunchedEffect
        visiblePageIndices.forEach { pageIndex ->
            loadPage(pageIndex, totalCount)
        }
    }

    LaunchedEffect(copiedSolutions.keys.toList()) {
        if (copiedSolutions.isNotEmpty()) {
            delay(1000)
            copiedSolutions.clear()
        }
    }

    Card(
        modifier = modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Column {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable {
                        val newState = !isExpanded
                        isExpanded = newState
                        onExpandChange?.invoke(newState)
                    }
                    .padding(16.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Raw Solutions",
                        style = MaterialTheme.typography.titleSmall
                    )
                    if (solverState.solutionCount > 0) {
                        Text(
                            text = "(${solverState.solutionCount})",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }

                AnimatedContent(
                    targetState = isExpanded,
                    transitionSpec = {
                        (
                            scaleIn(
                                spring(
                                    dampingRatio = Spring.DampingRatioMediumBouncy,
                                    stiffness = Spring.StiffnessMediumLow
                                )
                            ) + fadeIn()
                            ).togetherWith(
                            scaleOut(
                                spring(
                                    dampingRatio = Spring.DampingRatioMediumBouncy,
                                    stiffness = Spring.StiffnessMediumLow
                                )
                            ) + fadeOut()
                        )
                    }
                ) { expanded ->
                    Icon(
                        imageVector = if (expanded) {
                            Icons.Filled.KeyboardArrowUp
                        } else {
                            Icons.Filled.KeyboardArrowDown
                        },
                        contentDescription = if (expanded) "Collapse" else "Expand"
                    )
                }
            }

            AnimatedVisibility(
                visible = isExpanded,
                enter = expandVertically(
                    animationSpec = spring(
                        dampingRatio = Spring.DampingRatioMediumBouncy,
                        stiffness = Spring.StiffnessMediumLow
                    )
                ) + fadeIn(
                    animationSpec = spring(stiffness = Spring.StiffnessLow)
                ),
                exit = shrinkVertically(
                    animationSpec = spring(
                        dampingRatio = Spring.DampingRatioNoBouncy,
                        stiffness = Spring.StiffnessMedium
                    )
                ) + fadeOut(
                    animationSpec = spring(stiffness = Spring.StiffnessLow)
                )
            ) {
                when {
                    pageCache.isEmpty() && totalCount == 0 -> {
                        Text(
                            text = if (solverState.isSearching) {
                                "Searching..."
                            } else {
                                "No solutions found yet."
                            },
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(16.dp)
                        )
                    }

                    else -> {
                        if (listHeight != null) {
                            LazyColumn(
                                state = listState,
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .height(listHeight.dp)
                                    .animateContentSize(
                                        animationSpec = spring(
                                            dampingRatio = Spring.DampingRatioMediumBouncy,
                                            stiffness = Spring.StiffnessMediumLow
                                        )
                                    ),
                                contentPadding = PaddingValues(
                                    start = 16.dp,
                                    end = 16.dp,
                                    bottom = 16.dp
                                ),
                                verticalArrangement = Arrangement.spacedBy(4.dp)
                            ) {
                                items(totalCount, key = { it }) { index ->
                                    val pageIndex = index / PAGE_SIZE
                                    val pageOffset = index % PAGE_SIZE
                                    val solution = pageCache[pageIndex]?.getOrNull(pageOffset)

                                    if (solution != null) {
                                        SolutionItem(
                                            solution = solution,
                                            isCopied = copiedSolutions[solution] == true,
                                            onCopy = {
                                                scope.launch { clipboard.setPlainText(solution) }
                                                copiedSolutions[solution] = true
                                            }
                                        )
                                    } else {
                                        ensureItemLoaded(index, totalCount)
                                        Text(
                                            text = "Loading...",
                                            style = MaterialTheme.typography.bodySmall,
                                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                                            modifier = Modifier.padding(vertical = 6.dp)
                                        )
                                    }
                                }
                            }
                        } else {
                            LaunchedEffect(totalCount, tempFilePath) {
                                if (totalCount > 0) {
                                    val maxPage = minOf(2, (totalCount - 1) / PAGE_SIZE)
                                    for (page in 0..maxPage) {
                                        loadPage(page, totalCount)
                                    }
                                }
                            }

                            Column(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .animateContentSize(
                                        animationSpec = spring(
                                            dampingRatio = Spring.DampingRatioMediumBouncy,
                                            stiffness = Spring.StiffnessMediumLow
                                        )
                                    )
                                    .padding(
                                        start = 16.dp,
                                        end = 16.dp,
                                        bottom = 16.dp
                                    ),
                                verticalArrangement = Arrangement.spacedBy(4.dp)
                            ) {
                                val loadedCount = pageCache.values.sumOf { it.size }
                                val displayCount = minOf(loadedCount, totalCount)

                                for (index in 0 until displayCount) {
                                    val pageIndex = index / PAGE_SIZE
                                    val pageOffset = index % PAGE_SIZE
                                    val solution = pageCache[pageIndex]?.getOrNull(pageOffset)

                                    if (solution != null) {
                                        SolutionItem(
                                            solution = solution,
                                            isCopied = copiedSolutions[solution] == true,
                                            onCopy = {
                                                scope.launch { clipboard.setPlainText(solution) }
                                                copiedSolutions[solution] = true
                                            }
                                        )
                                    }
                                }

                                if (loadedCount < totalCount) {
                                    Text(
                                        text = "Loading ${totalCount - loadedCount} more...",
                                        style = MaterialTheme.typography.bodySmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                                        modifier = Modifier.padding(vertical = 6.dp)
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun SolutionItem(
    solution: String,
    isCopied: Boolean,
    onCopy: () -> Unit,
    modifier: Modifier = Modifier
) {
    val interactionSource = remember { MutableInteractionSource() }
    val isHovered by interactionSource.collectIsHoveredAsState()

    Row(
        modifier = modifier
            .fillMaxWidth()
            .hoverable(interactionSource)
            .clickable(onClick = onCopy)
            .padding(vertical = 6.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text(
            text = solution,
            style = MaterialTheme.typography.bodySmall,
            fontFamily = FontFamily.Monospace,
            modifier = Modifier.weight(1f)
        )

        AnimatedContent(
            targetState = isCopied,
            transitionSpec = {
                (
                    scaleIn(
                        spring(
                            dampingRatio = Spring.DampingRatioMediumBouncy,
                            stiffness = Spring.StiffnessMediumLow
                        )
                    ) + fadeIn()
                    ).togetherWith(
                    scaleOut(spring(stiffness = Spring.StiffnessMediumLow)) + fadeOut()
                )
            },
            modifier = Modifier.width(40.dp)
        ) { copied ->
            if (copied) {
                Icon(
                    imageVector = Icons.Filled.Done,
                    contentDescription = "Copied",
                    modifier = Modifier.size(18.dp),
                    tint = MaterialTheme.colorScheme.primary
                )
            } else {
                IconButton(
                    onClick = onCopy,
                    modifier = Modifier.size(32.dp)
                ) {
                    Icon(
                        imageVector = Icons.Filled.ContentCopy,
                        contentDescription = "Copy",
                        modifier = Modifier.size(16.dp),
                        tint = if (isHovered) {
                            MaterialTheme.colorScheme.primary
                        } else {
                            MaterialTheme.colorScheme.outline
                        }
                    )
                }
            }
        }
    }
}
