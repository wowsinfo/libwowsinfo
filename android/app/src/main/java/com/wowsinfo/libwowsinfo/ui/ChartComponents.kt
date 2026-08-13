package com.wowsinfo.libwowsinfo.ui

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp

/** Material 500 palette used by the original app's charts. */
private val ChartPalette = listOf(
    Color(0xFFF44336), // red
    Color(0xFF2196F3), // blue
    Color(0xFF4CAF50), // green
    Color(0xFF9C27B0), // purple
    Color(0xFFFF5722), // deep orange
    Color(0xFF00BCD4), // cyan
    Color(0xFF3F51B5), // indigo
    Color(0xFFCDDC39), // lime
    Color(0xFFE91E63), // pink
    Color(0xFF9E9E9E), // gray
    Color(0xFF009688), // teal
    Color(0xFFFFEB3B), // yellow
)

fun chartColor(index: Int): Color = ChartPalette[index % ChartPalette.size]

/** Section title, optional subtitle and horizontal bars. */
@Composable
fun ChartSection(
    title: String,
    entries: List<Pair<String, Double>>,
    subtitle: String? = null,
    valueFormat: (Double) -> String = { formatNumber(it) },
    color: Color? = null,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle(title)
        if (subtitle != null) {
            Text(
                subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (entries.isEmpty()) {
            Text(
                "No battles",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            val max = entries.maxOf { it.second }.coerceAtLeast(1.0)
            entries.forEachIndexed { index, (label, value) ->
                BarRow(
                    label = label,
                    valueText = valueFormat(value),
                    fraction = value / max,
                    color = color ?: chartColor(index),
                )
            }
        }
    }
}

@Composable
fun BarRow(
    label: String,
    valueText: String,
    fraction: Double,
    color: Color = Color.Unspecified,
) {
    val barColor = if (color == Color.Unspecified) {
        MaterialTheme.colorScheme.primary
    } else {
        color
    }
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodySmall,
            maxLines = 1,
            overflow = TextOverflow.Ellipsis,
            modifier = Modifier.width(110.dp),
        )
        Box(
            modifier = Modifier
                .weight(1f)
                .height(14.dp)
                .background(MaterialTheme.colorScheme.surfaceVariant),
        ) {
            Box(
                modifier = Modifier
                    .fillMaxWidth(fraction.toFloat())
                    .height(14.dp)
                    .background(barColor),
            )
        }
        Text(
            text = valueText,
            style = MaterialTheme.typography.bodySmall,
            textAlign = TextAlign.End,
            modifier = Modifier.width(64.dp).padding(start = 4.dp),
        )
    }
}

/** Simple line chart for the recent 10-day stats. */
@Composable
fun LineChartSection(
    title: String,
    subtitle: String,
    points: List<Pair<String, Double>>,
    color: Color = Color.Unspecified,
) {
    Column(verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle(title)
        Text(
            subtitle,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        if (points.size < 2) {
            Text(
                "Not enough data",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            val lineColor = if (color == Color.Unspecified) {
                MaterialTheme.colorScheme.primary
            } else {
                color
            }
            Canvas(modifier = Modifier.fillMaxWidth().height(140.dp)) {
                val max = points.maxOf { it.second }
                val min = points.minOf { it.second }
                val range = (max - min).coerceAtLeast(1.0)
                val stepX = size.width / (points.size - 1)
                val path = Path()
                points.forEachIndexed { index, (_, value) ->
                    val x = index * stepX
                    val y = size.height - ((value - min) / range * size.height).toFloat()
                    if (index == 0) path.moveTo(x, y) else path.lineTo(x, y)
                }
                drawPath(
                    path = path,
                    color = lineColor,
                    style = Stroke(width = 3f, cap = StrokeCap.Round),
                )
            }
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    shortDate(points.first().first),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Text(
                    shortDate(points.last().first),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

private fun shortDate(yyyymmdd: String): String =
    if (yyyymmdd.length == 8) {
        "${yyyymmdd.substring(4, 6)}-${yyyymmdd.substring(6, 8)}"
    } else {
        yyyymmdd
    }
