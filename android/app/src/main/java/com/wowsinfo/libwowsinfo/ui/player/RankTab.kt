package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.RankSeason
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent

/**
 * Ranked-season tab mirroring the Flutter `PlayerRankInfoPage`: one card per
 * ranked season with rank progress and the solo/div2/div3 battle stats.
 */
@Composable
fun RankTab(player: PlayerView) {
    val seasons = player.rank?.seasons.orEmpty()
    val entries = seasons.entries.sortedByDescending { it.key.toIntOrNull() ?: 0 }
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        if (entries.isEmpty()) {
            item {
                Text(
                    text = "No ranked data for this season cycle.",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            return@LazyColumn
        }
        item { SectionTitle("Ranked seasons") }
        items(entries.size) { index ->
            SeasonCard(entries[index].key, entries[index].value)
        }
    }
}

@Composable
private fun SeasonCard(seasonId: String, season: RankSeason) {
    val rankInfo = season.rankInfo
    // The API nests stats per rank key; `-1` aggregates all ranks, otherwise
    // fall back to the first rank bucket that has any battles.
    val mode = season.ranks["-1"]
        ?: season.ranks.values.maxByOrNull { it.rankSolo?.battles ?: 0 }
    val solo = mode?.rankSolo
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(12.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    text = "Season $seasonId",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                )
                if (rankInfo != null) {
                    Text(
                        text = "Rank ${rankInfo.rank}/${rankInfo.maxRank}",
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.primary,
                    )
                }
            }
            if (rankInfo != null) {
                Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
                    Stat("Stars", formatNumber(rankInfo.stars))
                    Stat("Stage", formatNumber(rankInfo.stage))
                    Stat("Start", formatNumber(rankInfo.startRank))
                }
            }
            if (solo != null) {
                Text(
                    text = "Solo",
                    style = MaterialTheme.typography.labelLarge,
                    color = MaterialTheme.colorScheme.primary,
                )
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceAround,
                ) {
                    Stat("Battles", formatNumber(solo.battles), modifier = Modifier.weight(1f))
                    Stat("WR", formatPercent(solo.winRate()), modifier = Modifier.weight(1f))
                    Stat("DMG", formatNumber(solo.avgDamage()), modifier = Modifier.weight(1f))
                }
            }
            listOf(
                "Div2" to mode?.rankDiv2,
                "Div3" to mode?.rankDiv3,
            ).forEach { (label, stats) ->
                if (stats != null && stats.battles > 0) {
                    Text(
                        text = label,
                        style = MaterialTheme.typography.labelLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceAround,
                    ) {
                        Stat("Battles", formatNumber(stats.battles), modifier = Modifier.weight(1f))
                        Stat("WR", formatPercent(stats.winRate()), modifier = Modifier.weight(1f))
                        Stat("DMG", formatNumber(stats.avgDamage()), modifier = Modifier.weight(1f))
                    }
                }
            }
        }
    }
}

private fun com.wowsinfo.libwowsinfo.PvpStats.winRate(): Double =
    if (battles > 0) wins * 100.0 / battles else 0.0

private fun com.wowsinfo.libwowsinfo.PvpStats.avgDamage(): Double =
    if (battles > 0) damageDealt.toDouble() / battles else 0.0
