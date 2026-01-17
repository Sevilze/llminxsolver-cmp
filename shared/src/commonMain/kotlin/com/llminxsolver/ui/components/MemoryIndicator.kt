package com.llminxsolver.ui.components

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MotionScheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.llminxsolver.platform.MemoryMonitor

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun MemoryIndicator(modifier: Modifier = Modifier, pollingIntervalMs: Long = 2000L) {
    val memoryMonitor = remember { MemoryMonitor() }
    var memoryInfo by remember { mutableStateOf(memoryMonitor.getMemoryInfo()) }

    DisposableEffect(memoryMonitor) {
        memoryMonitor.startMonitoring(pollingIntervalMs) { info ->
            memoryInfo = info
        }
        onDispose {
            memoryMonitor.stopMonitoring()
        }
    }

    val statusColor by animateColorAsState(
        targetValue = when {
            memoryInfo.usagePercent < 0.6f -> Color(0xFF4CAF50)
            memoryInfo.usagePercent < 0.8f -> Color(0xFFFF9800)
            else -> Color(0xFFF44336)
        },
        animationSpec = MotionScheme.expressive().defaultEffectsSpec(),
        label = "statusColor"
    )

    val animatedProgress by animateFloatAsState(
        targetValue = memoryInfo.usagePercent.coerceIn(0f, 1f),
        animationSpec = MotionScheme.expressive().slowSpatialSpec(),
        label = "memoryProgress"
    )

    val progressGradient = Brush.horizontalGradient(
        colors = listOf(
            statusColor.copy(alpha = 0.85f),
            statusColor,
            statusColor
        )
    )

    val trackShape = RoundedCornerShape(50)

    Row(
        modifier = modifier,
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Box(
            modifier = Modifier
                .width(70.dp)
                .height(12.dp)
                .clip(trackShape)
                .background(MaterialTheme.colorScheme.surfaceContainerLowest)
                .border(
                    width = 1.5.dp,
                    color = MaterialTheme.colorScheme.outline.copy(alpha = 0.4f),
                    shape = trackShape
                )
        ) {
            Box(
                modifier = Modifier
                    .width(70.dp * animatedProgress)
                    .fillMaxHeight()
                    .clip(trackShape)
                    .background(progressGradient)
            )
        }

        Column(
            verticalArrangement = Arrangement.spacedBy(1.dp)
        ) {
            Text(
                text = "${memoryInfo.usedMB}/${memoryInfo.totalMB} MB",
                style = MaterialTheme.typography.labelSmall,
                fontWeight = FontWeight.Medium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis
            )

            if (memoryInfo.appMB > 0) {
                Text(
                    text = "App: ${memoryInfo.appMB} MB",
                    style = MaterialTheme.typography.labelSmall,
                    color = statusColor,
                    fontWeight = FontWeight.SemiBold,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}
