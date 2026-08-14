package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.Checkbox
import androidx.compose.material3.FilterChip
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.LocalSkillWikiEntry
import com.wowsinfo.libwowsinfo.ui.chartColor

/** Commander skill list + RN-style point-budget builder from the bundled data. */
@Composable
fun WikiSkillsTab(skills: List<LocalSkillWikiEntry>) {
    if (skills.isEmpty()) {
        LoadingHint("Loading commander skills...")
        return
    }
    var query by rememberSaveable { mutableStateOf("") }
    var builderMode by rememberSaveable { mutableStateOf(false) }
    var shipClass by rememberSaveable { mutableStateOf("") }
    var selected by remember { mutableStateOf(setOf<String>()) }
    val classes = remember(skills) {
        skills.flatMap { skill -> skill.tiers.map { it.shipClass } }.distinct().sorted()
    }
    val currentClass = shipClass.ifBlank { classes.firstOrNull() ?: "" }
    val usedPoints = remember(skills, selected, currentClass) {
        skills.filter { it.key in selected }.sumOf { skill ->
            skill.tiers.firstOrNull { it.shipClass == currentClass }?.tier ?: 0
        }
    }
    val pointsLeft = 19 - usedPoints
    Column(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp, vertical = 2.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            FilterChip(
                selected = !builderMode,
                onClick = { builderMode = false },
                label = { Text("List") },
            )
            FilterChip(
                selected = builderMode,
                onClick = { builderMode = true },
                label = { Text("Builder") },
            )
            if (builderMode) {
                Text(
                    text = "Points $pointsLeft / 19",
                    modifier = Modifier.padding(top = 8.dp),
                    style = MaterialTheme.typography.labelLarge,
                    color = if (pointsLeft < 0) MaterialTheme.colorScheme.error
                    else MaterialTheme.colorScheme.onSurface,
                )
                TextButton(onClick = { selected = emptySet() }) { Text("Reset") }
            }
        }
        if (builderMode) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState())
                    .padding(horizontal = 8.dp),
                horizontalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                classes.forEach { cls ->
                    FilterChip(
                        selected = cls == currentClass,
                        onClick = { shipClass = cls },
                        label = { Text(cls) },
                    )
                }
            }
            LazyColumn(
                modifier = Modifier.fillMaxWidth(),
                contentPadding = PaddingValues(8.dp),
            ) {
                val grouped = skills
                    .mapNotNull { skill ->
                        skill.tiers
                            .firstOrNull { it.shipClass == currentClass }
                            ?.let { tier -> skill to tier.tier }
                    }
                    .groupBy({ it.second }, { it.first })
                    .toSortedMap()
                grouped.forEach { (tier, tierSkills) ->
                    item(key = "tier_$tier") {
                        Text(
                            text = "Tier $tier",
                            modifier = Modifier.padding(vertical = 4.dp),
                            style = MaterialTheme.typography.titleSmall,
                            fontWeight = FontWeight.Bold,
                            color = chartColor(tier.toInt()),
                        )
                    }
                    items(tierSkills, key = { it.key }) { skill ->
                        val isSelected = skill.key in selected
                        val affordable = tier <= pointsLeft || isSelected
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            verticalAlignment = Alignment.CenterVertically,
                        ) {
                            SkillIcon(skill.key)
                            Checkbox(
                                checked = isSelected,
                                onCheckedChange = {
                                    selected = if (isSelected) {
                                        selected - skill.key
                                    } else if (affordable) {
                                        selected + skill.key
                                    } else {
                                        selected
                                    }
                                },
                                enabled = affordable,
                            )
                            Column(modifier = Modifier.padding(vertical = 4.dp)) {
                                Text(
                                    text = skill.name,
                                    style = MaterialTheme.typography.bodyMedium,
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
                        HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
                    }
                }
            }
        } else {
            OutlinedTextField(
                value = query,
                onValueChange = { query = it },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 8.dp, vertical = 4.dp),
                placeholder = { Text("Search skills…") },
                singleLine = true,
            )
            val filtered = if (query.isBlank()) {
                skills
            } else {
                skills.filter {
                    it.name.contains(query, ignoreCase = true) ||
                        it.description.contains(query, ignoreCase = true) ||
                        it.tierDisplay.contains(query, ignoreCase = true)
                }
            }
            LazyColumn(
                modifier = Modifier.fillMaxWidth(),
                contentPadding = PaddingValues(8.dp),
            ) {
                items(filtered, key = { it.key }) { skill ->
                    Column(
                        modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                        verticalArrangement = Arrangement.spacedBy(2.dp),
                    ) {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            SkillIcon(skill.key)
                            Text(
                                text = skill.name,
                                style = MaterialTheme.typography.bodyLarge,
                                color = chartColor(((skill.tierDisplay.hashCode() % 12) + 12) % 12),
                            )
                        }
                        skill.tierDisplay.takeIf { it.isNotBlank() }?.let {
                            Text(
                                text = it,
                                style = MaterialTheme.typography.labelSmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                        skill.summary.takeIf { it.isNotEmpty() }?.let {
                            Text(
                                text = it,
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                        Text(
                            text = skill.description,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                    HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
                }
            }
        }
    }
}

@Composable
private fun SkillIcon(key: String) {
    AsyncImage(
        model = "file:///android_asset/skills/$key.png",
        contentDescription = null,
        modifier = Modifier.padding(end = 8.dp),
    )
}
