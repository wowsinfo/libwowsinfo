package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ScrollableTabRow
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ViewModel

private enum class WikiTab(val label: String) {
    Ships("Ships"),
    Consumables("Consumables"),
    Skills("Commander Skills"),
    Collections("Collections"),
}

/** Wiki browser: ships, consumables, commander skills and collections. */
@Composable
fun WikiScreen(
    viewModel: ViewModel,
    onBack: () -> Unit,
    onShipClick: (ULong) -> Unit,
    modifier: Modifier = Modifier,
) {
    var tabIndex by rememberSaveable { mutableStateOf(0) }
    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Back") }
            Text(
                text = "Wiki",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
            )
        }
        ScrollableTabRow(selectedTabIndex = tabIndex) {
            WikiTab.entries.forEachIndexed { index, tab ->
                Tab(
                    selected = tabIndex == index,
                    onClick = { tabIndex = index },
                    text = { Text(tab.label) },
                )
            }
        }
        when (WikiTab.entries[tabIndex]) {
            WikiTab.Ships -> WikiShipsTab(viewModel.warship, onShipClick)
            WikiTab.Consumables -> WikiConsumablesTab(viewModel.wikiConsumables)
            WikiTab.Skills -> WikiSkillsTab(viewModel.wikiCommanderSkills)
            WikiTab.Collections -> WikiCollectionsTab(viewModel.wikiCollections)
        }
    }
}
