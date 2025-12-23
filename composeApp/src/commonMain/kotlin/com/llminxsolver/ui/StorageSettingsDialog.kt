package com.llminxsolver.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
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
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialExpressiveTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.llminxsolver.platform.PruningTableInfo
import com.llminxsolver.platform.StorageManager
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun StorageSettingsDialog(onDismiss: () -> Unit) {
    val storageManager = remember { StorageManager() }
    var tables by remember { mutableStateOf(storageManager.getPruningTables()) }
    var skipDeletionWarning by remember { mutableStateOf(false) }
    var tableToDelete by remember { mutableStateOf<PruningTableInfo?>(null) }
    var isVisible by remember { mutableStateOf(false) }
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
                    .width(320.dp)
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
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Text(
                            text = "Storage Settings",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.Bold
                        )
                        IconButton(onClick = handleDismiss) {
                            Icon(
                                imageVector = Icons.Default.Close,
                                contentDescription = "Close"
                            )
                        }
                    }

                    Spacer(modifier = Modifier.height(16.dp))

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

                    Spacer(modifier = Modifier.height(16.dp))

                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Column(modifier = Modifier.weight(1f)) {
                            Text(
                                text = "Skip Deletion Warning",
                                style = MaterialTheme.typography.bodyMedium
                            )
                            Text(
                                text = "Delete tables without confirmation",
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant
                            )
                        }

                        MaterialExpressiveTheme(
                            motionScheme = MotionScheme.expressive(),
                            colorScheme = MaterialTheme.colorScheme,
                            typography = MaterialTheme.typography,
                            shapes = MaterialTheme.shapes
                        ) {
                            Switch(
                                checked = skipDeletionWarning,
                                onCheckedChange = { skipDeletionWarning = it }
                            )
                        }
                    }

                    Spacer(modifier = Modifier.height(16.dp))
                    HorizontalDivider()
                    Spacer(modifier = Modifier.height(8.dp))

                    Text(
                        text = "Pruning Tables (${tables.size})",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )

                    Spacer(modifier = Modifier.height(8.dp))

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
                                    onDelete = {
                                        if (skipDeletionWarning) {
                                            storageManager.deletePruningTable(table.filename)
                                            tables = storageManager.getPruningTables()
                                        } else {
                                            tableToDelete = table
                                        }
                                    }
                                )
                            }
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
