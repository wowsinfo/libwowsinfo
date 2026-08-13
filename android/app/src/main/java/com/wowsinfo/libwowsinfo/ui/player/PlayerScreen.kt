package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Tab
import androidx.compose.material3.SecondaryTabRow
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.Event
import com.wowsinfo.libwowsinfo.ViewModel
import com.wowsinfo.libwowsinfo.core.Core

private enum class PlayerTab(val label: String) {
    General("General"),
    Achievement("Achievement"),
    Charts("Charts"),
    Ships("Ships"),
}

@Composable
fun PlayerScreen(
    core: Core,
    viewModel: ViewModel,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val player = viewModel.player
    var tabIndex by rememberSaveable { mutableStateOf(0) }

    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Search") }
            Spacer(Modifier.weight(1f))
            TextButton(onClick = { core.update(Event.Refresh) }) { Text("Refresh") }
        }

        if (player == null) {
            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    CircularProgressIndicator()
                    Spacer(Modifier.height(12.dp))
                    Text("Loading player…")
                }
            }
        } else {
            SecondaryTabRow(selectedTabIndex = tabIndex) {
                PlayerTab.entries.forEachIndexed { index, tab ->
                    Tab(
                        selected = tabIndex == index,
                        onClick = { tabIndex = index },
                        text = { Text(tab.label) },
                    )
                }
            }
            when (PlayerTab.entries[tabIndex]) {
                PlayerTab.General -> GeneralTab(player)
                PlayerTab.Achievement -> AchievementTab(player.achievements)
                PlayerTab.Charts -> ChartsTab(player.ships)
                PlayerTab.Ships -> ShipsTab(player.ships)
            }
        }
    }
}
