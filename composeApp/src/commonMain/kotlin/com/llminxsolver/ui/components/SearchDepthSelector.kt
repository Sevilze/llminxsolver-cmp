package com.llminxsolver.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.expandVertically
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.MaterialExpressiveTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlin.math.roundToInt

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun SearchDepthSelector(
    limitDepth: Boolean,
    maxDepth: Int,
    onLimitChange: (Boolean) -> Unit,
    onDepthChange: (Int) -> Unit,
    enabled: Boolean = true,
    modifier: Modifier = Modifier
) {
    Column(modifier = modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = "Limit Search Depth",
                    style = MaterialTheme.typography.bodyMedium
                )
                Text(
                    text = if (limitDepth) "Max depth: $maxDepth" else "No limit",
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
                    checked = limitDepth,
                    onCheckedChange = { if (enabled) onLimitChange(it) },
                    enabled = enabled
                )
            }
        }

        AnimatedVisibility(
            visible = limitDepth,
            enter = expandVertically(
                animationSpec = MotionScheme.expressive().defaultSpatialSpec()
            ),
            exit = shrinkVertically(
                animationSpec = MotionScheme.expressive().defaultSpatialSpec()
            )
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = 8.dp)
            ) {
                Slider(
                    value = maxDepth.toFloat(),
                    onValueChange = { if (enabled) onDepthChange(it.roundToInt()) },
                    valueRange = 1f..30f,
                    enabled = enabled
                )

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Text(text = "1", style = MaterialTheme.typography.labelSmall)
                    Text(text = "30", style = MaterialTheme.typography.labelSmall)
                }
            }
        }
    }
}
