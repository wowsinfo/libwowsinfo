package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.AssistChip
import androidx.compose.material3.Card
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.LocalShipWiki
import com.wowsinfo.libwowsinfo.LocalCompare
import com.wowsinfo.libwowsinfo.SimilarShip
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber

private val TIER_ROMAN = listOf(
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI",
)

fun tierRoman(tier: Long): String = TIER_ROMAN.getOrElse(tier.toInt() - 1) { tier.toString() }

/** Full wiki ship detail from the bundled `wowsinfo.json` game data. */
@Composable
fun WikiShipDetailScreen(
    ship: LocalShipWiki,
    compare: LocalCompare?,
    onBack: () -> Unit,
    onShipClick: (ULong) -> Unit,
    onSelectModule: (slot: String, index: Long) -> Unit,
    onCompare: (List<ULong>) -> Unit,
    onToggleSkill: (String) -> Unit,
    onToggleUpgrade: (String) -> Unit,
    onToggleFlag: (String) -> Unit,
    onSetHp: (Double) -> Unit,
    onSetSpotted: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Back") }
            Text(
                text = "${ship.index} · ${ship.shipId}",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
            )
        }
        LazyColumn(modifier = Modifier.fillMaxSize()) {
            item { ShipTitleCard(ship, onShipClick) }
            if (ship.modules.isNotEmpty()) {
                item { ModuleSelectorEntry(ship) { slot, index -> onSelectModule(slot, index) } }
            }
            item { ConditionsSection(ship.hpFraction, ship.spotted, onSetHp, onSetSpotted) }
            ship.hull?.let { hull ->
                item { SurvivabilitySection(hull, ship.adjusted) }
            }
            if (ship.camos.isNotEmpty()) {
                item { CamoSection(ship.camos) }
            }
            if (ship.consumables.isNotEmpty()) {
                item { ConsumablesSection(ship.consumables, ship.adjusted) }
            }
            ship.mainBattery?.let { battery ->
                item { MainBatterySection(battery, ship.adjusted, ship.penetrationCurves) }
            }
            ship.secondaries?.let { battery ->
                item { SecondarySection(battery, ship.adjusted) }
            }
            ship.torpedoes?.let { torpedo ->
                item { TorpedoSection(torpedo, ship.adjusted) }
            }
            ship.airDefense?.let { airDefense ->
                item { AirDefenseSection(airDefense, ship.adjusted) }
            }
            ship.airSupport?.let { airSupport ->
                item { AirSupportSection(airSupport, ship.airSupportPlane) }
            }
            ship.pinger?.let { pinger ->
                item { PingerSection(pinger, ship.adjusted) }
            }
            ship.depthCharges?.let { depth ->
                item { DepthChargeSection(depth) }
            }
            ship.specialAbility?.let { special ->
                item { SpecialSection(special) }
            }
            ship.aircraft.forEach { slot ->
                item(key = "aircraft_${slot.slot}") { AircraftSection(slot) }
            }
            ship.hull?.let { hull ->
                item { MobilitySection(hull.mobility, ship.adjusted) }
                item { ConcealmentSection(hull.visibility, ship.adjusted) }
                hull.submarineBattery?.let { battery ->
                    item { SubmarineBatterySection(battery, ship.adjusted) }
                }
            }
            if (ship.skills.isNotEmpty()) {
                item { SkillsSection(ship.skills, onToggleSkill) }
            }
            if (ship.upgrades.isNotEmpty()) {
                item { UpgradesSection(ship.upgrades, onToggleUpgrade) }
            }
            if (ship.flags.isNotEmpty()) {
                item { FlagsSection(ship.flags, onToggleFlag) }
            }
            if (ship.similarShips.isNotEmpty()) {
                item { SimilarShipsHeader(ship, compare, onCompare) }
                items(ship.similarShips, key = { it.shipId.toString() }) { similar ->
                    SimilarShipRow(similar, onClick = { onShipClick(similar.shipId) })
                }
            }
        }
    }
}

@Composable
private fun ShipTitleCard(ship: LocalShipWiki, onShipClick: (ULong) -> Unit) {
    Card(modifier = Modifier.fillMaxWidth().padding(8.dp)) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(6.dp)) {
            Text(
                text = ship.name,
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold,
            )
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                AssistChip(onClick = {}, label = { Text("Tier ${tierRoman(ship.tier)}") })
                AssistChip(onClick = {}, label = { Text(ship.shipType) })
                AssistChip(onClick = {}, label = { Text(ship.region) })
                if (ship.premium) AssistChip(onClick = {}, label = { Text("Premium") })
                if (ship.special) AssistChip(onClick = {}, label = { Text("Special") })
                if (ship.camoCount > 0) {
                    AssistChip(onClick = {}, label = { Text("Camos ${ship.camoCount}") })
                }
            }
            ship.description.takeIf { it.isNotBlank() && !it.startsWith("IDS_") }?.let {
                Text(
                    text = it,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceAround,
            ) {
                if (ship.costCredit > 0) {
                    Stat("Credits", formatNumber(ship.costCredit), modifier = Modifier.weight(1f))
                }
                if (ship.costGold > 0) {
                    Stat("Gold", formatNumber(ship.costGold), modifier = Modifier.weight(1f))
                }
                if (ship.costXp > 0) {
                    Stat("XP", formatNumber(ship.costXp), modifier = Modifier.weight(1f))
                }
            }
            if (ship.nextShips.isNotEmpty()) {
                Text(
                    text = "Next ships",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .horizontalScroll(rememberScrollState()),
                    horizontalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    ship.nextShips.forEach { nextId ->
                        val next = nextId
                        AssistChip(
                            onClick = { onShipClick(next.shipId) },
                            label = { Text("${next.index} · ${next.name}") },
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun ModuleSelectorEntry(ship: LocalShipWiki, onSelectModule: (String, Long) -> Unit) {
    Column(modifier = Modifier.padding(horizontal = 8.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
        SectionTitle("Modules")
        var open by remember { mutableStateOf(false) }
        TextButton(onClick = { open = true }) {
            Text("Change Ship Modules (${ship.modules.size} slots)")
        }
        if (open) {
            WikiShipModulesDialog(
                slots = ship.modules,
                onDismiss = { open = false },
                onSelect = { slot, index ->
                    open = false
                    onSelectModule(slot, index)
                },
            )
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun SimilarShipsHeader(
    ship: LocalShipWiki,
    compare: LocalCompare?,
    onCompare: (List<ULong>) -> Unit,
) {
    var compareOpen by remember { mutableStateOf(false) }
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        SectionTitle("Similar Ships")
        var pickerOpen by remember { mutableStateOf(false) }
        TextButton(onClick = { pickerOpen = true }) { Text("Compare") }
        if (pickerOpen) {
            SimilarShipPicker(
                ship = ship,
                onDismiss = { pickerOpen = false },
                onConfirm = { selected ->
                    pickerOpen = false
                    compareOpen = true
                    onCompare(listOf(ship.shipId) + selected)
                },
            )
        }
    }
    if (compareOpen && compare != null) {
        WikiCompareDialog(compare = compare, onDismiss = { compareOpen = false })
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun SimilarShipPicker(
    ship: LocalShipWiki,
    onDismiss: () -> Unit,
    onConfirm: (List<ULong>) -> Unit,
) {
    var selected by remember { mutableStateOf(setOf<ULong>()) }
    androidx.compose.material3.AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Compare up to 4 ships") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                FilterChip(
                    selected = true,
                    onClick = {},
                    label = { Text("${ship.index} · ${ship.name}") },
                )
                FlowRow(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    ship.similarShips.take(20).forEach { similar ->
                        val checked = selected.contains(similar.shipId)
                        FilterChip(
                            selected = checked,
                            enabled = checked || selected.size < 3,
                            onClick = {
                                selected = if (checked) {
                                    selected - similar.shipId
                                } else {
                                    selected + similar.shipId
                                }
                            },
                            label = { Text("${similar.index} · ${similar.name}") },
                        )
                    }
                }
            }
        },
        confirmButton = {
            TextButton(
                onClick = { onConfirm(selected.toList()) },
                enabled = selected.isNotEmpty(),
            ) { Text("Compare") }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) { Text("Cancel") }
        },
    )
}

@Composable
private fun SimilarShipRow(ship: SimilarShip, onClick: () -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
            .padding(horizontal = 12.dp, vertical = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Text(
            text = ship.index,
            style = MaterialTheme.typography.labelMedium,
            color = chartColor(1),
            modifier = Modifier.weight(0.35f),
        )
        Text(
            text = ship.name,
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.weight(1.65f),
        )
        Text(
            text = ship.nation,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}
