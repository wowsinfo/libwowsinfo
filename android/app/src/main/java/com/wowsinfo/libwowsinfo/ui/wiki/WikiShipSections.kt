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
import com.wowsinfo.libwowsinfo.AirstrikeView
import com.wowsinfo.libwowsinfo.AircraftDetail
import com.wowsinfo.libwowsinfo.AircraftSlotView
import com.wowsinfo.libwowsinfo.AdjustedStats
import com.wowsinfo.libwowsinfo.ArmorView
import com.wowsinfo.libwowsinfo.BurstInfo
import com.wowsinfo.libwowsinfo.AuraInfo
import com.wowsinfo.libwowsinfo.DepthChargeView
import com.wowsinfo.libwowsinfo.DispersionView
import com.wowsinfo.libwowsinfo.FiringArcView
import com.wowsinfo.libwowsinfo.HullStats
import com.wowsinfo.libwowsinfo.MainBatteryView
import com.wowsinfo.libwowsinfo.MobilityStats
import com.wowsinfo.libwowsinfo.PenCurveView
import com.wowsinfo.libwowsinfo.PingerStats
import com.wowsinfo.libwowsinfo.ShellDpmView
import com.wowsinfo.libwowsinfo.ShellView
import com.wowsinfo.libwowsinfo.SpecialAbilityView
import com.wowsinfo.libwowsinfo.SubmarineBatteryStats
import com.wowsinfo.libwowsinfo.TorpedoDetailView
import com.wowsinfo.libwowsinfo.TorpedoView
import com.wowsinfo.libwowsinfo.VisibilityStats
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber
import coil.compose.AsyncImage
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
        hull.survivability?.let { surv ->
            if (surv.sections.isNotEmpty()) {
                Text(
                    text = "HP Sections",
                    style = MaterialTheme.typography.labelLarge,
                    fontWeight = FontWeight.Bold,
                )
                surv.sections.forEach { section ->
                    val regenPct = section.regenRatio * 100
                    val regenText = if (regenPct == floor(regenPct)) fmtInt(regenPct) else fmt(regenPct)
                    Text(
                        text = "${section.name}: ${fmtInt(section.hp)} HP · Regen $regenText%",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            surv.fire?.let { fire ->
                Text(
                    text = "Fire: ${fire.spots} spots · ${fmt(fire.duration)} s · " +
                        "${fmtInt(fire.dps)} DPS · ${fmtInt(fire.totalDamage)} dmg",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            surv.flood?.let { flood ->
                Text(
                    text = "Flood: ${flood.spots} spots · ${fmt(flood.duration)} s · " +
                        "${fmtInt(flood.dps)} DPS · ${fmtInt(flood.totalDamage)} dmg",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
fun ArmorSection(armor: ArmorView) {
    SectionCard("Armor") {
        StatGrid(
            listOf(
                Triple("Hull zones", "${armor.zoneCount}", 1),
                Triple("Max thickness", "${fmt(armor.maxZoneThickness)} mm", 2),
            ),
        )
        if (armor.zoneGroups.isNotEmpty()) {
            armor.zoneGroups.take(8).forEach { group ->
                Text(
                    text = "${fmt(group.thickness)} mm × ${group.count}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
        if (armor.turrets.isNotEmpty()) {
            Text(
                text = "Turret armor",
                style = MaterialTheme.typography.labelLarge,
                fontWeight = FontWeight.Bold,
            )
            armor.turrets.forEachIndexed { index, turret ->
                Text(
                    text = "Turret ${index + 1}: ${turret.barrels}×${fmt(turret.caliber * 1000, 0)} mm · " +
                        "Armor ${fmt(turret.armor, 0)} mm · Barbette ${fmt(turret.barbette, 0)} mm",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
fun CamoSection(camos: List<String>, camoKeys: List<String>) {
    SectionCard("Camos") {
        camos.forEachIndexed { index, camo ->
            Row(
                modifier = Modifier.fillMaxWidth().padding(vertical = 2.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                val key = camoKeys.getOrNull(index).orEmpty()
                if (key.isNotBlank()) {
                    val folder = if (key.startsWith("PCEC")) "camouflages" else "permoflages"
                    AsyncImage(
                        model = "file:///android_asset/$folder/$key.png",
                        contentDescription = null,
                        modifier = Modifier.padding(end = 6.dp),
                    )
                }
                Text(
                    text = camo,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
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
                Triple(
                    "Caliber",
                    if (battery.caliberMm > 0) "${fmt(battery.caliberMm, 0)} mm" else "—",
                    0,
                ),
                Triple("Configuration", battery.configuration, 1),
                Triple("Range", statWithBase(battery.rangeM / 1000, adjusted.gunRangeM / 1000, "km"), 2),
                Triple("Reload", statWithBase(battery.reloadS, adjusted.gunReloadS, "s"), 3),
                Triple("Rotation", statWithBase(battery.rotationDegS, adjusted.gunRotationDegS, "°/s"), 4),
            ),
        )
        StatGrid(
            listOf(
                Triple(
                    "Turn 180°",
                    if (battery.turnTimeS > 0) "${fmt(battery.turnTimeS)} s" else "—",
                    5,
                ),
                Triple(
                    "RoF",
                    if (battery.rof > 0) fmt(battery.rof * battery.barrels, 0) else "—",
                    6,
                ),
                Triple("Sigma", fmt(battery.sigma), 7),
            ),
        )
        if (battery.ammoSwitchS > 0) {
            StatGrid(
                listOf(
                    Triple("Ammo switch", "${fmt(battery.ammoSwitchS)} s", 1),
                    Triple("Barrels", battery.barrels.toString(), 2),
                ),
            )
        }
        battery.dispersion?.let { DispersionSection(it) }
        if (battery.firingArcs.isNotEmpty()) {
            FiringArcsSection(battery.firingArcs)
        }
        battery.burst?.let { burst -> BurstSection(burst) }
        battery.shells.forEach { shell ->
            ShellCard(
                shell = shell,
                curves = curves,
                dpm = battery.perShellDpm.firstOrNull { it.shellKey == shell.key },
            )
        }
    }
}

@Composable
private fun DispersionSection(disp: DispersionView) {
    Text(
        text = "Dispersion",
        style = MaterialTheme.typography.labelLarge,
        color = chartColor(6),
        fontWeight = FontWeight.Bold,
    )
    Text(
        text = "Max range: H ${fmt(disp.atMax.horizontalM)} m · V ${fmt(disp.atMax.verticalM)} m",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
    disp.samples.forEach { point ->
        Text(
            text = "${fmt(point.rangeM / 1000)} km: H ${fmt(point.horizontalM)} m · " +
                "V ${fmt(point.verticalM)} m",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
    Text(
        text = "Formula (X = range in km):\nH = ${disp.formulaHorizontal}\n" +
            "V ≥ delim = ${disp.formulaVertical}\nV < delim = ${disp.formulaVerticalShort}",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
    Text(
        text = "Delimiter ${fmt(disp.delimDistM / 1000)} km · Taper ${fmt(disp.taperDistM / 1000)} km",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
}

@Composable
private fun FiringArcsSection(arcs: List<FiringArcView>) {
    Text(
        text = "Firing Arcs",
        style = MaterialTheme.typography.labelLarge,
        color = chartColor(4),
        fontWeight = FontWeight.Bold,
    )
    arcs.forEach { arc ->
        Text(
            text = "${arc.name}: H ${fmt(arc.horizMin, 0)}°…${fmt(arc.horizMax, 0)}° · " +
                "V ${fmt(arc.vertMin, 0)}°…${fmt(arc.vertMax, 0)}°",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
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
                Triple(
                    "Turn 180°",
                    if (torpedo.turnTimeS > 0) "${fmt(torpedo.turnTimeS)} s" else "—",
                    4,
                ),
            ),
        )
        if (torpedo.torpedoCount > 0) {
            StatGrid(
                listOf(
                    Triple("Tubes", torpedo.torpedoCount.toString(), 5),
                    Triple("Single shot", if (torpedo.singleShot) "Yes" else "No", 6),
                ),
            )
        }
        torpedo.torpedoes.forEach { torp -> TorpedoDetailCard(torp) }
        if (torpedo.torpedoes.isEmpty()) {
            torpedo.shells.forEach { shell -> TorpedoShellCard(shell) }
        }
    }
}

@Composable
private fun TorpedoDetailCard(torp: TorpedoDetailView) {
    Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
        Text(
            text = buildString {
                append(torp.name)
                if (torp.deepWater) append(" · Deep Water")
            },
            style = MaterialTheme.typography.labelLarge,
            color = chartColor(3),
            fontWeight = FontWeight.Bold,
        )
        Text(
            text = buildString {
                append("DMG ${formatNumber(torp.damage)}")
                if (torp.alphaDamage > 0) append(" · Alpha ${formatNumber(torp.alphaDamage)}")
                if (torp.rangeKm > 0) append(" · ${fmt(torp.rangeKm)} km")
                append(" · ${fmt(torp.speedKt)} kn")
                if (torp.detectabilityKm > 0) append(" · Detect ${fmt(torp.detectabilityKm)} km")
                if (torp.reactionTimeS > 0) append(" · Reaction ${fmt(torp.reactionTimeS)} s")
                torp.floodChance?.let { append(" · Flood ${fmt(it)}%") }
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            text = buildString {
                torp.armingDistanceM?.let { append("Arming ${fmtInt(it)} m · ") }
                torp.depthM?.let { append("Depth ${fmt(it, 2)} m · ") }
                torp.splashArmorCoeff?.let { append("Splash armor ${fmt(it)} · ") }
                torp.splashCubeSize?.let { append("Cube ${fmt(it)} · ") }
                torp.pingDamageCoeff?.let { append("Ping dmg ${fmt(it)}× · ") }
                append("Salvo ${formatNumber(torp.salvoDamage)}")
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        torp.acousticDetection?.let { acoustic ->
            Text(
                text = "Acoustic homing: radius ${fmtInt(acoustic.searchRadius)} m · " +
                    "angle ${fmt(acoustic.searchAngle, 0)}° · yaw ${fmt(acoustic.yawChangeSpeed)}°/s · " +
                    "depth ${fmt(acoustic.maxDepthLevel)}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (torp.canHitClasses.isNotEmpty()) {
            Text(
                text = "Hits: ${torp.canHitClasses.joinToString(", ")}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
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
        airDefense.bubbles?.let { bubbles ->
            Text(
                text = "Flak Clouds",
                style = MaterialTheme.typography.labelLarge,
                color = chartColor(5),
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = "Inner ${bubbles.inner} · Outer ${bubbles.outer} · " +
                    "${fmt(bubbles.minRange)}-${fmt(bubbles.maxRange)} km · " +
                    "DMG ${fmtInt(bubbles.damage)} · Hit ${fmt(bubbles.hitChance * 100)}% · " +
                    "Spawn ${fmt(bubbles.spawnTime)} s",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
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
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp),
            verticalArrangement = Arrangement.spacedBy(2.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
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
            val details = buildString {
                if (aura.explosionCount > 0) append("Explosions ${aura.explosionCount} · ")
                if (aura.shotTravelTime > 0) append("Shot travel ${fmt(aura.shotTravelTime)} s · ")
                if (aura.shotDelay > 0) append("Shot delay ${fmt(aura.shotDelay)} s · ")
                if (aura.damage > 0) append("DMG ${fmtInt(aura.damage)} · ")
                if (aura.innerBubbleCount > 0 || aura.outerBubbleCount > 0) {
                    append(
                        "Flak ${aura.innerBubbleCount}+${aura.outerBubbleCount} · " +
                            "R ${fmt(aura.bubbleRadius)} km · ${fmt(aura.bubbleDuration)} s · " +
                            "DMG ${fmtInt(aura.bubbleDamage)} · ",
                    )
                }
                if (aura.guns.isNotEmpty()) {
                    append("Guns ${aura.guns.joinToString(" + ") { "${it.count}×${it.each}" }}")
                }
            }
            if (details.isNotEmpty()) {
                Text(
                    text = details,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
fun MobilitySection(hull: HullStats, adjusted: AdjustedStats) {
    SectionCard("Mobility") {
        StatGrid(
            listOf(
                Triple("Speed", statWithBase(hull.mobility.speed, adjusted.speed, "kn"), 1),
                Triple("Turning radius", "${fmtInt(hull.mobility.turningRadius)} m", 2),
                Triple("Rudder", statWithBase(hull.mobility.rudderTime, adjusted.rudderTime, "s"), 3),
            ),
        )
        hull.maneuverability?.let { man ->
            StatGrid(
                listOf(
                    Triple("Reverse speed", "${fmt(man.maxReverseSpeed)} kn", 4),
                    Triple(
                        "Dive speed",
                        man.submarine?.let { "${fmt(it.diveSpeed)} m/s" } ?: "—",
                        5,
                    ),
                ),
            )
            man.submarine?.let { sub ->
                Text(
                    text = "Submarine modes",
                    style = MaterialTheme.typography.labelLarge,
                    fontWeight = FontWeight.Bold,
                )
                SubmarineModeLine("Surface", sub.surfaceSpeed, sub.surfaceReverse)
                SubmarineModeLine("Periscope", sub.periscopeSpeed, sub.periscopeReverse)
                SubmarineModeLine("Max depth", sub.maxDepthSpeed, sub.maxDepthReverse)
                Text(
                    text = "Diving plane shift: ${fmt(sub.divingPlaneShiftTime)} s",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            man.raw?.let { raw ->
                Text(
                    text = buildString {
                        append("Engine power ${fmt(raw.enginePower, 0)}")
                        append(" · Side drag ${fmt(raw.sideDragCoef, 0)}")
                        append(" · Backward drag ${fmt(raw.backwardMovementDragCoef)}")
                        append(" · Backward power ×${fmt(raw.backwardPowerCoef)}")
                        append(" · Speed coef ${fmt(raw.speedCoef)}")
                        append(" · Rudder angle ${fmt(raw.maxRudderAngle)}°")
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun SubmarineModeLine(label: String, forward: Double, reverse: Double) {
    Text(
        text = "$label: ${fmt(forward)} / ${fmt(reverse)} kn",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
}

@Composable
fun ConcealmentSection(hull: HullStats, adjusted: AdjustedStats) {
    val visibility = hull.visibility
    SectionCard("Concealment") {
        StatGrid(
            listOf(
                Triple("Sea", statWithBase(visibility.sea, adjusted.concealmentSea, "km"), 1),
                Triple("Air", statWithBase(visibility.plane, adjusted.concealmentAir, "km"), 2),
                Triple("Submarine", "${fmt(visibility.submarine)} km", 3),
            ),
        )
        hull.concealment?.let { concealment ->
            StatGrid(
                listOf(
                    Triple("Fire (sea)", "${fmt(concealment.seaFire)} km", 4),
                    Triple("Fire (air)", "${fmt(concealment.airFire)} km", 5),
                    Triple("In smoke", "${fmt(visibility.seaInSmoke)} km", 6),
                ),
            )
            if (concealment.periscopeDepth > 0 || concealment.deepWaterDepth > 0) {
                Text(
                    text = "Submarine detectability",
                    style = MaterialTheme.typography.labelLarge,
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    text = "Periscope depth: ${fmt(concealment.periscopeDepth)} km · " +
                        "Deep water: ${fmt(concealment.deepWaterDepth)} km",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            if (concealment.bySubmarineDepth.isNotEmpty()) {
                Text(
                    text = "By depth: " + concealment.bySubmarineDepth.joinToString(" · ") {
                        "${it.first.replace('_', ' ')} ${fmt(it.second)} km"
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            if (concealment.underwaterDepthCoeff.isNotEmpty()) {
                Text(
                    text = buildString {
                        append("Depth coeff: ")
                        append(
                            concealment.underwaterDepthCoeff.joinToString(" · ") {
                                "${fmt(it.first)} m ×${fmt(it.second)}"
                            },
                        )
                        if (concealment.underwaterDepthCoeffPlane.isNotEmpty()) {
                            append(" · plane ")
                            append(
                                concealment.underwaterDepthCoeffPlane.joinToString(" · ") {
                                    "${fmt(it.first)} m ×${fmt(it.second)}"
                                },
                            )
                        }
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Text(
                text = buildString {
                    append("Smoke factor ${fmt(concealment.smokeFactor)}")
                    if (concealment.smokeFactorGk > 0) {
                        append(" · GK smoke ${fmt(concealment.smokeFactorGk)}")
                    }
                    if (concealment.visibilityCoefGkByPlane > 0) {
                        append(" · GK plane ${fmt(concealment.visibilityCoefGkByPlane)}")
                    }
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } ?: StatGrid(
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
fun AirSupportSection(airSupport: AirstrikeView, plane: AircraftDetail?) {
    SectionCard("Air Support") {
        StatGrid(
            listOf(
                Triple("Charges", airSupport.charges.toString(), 1),
                Triple("Reload", "${fmt(airSupport.reloadS)} s", 2),
                Triple("Range", "${fmt(airSupport.rangeKm)} km", 3),
                Triple("Auto", if (airSupport.autoUsage) "Yes" else "No", 4),
            ),
        )
        StatGrid(
            listOf(
                Triple("Min dist", "${fmtInt(airSupport.minDistM)} m", 1),
                Triple("Max dist", "${fmtInt(airSupport.maxDistM)} m", 2),
                Triple("Flight dist", "${fmtInt(airSupport.maxPlaneFlightDistM)} m", 3),
                Triple("Climb", "${fmt(airSupport.climbAngleDeg)}°", 4),
            ),
        )
        StatGrid(
            listOf(
                Triple("Fly away", "${fmt(airSupport.flyAwayTimeS)} s", 1),
                Triple("Between shots", "${fmt(airSupport.timeBetweenShotsS)} s", 2),
                Triple("Drop time", "${fmt(airSupport.timeFromHeavenS)} s", 3),
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
        StatGrid(
            listOf(
                Triple(
                    "Per attack",
                    "${aircraft.attacker ?: 1} × ${aircraft.attackCount ?: 1}",
                    4,
                ),
                Triple("Cooldown", aircraft.attackCooldown?.let { "${fmt(it)} s" } ?: "—", 5),
                Triple("Interval", aircraft.attackInterval?.let { "${fmt(it)} s" } ?: "—", 0),
                Triple("Hangar", aircraft.maxAircraft?.toString() ?: "—", 1),
            ),
        )
        StatGrid(
            listOf(
                Triple("Restore", aircraft.restoreTime?.let { "${fmt(it)} s" } ?: "—", 2),
                Triple("On deck", aircraft.maxNumberOnDeck?.toString() ?: "—", 3),
                Triple("Start deck", aircraft.startOnDeck?.toString() ?: "—", 4),
                Triple("Restore amt", aircraft.restoreAmount?.toString() ?: "—", 5),
            ),
        )
        StatGrid(
            listOf(
                Triple("Aim time", aircraft.aimingTime?.let { "${fmt(it)} s" } ?: "—", 0),
                Triple("Aim speed", aircraft.aimingSpeedLimits.ifBlank { "—" }, 1),
                Triple("Aim turn", aircraft.aimingTurnSpeedLimit?.let { fmt(it) } ?: "—", 2),
                Triple(
                    "Aim acc",
                    aircraft.aimingAccuracyIncreaseRate?.let { "${fmt(it, 2)}/s" } ?: "—",
                    3,
                ),
            ),
        )
        StatGrid(
            listOf(
                Triple("Prep time", aircraft.preparationTime?.let { "${fmt(it)} s" } ?: "—", 4),
                Triple("Prep speed", aircraft.preparationSpeedLimits.ifBlank { "—" }, 5),
                Triple("Prep turn", aircraft.preparationTurnSpeedLimit?.let { fmt(it) } ?: "—", 0),
                Triple(
                    "Drop point",
                    aircraft.bombingDropPointTime?.let { "${fmt(it)} s" } ?: "—",
                    1,
                ),
            ),
        )
        StatGrid(
            listOf(
                Triple("Climb", aircraft.angleOfClimb?.let { "${fmt(it)}°" } ?: "—", 2),
                Triple("Dive", aircraft.angleOfDive?.let { "${fmt(it)}°" } ?: "—", 3),
                Triple("Climb spd", aircraft.climbSpeedCoef?.let { "×${fmt(it)}" } ?: "—", 4),
                Triple("Dive spd", aircraft.diveSpeedCoef?.let { "×${fmt(it)}" } ?: "—", 5),
            ),
        )
        StatGrid(
            listOf(
                Triple("JATO", aircraft.jatoDuration?.let { "${fmt(it)} s" } ?: "—", 0),
                Triple("JATO spd", aircraft.jatoSpeedMultiplier?.let { "×${fmt(it)}" } ?: "—", 1),
                Triple("Boost", aircraft.maxForsageAmount?.let { fmt(it) } ?: "—", 2),
                Triple("Boost regen", aircraft.forsageRegeneration?.let { fmt(it) } ?: "—", 3),
            ),
        )
        StatGrid(
            listOf(
                Triple("Dmg taken", aircraft.damageTakenMultiplier?.let { "×${fmt(it)}" } ?: "—", 4),
                Triple(
                    "Attacker dmg",
                    aircraft.attackerDamageTakenMultiplier?.let { "×${fmt(it)}" } ?: "—",
                    5,
                ),
                Triple(
                    "Post-attack inv",
                    aircraft.postAttackInvulnerabilityDuration?.let { "${fmt(it)} s" } ?: "—",
                    0,
                ),
                Triple("Bomb fall", aircraft.bombFallingTime?.let { "${fmt(it)} s" } ?: "—", 1),
            ),
        )
        if (aircraft.planeConsumables.isNotEmpty()) {
            Text(
                text = "Plane consumables: " + aircraft.planeConsumables.joinToString(" · ") {
                    "Slot ${it.slot}: " + it.abilities.joinToString(", ") +
                        if (it.special) " (special)" else ""
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
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
fun DepthChargeSection(depth: DepthChargeView) {
    SectionCard("Depth Charges") {
        if (depth.name.isNotBlank()) {
            Text(
                text = depth.name,
                style = MaterialTheme.typography.labelLarge,
                color = chartColor(4),
                fontWeight = FontWeight.Bold,
            )
        }
        StatGrid(
            listOf(
                Triple("Reload", "${fmt(depth.reloadS)} s", 1),
                Triple("Config", "${depth.groups} × ${depth.bombs}", 2),
                Triple("Damage", formatNumber(depth.damage), 3),
                Triple("Fire", "${fmt(depth.fireChance)}%", 4),
                Triple("Flood", "${fmt(depth.floodChance)}%", 5),
            ),
        )
        depth.packs?.let { packs ->
            StatGrid(
                listOf(
                    Triple("Shots/pack", packs.shots.toString(), 1),
                    Triple("Max packs", packs.maxPacks.toString(), 2),
                    Triple("Shot delay", "${fmt(packs.shotDelayS)} s", 3),
                    Triple("Zone width", fmt(packs.centerZoneWidthPart), 4),
                ),
            )
        }
        if (depth.launchers.isNotEmpty()) {
            Text(
                text = "${depth.launcherCount} throwers · ${depth.bombsPerCharge} bombs per charge",
                style = MaterialTheme.typography.labelMedium,
                fontWeight = FontWeight.Bold,
            )
            depth.launchers.forEach { launcher ->
                Text(
                    text = "${launcher.name}: ${launcher.bombs} bomb(s) · " +
                        "${fmt(launcher.shootAngleDeg)}° · dist ${fmt(launcher.shootDistance)} · " +
                        "${launcher.horizontalSector} H / ${launcher.verticalSector} V",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
        Text(
            text = buildString {
                depth.sinkSpeed?.let { append("Sink speed (raw) ${fmt(it)} · ") }
                depth.detonationDepthM?.let { append("Detonation depth ${fmt(it)} m · ") }
                depth.splashRadiusM?.let { append("Splash radius ${fmt(it)} m · ") }
                depth.alertDist?.let { append("Alert ${fmt(it)} · ") }
                depth.explosivePower?.let { append("Explosive power ${fmt(it)} · ") }
                depth.integralPower?.let { append("Integral power ${fmt(it)} · ") }
                depth.fallDistance?.let { append("Fall dist ${fmt(it)} · ") }
                depth.fallTime?.let { append("Fall time ${fmt(it)} s · ") }
            }.trimEnd(' ', '·'),
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        if (depth.pointsOfDamage.isNotEmpty()) {
            Text(
                text = "Damage falloff: " + depth.pointsOfDamage.joinToString(" · ") {
                    "${it.range} → ${fmt(it.coefficient * 100)}%"
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (depth.canHitClasses.isNotEmpty()) {
            Text(
                text = "Can hit: ${depth.canHitClasses.joinToString(", ")}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (depth.buoyancy.isNotEmpty()) {
            Text(
                text = "Buoyancy: " + depth.buoyancy.joinToString(" · ") {
                    "${it.state} ×${fmt(it.coefficient)}"
                },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
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
fun SpecialSection(special: SpecialAbilityView) {
    SectionCard("Special Ability") {
        Text(
            text = special.name,
            style = MaterialTheme.typography.labelLarge,
            color = chartColor(1),
            fontWeight = FontWeight.Bold,
        )
        StatGrid(
            listOf(
                Triple("Duration", "${fmt(special.durationS)} s", 1),
                Triple(
                    "Preparation",
                    if (special.preparationS > 0) "${fmt(special.preparationS)} s" else "—",
                    2,
                ),
                Triple("Required", special.requiredCount.toString(), 3),
                Triple("Auto usage", if (special.autoUsage) "Yes" else "No", 4),
            ),
        )
        if (special.progressPerAction > 0) {
            Text(
                text = "Progress: ${fmt(special.progressPerAction)} per ${special.progressName.replace('_', ' ')}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (special.subRibbons.isNotEmpty()) {
            Text(
                text = "Ribbons: ${special.subRibbons.joinToString(", ")}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (special.inactivityDelayS > 0 && special.progressLossPerInterval > 0) {
            Text(
                text = "Inactive ${fmt(special.inactivityDelayS)} s · lose " +
                    "${fmt(special.progressLossPerInterval)} every " +
                    "${fmt(special.progressLossIntervalS)} s",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (special.modifiers.isNotEmpty()) {
            Text(
                text = "Modifiers",
                style = MaterialTheme.typography.labelLarge,
                color = chartColor(6),
                fontWeight = FontWeight.Bold,
            )
            special.modifiers.forEach { line ->
                Text(
                    text = line,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun ShellCard(shell: ShellView, curves: List<PenCurveView>, dpm: ShellDpmView? = null) {
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
                dpm?.let {
                    append(" · DPM ${formatNumber(it.dpm)}")
                    append(" · Salvo ${formatNumber(it.salvoDamage)} dmg")
                    if (it.salvoWeightKg > 0) append(" · ${fmtInt(it.salvoWeightKg)} kg")
                }
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
        Text(
            text = buildString {
                shell.airDrag?.let { append("Air drag ${fmt(it)} · ") }
                shell.waterDrag?.let { append("Water drag ${fmt(it)} · ") }
                shell.krupp?.let { append("Krupp ${fmtInt(it)} · ") }
                shell.armingThreshold?.let { append("Arming ${fmtInt(it)} mm · ") }
                shell.shellCap?.let { append(if (it) "Shell cap yes · " else "Shell cap no · ") }
                shell.capNormalizeMaxAngle?.let { append("Cap angle ${fmt(it)}° · ") }
                shell.underwaterDistFactor?.let { append("UW dist ×${fmt(it)} · ") }
                shell.underwaterPenetrationFactor?.let { append("UW pen ×${fmt(it)} · ") }
                shell.explosionRadius?.let { append("Blast radius ${fmt(it)} m · ") }
                shell.splashRadius?.let { append("Splash radius ${fmt(it)} m · ") }
                if (shell.distParams.isNotEmpty()) {
                    append("Dist params [${shell.distParams.joinToString(", ") { fmt(it) }}] · ")
                }
                shell.distTile?.let { append("Dist tile ${fmt(it)} · ") }
            }.trimEnd(' ', '·'),
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
