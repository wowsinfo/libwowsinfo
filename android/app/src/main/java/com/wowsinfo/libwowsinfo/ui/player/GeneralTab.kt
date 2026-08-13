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
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Card
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
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.PvpStats
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.WeaponStats
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatEpochDate
import com.wowsinfo.libwowsinfo.ui.formatDecimal
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import com.wowsinfo.libwowsinfo.ui.formatRating
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

enum class StatMode(val label: String) {
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
        player.clan?.let { clan ->
            item { ClanCard(clan) }
        }
        item { ModeSelector(mode) { mode = it } }
        item { ModeStatsGrid(stats) }
        item { BestShipsSection(player.ships) }
    }
}

/** Compact clan summary card shown above the mode stats. */
@Composable
private fun ClanCard(clan: com.wowsinfo.libwowsinfo.ClanInfo) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(12.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Text(
                text = "${clan.name} [${clan.tag}]",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = "${formatNumber(clan.membersCount)} members" +
                    clan.leaderName.takeIf { it.isNotEmpty() }?.let { " · Leader: $it" }.orEmpty(),
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
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
                player.levelingTier?.let { "Lv $it" } ?: "Unknown",
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
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat(
                "Total battles",
                formatNumber(player.statistics.battles),
                modifier = Modifier.weight(1f),
            )
            Stat(
                "Distance",
                "${formatNumber(player.statistics.distance)} km",
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
fun ModeSelector(selected: StatMode, onSelect: (StatMode) -> Unit) {
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

private data class BestEntry(val label: String, val ship: ShipStatLine, val value: String)

@Composable
private fun BestShipsSection(ships: List<ShipStatLine>) {
    val best = remember(ships) {
        listOfNotNull(
            ships.maxByOrNull { it.avgDmg }
                ?.let { BestEntry("Damage", it, formatNumber(it.avgDmg)) },
            ships.maxByOrNull { it.avgWinrate }
                ?.let { BestEntry("Winrate", it, formatPercent(it.avgWinrate)) },
            ships.maxByOrNull { it.avgFrags }
                ?.let { BestEntry("Frags", it, formatNumber(it.avgFrags)) },
            ships.maxByOrNull { it.rating }
                ?.let { BestEntry("Rating", it, formatRating(it.rating)) },
            ships.maxByOrNull { it.statistics.pvp?.torpedoes?.maxFragsBattle ?: 0L }
                ?.takeIf { (it.statistics.pvp?.torpedoes?.maxFragsBattle ?: 0L) > 0 }
                ?.let {
                    BestEntry(
                        "Torp",
                        it,
                        "${formatNumber(it.statistics.pvp?.torpedoes?.maxFragsBattle ?: 0L)} frags",
                    )
                },
            ships.maxByOrNull { it.statistics.pvp?.ramming?.maxFragsBattle ?: 0L }
                ?.takeIf { (it.statistics.pvp?.ramming?.maxFragsBattle ?: 0L) > 0 }
                ?.let {
                    BestEntry(
                        "Ramming",
                        it,
                        "${formatNumber(it.statistics.pvp?.ramming?.maxFragsBattle ?: 0L)} frags",
                    )
                },
        )
    }
    if (best.isEmpty()) return
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Best ships")
        best.forEach { entry ->
            Row(
                modifier = Modifier.fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    entry.label,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.width(72.dp),
                )
                Text(
                    entry.ship.name,
                    style = MaterialTheme.typography.bodyMedium,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    entry.value,
                    style = MaterialTheme.typography.bodyMedium,
                    color = remember(entry.ship.ratingColour) {
                        parseRatingColor(entry.ship.ratingColour)
                    },
                )
            }
        }
    }
}

