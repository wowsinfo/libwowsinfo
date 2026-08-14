package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.material3.AlertDialog
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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.ColorPainter
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.wowsinfo.libwowsinfo.LocalAchievementEntry

/** Achievement grid from the bundled game data (icons + detail dialog). */
@Composable
fun WikiAchievementsTab(achievements: List<LocalAchievementEntry>) {
    if (achievements.isEmpty()) {
        LoadingHint("Loading achievements...")
        return
    }
    LazyVerticalGrid(
        columns = GridCells.Fixed(4),
        contentPadding = PaddingValues(8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
        modifier = Modifier.fillMaxWidth(),
    ) {
        items(achievements, key = { it.key }) { entry ->
            AchievementCell(entry)
        }
    }
}

@Composable
private fun AchievementCell(entry: LocalAchievementEntry) {
    var showInfo by remember { mutableStateOf(false) }
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = Modifier
            .fillMaxWidth()
            .clickable { showInfo = true }
            .padding(4.dp),
    ) {
        if (entry.icon.isEmpty()) {
            AchievementPlaceholder()
        } else {
            AsyncImage(
                model = "file:///android_asset/achievements/${entry.icon}.png",
                contentDescription = entry.name,
                modifier = Modifier.size(64.dp),
                error = ColorPainter(Color(0xFFE0E0E0)),
            )
        }
        Text(
            text = entry.name,
            style = MaterialTheme.typography.labelSmall,
            textAlign = TextAlign.Center,
            maxLines = 2,
            overflow = TextOverflow.Ellipsis,
        )
    }

    if (showInfo) {
        AlertDialog(
            onDismissRequest = { showInfo = false },
            title = { Text(entry.name) },
            text = { Text(entry.description) },
            confirmButton = {
                TextButton(onClick = { showInfo = false }) { Text("OK") }
            },
        )
    }
}

@Composable
private fun AchievementPlaceholder() {
    Box(
        modifier = Modifier
            .size(64.dp)
            .background(Color(0xFFE0E0E0), MaterialTheme.shapes.small),
        contentAlignment = Alignment.Center,
    ) {
        Text("?", style = MaterialTheme.typography.titleMedium)
    }
}
