package com.wowsinfo.libwowsinfo.ui.charts

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ui.BarRow
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent

/**
 * Per-class averages: one card per ship class with damage, winrate, frags and
 * XP bars plus survival/accuracy captions.
 */
@Composable
fun PerClassSection(
    classes: List<ClassAverage>,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(12.dp)) {
        SectionTitle("Per-class averages")
        classes.forEachIndexed { index, average ->
            Column(
                modifier = Modifier.fillMaxWidth(),
                verticalArrangement = Arrangement.spacedBy(4.dp),
            ) {
                Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
                    Text(
                        text = average.className,
                        style = MaterialTheme.typography.titleSmall,
                        color = chartColor(index),
                    )
                    Spacer(Modifier.weight(1f))
                    Text(
                        text = "${formatNumber(average.battles)} battles",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
                val maxDmg = classes.maxOf { it.avgDmg }.coerceAtLeast(1.0)
                BarRow(
                    label = "Damage",
                    valueText = formatNumber(average.avgDmg),
                    fraction = average.avgDmg / maxDmg,
                    color = chartColor(index),
                )
                val maxWinrate = classes.maxOf { it.avgWinrate }.coerceAtLeast(1.0)
                BarRow(
                    label = "Winrate",
                    valueText = formatPercent(average.avgWinrate),
                    fraction = average.avgWinrate / maxWinrate,
                    color = chartColor(index),
                )
                val maxFrags = classes.maxOf { it.avgFrags }.coerceAtLeast(0.1)
                BarRow(
                    label = "Frags",
                    valueText = String.format("%.2f", average.avgFrags),
                    fraction = average.avgFrags / maxFrags,
                    color = chartColor(index),
                )
                BarRow(
                    label = "XP",
                    valueText = formatNumber(average.avgXp),
                    fraction = average.avgXp / classes.maxOf { it.avgXp }.coerceAtLeast(1.0),
                    color = chartColor(index),
                )
                Text(
                    text = "Survival ${formatPercent(average.survival)} · " +
                        "Accuracy ${formatPercent(average.accuracy)}",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.padding(start = 4.dp),
                )
            }
        }
    }
}
