package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.formatNumber
import java.util.Locale

/**
 * Battle distribution charts, mirroring the original app's Graph screen
 * (battles by tier / type / nation, plus average tier).
 */
@Composable
fun ChartsTab(ships: List<ShipStatLine>) {
    val tier = remember(ships) { aggregate(ships) { it.tier.toString() } }
    val type = remember(ships) { aggregate(ships) { it.type } }
    val nation = remember(ships) { aggregate(ships) { it.nation } }
    val avgTier = remember(ships) {
        val battles = ships.sumOf { it.battles }
        if (battles == 0L) 0.0 else ships.sumOf { it.tier * it.battles }.toDouble() / battles
    }

    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        item {
            Text(
                "Average Tier - ${String.format(Locale.US, "%.1f", avgTier)}",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
                modifier = Modifier.fillMaxWidth().padding(top = 8.dp),
            )
        }
        item { ChartSection("Battles by Tier", tier) }
        item { ChartSection("Battles by Type", type) }
        item { ChartSection("Battles by Nation", nation) }
    }
}

private fun aggregate(
    ships: List<ShipStatLine>,
    keyOf: (ShipStatLine) -> String,
): List<Pair<String, Long>> =
    ships.groupBy(keyOf)
        .map { (key, group) -> key to group.sumOf { it.battles } }
        .filter { it.second > 0 }
        .sortedByDescending { it.second }

@Composable
private fun ChartSection(title: String, data: List<Pair<String, Long>>) {
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle(title)
        if (data.isEmpty()) {
            Text(
                "No battles",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            val max = data.maxOf { it.second }.coerceAtLeast(1)
            data.forEach { (label, value) ->
                BarRow(label, value, value.toDouble() / max)
            }
        }
    }
}

@Composable
private fun BarRow(label: String, value: Long, fraction: Double) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = label.replace('_', ' ').replaceFirstChar { it.uppercase() },
            style = MaterialTheme.typography.bodySmall,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            modifier = Modifier.width(110.dp),
        )
        Box(
            modifier = Modifier
                .weight(1f)
                .height(14.dp)
                .background(MaterialTheme.colorScheme.surfaceVariant),
        ) {
            Box(
                modifier = Modifier
                    .fillMaxWidth(fraction.toFloat())
                    .height(14.dp)
                    .background(MaterialTheme.colorScheme.primary),
            )
        }
        Text(
            text = formatNumber(value),
            style = MaterialTheme.typography.bodySmall,
            textAlign = TextAlign.End,
            modifier = Modifier.width(56.dp).padding(start = 4.dp),
        )
    }
}
