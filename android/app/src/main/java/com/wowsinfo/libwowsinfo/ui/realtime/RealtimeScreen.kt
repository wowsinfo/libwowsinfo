package com.wowsinfo.libwowsinfo.ui.realtime

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Card
import androidx.compose.material3.CircularProgressIndicator
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
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

/**
 * Realtime battle screen mirroring the app's RS feature: connect to the WoWs-RS
 * companion on the LAN, fetch `tempArenaInfo.json` and show both teams.
 */
@Composable
fun RealtimeScreen(
    online: Long,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val scope = rememberCoroutineScope()
    var ip by rememberSaveable { mutableStateOf("") }
    var arena by remember { mutableStateOf<ArenaInfo?>(null) }
    var loading by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Back") }
            Text(
                text = "RS Beta",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.weight(1f),
            )
            Text(
                text = if (online >= 0) "Online: $online" else "Online: -",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                OutlinedTextField(
                    value = ip,
                    onValueChange = { ip = it },
                    modifier = Modifier.fillMaxWidth(),
                    placeholder = { Text("192.168.1.x") },
                    singleLine = true,
                )
            }
            item {
                OutlinedButton(
                    onClick = {
                        loading = true
                        error = null
                        scope.launch {
                            val result = withContext(Dispatchers.IO) { fetchArena(ip) }
                            arena = result.first
                            error = result.second
                            loading = false
                        }
                    },
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("Connect")
                }
            }
            error?.let { message ->
                item {
                    Text(
                        message,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.error,
                    )
                }
            }
            if (loading) {
                item {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.Center,
                    ) {
                        CircularProgressIndicator()
                    }
                }
            }
            arena?.let { info ->
                item { ArenaHeader(info) }
                val (ally, enemy) = splitTeams(info.players)
                item { TeamSection("Ally", ally) }
                item { TeamSection("Enemy", enemy) }
            }
        }
    }
}

private fun fetchArena(ip: String): Pair<ArenaInfo?, String?> {
    val url = "http://${ip.trim().trim('/')}:8605"
    return try {
        val connection = java.net.URL(url).openConnection() as java.net.HttpURLConnection
        connection.connectTimeout = 4000
        connection.readTimeout = 4000
        val text = connection.inputStream.bufferedReader().use { it.readText() }
        if (text.isBlank() || text == "[]") {
            null to "No match yet (waiting for a battle)."
        } else {
            parseArena(text) to null
        }
    } catch (e: Exception) {
        null to "Cannot reach $url: ${e.message}"
    }
}

@Composable
private fun ArenaHeader(info: ArenaInfo) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(4.dp)) {
            Text(
                text = info.name.ifBlank { info.gameType },
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = "${info.mapName} · ${info.scenario} · ${info.dateTime}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun TeamSection(title: String, players: List<ArenaPlayer>) {
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Text(
            text = "$title (${players.size})",
            style = MaterialTheme.typography.titleSmall,
            color = MaterialTheme.colorScheme.primary,
        )
        if (players.isEmpty()) {
            Text(
                "No players",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            players.forEach { player ->
                Row(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 6.dp, horizontal = 4.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text(
                        text = if (isBot(player.name)) "[Bot] ${player.name}" else player.name,
                        style = MaterialTheme.typography.bodyMedium,
                        modifier = Modifier.weight(1f),
                    )
                    Text(
                        text = player.shipId.toString(),
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
                HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
            }
        }
    }
}
