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
import androidx.compose.foundation.lazy.LazyColumn
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
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
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
    val stats = ship.statistics.pvp

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
            item { SectionTitle("PvP") }
            if (stats != null && stats.battles > 0) {
                item { ModeStatsGrid(stats) }
            } else {
                item {
                    Text(
                        "No battles",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }
    }
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
