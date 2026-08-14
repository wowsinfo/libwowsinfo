package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.WikiMap

/** Battle arena / map list (WG API), tap a map to view its image. */
@Composable
fun WikiMapsTab(maps: Map<ULong, WikiMap>) {
    if (maps.isEmpty()) {
        LoadingHint("Loading maps...")
        return
    }
    val sorted = maps.values.sortedBy { it.name }
    var shown by remember { mutableStateOf<WikiMap?>(null) }
    shown?.let { map ->
        AlertDialog(
            onDismissRequest = { shown = null },
            title = { Text(map.name) },
            text = {
                Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    if (map.icon.isNotBlank()) {
                        AsyncImage(
                            model = map.icon,
                            contentDescription = map.name,
                            modifier = Modifier.fillMaxWidth().aspectRatio(16f / 9f),
                        )
                    } else {
                        Text("No map image")
                    }
                    map.description.takeIf { it.isNotBlank() }?.let {
                        Text(
                            text = it,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            },
            confirmButton = {
                TextButton(onClick = { shown = null }) { Text("Close") }
            },
        )
    }
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(8.dp),
    ) {
        items(sorted, key = { it.arenaId.toString() }) { map ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { shown = map }
                    .padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Text(
                    text = map.name,
                    style = MaterialTheme.typography.bodyLarge,
                )
                map.description.takeIf { it.isNotBlank() }?.let {
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
