package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.background
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.LocalCompare
import com.wowsinfo.libwowsinfo.ui.chartColor

/**
 * Side-by-side ship comparison table (Flutter `compare_ship_page` port). The
 * first column is the stat name; each following column is one ship.
 */
@Composable
fun WikiCompareDialog(
    compare: LocalCompare,
    onDismiss: () -> Unit,
) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Compare Ships") },
        text = {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
            ) {
                Row(modifier = Modifier.background(MaterialTheme.colorScheme.surfaceVariant)) {
                    HeaderCell("")
                    compare.ships.forEachIndexed { index, ship ->
                        HeaderCell(ship.index, color = chartColor(index + 1))
                    }
                }
                compare.rows.forEachIndexed { rowIndex, row ->
                    Row(
                        modifier = Modifier.background(
                            if (rowIndex % 2 == 0) {
                                MaterialTheme.colorScheme.surface
                            } else {
                                MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.4f)
                            },
                        ),
                    ) {
                        LabelCell(row.label)
                        row.values.forEach { value -> ValueCell(value) }
                    }
                }
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) { Text("Close") }
        },
    )
}

@Composable
private fun HeaderCell(text: String, color: androidx.compose.ui.graphics.Color? = null) {
    Text(
        text = text,
        modifier = Modifier.padding(horizontal = 6.dp, vertical = 8.dp).background(androidx.compose.ui.graphics.Color.Transparent, shape = MaterialTheme.shapes.small),
        style = MaterialTheme.typography.labelMedium,
        fontWeight = FontWeight.Bold,
        color = color ?: MaterialTheme.colorScheme.onSurface,
    )
}

@Composable
private fun LabelCell(text: String) {
    Text(
        text = text,
        modifier = Modifier.padding(horizontal = 6.dp, vertical = 6.dp),
        style = MaterialTheme.typography.bodySmall,
        fontWeight = FontWeight.Bold,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
}

@Composable
private fun ValueCell(text: String) {
    Text(
        text = text,
        modifier = Modifier.padding(horizontal = 6.dp, vertical = 6.dp),
        style = MaterialTheme.typography.bodySmall,
    )
}
