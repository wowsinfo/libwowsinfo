package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.PvpStats
import com.wowsinfo.libwowsinfo.WeaponStats
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatDecimal
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent

/** Full stat grid for one mode (PvP, Solo, Div2, Div3, PvE, Rank). */
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
    val maxScouting = stats?.maxDamageScouting ?: 0L
    val toBuildings = stats?.maxDamageDealtToBuildings ?: 0L
    val maxSuppressions = stats?.maxSuppressionsCount ?: 0L

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
            StatCell("Max spotting", formatNumber(maxScouting)),
            StatCell("To buildings", formatNumber(toBuildings)),
            StatCell("Max suppression", formatNumber(maxSuppressions)),
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
