package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
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
import com.wowsinfo.libwowsinfo.AirDefenseStats
import com.wowsinfo.libwowsinfo.AirSupportStats
import com.wowsinfo.libwowsinfo.AircraftDetail
import com.wowsinfo.libwowsinfo.AircraftSlotView
import com.wowsinfo.libwowsinfo.AdjustedStats
import com.wowsinfo.libwowsinfo.BurstInfo
import com.wowsinfo.libwowsinfo.AuraInfo
import com.wowsinfo.libwowsinfo.DepthChargeStats
import com.wowsinfo.libwowsinfo.HullStats
import com.wowsinfo.libwowsinfo.MainBatteryView
import com.wowsinfo.libwowsinfo.MobilityStats
import com.wowsinfo.libwowsinfo.PenCurveView
import com.wowsinfo.libwowsinfo.PingerStats
import com.wowsinfo.libwowsinfo.ShellView
import com.wowsinfo.libwowsinfo.SpecialStats
import com.wowsinfo.libwowsinfo.SubmarineBatteryStats
import com.wowsinfo.libwowsinfo.TorpedoView
import com.wowsinfo.libwowsinfo.VisibilityStats
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber
import java.util.Locale
import kotlin.math.abs
import kotlin.math.floor

private fun fmt(value: Double, digits: Int = 1): String =
    String.format(Locale.US, "%,.${digits}f", value)

private fun fmtInt(value: Double): String = formatNumber(value.toLong())

@Composable
private fun StatGrid(entries: List<Triple<String, String, Int>>) {
    Row(modifier = Modifier.fillMaxWidth()) {
        entries.forEach { (label, value, colorIndex) ->
            Stat(label, value, modifier = Modifier.weight(1f), color = chartColor(colorIndex))
        }
    }
}

@Composable
private fun SectionCard(title: String, content: @Composable () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        SectionTitle(title)
        content()
    }
}

@Composable
fun SurvivabilitySection(hull: HullStats, adjusted: AdjustedStats) {
    SectionCard("Survivability") {
        val health = adjusted.health.coerceAtLeast(0.0)
        StatGrid(
            listOf(
                Triple("Health", healthText(hull.health, health), 0),
                Triple("Torpedo protection", "${fmt(hull.protection)}%", 1),
            ),
        )
    }
}

@Composable
private fun healthText(base: Double, adjusted: Double): String {
    val adjustedText = fmtInt(adjusted)
    return if (abs(base - adjusted) > 0.5) "$adjustedText ($base)" else adjustedText
}

private fun statWithBase(base: Double, adjusted: Double, suffix: String): String {
    val value = adjusted.coerceAtLeast(0.0)
    val text = if (suffix.isEmpty() && value == floor(value)) {
        fmtInt(value)
    } else {
        "${fmt(value)}${if (suffix.isEmpty()) "" else " $suffix"}"
    }
    return if (abs(base - adjusted) > 0.01) "$text ($base)" else text
}

@Composable
fun MainBatterySection(battery: MainBatteryView, adjusted: AdjustedStats, curves: List<PenCurveView>) {
    SectionCard("Main Battery") {
        if (battery.name.isNotBlank() && !battery.name.startsWith("IDS_")) {
            Text(
                text = battery.name,
                style = MaterialTheme.typography.labelLarge,
                fontWeight = FontWeight.Bold,
            )
        }
        StatGrid(
            listOf(
                Triple("Configuration", battery.configuration, 1),
                Triple("Range", statWithBase(battery.rangeM / 1000, adjusted.gunRangeM / 1000, "km"), 2),
                Triple("Reload", statWithBase(battery.reloadS, adjusted.gunReloadS, "s"), 3),
                Triple("Rotation", statWithBase(battery.rotationDegS, adjusted.gunRotationDegS, "°/s"), 4),
            ),
        )
        StatGrid(
            listOf(
                Triple("Sigma", fmt(battery.sigma), 5),
                Triple("Burst", battery.burst?.let { "${it.shotsCount} shells" } ?: "—", 6),
            ),
        )
        battery.burst?.let { burst -> BurstSection(burst) }
        battery.shells.forEach { shell ->
            ShellCard(shell = shell, curves = curves)
        }
    }
}

@Composable
fun SecondarySection(battery: MainBatteryView, adjusted: AdjustedStats) {
    SectionCard("Secondaries") {
        StatGrid(
            listOf(
                Triple("Configuration", battery.configuration, 1),
                Triple("Range", statWithBase(battery.rangeM / 1000, adjusted.secondaryRangeM / 1000, "km"), 2),
                Triple("Reload", statWithBase(battery.reloadS, adjusted.secondaryReloadS, "s"), 3),
            ),
        )
        battery.shells.forEach { shell -> ShellCard(shell = shell, curves = emptyList()) }
    }
}

@Composable
fun TorpedoSection(torpedo: TorpedoView, adjusted: AdjustedStats) {
    SectionCard("Torpedoes") {
        if (torpedo.name.isNotBlank() && !torpedo.name.startsWith("IDS_")) {
            Text(
                text = torpedo.name,
                style = MaterialTheme.typography.labelLarge,
                fontWeight = FontWeight.Bold,
            )
        }
        StatGrid(
            listOf(
                Triple("Launchers", torpedo.configuration, 1),
                Triple("Reload", statWithBase(torpedo.reloadS, adjusted.torpReloadS, "s"), 2),
                Triple("Rotation", statWithBase(torpedo.rotationDegS, adjusted.torpRotationDegS, "°/s"), 3),
                Triple("Single shot", if (torpedo.singleShot) "Yes" else "No", 4),
            ),
        )
        torpedo.shells.forEach { shell -> TorpedoShellCard(shell) }
    }
}

@Composable
fun AirDefenseSection(airDefense: AirDefenseStats, adjusted: AdjustedStats) {
    SectionCard("Anti-Aircraft") {
        val baseDps = airDefense.near.sumOf { it.dps } + airDefense.medium.sumOf { it.dps } +
            airDefense.far.sumOf { it.dps }
        if (baseDps > 0) {
            StatGrid(
                listOf(
                    Triple("Total DPS", statWithBase(baseDps, adjusted.aaDps, ""), 0),
                ),
            )
        }
        val bands = listOf(
            "Near" to airDefense.near,
            "Medium" to airDefense.medium,
            "Far" to airDefense.far,
        )
        bands.forEachIndexed { index, (label, auras) ->
            auras.forEach { aura -> AuraRow(label, aura, index + 1) }
        }
    }
}

@Composable
private fun AuraRow(label: String, aura: AuraInfo, colorIndex: Int) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(
                text = label,
                style = MaterialTheme.typography.labelMedium,
                color = chartColor(colorIndex),
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = "DPS ${fmtInt(aura.dps)} · " +
                    "${fmt(aura.minRange)}-${fmt(aura.maxRange)} km · " +
                    "Hit ${fmt(aura.hitChance * 100)}%",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
fun MobilitySection(mobility: MobilityStats, adjusted: AdjustedStats) {
    SectionCard("Mobility") {
        StatGrid(
            listOf(
                Triple("Speed", statWithBase(mobility.speed, adjusted.speed, "kn"), 1),
                Triple("Turning radius", "${fmtInt(mobility.turningRadius)} m", 2),
                Triple("Rudder", statWithBase(mobility.rudderTime, adjusted.rudderTime, "s"), 3),
            ),
        )
    }
}

@Composable
fun ConcealmentSection(visibility: VisibilityStats, adjusted: AdjustedStats) {
    SectionCard("Concealment") {
        StatGrid(
            listOf(
                Triple("Sea", statWithBase(visibility.sea, adjusted.concealmentSea, "km"), 1),
                Triple("Air", statWithBase(visibility.plane, adjusted.concealmentAir, "km"), 2),
                Triple("Submarine", "${fmt(visibility.submarine)} km", 3),
            ),
        )
        StatGrid(
            listOf(
                Triple("In smoke", "${fmt(visibility.seaInSmoke)} km", 4),
                Triple("Fire (sea)", "${fmt(visibility.seaFireCoeff)} km", 5),
                Triple("Fire (air)", "${fmt(visibility.planeFireCoeff)} km", 6),
            ),
        )
    }
}

@Composable
fun BurstSection(burst: BurstInfo) {
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(
            text = "Burst Fire",
            style = MaterialTheme.typography.labelLarge,
            color = chartColor(6),
            fontWeight = FontWeight.Bold,
        )
        Text(
            text = "${burst.shotsCount} shells per salvo · reload ${fmt(burst.burstReloadTime)} s · " +
                "full reload ${fmt(burst.fullReloadTime)} s · intensity ${fmt(burst.shotIntensity)}",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
fun AirSupportSection(airSupport: AirSupportStats, plane: AircraftDetail?) {
    SectionCard("Air Support") {
        StatGrid(
            listOf(
                Triple("Charges", airSupport.chargesNum.toString(), 1),
                Triple("Reload", "${fmt(airSupport.reload)} s", 2),
                Triple("Range", "${fmt(airSupport.range / 1000)} km", 3),
            ),
        )
        plane?.let { aircraft ->
            Text(
                text = buildString {
                    append(aircraft.name)
                    append(" · HP ${fmtInt(aircraft.health)}")
                    append(" · ${aircraft.totalPlanes} planes")
                    append(" · ${fmt(aircraft.speed)} kn")
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
fun AircraftSection(slot: AircraftSlotView) {
    val selected = slot.options.firstOrNull { it.index == slot.selected } ?: slot.options.firstOrNull()
        ?: return
    SectionCard(slot.label) {
        Text(
            text = selected.name,
            style = MaterialTheme.typography.labelLarge,
            fontWeight = FontWeight.Bold,
            color = chartColor(1),
        )
        val aircraft = selected.aircraft
        if (aircraft == null) {
            Text(
                text = "No aircraft data",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            return@SectionCard
        }
        StatGrid(
            listOf(
                Triple("HP", statWithBase(aircraft.health, aircraft.adjustedHealth, ""), 0),
                Triple("Planes", aircraft.totalPlanes.toString(), 1),
                Triple("Speed", statWithBase(aircraft.speed, aircraft.adjustedSpeed, "kn"), 2),
                Triple("Visibility", "${fmt(aircraft.visibility)} km", 3),
            ),
        )
        aircraft.attackCount?.let { attack ->
            val attacker = aircraft.attacker ?: 1
            StatGrid(
                listOf(
                    Triple("Per attack", "$attacker x $attack", 4),
                    Triple("Hangar", aircraft.maxAircraft?.toString() ?: "—", 5),
                    Triple("Restore", aircraft.restoreTime?.let { "${fmt(it)} s" } ?: "—", 6),
                ),
            )
        }
        aircraft.bomb?.let { bomb ->
            Text(
                text = buildString {
                    append("Bomb: ${bomb.name}")
                    append(" · DMG ${formatNumber(bomb.damage)}")
                    bomb.burnChance?.let { append(" · Fire ${fmt(it * 100)}%") }
                    bomb.floodChance?.let { append(" · Flood ${fmt(it)}%") }
                    bomb.penHe?.let { append(" · Pen ${fmtInt(it)} mm") }
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
fun PingerSection(pinger: PingerStats, adjusted: AdjustedStats) {
    SectionCard("Sonar") {
        StatGrid(
            listOf(
                Triple("Range", "${fmt(pinger.range / 1000)} km", 1),
                Triple("Reload", statWithBase(pinger.reload, adjusted.pingerReloadS, "s"), 2),
                Triple("Speed", statWithBase(pinger.speed, adjusted.pingerSpeed, "m/s"), 3),
                Triple("Duration", "${fmt(pinger.lifeTime1)} | ${fmt(pinger.lifeTime2)} s", 4),
            ),
        )
    }
}

@Composable
fun DepthChargeSection(depth: DepthChargeStats) {
    SectionCard("Depth Charges") {
        StatGrid(
            listOf(
                Triple("Reload", "${fmt(depth.reload)} s", 1),
                Triple("Bombs", depth.bombs.toString(), 2),
                Triple("Groups", depth.groups.toString(), 3),
            ),
        )
    }
}

@Composable
fun SubmarineBatterySection(battery: SubmarineBatteryStats, adjusted: AdjustedStats) {
    SectionCard("Battery") {
        StatGrid(
            listOf(
                Triple("Capacity", statWithBase(battery.capacity.toDouble(), adjusted.batteryCapacity, ""), 1),
                Triple("Regen", statWithBase(battery.regen, adjusted.batteryRegen, "/s"), 2),
            ),
        )
    }
}

@Composable
fun SpecialSection(special: SpecialStats) {
    SectionCard("Rage Mode") {
        StatGrid(
            listOf(
                Triple("Duration", "${fmt(special.boostDuration)} s", 1),
                Triple("Hits", special.requiredHits.toString(), 2),
                Triple("Radius", "${fmt(special.radius / 1000)} km", 3),
            ),
        )
    }
}

@Composable
private fun ShellCard(shell: ShellView, curves: List<PenCurveView>) {
    val color = when (shell.ammoType) {
        "AP" -> chartColor(0)
        "SAP" -> chartColor(4)
        else -> chartColor(5)
    }
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(
                text = "${shell.ammoType} · ${shell.name}",
                style = MaterialTheme.typography.labelLarge,
                color = color,
                fontWeight = FontWeight.Bold,
            )
            val curve = curves.firstOrNull { it.shellKey == shell.key }
            if (curve != null && curve.points.isNotEmpty()) {
                var open by remember { mutableStateOf(false) }
                TextButton(onClick = { open = true }) { Text("Penetration") }
                if (open) {
                    WikiPenetrationDialog(
                        curves = listOf(curve),
                        onDismiss = { open = false },
                    )
                }
            }
        }
        Text(
            text = buildString {
                append("DMG ${formatNumber(shell.damage)}")
                if (shell.burnChance != null) append(" · Fire ${fmt(shell.burnChance!! * 100)}%")
                if (shell.penHe != null) append(" · HE pen ${fmtInt(shell.penHe!!)} mm")
                if (shell.penSap != null) append(" · SAP pen ${fmtInt(shell.penSap!!)} mm")
                append(" · ${fmtInt(shell.weight)} kg · ${fmtInt(shell.speed)} m/s")
                shell.overmatch?.let {
                    append(" · Overmatch ${fmtInt(shell.calibreMm / it)} mm")
                }
                shell.ricochetAngle?.let { append(" · Ricochet ${fmt(it)}°") }
                shell.ricochetAlways?.let { append(" · Always bounces ${fmt(it)}°") }
                shell.fuseTime?.let { append(" · Fuse ${String.format(Locale.US, "%.3f", it)} s") }
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
private fun TorpedoShellCard(shell: ShellView) {
    val actualDamage = ((shell.alphaDamage ?: 0.0) / 3.0 + shell.damage).toLong()
    val rangeKm = (shell.range ?: 0.0) / (100.0 / 3.0)
    val reaction = (shell.visibility ?: 0.0) / (shell.speed.coerceAtLeast(1.0)) / 2.6854 * 1000
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(
            text = shell.name,
            style = MaterialTheme.typography.labelLarge,
            color = chartColor(3),
            fontWeight = FontWeight.Bold,
        )
        Text(
            text = buildString {
                append("DMG ${formatNumber(actualDamage)}")
                if (rangeKm > 0) append(" · ${fmt(rangeKm)} km")
                append(" · ${fmt(shell.speed)}-${fmt((shell.speed + 5) * 1.05)} kn")
                shell.visibility?.let { append(" · Detect ${fmt(it)} km") }
                shell.floodChance?.let { append(" · Flood ${fmt(it)}%") }
                if (reaction > 0) append(" · Reaction ${fmt(reaction)} s")
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}
