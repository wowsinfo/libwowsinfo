package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
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
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ConsumableView
import java.util.Locale

/** Consumable list from the bundled game data, sorted by type then name. */
@Composable
fun WikiConsumablesTab(consumables: List<ConsumableView>) {
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
        LazyColumn(
            modifier = Modifier.fillMaxWidth(),
            contentPadding = PaddingValues(8.dp),
        ) {
            items(filtered, key = { it.key }) { consumable ->
            Column(
                modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Text(
                    text = consumable.name,
                    style = MaterialTheme.typography.bodyLarge,
                )
                Text(
                    text = buildString {
                        append(consumable.type)
                        if (consumable.workS > 0) append(" · ${fmt(consumable.workS)} s")
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
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
            }
        }
    }
}

private fun fmt(value: Double): String = String.format(Locale.US, "%.1f", value)
