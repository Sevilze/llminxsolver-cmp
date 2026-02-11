package com.llminxsolver.ui.batch

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.NavigateBefore
import androidx.compose.material.icons.automirrored.filled.NavigateNext
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.FilledIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.BatchState
import com.llminxsolver.data.IgnoreFlags
import com.llminxsolver.data.MegaminxState
import com.llminxsolver.theme.MegaminxColorScheme
import com.llminxsolver.ui.components.IgnoreOptions
import com.llminxsolver.ui.megaminx.MegaminxViewer

@Composable
fun BatchStateNavigator(
    states: List<BatchState>,
    currentIndex: Int,
    onIndexChange: (Int) -> Unit,
    colorScheme: MegaminxColorScheme,
    ignoreFlags: IgnoreFlags = IgnoreFlags(),
    onIgnoreFlagChange: (String, Boolean) -> Unit = { _, _ -> },
    enabled: Boolean = true,
    modifier: Modifier = Modifier
) {
    val currentState = states.getOrNull(currentIndex)

    Column(
        modifier = modifier.fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        if (currentState != null) {
            Box(
                modifier = Modifier.fillMaxWidth(0.9f),
                contentAlignment = Alignment.Center
            ) {
                MegaminxViewer(
                    puzzleState = currentState.megaminxState,
                    ignoreFlags = ignoreFlags,
                    colorScheme = colorScheme,
                    enabled = false,
                    modifier = Modifier.fillMaxWidth()
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            IgnoreOptions(
                flags = ignoreFlags,
                onChange = onIgnoreFlagChange,
                enabled = enabled,
                compact = true
            )
        } else {
            Box(
                modifier = Modifier.fillMaxWidth().height(200.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = "No states generated",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }

        Spacer(modifier = Modifier.height(20.dp))

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically
        ) {
            FilledIconButton(
                onClick = { if (currentIndex > 0) onIndexChange(currentIndex - 1) },
                enabled = currentIndex > 0,
                modifier = Modifier.size(48.dp),
                colors = IconButtonDefaults.filledIconButtonColors(
                    containerColor = MaterialTheme.colorScheme.secondaryContainer,
                    contentColor = MaterialTheme.colorScheme.onSecondaryContainer
                )
            ) {
                Icon(
                    imageVector = Icons.AutoMirrored.Filled.NavigateBefore,
                    contentDescription = "Previous"
                )
            }

            Text(
                text = if (states.isNotEmpty()) {
                    "${currentIndex + 1} / ${states.size}"
                } else {
                    "0 / 0"
                },
                style = MaterialTheme.typography.titleMedium,
                modifier = Modifier.padding(horizontal = 24.dp)
            )

            FilledIconButton(
                onClick = { if (currentIndex < states.size - 1) onIndexChange(currentIndex + 1) },
                enabled = currentIndex < states.size - 1,
                modifier = Modifier.size(48.dp),
                colors = IconButtonDefaults.filledIconButtonColors(
                    containerColor = MaterialTheme.colorScheme.secondaryContainer,
                    contentColor = MaterialTheme.colorScheme.onSecondaryContainer
                )
            ) {
                Icon(
                    imageVector = Icons.AutoMirrored.Filled.NavigateNext,
                    contentDescription = "Next"
                )
            }
        }
    }
}
