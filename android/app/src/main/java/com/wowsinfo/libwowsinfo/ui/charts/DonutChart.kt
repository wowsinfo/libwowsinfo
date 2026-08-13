package com.wowsinfo.libwowsinfo.ui.charts

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.drawText
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatNumber
import kotlin.math.PI

/** Donut chart of the game-mode battle distribution with a legend. */
@Composable
fun DonutChartSection(
    modes: List<ModeBattles>,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(8.dp)) {
        SectionTitle("Battles by game mode")
        val total = modes.sumOf { it.battles }
        val textMeasurer = rememberTextMeasurer()
        val centerStyle = MaterialTheme.typography.titleMedium
        val onSurface = MaterialTheme.colorScheme.onSurface
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Canvas(modifier = Modifier.size(160.dp)) {
                val stroke = 26.dp.toPx()
                val arcSize = Size(size.width - stroke, size.height - stroke)
                val topLeft = Offset(stroke / 2f, stroke / 2f)
                var start = -90f
                modes.forEachIndexed { index, mode ->
                    val sweep = 360f * mode.battles / total.toFloat()
                    drawArc(
                        color = chartColor(index),
                        startAngle = start,
                        sweepAngle = sweep,
                        useCenter = false,
                        topLeft = topLeft,
                        size = arcSize,
                        style = Stroke(width = stroke),
                    )
                    start += sweep
                }
                val center = textMeasurer.measure(formatNumber(total), style = centerStyle)
                drawText(
                    textLayoutResult = center,
                    topLeft = Offset(
                        (size.width - center.size.width) / 2f,
                        (size.height - center.size.height) / 2f,
                    ),
                    color = onSurface,
                )
            }
            Column(
                modifier = Modifier.padding(start = 12.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                modes.forEachIndexed { index, mode ->
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Canvas(modifier = Modifier.size(10.dp)) {
                            drawCircle(color = chartColor(index))
                        }
                        Text(
                            text = mode.mode,
                            style = MaterialTheme.typography.bodySmall,
                            modifier = Modifier.width(48.dp).padding(start = 6.dp),
                        )
                        Text(
                            text = "${formatNumber(mode.battles)} (${
                                String.format("%.1f%%", mode.battles * 100.0 / total)
                            })",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
        }
    }
}
