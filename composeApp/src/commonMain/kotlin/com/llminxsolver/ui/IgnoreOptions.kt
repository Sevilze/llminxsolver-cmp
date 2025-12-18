package com.llminxsolver.ui

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.spring
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MultiChoiceSegmentedButtonRow
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
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

        MultiChoiceSegmentedButtonRow(
            modifier = Modifier.fillMaxWidth()
        ) {
            options.forEachIndexed { index, option ->
                SegmentedButton(
                    shape = SegmentedButtonDefaults.itemShape(
                        index = index,
                        count = options.size
                    ),
                    checked = option.isActive,
                    onCheckedChange = { checked ->
                        if (enabled) onChange(option.key, !checked)
                    },
                    enabled = enabled,
                    icon = {
                        if (option.isActive) {
                            Icon(
                                imageVector = Icons.Filled.Check,
                                contentDescription = null
                            )
                        }
                    }
                ) {
                    Text(option.label)
                }
            }
        }
    }
}
