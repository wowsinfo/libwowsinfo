package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.ChartSection
import com.wowsinfo.libwowsinfo.ui.LineChartSection
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import java.util.Locale

/**
 * The nine charts from the Flutter app: recent battles/winrate/damage (line
 * charts), battles by nation/type/tier, and top ten ships by battles,
 * winrate and damage.
 */
@Composable
fun ChartsTab(player: PlayerView) {
    val ships = player.ships
    val tier = remember(ships) { aggregate(ships) { it.tier.toString() } }
    val type = remember(ships) { aggregate(ships) { it.type } }
    val nation = remember(ships) { aggregate(ships) { it.nation } }
    val avgTier = remember(ships) {
        val battles = ships.sumOf { it.battles }
        if (battles == 0L) 0.0 else ships.sumOf { it.tier * it.battles }.toDouble() / battles
    }
    val topBattles = remember(ships) { topTen(ships) { it.battles.toDouble() } }
    val topWinrate = remember(ships) { topTen(ships, maxValue = 99.0) { it.avgWinrate } }
    val topDamage = remember(ships) { topTen(ships) { it.avgDmg } }
    val recent = player.recent

    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        recent?.takeIf { it.days.size >= 2 }?.let { overview ->
            item {
                LineChartSection(
                    title = "Recent battles",
                    subtitle = "${formatNumber(overview.totalBattles)} battles",
                    points = overview.days.map { it.date to it.battles.toDouble() },
                    color = Color(0xFF2196F3),
                )
            }
            item {
                LineChartSection(
                    title = "Recent average winrate",
                    subtitle = formatPercent(overview.avgWinrate),
                    points = overview.days.map { it.date to it.winrate },
                    color = Color(0xFF4CAF50),
                )
            }
            item {
                LineChartSection(
                    title = "Recent average damage",
                    subtitle = formatNumber(overview.avgDamage),
                    points = overview.days.map { it.date to it.avgDamage },
                    color = Color(0xFFD32F2F),
                )
            }
        }
        item {
            Text(
                "Average Tier - ${String.format(Locale.US, "%.1f", avgTier)}",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
                modifier = Modifier.fillMaxWidth().padding(top = 8.dp),
            )
        }
        item {
            ChartSection(
                title = "Battles by Tier",
                entries = tier,
                subtitle = "Avg tier - ${String.format(Locale.US, "%.1f", avgTier)}",
            )
        }
        item { ChartSection("Battles by Type", type) }
        item { ChartSection("Battles by Nation", nation) }
        item {
            ChartSection(
                title = "Top ten ships by battles",
                entries = topBattles,
                color = Color(0xFFD32F2F),
            )
        }
        item {
            ChartSection(
                title = "Top ten ships by winrate",
                entries = topWinrate,
                valueFormat = { formatPercent(it) },
                color = Color(0xFF4CAF50),
            )
        }
        item {
            ChartSection(
                title = "Top ten ships by damage",
                entries = topDamage,
                color = Color(0xFF2196F3),
            )
        }
    }
}

private fun aggregate(
    ships: List<ShipStatLine>,
    keyOf: (ShipStatLine) -> String,
): List<Pair<String, Double>> =
    ships.groupBy(keyOf)
        .map { (key, group) -> key to group.sumOf { it.battles }.toDouble() }
        .filter { it.second > 0 }
        .sortedByDescending { it.second }

private fun topTen(
    ships: List<ShipStatLine>,
    maxValue: Double = Double.MAX_VALUE,
    valueOf: (ShipStatLine) -> Double,
): List<Pair<String, Double>> =
    ships.asSequence()
        .filter { it.battles > 5 }
        .map { it.name to valueOf(it) }
        .filter { it.second.isFinite() && it.second < maxValue }
        .sortedByDescending { it.second }
        .take(10)
        .toList()
