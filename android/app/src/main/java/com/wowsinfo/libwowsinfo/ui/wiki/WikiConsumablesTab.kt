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
import com.wowsinfo.libwowsinfo.Consumable
import com.wowsinfo.libwowsinfo.ui.formatNumber

/** Consumable list sorted by type then name. */
@Composable
fun WikiConsumablesTab(consumables: Map<ULong, Consumable>) {
    if (consumables.isEmpty()) {
        LoadingHint("Loading consumables...")
        return
    }
    val sorted = consumables.values.sortedWith(compareBy({ it.type }, { it.name }))
    LazyColumn(
        modifier = Modifier.fillMaxWidth(),
        contentPadding = PaddingValues(8.dp),
    ) {
        items(sorted, key = { it.consumableId.toString() }) { consumable ->
            Column(
                modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                verticalArrangement = Arrangement.spacedBy(2.dp),
            ) {
                Text(
                    text = consumable.name,
                    style = MaterialTheme.typography.bodyLarge,
                )
                Text(
                    text = "${consumable.type} · ${consumable.profile.joinToString(" ") { it.description }}" +
                        (if (consumable.priceGold > 0) " · ${formatNumber(consumable.priceGold)} gold"
                        else if (consumable.priceCredit > 0) " · ${formatNumber(consumable.priceCredit)} cr"
                        else ""),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}
