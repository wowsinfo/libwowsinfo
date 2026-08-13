package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.background
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
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
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.PvpStats
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatEpochDate
import com.wowsinfo.libwowsinfo.ui.formatDecimal
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

private enum class StatMode(val label: String) {
    PvP("PvP"),
    Solo("Solo"),
    Div2("Div2"),
    Div3("Div3"),
    PvE("PvE"),
    Rank("Rank"),
}

@Composable
fun GeneralTab(player: PlayerView) {
    var mode by remember { mutableStateOf(StatMode.PvP) }
    val stats = when (mode) {
        StatMode.PvP -> player.statistics.pvp
        StatMode.Solo -> player.statistics.solo
        StatMode.Div2 -> player.statistics.div2
        StatMode.Div3 -> player.statistics.div3
        StatMode.PvE -> player.statistics.pve
        StatMode.Rank -> player.statistics.rankSolo
    }

    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item { PlayerHeader(player) }
        item { ModeSelector(mode) { mode = it } }
        if (stats != null && stats.battles > 0) {
            item { StatsGrid(stats) }
        } else {
            item {
                Text(
                    "No battles in this mode",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun PlayerHeader(player: PlayerView) {
    val ratingColor = remember(player.ratingColour) { parseRatingColor(player.ratingColour) }
    val displayName =
        if (player.clanTag.isNotEmpty()) {
            "${player.clanTag}\n${player.nickname}"
        } else {
            player.nickname
        }
    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Text(
            text = displayName,
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Medium,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Server", player.server.uppercase(), modifier = Modifier.weight(1f))
            Stat(
                "Level",
                player.levelingTier?.toString() ?: "Unknown",
                modifier = Modifier.weight(1f),
            )
        }
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Created", formatEpochDate(player.createdAt), modifier = Modifier.weight(1f))
            Stat(
                "Last battle",
                formatEpochDate(player.lastBattleTime),
                modifier = Modifier.weight(1f),
            )
        }
        Box(
            modifier = Modifier.fillMaxWidth().background(ratingColor).padding(vertical = 10.dp),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = player.ratingComment,
                color = Color.White,
                fontWeight = FontWeight.Bold,
                fontSize = 16.sp,
            )
        }
        if (player.hiddenProfile) {
            Text(
                "Hidden profile",
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
                textAlign = TextAlign.Center,
                modifier = Modifier.fillMaxWidth(),
            )
        }
        Text(
            "Average performance: ${formatNumber(player.ap)}",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
    }
}

@Composable
private fun ModeSelector(selected: StatMode, onSelect: (StatMode) -> Unit) {
    Row(
        modifier = Modifier.fillMaxWidth().horizontalScroll(rememberScrollState()),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        StatMode.entries.forEach { mode ->
            FilterChip(
                selected = selected == mode,
                onClick = { onSelect(mode) },
                label = { Text(mode.label) },
            )
        }
    }
}

@Composable
private fun StatsGrid(stats: PvpStats) {
    val battles = stats.battles
    if (battles <= 0) return
    val winRate = stats.wins.toDouble() / battles * 100.0
    val avgDmg = stats.damageDealt.toDouble() / battles
    val avgXp = stats.xp.toDouble() / battles
    val deaths = (battles - stats.survivedBattles).coerceAtLeast(1)
    val killDeath = stats.frags.toDouble() / deaths
    val survivedRate = stats.survivedBattles.toDouble() / battles * 100.0
    val potential = stats.artAgro + stats.torpedoAgro

    Column(verticalArrangement = Arrangement.spacedBy(12.dp)) {
        StatRow(
            StatCell("Battles", formatNumber(battles)),
            StatCell("WR", formatPercent(winRate)),
            StatCell("DMG", formatNumber(avgDmg)),
        )
        StatRow(
            StatCell("Avg XP", formatNumber(avgXp)),
            StatCell("K/D", formatDecimal(killDeath)),
            StatCell("Survived", formatPercent(survivedRate)),
        )
        StatRow(
            StatCell("Planes", formatNumber(stats.planesKilled)),
            StatCell("Spotted", formatNumber(stats.shipsSpotted)),
            StatCell("Max DMG", formatNumber(stats.maxDamageDealt)),
        )
        StatRow(
            StatCell("Max Frags", formatNumber(stats.maxFragsBattle)),
            StatCell("Max XP", formatNumber(stats.maxXp)),
            StatCell("Draws", formatNumber(stats.draws)),
        )
        StatRow(
            StatCell("Potential", formatNumber(potential)),
            StatCell("Capture", formatNumber(stats.capturePoints)),
            StatCell("Team cap", formatNumber(stats.teamCapturePoints)),
        )
        StatRow(
            StatCell("Max planes", formatNumber(stats.maxPlanesKilled)),
            StatCell("Max spotted", formatNumber(stats.maxShipsSpotted)),
            StatCell("Max potential", formatNumber(stats.maxTotalAgro)),
        )
    }
}

private data class StatCell(val label: String, val value: String)

@Composable
private fun StatRow(first: StatCell, second: StatCell, third: StatCell) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceAround,
    ) {
        Stat(first.label, first.value, modifier = Modifier.weight(1f))
        Stat(second.label, second.value, modifier = Modifier.weight(1f))
        Stat(third.label, third.value, modifier = Modifier.weight(1f))
    }
}
