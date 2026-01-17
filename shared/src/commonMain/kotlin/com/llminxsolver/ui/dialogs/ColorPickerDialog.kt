package com.llminxsolver.ui.dialogs

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog

@Composable
internal fun ColorPickerDialog(
    initialColor: Color,
    onColorSelected: (Color) -> Unit,
    onDismiss: () -> Unit
) {
    val argb = initialColor.toArgb()
    var alpha by remember { mutableFloatStateOf(((argb shr 24) and 0xFF) / 255f) }
    var red by remember { mutableFloatStateOf(((argb shr 16) and 0xFF) / 255f) }
    var green by remember { mutableFloatStateOf(((argb shr 8) and 0xFF) / 255f) }
    var blue by remember { mutableFloatStateOf((argb and 0xFF) / 255f) }

    val currentColor = remember(alpha, red, green, blue) {
        Color(red = red, green = green, blue = blue, alpha = alpha)
    }

    Dialog(onDismissRequest = onDismiss) {
        Surface(
            shape = RoundedCornerShape(16.dp),
            color = MaterialTheme.colorScheme.surface,
            tonalElevation = 8.dp
        ) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.Center,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Box(
                        modifier = Modifier
                            .size(72.dp)
                            .clip(RoundedCornerShape(12.dp))
                            .background(initialColor)
                    )
                    Text(
                        text = "->",
                        style = MaterialTheme.typography.titleMedium,
                        modifier = Modifier.padding(horizontal = 24.dp)
                    )
                    Box(
                        modifier = Modifier
                            .size(72.dp)
                            .clip(RoundedCornerShape(12.dp))
                            .background(currentColor)
                    )
                }

                ColorSliderRow(
                    label = "R",
                    value = red,
                    onValueChange = { red = it }
                )

                ColorSliderRow(
                    label = "G",
                    value = green,
                    onValueChange = { green = it }
                )

                ColorSliderRow(
                    label = "B",
                    value = blue,
                    onValueChange = { blue = it }
                )

                ColorSliderRow(
                    label = "A",
                    value = alpha,
                    onValueChange = { alpha = it }
                )

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End
                ) {
                    TextButton(onClick = onDismiss) {
                        Text("Cancel")
                    }
                    TextButton(
                        onClick = {
                            onColorSelected(currentColor)
                            onDismiss()
                        }
                    ) {
                        Text("Select")
                    }
                }
            }
        }
    }
}

@Composable
internal fun ColorSliderRow(label: String, value: Float, onValueChange: (Float) -> Unit) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelMedium,
            modifier = Modifier.width(20.dp)
        )
        Slider(
            value = value,
            onValueChange = onValueChange,
            valueRange = 0f..1f,
            modifier = Modifier.weight(1f)
        )
        Text(
            text = (value * 255).toInt().toString(),
            style = MaterialTheme.typography.bodySmall,
            modifier = Modifier.width(32.dp)
        )
    }
}
