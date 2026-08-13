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
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.WeaponStats
import com.wowsinfo.libwowsinfo.ui.SectionTitle
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
        item { ModeStatsGrid(stats) }
        item { BestShipsSection(player.ships) }
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
fun ModeStatsGrid(stats: PvpStats?) {
    val battles = stats?.battles ?: 0L
    val wins = stats?.wins ?: 0L
    val damage = stats?.damageDealt ?: 0L
    val xp = stats?.xp ?: 0L
    val frags = stats?.frags ?: 0L
    val survivedBattles = stats?.survivedBattles ?: 0L
    val survivedWins = stats?.survivedWins ?: 0L
    val planes = stats?.planesKilled ?: 0L
    val spotted = stats?.shipsSpotted ?: 0L
    val maxDamage = stats?.maxDamageDealt ?: 0L
    val maxFrags = stats?.maxFragsBattle ?: 0L
    val maxXp = stats?.maxXp ?: 0L
    val draws = stats?.draws ?: 0L
    val potential = (stats?.artAgro ?: 0L) + (stats?.torpedoAgro ?: 0L)
    val capture = stats?.capturePoints ?: 0L
    val teamCapture = stats?.teamCapturePoints ?: 0L
    val maxPlanes = stats?.maxPlanesKilled ?: 0L
    val maxSpotted = stats?.maxShipsSpotted ?: 0L
    val maxTotalAgro = stats?.maxTotalAgro ?: 0L

    val winRate = if (battles > 0) wins.toDouble() / battles * 100.0 else 0.0
    val avgDmg = if (battles > 0) damage.toDouble() / battles else 0.0
    val avgXp = if (battles > 0) xp.toDouble() / battles else 0.0
    val deaths = (battles - survivedBattles).coerceAtLeast(1)
    val killDeath = frags.toDouble() / deaths
    val survivedRate = if (battles > 0) survivedBattles.toDouble() / battles * 100.0 else 0.0
    val survivedWinsRate = if (battles > 0) survivedWins.toDouble() / battles * 100.0 else 0.0

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
            StatCell("Planes", formatNumber(planes)),
            StatCell("Spotted", formatNumber(spotted)),
            StatCell("Max DMG", formatNumber(maxDamage)),
        )
        StatRow(
            StatCell("Max Frags", formatNumber(maxFrags)),
            StatCell("Max XP", formatNumber(maxXp)),
            StatCell("Draws", formatNumber(draws)),
        )
        StatRow(
            StatCell("Potential", formatNumber(potential)),
            StatCell("Capture", formatNumber(capture)),
            StatCell("Team cap", formatNumber(teamCapture)),
        )
        StatRow(
            StatCell("Max planes", formatNumber(maxPlanes)),
            StatCell("Max spotted", formatNumber(maxSpotted)),
            StatCell("Max potential", formatNumber(maxTotalAgro)),
        )
        StatRow(
            StatCell("Main hit", weaponHitRate(stats?.mainBattery)),
            StatCell("Torp hit", weaponHitRate(stats?.torpedoes)),
            StatCell("Sec hit", weaponHitRate(stats?.secondBattery)),
        )
        StatRow(
            StatCell("Aircraft hit", weaponHitRate(stats?.aircraft)),
            StatCell("Ramming hit", weaponHitRate(stats?.ramming)),
            StatCell("Survived wins", formatPercent(survivedWinsRate)),
        )
    }
}

private fun weaponHitRate(weapon: WeaponStats?): String {
    if (weapon == null || weapon.shots <= 0) {
        return "—"
    }
    return formatPercent(weapon.hits.toDouble() / weapon.shots * 100.0)
}

@Composable
private fun BestShipsSection(ships: List<ShipStatLine>) {
    val best = remember(ships) {
        ships.filter { it.battles > 0 }.sortedByDescending { it.rating }.take(5)
    }
    if (best.isEmpty()) return
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Best ships")
        Row(
            modifier = Modifier.fillMaxWidth().horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            best.forEach { ship ->
                ShipCell(ship, modifier = Modifier.width(120.dp))
            }
        }
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
