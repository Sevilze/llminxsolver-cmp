package com.llminxsolver.ui.settings

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Info
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.ExposedDropdownMenuAnchorType
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.PlainTooltip
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.material3.TooltipAnchorPosition
import androidx.compose.material3.TooltipBox
import androidx.compose.material3.TooltipDefaults
import androidx.compose.material3.rememberTooltipState
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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.DynamicColorMode
import com.llminxsolver.data.SchemeType
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.dialogs.ColorPickerDialog
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class, ExperimentalMaterial3ExpressiveApi::class)
@Composable
internal fun GraphicsTabContent(
    colorScheme: MegaminxColorScheme,
    onColorSchemeChange: ((MegaminxColorScheme) -> Unit)?,
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
    val faceLabels = listOf("U Face", "F Face", "L Face", "bL Face", "bR Face", "R Face")
    var pathInput by remember { mutableStateOf(wallpaperPath ?: "") }
    val sectionSpacing = 16.dp

    LaunchedEffect(wallpaperPath) {
        pathInput = wallpaperPath ?: ""
    }

    LazyColumn(
        modifier = Modifier.fillMaxSize()
    ) {
        item {
            AnimatedVisibility(
                visible = showDynamicColorModeConfig,
                enter = fadeIn(MotionScheme.expressive().defaultEffectsSpec()) +
                    expandVertically(MotionScheme.expressive().defaultSpatialSpec()),
                exit = fadeOut(MotionScheme.expressive().defaultEffectsSpec()) +
                    shrinkVertically(MotionScheme.expressive().defaultSpatialSpec())
            ) {
                Column(
                    modifier = Modifier.padding(bottom = sectionSpacing),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(4.dp)
                    ) {
                        Text(
                            text = "Dynamic Color Mode",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        val dynamicColorTooltipState = rememberTooltipState(isPersistent = true)
                        val tooltipScope = rememberCoroutineScope()
                        TooltipBox(
                            positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                                TooltipAnchorPosition.Above
                            ),
                            tooltip = {
                                PlainTooltip(
                                    modifier = Modifier.widthIn(max = 240.dp)
                                ) {
                                    Text(
                                        "Built-in uses the system native wallpaper color engine. " +
                                            "Matugen uses a different algorithm to generate colors from a given image."
                                    )
                                }
                            },
                            state = dynamicColorTooltipState
                        ) {
                            Icon(
                                imageVector = Icons.Outlined.Info,
                                contentDescription = "Info",
                                modifier = Modifier
                                    .size(16.dp)
                                    .clickable {
                                        tooltipScope.launch { dynamicColorTooltipState.show() }
                                    },
                                tint = MaterialTheme.colorScheme.onSurfaceVariant
                            )
                        }
                    }
                    SingleChoiceSegmentedButtonRow(modifier = Modifier.fillMaxWidth()) {
                        DynamicColorMode.entries.forEachIndexed { index, mode ->
                            SegmentedButton(
                                shape = SegmentedButtonDefaults.itemShape(
                                    index = index,
                                    count = DynamicColorMode.entries.size
                                ),
                                onClick = { onDynamicColorModeChange?.invoke(mode) },
                                selected = dynamicColorMode == mode,
                                label = { Text(mode.name) }
                            )
                        }
                    }
                }
            }
        }

        item {
            AnimatedVisibility(
                visible = showWallpaperConfig,
                enter = fadeIn(MotionScheme.expressive().defaultEffectsSpec()) +
                    expandVertically(MotionScheme.expressive().defaultSpatialSpec()),
                exit = fadeOut(MotionScheme.expressive().defaultEffectsSpec()) +
                    shrinkVertically(MotionScheme.expressive().defaultSpatialSpec())
            ) {
                Column(
                    modifier = Modifier.padding(bottom = sectionSpacing),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(4.dp)
                    ) {
                        Text(
                            text = "Wallpaper Path",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        val wallpaperTooltipState = rememberTooltipState(isPersistent = true)
                        val wallpaperTooltipScope = rememberCoroutineScope()
                        TooltipBox(
                            positionProvider = TooltipDefaults.rememberTooltipPositionProvider(
                                TooltipAnchorPosition.Above
                            ),
                            tooltip = {
                                PlainTooltip {
                                    Text("Set image path for dynamic colors")
                                }
                            },
                            state = wallpaperTooltipState
                        ) {
                            Icon(
                                imageVector = Icons.Outlined.Info,
                                contentDescription = "Info",
                                modifier = Modifier
                                    .size(16.dp)
                                    .clickable {
                                        wallpaperTooltipScope.launch {
                                            wallpaperTooltipState.show()
                                        }
                                    },
                                tint = MaterialTheme.colorScheme.onSurfaceVariant
                            )
                        }
                    }
                    OutlinedTextField(
                        value = pathInput,
                        onValueChange = { input: String ->
                            pathInput = input
                            onWallpaperPathChange?.invoke(input.ifBlank { null })
                        },
                        modifier = Modifier.fillMaxWidth(),
                        placeholder = { Text("/path/to/wallpaper.png") },
                        singleLine = true,
                        shape = RoundedCornerShape(12.dp),
                        textStyle = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }

        item {
            val showSchemeTypeConfig = !showDynamicColorModeConfig ||
                dynamicColorMode != DynamicColorMode.BuiltIn
            AnimatedVisibility(
                visible = showSchemeTypeConfig,
                enter = fadeIn(MotionScheme.expressive().defaultEffectsSpec()) +
                    expandVertically(MotionScheme.expressive().defaultSpatialSpec()),
                exit = fadeOut(MotionScheme.expressive().defaultEffectsSpec()) +
                    shrinkVertically(MotionScheme.expressive().defaultSpatialSpec())
            ) {
                Column(
                    modifier = Modifier.padding(bottom = sectionSpacing),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Color Scheme",
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    var expanded by remember { mutableStateOf(false) }
                    ExposedDropdownMenuBox(
                        expanded = expanded,
                        onExpandedChange = { expanded = it }
                    ) {
                        OutlinedTextField(
                            value = schemeType.name.replace(Regex("([A-Z])"), " $1").trim(),
                            onValueChange = {},
                            readOnly = true,
                            trailingIcon = {
                                ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded)
                            },
                            modifier = Modifier
                                .menuAnchor(ExposedDropdownMenuAnchorType.PrimaryNotEditable)
                                .fillMaxWidth(),
                            shape = RoundedCornerShape(12.dp)
                        )
                        ExposedDropdownMenu(
                            expanded = expanded,
                            onDismissRequest = { expanded = false }
                        ) {
                            SchemeType.entries.forEach { type ->
                                DropdownMenuItem(
                                    text = {
                                        Text(type.name.replace(Regex("([A-Z])"), " $1").trim())
                                    },
                                    onClick = {
                                        onSchemeTypeChange?.invoke(type)
                                        expanded = false
                                    }
                                )
                            }
                        }
                    }
                }
            }
        }

        item {
            Column(
                modifier = Modifier.padding(bottom = sectionSpacing),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Text(
                    text = "Theme Mode",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                SingleChoiceSegmentedButtonRow(modifier = Modifier.fillMaxWidth()) {
                    ThemeMode.entries.forEachIndexed { index, mode ->
                        SegmentedButton(
                            shape = SegmentedButtonDefaults.itemShape(
                                index = index,
                                count = ThemeMode.entries.size
                            ),
                            onClick = { onThemeModeChange?.invoke(mode) },
                            selected = themeMode == mode,
                            label = { Text(mode.name) }
                        )
                    }
                }
            }
        }

        item {
            Text(
                text = "Face Colors",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(bottom = 8.dp)
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
                onColorChange = { newColor: Color ->
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
    }
}

@Composable
internal fun FaceColorEditor(label: String, color: Color, onColorChange: (Color) -> Unit) {
    var showColorPicker by remember { mutableStateOf(false) }
    var hexInput by remember(color) {
        mutableStateOf(String.format("%06X", color.toArgb() and 0xFFFFFF))
    }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.width(64.dp)
        )

        OutlinedTextField(
            value = "#$hexInput",
            onValueChange = { input ->
                val cleaned = input.removePrefix("#").filter {
                    it.isDigit() || it in 'A'..'F' || it in 'a'..'f'
                }.take(6)
                hexInput = cleaned.uppercase()
                if (cleaned.length == 6) {
                    try {
                        val colorInt = cleaned.toLong(16)
                        onColorChange(Color(0xFF000000 or colorInt))
                    } catch (_: Exception) { }
                }
            },
            modifier = Modifier.weight(1f).height(56.dp),
            singleLine = true,
            shape = RoundedCornerShape(12.dp),
            textStyle = MaterialTheme.typography.bodySmall
        )

        Box(
            modifier = Modifier
                .size(40.dp)
                .clip(RoundedCornerShape(8.dp))
                .background(color)
                .clickable { showColorPicker = true }
        )
    }

    if (showColorPicker) {
        ColorPickerDialog(
            initialColor = color,
            onColorSelected = { newSelectedColor: Color ->
                onColorChange(newSelectedColor)
                hexInput = String.format("%06X", newSelectedColor.toArgb() and 0xFFFFFF)
            },
            onDismiss = { showColorPicker = false }
        )
    }
}
