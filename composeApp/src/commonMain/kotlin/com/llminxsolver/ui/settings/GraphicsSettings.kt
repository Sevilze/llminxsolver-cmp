package com.llminxsolver.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.dialogs.ColorPickerDialog

@Composable
internal fun GraphicsTabContent(
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
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.width(56.dp)
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
            modifier = Modifier
                .weight(1f)
                .height(48.dp),
            singleLine = true,
            textStyle = MaterialTheme.typography.bodySmall
        )

        Box(
            modifier = Modifier
                .size(32.dp)
                .clip(RoundedCornerShape(6.dp))
                .background(color)
                .clickable { showColorPicker = true }
        )
    }

    if (showColorPicker) {
        ColorPickerDialog(
            initialColor = color,
            onColorSelected = { newColor ->
                onColorChange(newColor)
                hexInput = String.format("%06X", newColor.toArgb() and 0xFFFFFF)
            },
            onDismiss = { showColorPicker = false }
        )
    }
}
