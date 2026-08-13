package com.wowsinfo.libwowsinfo.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.Event
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ViewModel
import com.wowsinfo.libwowsinfo.core.Core

private val PremiumColor = Color(0xFFFF9800)

@Composable
fun PlayerScreen(
    core: Core,
    viewModel: ViewModel,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val player = viewModel.player
    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Search") }
            Spacer(Modifier.weight(1f))
            TextButton(onClick = { core.update(Event.Refresh) }) { Text("Refresh") }
        }

        if (player == null) {
            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    CircularProgressIndicator()
                    Spacer(Modifier.height(12.dp))
                    Text("Loading player…")
                }
            }
        } else {
            PlayerContent(player)
        }
    }
}

@Composable
private fun PlayerContent(player: PlayerView) {
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(start = 16.dp, end = 16.dp, bottom = 16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item { PlayerHeader(player) }
        item { SectionTitle("Ships (${player.ships.size})") }
        items(player.ships, key = { it.shipId.toString() }) { ship ->
            ShipRow(ship)
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}

@Composable
private fun PlayerHeader(player: PlayerView) {
    val ratingColor = remember(player.ratingColour) { parseRatingColor(player.ratingColour) }
    val (battles, winRate, avgDamage) = remember(player.ships) { shipTotals(player.ships) }

    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        // Like the original stats header: a square, colored banner showing
        // the rating comment.
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
        Text(
            text = player.nickname,
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Medium,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = "Server: ${player.server.uppercase()}",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        if (player.hiddenProfile) {
            Text(
                "Hidden profile",
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
                textAlign = TextAlign.Center,
                modifier = Modifier.fillMaxWidth(),
            )
        }
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Battles", formatNumber(battles))
            Stat("WR", formatPercent(winRate))
            Stat("DMG", formatNumber(avgDamage))
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
private fun ShipRow(ship: ShipStatLine) {
    val ratingColor = remember(ship.ratingColour) { parseRatingColor(ship.ratingColour) }
    Row(
        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        AsyncImage(
            model = ship.icon,
            contentDescription = null,
            modifier = Modifier.size(width = 76.dp, height = 44.dp),
        )
        Column(
            modifier = Modifier.weight(1f).padding(start = 12.dp),
            verticalArrangement = Arrangement.spacedBy(2.dp),
        ) {
            Text(
                text = "T${ship.tier} ${ship.name}",
                style = MaterialTheme.typography.titleSmall,
                color = if (ship.premium) PremiumColor else Color.Unspecified,
            )
            Text(
                "${ship.type.uppercase()} · ${ship.nation.replace('_', ' ').replaceFirstChar { it.uppercase() }}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                Stat("Battles", formatNumber(ship.battles))
                Stat("DMG", formatNumber(ship.avgDmg))
                Stat("WR", formatPercent(ship.avgWinrate))
                Stat("Frags", formatNumber(ship.avgFrags))
            }
            Row(horizontalArrangement = Arrangement.spacedBy(12.dp)) {
                Stat("Rating", formatRating(ship.rating), ratingColor)
                Stat("AP", formatNumber(ship.ap))
            }
        }
    }
}

@Composable
private fun Stat(label: String, value: String, color: Color = Color.Unspecified) {
    Column {
        Text(
            label,
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(value, style = MaterialTheme.typography.bodyMedium, color = color)
    }
}

/** Battles, win rate and average damage aggregated across the ship list. */
private fun shipTotals(ships: List<ShipStatLine>): Triple<Long, Double, Double> {
    val battles = ships.sumOf { it.battles }
    if (battles == 0L) {
        return Triple(0L, 0.0, 0.0)
    }
    val damage = ships.sumOf { it.avgDmg * it.battles }
    val wins = ships.sumOf { it.avgWinrate * it.battles / 100.0 }
    return Triple(battles, wins / battles * 100.0, damage / battles)
}
