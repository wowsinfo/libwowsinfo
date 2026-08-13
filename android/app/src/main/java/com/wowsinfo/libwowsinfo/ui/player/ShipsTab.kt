package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import com.wowsinfo.libwowsinfo.ui.formatRating
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

private val PremiumColor = Color(0xFFFF9800)

@Composable
fun ShipsTab(ships: List<ShipStatLine>) {
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        item { SectionTitle("Ships (${ships.size})") }
        items(ships, key = { it.shipId.toString() }) { ship ->
            ShipRow(ship)
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
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
                Stat("Rating", formatRating(ship.rating), color = ratingColor)
                Stat("AP", formatNumber(ship.ap))
            }
        }
    }
}
