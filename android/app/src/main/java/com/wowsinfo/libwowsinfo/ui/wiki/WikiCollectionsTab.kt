package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
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
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.CollectionCard
import com.wowsinfo.libwowsinfo.WikiCollection

/** Collection list ordered by name; tap a collection to view its cards (RN parity). */
@Composable
fun WikiCollectionsTab(
    collections: Map<ULong, WikiCollection>,
    cards: Map<ULong, CollectionCard>,
) {
    if (collections.isEmpty()) {
        LoadingHint("Loading collections...")
        return
    }
    val sorted = collections.values.sortedBy { it.name }
    var selected by remember { mutableStateOf<WikiCollection?>(null) }
    selected?.let { collection ->
        val collectionCards = cards.values.filter { it.collectionId == collection.collectionId }
        AlertDialog(
            onDismissRequest = { selected = null },
            title = { Text(collection.name) },
            text = {
                if (collectionCards.isEmpty()) {
                    Text("Loading cards...")
                } else {
                    LazyColumn(
                        modifier = Modifier.heightIn(max = 420.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        items(collectionCards, key = { it.cardId.toString() }) { card ->
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                verticalAlignment = Alignment.CenterVertically,
                            ) {
                                if (card.image.isNotBlank()) {
                                    AsyncImage(
                                        model = card.image,
                                        contentDescription = null,
                                        modifier = Modifier.width(72.dp),
                                    )
                                }
                                Column(modifier = Modifier.padding(start = 8.dp)) {
                                    Text(
                                        text = card.name,
                                        style = MaterialTheme.typography.bodyMedium,
                                        fontWeight = androidx.compose.ui.text.font.FontWeight.Bold,
                                    )
                                    card.description.takeIf { it.isNotBlank() }?.let {
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
            },
            confirmButton = {
                TextButton(onClick = { selected = null }) { Text("Close") }
            },
        )
    }
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(8.dp),
    ) {
        items(sorted, key = { it.collectionId.toString() }) { collection ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { selected = collection }
                    .padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Text(
                    text = collection.name,
                    style = MaterialTheme.typography.bodyLarge,
                )
                collection.description.takeIf { it.isNotBlank() }?.let {
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
