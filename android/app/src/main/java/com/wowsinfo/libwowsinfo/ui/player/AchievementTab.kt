package com.wowsinfo.libwowsinfo.ui.player

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.Achievement
import com.wowsinfo.libwowsinfo.ui.SectionTitle

@Composable
fun AchievementTab(achievements: List<Achievement>) {
    LazyColumn(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        item { SectionTitle("Achievements (${achievements.size})") }
        if (achievements.isEmpty()) {
            item {
                Text(
                    "No achievements",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
        items(achievements, key = { it.id }) { achievement ->
            Row(
                modifier = Modifier.fillMaxWidth().padding(vertical = 8.dp, horizontal = 4.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                AsyncImage(
                    model = achievement.icon,
                    contentDescription = null,
                    modifier = Modifier.size(36.dp),
                )
                Column(
                    modifier = Modifier.weight(1f).padding(start = 12.dp),
                ) {
                    Text(
                        achievement.name.ifEmpty { "Achievement ${achievement.id}" },
                        style = MaterialTheme.typography.bodyMedium,
                    )
                    Text(
                        "ID ${achievement.id}",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
                Text(
                    "× ${achievement.count}",
                    style = MaterialTheme.typography.bodyMedium,
                )
            }
            HorizontalDivider(color = MaterialTheme.colorScheme.surfaceVariant)
        }
    }
}
