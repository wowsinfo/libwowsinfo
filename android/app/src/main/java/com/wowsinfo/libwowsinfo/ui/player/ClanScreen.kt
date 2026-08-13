package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ClanInfo
import com.wowsinfo.libwowsinfo.ClanMember
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.Stat
import com.wowsinfo.libwowsinfo.ui.formatEpochDate
import com.wowsinfo.libwowsinfo.ui.formatNumber

/**
 * Clan screen mirroring the Flutter `ClanInfoPage`: clan summary and a member
 * list where every member opens the player screen.
 */
@Composable
fun ClanScreen(
    clan: ClanInfo,
    onBack: () -> Unit,
    onMemberClick: (ClanMember) -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier.fillMaxSize()) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            TextButton(onClick = onBack) { Text("‹ Search") }
        }
        LazyColumn(
            modifier = Modifier.fillMaxWidth(),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
                    Text(
                        text = "${clan.name} [${clan.tag}]",
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                    )
                    if (clan.description.isNotEmpty()) {
                        Text(
                            text = clan.description,
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceAround,
                    ) {
                        Stat("Members", formatNumber(clan.membersCount), modifier = Modifier.weight(1f))
                        Stat(
                            "Created",
                            formatEpochDate(clan.createdAt),
                            modifier = Modifier.weight(1f),
                        )
                    }
                    clan.leaderName.takeIf { it.isNotEmpty() }?.let { leader ->
                        Text(
                            text = "Leader: $leader",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
            item { SectionTitle("Members (${clan.members.size})") }
            if (clan.members.isEmpty()) {
                item {
                    Text(
                        "No member details available.",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            } else {
                items(clan.members, key = { it.accountId.toString() }) { member ->
                    MemberRow(member, onClick = { onMemberClick(member) })
                }
            }
        }
    }
}

@Composable
private fun MemberRow(member: ClanMember, onClick: () -> Unit) {
    Column(modifier = Modifier.fillMaxWidth().clickable(onClick = onClick)) {
        Row(
            modifier = Modifier.fillMaxWidth().padding(vertical = 10.dp, horizontal = 4.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = member.accountName,
                    style = MaterialTheme.typography.titleMedium,
                )
                Text(
                    text = member.role.replace('_', ' '),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Text(
                text = "›",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.primary,
            )
        }
        HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
    }
}
