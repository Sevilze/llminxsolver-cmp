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
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.PrimaryTabRow
import androidx.compose.material3.Surface
import androidx.compose.material3.Tab
import androidx.compose.material3.TabRowDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
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
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.DynamicColorMode
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.SchemeType
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.platform.MemoryInfo
import com.llminxsolver.platform.PruningTableInfo
import com.llminxsolver.platform.StorageManager
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.settings.GraphicsTabContent
import com.llminxsolver.ui.settings.MemoryTabContent
import com.llminxsolver.ui.settings.StorageTabContent
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
    pruningDepth: Int = 12,
    memoryInfo: MemoryInfo? = null,
    availableCpus: Int = 4,
    onParallelConfigChange: ((ParallelConfig) -> Unit)? = null,
    onPruningDepthChange: ((Int) -> Unit)? = null,
    megaminxColorScheme: MegaminxColorScheme = MegaminxColorScheme(),
    onMegaminxColorSchemeChange: ((MegaminxColorScheme) -> Unit)? = null,
    skipDeletionWarning: Boolean = false,
    onSkipDeletionWarningChange: ((Boolean) -> Unit)? = null,
    wallpaperPath: String? = null,
    onWallpaperPathChange: ((String?) -> Unit)? = null,
    showWallpaperConfig: Boolean = false,
    showDynamicColorModeConfig: Boolean = false,
    dynamicColorMode: DynamicColorMode = DynamicColorMode.BuiltIn,
    onDynamicColorModeChange: ((DynamicColorMode) -> Unit)? = null,
    schemeType: SchemeType = SchemeType.TonalSpot,
    onSchemeTypeChange: ((SchemeType) -> Unit)? = null,
    themeMode: ThemeMode = ThemeMode.System,
    onThemeModeChange: ((ThemeMode) -> Unit)? = null
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
                    .width(400.dp)
                    .fillMaxHeight(0.85f)
                    .clip(RoundedCornerShape(topStart = 16.dp, bottomStart = 16.dp)),
                color = MaterialTheme.colorScheme.surface,
                tonalElevation = 8.dp
            ) {
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(horizontal = 16.dp, vertical = 8.dp)
                ) {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(48.dp),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        PrimaryTabRow(
                            selectedTabIndex = selectedTab,
                            modifier = Modifier.weight(1f),
                            containerColor = MaterialTheme.colorScheme.surface,
                            divider = {},
                            indicator = {
                                TabRowDefaults.PrimaryIndicator(
                                    modifier = Modifier.tabIndicatorOffset(selectedTab),
                                    width = 32.dp,
                                    shape = RoundedCornerShape(topStart = 3.dp, topEnd = 3.dp),
                                    color = MaterialTheme.colorScheme.primary
                                )
                            }
                        ) {
                            SettingsTab.entries.forEachIndexed { index, tab ->
                                Tab(
                                    selected = selectedTab == index,
                                    onClick = { selectedTab = index },
                                    modifier = Modifier.clip(RoundedCornerShape(8.dp)),
                                    text = {
                                        Text(
                                            text = tab.label,
                                            style = MaterialTheme.typography.labelLarge
                                        )
                                    }
                                )
                            }
                        }

                        IconButton(onClick = handleDismiss) {
                            Icon(
                                imageVector = Icons.Default.Close,
                                contentDescription = "Close"
                            )
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
                                pruningDepth = pruningDepth,
                                memoryInfo = memoryInfo,
                                availableCpus = availableCpus,
                                onParallelConfigChange = onParallelConfigChange,
                                onPruningDepthChange = onPruningDepthChange
                            )

                            SettingsTab.Graphics -> GraphicsTabContent(
                                colorScheme = megaminxColorScheme,
                                onColorSchemeChange = onMegaminxColorSchemeChange,
                                wallpaperPath = wallpaperPath,
                                onWallpaperPathChange = onWallpaperPathChange,
                                showWallpaperConfig = showWallpaperConfig,
                                showDynamicColorModeConfig = showDynamicColorModeConfig,
                                dynamicColorMode = dynamicColorMode,
                                onDynamicColorModeChange = onDynamicColorModeChange,
                                schemeType = schemeType,
                                onSchemeTypeChange = onSchemeTypeChange,
                                themeMode = themeMode,
                                onThemeModeChange = onThemeModeChange
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
