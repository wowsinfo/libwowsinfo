package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AssistChip
import androidx.compose.material3.Card
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ArtilleryProfile
import com.wowsinfo.libwowsinfo.EncyclopediaShip
import com.wowsinfo.libwowsinfo.ShipWiki
import com.wowsinfo.libwowsinfo.ui.BarRow
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber

/** Full wiki ship detail: profile bars, armour, weapons, mobility, similar ships. */
@Composable
fun WikiShipDetailScreen(
    ship: ShipWiki,
    similarShips: List<EncyclopediaShip>,
    onBack: () -> Unit,
    onShipClick: (ULong) -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Wiki") }
            Text(
                text = ship.name,
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.weight(1f).padding(horizontal = 8.dp),
            )
        }
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            item { ShipHeader(ship) }
            item { ProfileBars(ship) }
            item { ArmourSection(ship) }
            ship.profile.artillery?.let { artillery ->
                item { ArtillerySection(artillery) }
                artillery.shells.firstOrNull { it.type == "AP" }?.let { ap ->
                    item {
                        ShellBallisticsSection(
                            shell = ap,
                            maxRangeKm = artillery.distance,
                        )
                    }
                }
            }
            ship.profile.torpedoes?.let { torpedoes ->
                item { TorpedoSection(torpedoes) }
            }
            ship.profile.antiAircraft?.let { aa ->
                item { AntiAircraftSection(aa) }
            }
            item { MobilitySection(ship) }
            if (similarShips.isNotEmpty()) {
                item { SimilarShipsSection(similarShips, onShipClick) }
            }
        }
    }
}

@Composable
private fun ShipHeader(ship: ShipWiki) {
    Column(
        modifier = Modifier.fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        AsyncImage(
            model = ship.image,
            contentDescription = ship.name,
            modifier = Modifier.fillMaxWidth().padding(horizontal = 48.dp),
        )
        Text(
            text = "T${ship.tier} ${ship.name}",
            style = MaterialTheme.typography.titleLarge,
            color = if (ship.isPremium) com.wowsinfo.libwowsinfo.ui.player.PremiumColor
            else MaterialTheme.colorScheme.onSurface,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = "${ship.nation.replace('_', ' ')} · ${ship.type}",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            textAlign = TextAlign.Center,
            modifier = Modifier.fillMaxWidth(),
        )
        if (ship.description.isNotEmpty()) {
            Text(
                text = ship.description,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                textAlign = TextAlign.Center,
                modifier = Modifier.fillMaxWidth(),
            )
        }
    }
}

/** The seven 0-100 profile bars like the original app's WarshipStat. */
@Composable
private fun ProfileBars(ship: ShipWiki) {
    val profile = ship.profile
    val entries = listOf(
        "Survivability" to profile.armour.total,
        "Artillery" to profile.weaponry.artillery,
        "Torpedoes" to profile.weaponry.torpedoes,
        "Anti-Aircraft" to profile.weaponry.antiAircraft,
        "Maneuverability" to profile.mobility.total,
        "Aircraft" to profile.weaponry.aircraft,
        "Concealment" to profile.concealment.total,
    )
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Profile")
        entries.forEachIndexed { index, (label, value) ->
            BarRow(
                label = label,
                valueText = value.toString(),
                fraction = value.coerceIn(0, 100) / 100.0,
                color = chartColor(index),
            )
        }
    }
}

@Composable
private fun ArmourSection(ship: ShipWiki) {
    val armour = ship.profile.armour
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Armour")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Rating", formatNumber(armour.total), modifier = Modifier.weight(1f))
            Stat("Health", formatNumber(armour.health), modifier = Modifier.weight(1f))
        }
        listOf(
            "Citadel" to armour.citadel,
            "Deck" to armour.deck,
            "Casemate" to armour.casemate,
            "Extremities" to armour.extremities,
        ).forEach { (label, value) ->
            val text = if (value.max > 0) "${value.min}-${value.max} mm" else "n/a"
            BarRow(
                label = label,
                valueText = text,
                fraction = 0.0,
            )
        }
    }
}

@Composable
private fun ArtillerySection(artillery: ArtilleryProfile) {
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Main battery")
        artillery.slots.forEachIndexed { index, slot ->
            BarRow(
                label = "Turret ${index + 1}",
                valueText = "${slot.barrels}x${slot.guns}",
                fraction = 1.0,
                color = chartColor(1),
            )
            Text(
                slot.name,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Rate", "${artillery.gunRate}/min", modifier = Modifier.weight(1f))
            Stat("Dispersion", formatNumber(artillery.maxDispersion), modifier = Modifier.weight(1f))
            Stat("Range", "${artillery.distance} km", modifier = Modifier.weight(1f))
        }
        artillery.shells.forEachIndexed { index, shell ->
            Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                Text(
                    text = shell.name,
                    style = MaterialTheme.typography.labelLarge,
                    color = chartColor(if (shell.type == "AP") 0 else 5),
                )
                Text(
                    text = "DMG ${formatNumber(shell.damage)} · Mass ${shell.bulletMass} kg · " +
                        "Speed ${shell.bulletSpeed.toInt()} m/s" +
                        shell.burnProbability?.let { " · Fire $it%" }.orEmpty(),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun TorpedoSection(torpedoes: com.wowsinfo.libwowsinfo.TorpedoProfile) {
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Torpedoes")
        Text(
            "Range ${torpedoes.distance} km",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        torpedoes.shells.forEach { shell ->
            Text(
                "${shell.name} · DMG ${formatNumber(shell.damage)}",
                style = MaterialTheme.typography.bodyMedium,
            )
        }
    }
}

@Composable
private fun AntiAircraftSection(aa: com.wowsinfo.libwowsinfo.AntiAircraftProfile) {
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Anti-aircraft")
        Text(
            "Defense ${aa.defense}",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        aa.slots.forEach { slot ->
            Text(
                "${slot.name} · ${slot.caliber} mm · ${slot.guns} guns",
                style = MaterialTheme.typography.bodyMedium,
            )
        }
    }
}

@Composable
private fun MobilitySection(ship: ShipWiki) {
    val mobility = ship.profile.mobility
    val concealment = ship.profile.concealment
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Mobility & concealment")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Speed", "${mobility.maxSpeed} kt", modifier = Modifier.weight(1f))
            Stat("Turning", "${mobility.turningRadius} m", modifier = Modifier.weight(1f))
            Stat("Rudder", "${mobility.rudderTime} s", modifier = Modifier.weight(1f))
        }
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround,
        ) {
            Stat("Detect (ship)", "${concealment.detectDistanceByShip} km", modifier = Modifier.weight(1f))
            Stat("Detect (plane)", "${concealment.detectDistanceByPlane} km", modifier = Modifier.weight(1f))
        }
    }
}

@Composable
private fun SimilarShipsSection(
    ships: List<EncyclopediaShip>,
    onShipClick: (ULong) -> Unit,
) {
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Similar ships")
        LazyRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            items(ships, key = { it.shipId.toString() }) { ship ->
                Card(modifier = Modifier.padding(vertical = 2.dp)) {
                    Column(
                        modifier = Modifier.padding(8.dp),
                        horizontalAlignment = Alignment.CenterHorizontally,
                    ) {
                        AsyncImage(
                            model = ship.icon,
                            contentDescription = null,
                            modifier = Modifier.fillMaxWidth(),
                        )
                        Text(
                            "T${ship.tier} ${ship.name}",
                            style = MaterialTheme.typography.bodySmall,
                        )
                        Spacer(Modifier.padding(0.dp))
                    }
                }
            }
        }
        AssistChip(
            onClick = { onShipClick(ships.first().shipId) },
            label = { Text("Compare") },
        )
    }
}
