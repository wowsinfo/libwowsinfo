package com.wowsinfo.libwowsinfo.ui

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
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.Event
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ViewModel
import com.wowsinfo.libwowsinfo.core.Core

@Composable
fun PlayerScreen(
    core: Core,
    viewModel: ViewModel,
    onBack: () -> Unit,
) {
    val player = viewModel.player
    Column(Modifier.fillMaxSize()) {
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
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item { PlayerHeader(player) }
        item {
            Text(
                "Ships (${player.ships.size})",
                style = MaterialTheme.typography.titleMedium,
                modifier = Modifier.padding(top = 8.dp),
            )
        }
        items(player.ships, key = { it.shipId.toString() }) { ship ->
            ShipRow(ship)
        }
    }
}

@Composable
private fun PlayerHeader(player: PlayerView) {
    val ratingColor = remember(player.ratingColour) { parseRatingColor(player.ratingColour) }
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                text = player.nickname,
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.weight(1f),
            )
            Text(
                text = "Server: ${player.server.uppercase()}",
                style = MaterialTheme.typography.bodySmall,
            )
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text(
                text = formatRating(player.rating),
                style = MaterialTheme.typography.headlineMedium,
                color = ratingColor,
            )
            Text(player.ratingComment, style = MaterialTheme.typography.bodyMedium)
        }
        Text(
            "Average performance: ${formatNumber(player.ap)}",
            style = MaterialTheme.typography.bodySmall,
        )
        if (player.hiddenProfile) {
            Text(
                "Hidden profile",
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall,
            )
        }
    }
}

@Composable
private fun ShipRow(ship: ShipStatLine) {
    val ratingColor = remember(ship.ratingColour) { parseRatingColor(ship.ratingColour) }
    Column(
        modifier = Modifier.fillMaxWidth().padding(vertical = 8.dp),
        verticalArrangement = Arrangement.spacedBy(2.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                text = "T${ship.tier} ${ship.name}",
                style = MaterialTheme.typography.titleSmall,
                modifier = Modifier.weight(1f),
            )
            if (ship.premium) {
                Text(
                    "Premium",
                    color = MaterialTheme.colorScheme.tertiary,
                    style = MaterialTheme.typography.labelSmall,
                )
            }
        }
        Text(
            "${ship.type.uppercase()} · ${ship.nation.replace('_', ' ').replaceFirstChar { it.uppercase() }}",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
            Stat("Battles", formatNumber(ship.battles))
            Stat("DMG", formatNumber(ship.avgDmg))
            Stat("WR", formatPercent(ship.avgWinrate))
            Stat("Frags", formatNumber(ship.avgFrags))
        }
        Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
            Stat("Rating", formatRating(ship.rating), ratingColor)
            Stat("AP", formatNumber(ship.ap))
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
