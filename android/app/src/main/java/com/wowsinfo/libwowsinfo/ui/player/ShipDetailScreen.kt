package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.PvpStats
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.charts.DonutChartSection
import com.wowsinfo.libwowsinfo.ui.charts.RadarChartSection
import com.wowsinfo.libwowsinfo.ui.charts.modeDistribution
import com.wowsinfo.libwowsinfo.ui.charts.shipRadar
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

/**
 * Single-ship detail mirroring the Flutter `PlayerShipDetailPage`: rating
 * banner, ship image, summary tiles and the full PvP stat blocks.
 */
@Composable
fun ShipDetailScreen(ship: ShipStatLine, onBack: () -> Unit) {
    val ratingColor = remember(ship.ratingColour) { parseRatingColor(ship.ratingColour) }
    val availableModes = remember(ship) {
        StatMode.entries.filter { modeStats(ship, it)?.battles?.let { battles -> battles > 0 } == true }
    }
    var mode by remember { mutableStateOf(availableModes.firstOrNull() ?: StatMode.PvP) }
    val stats = modeStats(ship, mode)
    val radar = remember(ship) { shipRadar(ship) }
    val shipModes = remember(ship) { modeDistribution(ship.statistics) }

    Column(modifier = Modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Ships") }
            Text(
                text = ship.name,
                style = MaterialTheme.typography.titleMedium,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
                modifier = Modifier.weight(1f).padding(horizontal = 8.dp),
            )
        }
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item { ShipBanner(ship, ratingColor) }
            item { ShipSummary(ship) }
            item { ShipAverage(ship) }
            radar?.let { values ->
                item { RadarChartSection(values) }
            }
            if (shipModes.isNotEmpty()) {
                item { DonutChartSection(shipModes) }
            }
            if (availableModes.size > 1) {
                item { ModeSelector(mode) { mode = it } }
            }
            item { ModeStatsGrid(stats) }
        }
    }
}

private fun modeStats(ship: ShipStatLine, mode: StatMode): PvpStats? = when (mode) {
    StatMode.PvP -> ship.statistics.pvp
    StatMode.Solo -> ship.statistics.solo
    StatMode.Div2 -> ship.statistics.div2
    StatMode.Div3 -> ship.statistics.div3
    StatMode.PvE -> ship.statistics.pve
    StatMode.Rank -> ship.statistics.rankSolo
}

@Composable
private fun ShipBanner(ship: ShipStatLine, ratingColor: Color) {
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Box(
            modifier = Modifier.fillMaxWidth().background(ratingColor).padding(vertical = 10.dp),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = ship.ratingComment,
                color = Color.White,
                fontWeight = FontWeight.Bold,
                fontSize = 16.sp,
            )
        }
        AsyncImage(
            model = ship.icon,
            contentDescription = null,
            modifier = Modifier.fillMaxWidth().aspectRatio(1.7f),
        )
        Text(
            text = "T${ship.tier} ${ship.name}",
            style = MaterialTheme.typography.titleLarge,
            color = if (ship.premium) PremiumColor else Color.Unspecified,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = "${ship.type.uppercase()} · ${ship.nation.replace('_', ' ').replaceFirstChar { it.uppercase() }}",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
    }
}

@Composable
private fun ShipSummary(ship: ShipStatLine) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceAround,
    ) {
        Stat("Battles", formatNumber(ship.battles), modifier = Modifier.weight(1f))
        Stat("DMG", formatNumber(ship.avgDmg), modifier = Modifier.weight(1f))
        Stat("WR", formatPercent(ship.avgWinrate), modifier = Modifier.weight(1f))
        Stat("Frags", formatNumber(ship.avgFrags), modifier = Modifier.weight(1f))
    }
}

/** Player stats vs the ship's average (from the PR table), like
 *  `ShipAverageStatistics` in the Flutter app. */
@Composable
private fun ShipAverage(ship: ShipStatLine) {
    if (ship.expectedDmg <= 0.0 && ship.expectedWinrate <= 0.0 && ship.expectedFrags <= 0.0) {
        return
    }
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Average")
        AverageRow("DMG", formatNumber(ship.avgDmg), formatNumber(ship.expectedDmg))
        AverageRow("WR", formatPercent(ship.avgWinrate), formatPercent(ship.expectedWinrate))
        AverageRow("Frags", formatNumber(ship.avgFrags), formatNumber(ship.expectedFrags))
    }
}

@Composable
private fun AverageRow(label: String, player: String, expected: String) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            label,
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.width(64.dp),
        )
        Text(
            player,
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.weight(1f),
        )
        Text(
            "avg $expected",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}
