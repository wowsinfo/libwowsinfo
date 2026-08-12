package com.wowsinfo.libwowsinfo.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
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
    onPlayerSelected: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    var query by rememberSaveable { mutableStateOf("") }
    var server by remember { mutableStateOf(Server.ASIA) }

    Column(
        modifier = Modifier.fillMaxSize().padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        ServerSelector(server) { selected ->
            server = selected
            core.update(Event.SetServer(selected))
        }

        OutlinedTextField(
            value = query,
            onValueChange = { query = it },
            modifier = Modifier.fillMaxWidth(),
            placeholder = { Text("Search player nickname") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
            keyboardActions = KeyboardActions(onSearch = {
                scope.launch { core.update(Event.SearchPlayer(query)) }
            }),
        )

        when (val phase = viewModel.phase) {
            is Phase.Searching -> Box(Modifier.fillMaxWidth(), contentAlignment = Alignment.Center) {
                CircularProgressIndicator()
            }

            is Phase.Error -> Text(
                text = phase.value,
                color = MaterialTheme.colorScheme.error,
            )

            is Phase.Idle ->
                if (viewModel.searchResults.isEmpty()) {
                    Text(
                        "Enter a nickname to look up a player.",
                        style = MaterialTheme.typography.bodyMedium,
                    )
                } else {
                    ResultList(viewModel.searchResults, core, onPlayerSelected)
                }

            is Phase.LoadingPlayer, is Phase.Player -> Unit
        }
    }
}

@Composable
private fun ResultList(
    results: List<SearchResult>,
    core: Core,
    onPlayerSelected: () -> Unit,
) {
    LazyColumn(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        items(results, key = { it.accountId.toString() }) { result ->
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable {
                        onPlayerSelected()
                        core.update(Event.SelectPlayer(result.accountId))
                    }
                    .padding(vertical = 8.dp),
            ) {
                Text(result.nickname, style = MaterialTheme.typography.titleMedium)
                Text(
                    "Account ID: ${result.accountId}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun ServerSelector(server: Server, onSelect: (Server) -> Unit) {
    var expanded by remember { mutableStateOf(false) }
    OutlinedButton(onClick = { expanded = true }) {
        Text("Server: ${server.displayName}")
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
