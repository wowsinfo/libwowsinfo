package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.EncyclopediaShip

/** Ship encyclopedia list ordered by tier then type. */
@Composable
fun WikiShipsTab(
    ships: Map<ULong, EncyclopediaShip>,
    onShipClick: (ULong) -> Unit,
) {
    if (ships.isEmpty()) {
        LoadingHint("Loading ships...")
        return
    }
    val sorted = ships.values.sortedWith(
        compareBy({ it.tier }, { it.type }, { it.name }),
    )
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(8.dp),
    ) {
        items(sorted, key = { it.shipId.toString() }) { ship ->
            RowShip(ship, onClick = { onShipClick(ship.shipId) })
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}

@Composable
private fun RowShip(ship: EncyclopediaShip, onClick: () -> Unit) {
    androidx.compose.foundation.layout.Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
            .padding(vertical = 6.dp, horizontal = 4.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        if (ship.icon.isBlank()) {
            Box(
                modifier = Modifier.padding(end = 8.dp),
                contentAlignment = Alignment.Center,
            ) {
                Text(
                    text = ship.name.take(6).uppercase(),
                    style = MaterialTheme.typography.labelMedium,
                    color = com.wowsinfo.libwowsinfo.ui.chartColor(ship.tier.toInt()),
                )
            }
        } else {
            AsyncImage(
                model = ship.icon,
                contentDescription = null,
                modifier = Modifier.padding(end = 8.dp),
            )
        }
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = "T${ship.tier} ${ship.name}",
                style = MaterialTheme.typography.bodyLarge,
                color = if (ship.premium) com.wowsinfo.libwowsinfo.ui.player.PremiumColor
                else MaterialTheme.colorScheme.onSurface,
            )
            Text(
                text = "${ship.nation.replace('_', ' ')} · ${ship.type}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
internal fun LoadingHint(text: String) {
    Column(
        modifier = Modifier.fillMaxWidth().padding(top = 48.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        CircularProgressIndicator()
        Text(
            text,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.padding(top = 8.dp),
        )
    }
}
