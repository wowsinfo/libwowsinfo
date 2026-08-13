package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.WikiCollection

/** Collection list ordered by name. */
@Composable
fun WikiCollectionsTab(collections: Map<ULong, WikiCollection>) {
    if (collections.isEmpty()) {
        LoadingHint("Loading collections...")
        return
    }
    val sorted = collections.values.sortedBy { it.name }
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(8.dp),
    ) {
        items(sorted, key = { it.collectionId.toString() }) { collection ->
            Column(
                modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Text(
                    text = collection.name,
                    style = MaterialTheme.typography.bodyLarge,
                )
                Text(
                    text = collection.description,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}
