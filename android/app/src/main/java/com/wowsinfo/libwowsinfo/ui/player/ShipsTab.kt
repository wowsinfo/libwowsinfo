package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.PlayerView
import com.wowsinfo.libwowsinfo.ShipStatLine
import com.wowsinfo.libwowsinfo.ui.formatNumber
import com.wowsinfo.libwowsinfo.ui.formatPercent
import com.wowsinfo.libwowsinfo.ui.formatRating
import com.wowsinfo.libwowsinfo.ui.parseRatingColor

val PremiumColor = Color(0xFFFF9800)

private val SortOptions = listOf(
    "Battles" to "battles",
    "Avg damage" to "avgDmg",
    "Winrate" to "avgWinrate",
    "Avg frags" to "avgFrags",
    "Rating" to "rating",
    "AP" to "ap",
    "Last battle" to "last_battle_time",
    "Max damage" to "pvp.max_damage_dealt",
    "Max XP" to "pvp.max_xp",
    "Max frags" to "pvp.max_frags_battle",
)

/** Ship grid mirroring `PlayerShipInfoPage` with search, filters and sorting. */
@Composable
fun ShipsTab(player: PlayerView, onShipClick: (ShipStatLine) -> Unit) {
    val ships = player.ships
    if (ships.isEmpty()) {
        Box(Modifier.fillMaxWidth().padding(24.dp), contentAlignment = Alignment.Center) {
            Text("No ship stats", style = MaterialTheme.typography.bodyMedium)
        }
        return
    }
    var query by rememberSaveable { mutableStateOf("") }
    var tierFilter by rememberSaveable { mutableStateOf(0L) }
    var typeFilter by rememberSaveable { mutableStateOf("") }
    var nationFilter by rememberSaveable { mutableStateOf("") }
    var sortKey by rememberSaveable { mutableStateOf("rating") }
    var sortAsc by rememberSaveable { mutableStateOf(false) }
    var sortMenu by remember { mutableStateOf(false) }

    val sorted = ships.filter { ship ->
        (tierFilter == 0L || ship.tier == tierFilter) &&
            (typeFilter.isEmpty() || ship.type == typeFilter) &&
            (nationFilter.isEmpty() || ship.nation.equals(nationFilter, ignoreCase = true)) &&
            (query.isBlank() || ship.name.contains(query, ignoreCase = true))
    }.sortedWith(
        compareBy<ShipStatLine> { ship -> shipSortValue(ship, sortKey) }
            .let { cmp -> if (sortAsc) cmp else cmp.reversed() },
    )

    val filteredRating = remember(sorted) { overallRating(sorted) }
    val ratingColor = remember(filteredRating) { parseRatingColor(ratingColourFor(filteredRating)) }
    val ratingComment = remember(filteredRating) { ratingCommentFor(filteredRating) }
    val types = ships.map { it.type }.distinct().sorted()
    val nations = ships.map { it.nation }.distinct().sorted()
    val tiers = ships.map { it.tier }.distinct().sorted()

    Column(modifier = Modifier.fillMaxWidth()) {
        Box(
            modifier = Modifier.fillMaxWidth().background(ratingColor).padding(vertical = 8.dp),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = "$ratingComment · ${formatRating(filteredRating)} · ${sorted.size} of ${ships.size} ships",
                color = Color.White,
                style = MaterialTheme.typography.titleSmall,
                fontWeight = androidx.compose.ui.text.font.FontWeight.Bold,
            )
        }
        OutlinedTextField(
            value = query,
            onValueChange = { query = it },
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp, vertical = 4.dp),
            placeholder = { Text("Search ships…") },
            singleLine = true,
        )
        FilterRow(types, typeFilter, "All types") { typeFilter = it }
        FilterRow(tiers.map { it.toString() }, tierFilter.toString(), "All tiers") {
            tierFilter = it.toLongOrNull() ?: 0L
        }
        FilterRow(nations.map { it.replace('_', ' ').replaceFirstChar { c -> c.uppercase() } },
            nationFilter.replace('_', ' ').replaceFirstChar { c -> c.uppercase() },
            "All nations"
        ) {
            nationFilter = it.replace(' ', '_').lowercase()
        }
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text("Sort", style = MaterialTheme.typography.labelMedium)
            Box {
                FilterChip(
                    selected = false,
                    onClick = { sortMenu = true },
                    label = { Text(SortOptions.first { it.second == sortKey }.first + if (sortAsc) " ↑" else " ↓") },
                )
                DropdownMenu(expanded = sortMenu, onDismissRequest = { sortMenu = false }) {
                    SortOptions.forEach { (label, key) ->
                        DropdownMenuItem(
                            text = { Text(label) },
                            onClick = {
                                if (sortKey == key) sortAsc = !sortAsc else {
                                    sortKey = key
                                    sortAsc = false
                                }
                                sortMenu = false
                            },
                        )
                    }
                }
            }
            Text(
                text = "${sorted.size} shown",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(start = 8.dp),
            )
        }
        LazyVerticalGrid(
            columns = GridCells.Fixed(3),
            modifier = Modifier.fillMaxWidth(),
            contentPadding = PaddingValues(12.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            items(sorted, key = { it.shipId.toString() }) { ship ->
                ShipCell(ship, onClick = { onShipClick(ship) })
            }
        }
    }
}

private val ratingRanges = listOf(0.0, 750.0, 1100.0, 1350.0, 1550.0, 1750.0, 2100.0, 2450.0, 9999.0)
private val ratingColours = listOf(
    "#607D8B", "#D32F2F", "#FF9800", "#FFB300", "#7CB342",
    "#388E3C", "#03A9F4", "#9C27B0", "#673AB7",
)
private val ratingLabels = listOf(
    "Unknown", "Bad", "Below Average", "Average", "Good", "Very Good", "Great", "Unicum", "Super Unicum",
)

/** Rating colour that tracks the filtered rating (Rust `get_colour`). */
private fun ratingColourFor(rating: Double): String {
    val index = ratingRanges.indexOfFirst { rating < it }.let { if (it == -1) 0 else it }
    return ratingColours.getOrElse(index) { "#607D8B" }
}

/** Rating label + diff that tracks the filtered rating (Rust `get_comment`). */
private fun ratingCommentFor(rating: Double): String {
    val index = ratingRanges.indexOfFirst { rating < it }.let { if (it == -1) 0 else it }
    val label = ratingLabels.getOrElse(index) { "Unknown" }
    val range = ratingRanges.getOrElse(index) { 0.0 }
    val diff = if (range == 9999.0) rating - 2450.0 else range - rating
    return "$label (+${formatNumber(diff)})"
}

/** Overall rating for a filtered ship set (RN `getOverallRating`). */
private fun overallRating(ships: List<ShipStatLine>): Double {
    var actualDmg = 0.0
    var expectedDmg = 0.0
    var actualWins = 0.0
    var expectedWins = 0.0
    var actualFrags = 0.0
    var expectedFrags = 0.0
    for (ship in ships) {
        val pvp = ship.statistics.pvp ?: continue
        if (pvp.battles == 0L) continue
        if (ship.expectedDmg <= 0.0 && ship.expectedWinrate <= 0.0 && ship.expectedFrags <= 0.0) {
            continue
        }
        actualDmg += pvp.damageDealt.toDouble() / pvp.battles
        actualWins += pvp.wins.toDouble() / pvp.battles * 100.0
        actualFrags += pvp.frags.toDouble() / pvp.battles
        expectedDmg += ship.expectedDmg
        expectedWins += ship.expectedWinrate
        expectedFrags += ship.expectedFrags
    }
    if (expectedDmg <= 0.0 || expectedWins <= 0.0 || expectedFrags <= 0.0) {
        return 0.0
    }
    val nDmg = ((actualDmg / expectedDmg - 0.4) / 0.6).coerceAtLeast(0.0)
    val nFrags = ((actualFrags / expectedFrags - 0.1) / 0.9).coerceAtLeast(0.0)
    val nWins = ((actualWins / expectedWins - 0.7) / 0.3).coerceAtLeast(0.0)
    return (700.0 * nDmg + 300.0 * nFrags + 150.0 * nWins).coerceAtMost(9999.0)
}

@Composable
private fun FilterRow(options: List<String>, selected: String, allLabel: String, onSelect: (String) -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .horizontalScroll(rememberScrollState())
            .padding(horizontal = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        FilterChip(
            selected = selected.isEmpty() || selected == "0",
            onClick = { onSelect("") },
            label = { Text(allLabel) },
        )
        options.forEach { option ->
            FilterChip(
                selected = selected == option,
                onClick = { onSelect(if (selected == option) "" else option) },
                label = { Text(option) },
            )
        }
    }
}

private fun shipSortValue(ship: ShipStatLine, key: String): Double = when (key) {
    "battles" -> ship.battles.toDouble()
    "avgDmg" -> ship.avgDmg
    "avgWinrate" -> ship.avgWinrate
    "avgFrags" -> ship.avgFrags
    "rating" -> ship.rating
    "ap" -> ship.ap
    "last_battle_time" -> ship.lastBattleTime.toDouble()
    "pvp.max_damage_dealt" -> (ship.statistics.pvp?.maxDamageDealt ?: 0L).toDouble()
    "pvp.max_xp" -> (ship.statistics.pvp?.maxXp ?: 0L).toDouble()
    else -> (ship.statistics.pvp?.maxFragsBattle ?: 0L).toDouble()
}

/** One ship cell: image, tier + name, battles / winrate / damage, rating bar. */
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
        val iconModel = if (ship.icon.isNotBlank()) {
            ship.icon
        } else if (ship.index.isNotBlank()) {
            "file:///android_asset/ships/${ship.index}.png"
        } else {
            ""
        }
        AsyncImage(
            model = iconModel.ifBlank { null },
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
        Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceAround) {
            Text(formatNumber(ship.battles), style = MaterialTheme.typography.labelSmall)
            Text(formatPercent(ship.avgWinrate), style = MaterialTheme.typography.labelSmall)
            Text(formatNumber(ship.avgDmg), style = MaterialTheme.typography.labelSmall)
        }
        Text(
            text = formatRating(ship.rating),
            style = MaterialTheme.typography.labelSmall,
            color = ratingColor,
            fontWeight = androidx.compose.ui.text.font.FontWeight.Bold,
        )
        Box(modifier = Modifier.fillMaxWidth().height(3.dp).background(ratingColor))
    }
}
