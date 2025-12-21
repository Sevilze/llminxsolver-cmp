package com.llminxsolver.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ButtonGroupDefaults
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.ToggleButton
import androidx.compose.material3.ToggleButtonDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.llminxsolver.data.IgnoreFlags

data class PieceOption(val key: String, val label: String, val isActive: Boolean)

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
fun IgnoreOptions(
    flags: IgnoreFlags,
    onChange: (String, Boolean) -> Unit,
    enabled: Boolean = true,
    compact: Boolean = false,
    modifier: Modifier = Modifier
) {
    val options = listOf(
        PieceOption("cornerPositions", "CP", !flags.cornerPositions),
        PieceOption("edgePositions", "EP", !flags.edgePositions),
        PieceOption("cornerOrientations", "CO", !flags.cornerOrientations),
        PieceOption("edgeOrientations", "EO", !flags.edgeOrientations)
    )

    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(if (compact) 0.dp else 8.dp)
    ) {
        if (!compact) {
            Text(
                text = "Solve For",
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(bottom = 4.dp)
            )
        }

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ButtonGroupDefaults.ConnectedSpaceBetween)
        ) {
            options.forEachIndexed { index, option ->
                ToggleButton(
                    checked = option.isActive,
                    onCheckedChange = { checked ->
                        if (enabled) onChange(option.key, !checked)
                    },
                    enabled = enabled,
                    modifier = Modifier.weight(1f),
                    shapes = when (index) {
                        0 -> ButtonGroupDefaults.connectedLeadingButtonShapes()
                        options.lastIndex -> ButtonGroupDefaults.connectedTrailingButtonShapes()
                        else -> ButtonGroupDefaults.connectedMiddleButtonShapes()
                    },
                    colors = ToggleButtonDefaults.toggleButtonColors(
                        checkedContainerColor = MaterialTheme.colorScheme.primary,
                        checkedContentColor = MaterialTheme.colorScheme.onPrimary,
                        containerColor = MaterialTheme.colorScheme.secondaryContainer,
                        contentColor = MaterialTheme.colorScheme.onSecondaryContainer
                    )
                ) {
                    Text(option.label)
                }
            }
        }
    }
}
