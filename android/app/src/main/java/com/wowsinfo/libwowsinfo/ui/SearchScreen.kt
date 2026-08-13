package com.wowsinfo.libwowsinfo.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.FilterChip
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.Event
import com.wowsinfo.libwowsinfo.Phase
import com.wowsinfo.libwowsinfo.SearchResult
import com.wowsinfo.libwowsinfo.Server
import com.wowsinfo.libwowsinfo.ViewModel
import com.wowsinfo.libwowsinfo.core.Core
import kotlinx.coroutines.launch

@Composable
fun SearchScreen(
    core: Core,
    viewModel: ViewModel,
    onPlayerSelected: (ULong) -> Unit,
    onClanSelected: (ULong) -> Unit,
    onRealtime: () -> Unit,
    onWiki: () -> Unit,
    onLanguage: (String) -> Unit,
    initialLanguage: String = "en",
    modifier: Modifier = Modifier,
) {
    val scope = rememberCoroutineScope()
    var query by rememberSaveable { mutableStateOf("") }
    var server by remember { mutableStateOf(Server.ASIA) }
    var clanMode by rememberSaveable { mutableStateOf(false) }
    var language by rememberSaveable { mutableStateOf(initialLanguage) }

    Column(
        modifier = modifier.fillMaxSize().padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            ServerSelector(server) { selected ->
                server = selected
                core.update(Event.SetServer(selected))
            }
            TextButton(onClick = onRealtime) { Text("Realtime") }
            TextButton(onClick = onWiki) { Text("Wiki") }
        }

        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            FilterChip(
                selected = !clanMode,
                onClick = { clanMode = false },
                label = { Text("Player") },
            )
            FilterChip(
                selected = clanMode,
                onClick = { clanMode = true },
                label = { Text("Clan") },
            )
        }

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            listOf(
                "en" to "EN",
                "ja" to "日本語",
                "zh_sg" to "简体",
                "zh_tw" to "繁體",
            ).forEach { (code, label) ->
                FilterChip(
                    selected = language == code,
                    onClick = {
                        language = code
                        onLanguage(code)
                    },
                    label = { Text(label) },
                )
            }
        }

        OutlinedTextField(
            value = query,
            onValueChange = { query = it },
            modifier = Modifier.fillMaxWidth(),
            placeholder = { Text(if (clanMode) "Search clan tag" else "Search player nickname") },
            singleLine = true,
            shape = RoundedCornerShape(24.dp),
            keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
            keyboardActions = KeyboardActions(onSearch = {
                scope.launch {
                    if (clanMode) core.update(Event.SearchClan(query)) else core.update(Event.SearchPlayer(query))
                }
            }),
        )

        if (clanMode) {
            if (viewModel.clanSearchResults.isEmpty()) {
                Text(
                    "Enter a clan tag to search.",
                    style = MaterialTheme.typography.bodyMedium,
                )
            } else {
                ClanResultList(viewModel.clanSearchResults, onClanSelected)
            }
        } else {
            when (val phase = viewModel.phase) {
                is Phase.Searching ->
                    Box(Modifier.fillMaxWidth().padding(top = 24.dp), contentAlignment = Alignment.Center) {
                        CircularProgressIndicator()
                    }

                is Phase.Error ->
                    Text(phase.value, color = MaterialTheme.colorScheme.error)

                is Phase.Idle ->
                    if (viewModel.searchResults.isEmpty()) {
                        Text("Enter a nickname to look up a player.", style = MaterialTheme.typography.bodyMedium)
                    } else {
                        ResultList(viewModel.searchResults, core, onPlayerSelected)
                    }

                is Phase.LoadingPlayer, is Phase.Player -> Unit
            }
        }
    }
}

@Composable
private fun ClanResultList(
    results: List<com.wowsinfo.libwowsinfo.ClanSearchResult>,
    onClanSelected: (ULong) -> Unit,
) {
    LazyColumn(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        item { SectionTitle("Clan - ${results.size}") }
        items(results, key = { it.clanId.toString() }) { result ->
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { onClanSelected(result.clanId) }
                    .padding(vertical = 12.dp, horizontal = 4.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    result.tag,
                    style = MaterialTheme.typography.titleMedium,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    result.clanId.toString(),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}

@Composable
private fun ResultList(
    results: List<SearchResult>,
    core: Core,
    onPlayerSelected: (ULong) -> Unit,
) {
    LazyColumn(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        item { SectionTitle("Player - ${results.size}") }
        items(results, key = { it.accountId.toString() }) { result ->
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable {
                        onPlayerSelected(result.accountId)
                    }
                    .padding(vertical = 12.dp, horizontal = 4.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    result.nickname,
                    style = MaterialTheme.typography.titleMedium,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    result.accountId.toString(),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}

@Composable
private fun ServerSelector(server: Server, onSelect: (Server) -> Unit) {
    var expanded by remember { mutableStateOf(false) }
    OutlinedButton(onClick = { expanded = true }) {
        Text("Server: ${server.displayName}", fontWeight = FontWeight.Medium)
    }
    DropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
        Server.entries.forEach { candidate ->
            DropdownMenuItem(
                text = { Text(candidate.displayName) },
                onClick = {
                    onSelect(candidate)
                    expanded = false
                },
            )
        }
    }
}
