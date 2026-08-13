package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.formatRating
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

val PremiumColor = Color(0xFFFF9800)

/** Ship grid like `PlayerShipInfoPage` (icon + tier label, tap for detail). */
@Composable
fun ShipsTab(ships: List<ShipStatLine>, onShipClick: (ShipStatLine) -> Unit) {
    LazyVerticalGrid(
        columns = GridCells.Fixed(3),
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(12.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        items(ships, key = { it.shipId.toString() }) { ship ->
            ShipCell(ship, onClick = { onShipClick(ship) })
        }
    }
}

/** One ship cell: image, tier + name, rating. */
@Composable
fun ShipCell(
    ship: ShipStatLine,
    modifier: Modifier = Modifier,
    onClick: (() -> Unit)? = null,
) {
    val ratingColor = remember(ship.ratingColour) { parseRatingColor(ship.ratingColour) }
    Column(
        modifier = modifier
            .fillMaxWidth()
            .clickable(enabled = onClick != null) { onClick?.invoke() },
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        AsyncImage(
            model = ship.icon,
            contentDescription = null,
            modifier = Modifier.fillMaxWidth().aspectRatio(1.7f),
        )
        Text(
            text = "T${ship.tier} ${ship.name}",
            style = MaterialTheme.typography.bodySmall,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            color = if (ship.premium) PremiumColor else Color.Unspecified,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = formatRating(ship.rating),
            style = MaterialTheme.typography.labelSmall,
            color = ratingColor,
        )
    }
}
