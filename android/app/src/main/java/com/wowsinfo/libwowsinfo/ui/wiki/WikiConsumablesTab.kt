package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.ConsumableView
import com.wowsinfo.libwowsinfo.LocalFlagEntry
import com.wowsinfo.libwowsinfo.ui.chartColor
import java.util.Locale

/** Consumable + signal-flag lists from the bundled game data (RN parity). */
@Composable
fun WikiConsumablesTab(consumables: List<ConsumableView>, flags: List<LocalFlagEntry>) {
    if (consumables.isEmpty()) {
        LoadingHint("Loading consumables...")
        return
    }
    var query by rememberSaveable { mutableStateOf("") }
    Column(modifier = Modifier.fillMaxWidth()) {
        OutlinedTextField(
            value = query,
            onValueChange = { query = it },
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 8.dp, vertical = 4.dp),
            placeholder = { Text("Search consumables…") },
            singleLine = true,
        )
        val filtered = if (query.isBlank()) {
            consumables
        } else {
            consumables.filter {
                it.name.contains(query, ignoreCase = true) ||
                    it.type.contains(query, ignoreCase = true)
            }
        }
        if (flags.isNotEmpty()) {
            Text(
                text = "Flags",
                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
                style = MaterialTheme.typography.titleSmall,
                color = chartColor(5),
            )
            LazyColumn(
                modifier = Modifier.fillMaxWidth(),
                contentPadding = PaddingValues(8.dp),
            ) {
                items(flags, key = { it.key }) { flag ->
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        AsyncImage(
                            model = "file:///android_asset/flags/${flag.key}.png",
                            contentDescription = null,
                            modifier = Modifier.padding(end = 8.dp),
                        )
                        Column(modifier = Modifier.weight(1f)) {
                            Text(
                                text = flag.name,
                                style = MaterialTheme.typography.bodyLarge,
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
                    HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
                }
            }
        }
        LazyColumn(
            modifier = Modifier.fillMaxWidth(),
            contentPadding = PaddingValues(8.dp),
        ) {
            items(filtered, key = { it.key }) { consumable ->
            Column(
                modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    AsyncImage(
                        model = "file:///android_asset/consumables/${consumable.key}.png",
                        contentDescription = null,
                        modifier = Modifier.padding(end = 8.dp),
                    )
                    Text(
                        text = consumable.name,
                        style = MaterialTheme.typography.bodyLarge,
                    )
                }
                Text(
                    text = buildString {
                        append(consumable.type)
                        if (consumable.workS > 0) append(" · ${fmt(consumable.workS)} s")
                        if (consumable.preparationS > 0) {
                            append(" · prep ${fmt(consumable.preparationS)} s")
                        }
                        if (consumable.reloadS > 0) append(" · reload ${fmt(consumable.reloadS)} s")
                        if (consumable.charges != -1L) append(" · ${consumable.charges}x")
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                consumable.description.takeIf { it.isNotBlank() && !it.startsWith("IDS_") }?.let {
                    Text(
                        text = it,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
                if (consumable.alters.isNotEmpty()) {
                    Text(
                        text = "Variants: " + consumable.alters.joinToString(" · ") { it.name },
                        style = MaterialTheme.typography.bodySmall,
                        color = chartColor(4),
                    )
                }
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
            }
        }
    }
}

private fun fmt(value: Double): String = String.format(Locale.US, "%.1f", value)
