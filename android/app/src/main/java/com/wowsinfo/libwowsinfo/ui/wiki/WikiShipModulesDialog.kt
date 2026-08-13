package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ModuleSlotView
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber

/**
 * Ship-builder style module picker: one slot (hull, main battery, torpedoes,
 * fire control, engine, aircraft) with its upgrade options and costs.
 */
@OptIn(ExperimentalLayoutApi::class)
@Composable
fun WikiShipModulesDialog(
    slots: List<ModuleSlotView>,
    onDismiss: () -> Unit,
    onSelect: (slot: String, index: Long) -> Unit,
) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Change Ship Modules") },
        text = {
            LazyColumn(modifier = Modifier.fillMaxWidth()) {
                slots.forEachIndexed { index, slot ->
                    item(key = slot.slot) {
                        Column(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(vertical = 8.dp),
                            verticalArrangement = Arrangement.spacedBy(6.dp),
                        ) {
                            Text(
                                text = slot.label,
                                style = MaterialTheme.typography.titleSmall,
                                fontWeight = FontWeight.Bold,
                                color = chartColor(index + 1),
                            )
                            FlowRow(
                                modifier = Modifier.fillMaxWidth(),
                                horizontalArrangement = Arrangement.spacedBy(6.dp),
                            ) {
                                slot.options.forEach { option ->
                                    FilterChip(
                                        selected = option.index == slot.selected,
                                        onClick = { onSelect(slot.slot, option.index) },
                                        label = {
                                            Text(
                                                text = buildString {
                                                    append(option.name)
                                                    if (option.costXp > 0) {
                                                        append(" · ${formatNumber(option.costXp)} XP")
                                                    }
                                                    if (option.costCr > 0) {
                                                        append(" · ${formatNumber(option.costCr)} cr")
                                                    }
                                                },
                                                style = MaterialTheme.typography.bodySmall,
                                            )
                                        },
                                    )
                                }
                            }
                        }
                    }
                }
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) { Text("Done") }
        },
    )
}
