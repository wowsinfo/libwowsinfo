package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Checkbox
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ConsumableView
import com.wowsinfo.libwowsinfo.AdjustedStats
import com.wowsinfo.libwowsinfo.FlagView
import com.wowsinfo.libwowsinfo.SkillView
import com.wowsinfo.libwowsinfo.UpgradeView
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber
import coil.compose.AsyncImage
import java.util.Locale
import kotlin.math.abs

private fun fmt(value: Double, digits: Int = 1): String =
    String.format(Locale.US, "%,.${digits}f", value)

@Composable
private fun AssetIcon(path: String) {
    AsyncImage(
        model = "file:///android_asset/$path",
        contentDescription = null,
        modifier = Modifier.padding(end = 6.dp),
    )
}

@Composable
fun ConsumablesSection(consumables: List<ConsumableView>, adjusted: AdjustedStats) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        SectionTitle("Consumables")
        consumables.forEachIndexed { index, consumable ->
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable(enabled = false) {}
                    .padding(vertical = 4.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                AssetIcon("consumables/${consumable.key}.png")
                Text(
                    text = consumable.name,
                    modifier = Modifier.weight(1.3f),
                    style = MaterialTheme.typography.bodyMedium,
                    fontWeight = FontWeight.Bold,
                    color = chartColor(index + 1),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
                Text(
                    text = buildString {
                        if (consumable.workS > 0) {
                            val work = consumable.workS * adjusted.consumableWorkMult
                            append("${fmt(work)} s")
                            if (abs(work - consumable.workS) > 0.1) append(" (${fmt(consumable.workS)})")
                            append(" · ")
                        }
                        if (consumable.reloadS > 0) {
                            val reload = consumable.reloadS * adjusted.consumableReloadMult
                            append("reload ${fmt(reload)} s")
                            if (abs(reload - consumable.reloadS) > 0.1) {
                                append(" (${fmt(consumable.reloadS)})")
                            }
                        }
                        if (consumable.charges != -1L) {
                            val charges = (consumable.charges + adjusted.consumableChargesExtra)
                                .times(adjusted.consumableCapacityMult)
                                .toLong()
                            if (consumable.reloadS > 0) append(" · ")
                            append("${charges}x")
                            if (charges != consumable.charges) append(" (${consumable.charges})")
                        }
                    },
                    modifier = Modifier.weight(1f),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            if (consumable.alters.isNotEmpty()) {
                Text(
                    text = "Variants: " + consumable.alters.joinToString(" · ") { it.name },
                    modifier = Modifier.padding(start = 8.dp, bottom = 4.dp),
                    style = MaterialTheme.typography.bodySmall,
                    color = chartColor(index + 1),
                )
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun SkillsSection(skills: List<SkillView>, onToggle: (String) -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        SectionTitle("Commander Skills")
        skills.groupBy { it.tier }.toSortedMap().forEach { (tier, tierSkills) ->
            Text(
                text = "Tier ${tierRoman(tier)}",
                style = MaterialTheme.typography.labelMedium,
                color = chartColor(tier.toInt()),
                fontWeight = FontWeight.Bold,
            )
            FlowRow(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                tierSkills.forEach { skill ->
                    Row(
                        modifier = Modifier
                            .clickable { onToggle(skill.key) }
                            .padding(vertical = 2.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        AssetIcon("skills/${skill.key}.png")
                        Checkbox(
                            checked = skill.selected,
                            onCheckedChange = { onToggle(skill.key) },
                        )
                        Column(modifier = Modifier.padding(end = 10.dp)) {
                            Text(
                                text = skill.name,
                                style = MaterialTheme.typography.bodyMedium,
                                fontWeight = FontWeight.Bold,
                                maxLines = 1,
                            )
                            skill.summary.takeIf { it.isNotEmpty() }?.let {
                                Text(
                                    text = it,
                                    style = MaterialTheme.typography.bodySmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun UpgradesSection(upgrades: List<UpgradeView>, onToggle: (String) -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        SectionTitle("Module Upgrades")
        upgrades.groupBy { it.slot }.toSortedMap().forEach { (slot, slotUpgrades) ->
            Text(
                text = "Slot ${slot + 1}",
                style = MaterialTheme.typography.labelMedium,
                color = chartColor((slot % 12).toInt() + 1),
                fontWeight = FontWeight.Bold,
            )
            FlowRow(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                slotUpgrades.forEach { upgrade ->
                    Row(
                        modifier = Modifier
                            .clickable { onToggle(upgrade.key) }
                            .padding(vertical = 2.dp),
                        verticalAlignment = Alignment.CenterVertically,
                    ) {
                        Checkbox(
                            checked = upgrade.selected,
                            onCheckedChange = { onToggle(upgrade.key) },
                        )
                        Column(modifier = Modifier.padding(end = 10.dp)) {
                            Text(
                                text = buildString {
                                    append(upgrade.name)
                                    if (upgrade.costCr > 0) append(" · ${formatNumber(upgrade.costCr)} cr")
                                },
                                style = MaterialTheme.typography.bodyMedium,
                                maxLines = 1,
                            )
                            upgrade.summary.takeIf { it.isNotEmpty() }?.let {
                                Text(
                                    text = it,
                                    style = MaterialTheme.typography.bodySmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun FlagsSection(flags: List<FlagView>, onToggle: (String) -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        SectionTitle("Flags")
        FlowRow(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            flags.forEach { flag ->
                Row(
                    modifier = Modifier
                        .clickable { onToggle(flag.key) }
                        .padding(vertical = 2.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    AssetIcon("flags/${flag.key}.png")
                    Checkbox(
                        checked = flag.selected,
                        onCheckedChange = { onToggle(flag.key) },
                    )
                    Column(modifier = Modifier.padding(end = 10.dp)) {
                        Text(
                            text = flag.name,
                            style = MaterialTheme.typography.bodyMedium,
                            maxLines = 1,
                        )
                        flag.summary.takeIf { it.isNotEmpty() }?.let {
                            Text(
                                text = it,
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    }
                }
            }
        }
    }
}

/** Spotted toggle + HP slider driving the conditional skills. */
@Composable
fun ConditionsSection(
    hpFraction: Double,
    spotted: Boolean,
    onHp: (Double) -> Unit,
    onSpotted: (Boolean) -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 8.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        SectionTitle("Conditions")
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(
                text = if (spotted) "Spotted" else "Unspotted",
                style = MaterialTheme.typography.bodyMedium,
            )
            Switch(checked = spotted, onCheckedChange = onSpotted)
        }
        Text(
            text = "HP ${(hpFraction * 100).toInt()}%",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Slider(
            value = hpFraction.toFloat(),
            onValueChange = { onHp(it.toDouble()) },
            valueRange = 0f..1f,
        )
    }
}
