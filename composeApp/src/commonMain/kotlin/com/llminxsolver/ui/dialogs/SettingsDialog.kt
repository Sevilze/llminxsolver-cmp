package com.llminxsolver.ui.dialogs

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ButtonGroup
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialExpressiveTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.ToggleButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.platform.PruningTableInfo
import com.llminxsolver.platform.StorageManager
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

private enum class SettingsTab(val label: String) {
    Storage("Storage"),
    Memory("Memory"),
    Graphics("Graphics")
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun SettingsDialog(
    onDismiss: () -> Unit,
    parallelConfig: ParallelConfig = ParallelConfig(),
    memoryInfo: MemoryInfo? = null,
    availableCpus: Int = 4,
    onParallelConfigChange: ((ParallelConfig) -> Unit)? = null,
    megaminxColorScheme: MegaminxColorScheme = MegaminxColorScheme.Classic,
    onMegaminxColorSchemeChange: ((MegaminxColorScheme) -> Unit)? = null,
    skipDeletionWarning: Boolean = false,
    onSkipDeletionWarningChange: ((Boolean) -> Unit)? = null
) {
    val storageManager = remember { StorageManager() }
    var tables by remember { mutableStateOf(storageManager.getPruningTables()) }
    var tableToDelete by remember { mutableStateOf<PruningTableInfo?>(null) }
    var isVisible by remember { mutableStateOf(false) }
    var selectedTab by remember { mutableIntStateOf(0) }
    val scope = rememberCoroutineScope()

    val totalUsedMB = (storageManager.getTotalStorageUsed() / (1024 * 1024)).toInt()
    val availableMB = (storageManager.getAvailableStorage() / (1024 * 1024)).toInt()

    val handleDismiss: () -> Unit = {
        scope.launch {
            isVisible = false
            delay(300)
            onDismiss()
        }
    }

    LaunchedEffect(Unit) {
        isVisible = true
    }

    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.CenterEnd
    ) {
        AnimatedVisibility(
            visible = isVisible,
            enter = fadeIn(spring(stiffness = Spring.StiffnessLow)),
            exit = fadeOut(spring(stiffness = Spring.StiffnessLow))
        ) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(MaterialTheme.colorScheme.scrim.copy(alpha = 0.32f))
                    .clickable(
                        interactionSource = remember { MutableInteractionSource() },
                        indication = null,
                        onClick = handleDismiss
                    )
            )
        }

        AnimatedVisibility(
            visible = isVisible,
            enter = slideInHorizontally(
                animationSpec = MotionScheme.expressive().defaultSpatialSpec()
            ) { it },
            exit = slideOutHorizontally(
                animationSpec = MotionScheme.expressive().defaultSpatialSpec()
            ) { it }
        ) {
            Surface(
                modifier = Modifier
                    .width(360.dp)
                    .fillMaxHeight(0.85f)
                    .clip(RoundedCornerShape(topStart = 16.dp, bottomStart = 16.dp)),
                color = MaterialTheme.colorScheme.surface,
                tonalElevation = 8.dp
            ) {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(16.dp)
                ) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.End
                    ) {
                        IconButton(onClick = handleDismiss) {
                            Icon(
                                imageVector = Icons.Default.Close,
                                contentDescription = "Close"
                            )
                        }
                    }

                    @Suppress("DEPRECATION")
                    ButtonGroup(
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        SettingsTab.entries.forEachIndexed { index, tab ->
                            ToggleButton(
                                checked = selectedTab == index,
                                onCheckedChange = { selectedTab = index },
                                modifier = Modifier.weight(1f)
                            ) {
                                Text(
                                    text = tab.label,
                                    style = MaterialTheme.typography.labelMedium
                                )
                            }
                        }
                    }

                    Spacer(modifier = Modifier.height(16.dp))

                    AnimatedContent(
                        targetState = selectedTab,
                        transitionSpec = {
                            val direction = if (targetState > initialState) 1 else -1
                            slideInHorizontally(
                                MotionScheme.expressive().defaultSpatialSpec()
                            ) { direction * it / 4 } + fadeIn() togetherWith
                                slideOutHorizontally(
                                    MotionScheme.expressive().defaultSpatialSpec()
                                ) { -direction * it / 4 } + fadeOut()
                        },
                        modifier = Modifier.weight(1f),
                        label = "settings_tab_content"
                    ) { tabIndex ->
                        when (SettingsTab.entries[tabIndex]) {
                            SettingsTab.Storage -> StorageTabContent(
                                tables = tables,
                                totalUsedMB = totalUsedMB,
                                availableMB = availableMB,
                                skipDeletionWarning = skipDeletionWarning,
                                onSkipDeletionWarningChange = {
                                    onSkipDeletionWarningChange?.invoke(it)
                                },
                                onDeleteTable = { table ->
                                    if (skipDeletionWarning) {
                                        storageManager.deletePruningTable(table.filename)
                                        tables = storageManager.getPruningTables()
                                    } else {
                                        tableToDelete = table
                                    }
                                }
                            )

                            SettingsTab.Memory -> MemoryTabContent(
                                parallelConfig = parallelConfig,
                                memoryInfo = memoryInfo,
                                availableCpus = availableCpus,
                                onParallelConfigChange = onParallelConfigChange
                            )

                            SettingsTab.Graphics -> GraphicsTabContent(
                                colorScheme = megaminxColorScheme,
                                onColorSchemeChange = onMegaminxColorSchemeChange
                            )
                        }
                    }
                }
            }
        }
    }

    tableToDelete?.let { table ->
        AlertDialog(
            onDismissRequest = { tableToDelete = null },
            title = { Text("Delete Table?") },
            text = {
                Text(
                    "Are you sure you want to delete ${table.displayName}? " +
                        "You will have to regenerate this pruning table."
                )
            },
            confirmButton = {
                TextButton(
                    onClick = {
                        storageManager.deletePruningTable(table.filename)
                        tables = storageManager.getPruningTables()
                        tableToDelete = null
                    }
                ) {
                    Text("Delete", color = MaterialTheme.colorScheme.error)
                }
            },
            dismissButton = {
                TextButton(onClick = { tableToDelete = null }) {
                    Text("Cancel")
                }
            }
        )
    }
}

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun StorageTabContent(
    tables: List<PruningTableInfo>,
    totalUsedMB: Int,
    availableMB: Int,
    skipDeletionWarning: Boolean,
    onSkipDeletionWarningChange: (Boolean) -> Unit,
    onDeleteTable: (PruningTableInfo) -> Unit
) {
    Column(
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceContainerHigh
            )
        ) {
            Column(modifier = Modifier.padding(12.dp)) {
                Text(
                    text = "Storage Usage",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "$totalUsedMB MB used",
                    style = MaterialTheme.typography.bodyLarge,
                    fontWeight = FontWeight.Medium
                )
                Text(
                    text = "$availableMB MB available",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Skip Deletion Warning",
                style = MaterialTheme.typography.bodyMedium
            )
            MaterialExpressiveTheme(
                motionScheme = MotionScheme.expressive(),
                colorScheme = MaterialTheme.colorScheme,
                typography = MaterialTheme.typography,
                shapes = MaterialTheme.shapes
            ) {
                Switch(
                    checked = skipDeletionWarning,
                    onCheckedChange = onSkipDeletionWarningChange
                )
            }
        }

        HorizontalDivider()

        Text(
            text = "Pruning Tables (${tables.size})",
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        if (tables.isEmpty()) {
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .weight(1f),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = "No pruning tables found",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        } else {
            LazyColumn(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(tables) { table ->
                    PruningTableItem(
                        table = table,
                        onDelete = { onDeleteTable(table) }
                    )
                }
            }
        }
    }
}

@Composable
private fun MemoryTabContent(
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

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
private fun GraphicsTabContent(
    colorScheme: MegaminxColorScheme,
    onColorSchemeChange: ((MegaminxColorScheme) -> Unit)?
) {
    val faceLabels = listOf("U Face", "F Face", "L Face", "bL Face", "bR Face", "R Face")

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        item {
            Text(
                text = "Face Colors",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        items(6) { index ->
            val currentColor = when (index) {
                0 -> colorScheme.uFace
                1 -> colorScheme.fFace
                2 -> colorScheme.lFace
                3 -> colorScheme.blFace
                4 -> colorScheme.brFace
                else -> colorScheme.rFace
            }
            FaceColorEditor(
                label = faceLabels[index],
                color = currentColor,
                onColorChange = { newColor ->
                    val updatedScheme = when (index) {
                        0 -> colorScheme.copy(uFace = newColor)
                        1 -> colorScheme.copy(fFace = newColor)
                        2 -> colorScheme.copy(lFace = newColor)
                        3 -> colorScheme.copy(blFace = newColor)
                        4 -> colorScheme.copy(brFace = newColor)
                        else -> colorScheme.copy(rFace = newColor)
                    }
                    onColorSchemeChange?.invoke(updatedScheme)
                }
            )
        }

        item {
            HorizontalDivider()
        }

        item {
            Text(
                text = "Presets",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        item {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                MegaminxColorScheme.presets.forEach { (name, preset) ->
                    TextButton(
                        onClick = { onColorSchemeChange?.invoke(preset) }
                    ) {
                        Text(name)
                    }
                }
            }
        }
    }
}

@Composable
private fun FaceColorEditor(label: String, color: Color, onColorChange: (Color) -> Unit) {
    var showColorPicker by remember { mutableStateOf(false) }
    var hexInput by remember(color) {
        mutableStateOf(String.format("%06X", color.toArgb() and 0xFFFFFF))
    }

    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceContainerLow
        )
    ) {
        Column(
            modifier = Modifier.padding(12.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = label,
                    style = MaterialTheme.typography.bodyMedium
                )
                Box(
                    modifier = Modifier
                        .size(32.dp)
                        .clip(RoundedCornerShape(8.dp))
                        .background(color)
                        .clickable { showColorPicker = !showColorPicker }
                )
            }

            AnimatedVisibility(visible = showColorPicker) {
                Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.spacedBy(4.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(
                            text = "#",
                            style = MaterialTheme.typography.bodyMedium
                        )
                        androidx.compose.material3.OutlinedTextField(
                            value = hexInput,
                            onValueChange = { input ->
                                val cleaned = input.filter {
                                    it.isDigit() || it in 'A'..'F' ||
                                        it in 'a'..'f'
                                }
                                    .take(6)
                                hexInput = cleaned.uppercase()
                                if (cleaned.length == 6) {
                                    try {
                                        val colorInt = cleaned.toLong(16)
                                        onColorChange(Color(0xFF000000 or colorInt))
                                    } catch (_: Exception) { }
                                }
                            },
                            modifier = Modifier.weight(1f),
                            singleLine = true,
                            textStyle = MaterialTheme.typography.bodyMedium
                        )
                    }

                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.spacedBy(4.dp)
                    ) {
                        quickColors.forEach { quickColor ->
                            Box(
                                modifier = Modifier
                                    .size(28.dp)
                                    .clip(RoundedCornerShape(6.dp))
                                    .background(quickColor)
                                    .clickable {
                                        onColorChange(quickColor)
                                        hexInput = String.format(
                                            "%06X",
                                            quickColor.toArgb() and 0xFFFFFF
                                        )
                                    }
                            )
                        }
                    }
                }
            }
        }
    }
}

private val quickColors = listOf(
    Color(0xFFE1E100), // Yellow
    Color(0xFFC80000), // Red
    Color(0xFFE16400), // Orange
    Color(0xFF00C800), // Green
    Color(0xFFFF9696), // Pink
    Color(0xFF000096), // Blue
    Color(0xFFFFFFFF), // White
    Color(0xFF000000) // Black
)

@Composable
private fun PruningTableItem(
    table: PruningTableInfo,
    onDelete: () -> Unit,
    modifier: Modifier = Modifier
) {
    Card(
        modifier = modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceContainerLow
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = table.displayName,
                    style = MaterialTheme.typography.bodyMedium,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
                Text(
                    text = String.format("%.1f MB", table.sizeMB),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            IconButton(
                onClick = onDelete,
                modifier = Modifier.size(36.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Delete,
                    contentDescription = "Delete",
                    tint = MaterialTheme.colorScheme.error
                )
            }
        }
    }
}
