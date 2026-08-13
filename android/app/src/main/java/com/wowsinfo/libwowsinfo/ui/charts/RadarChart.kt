package com.wowsinfo.libwowsinfo.ui.charts

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.drawText
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.chartColor
import com.wowsinfo.libwowsinfo.ui.formatPercent
import kotlin.math.cos
import kotlin.math.min
import kotlin.math.sin

/**
 * Stats-vs-average radar: damage, winrate and frags relative to the expected
 * values (100 = at average), like the community-assistant radar.
 */
@Composable
fun RadarChartSection(
    values: RadarValues,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Stats vs average")
        Text(
            "Player performance relative to the expected values (100 = average)",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        val textMeasurer = rememberTextMeasurer()
        val color = chartColor(1)
        val labelStyle = MaterialTheme.typography.labelSmall
        val outline = MaterialTheme.colorScheme.outlineVariant
        val onSurface = MaterialTheme.colorScheme.onSurface
        Canvas(modifier = Modifier.fillMaxWidth().height(220.dp)) {
            val center = Offset(size.width / 2f, size.height / 2f)
            val radius = min(size.width, size.height) / 2f - 28.dp.toPx()
            val axis = listOf(-90.0, 30.0, 150.0)
            fun point(angle: Double, fraction: Float) = Offset(
                x = center.x + (radius * fraction * cos(Math.toRadians(angle))).toFloat(),
                y = center.y + (radius * fraction * sin(Math.toRadians(angle))).toFloat(),
            )

            // Grid rings at 33/66/100% of the max draw scale.
            val drawMax = maxOf(100f, values.damage.toFloat(), values.winrate.toFloat(), values.frags.toFloat())
            for (ring in listOf(1f / 3f, 2f / 3f, 1f)) {
                val ringPath = Path()
                axis.forEachIndexed { index, angle ->
                    val p = point(angle, ring * radius / radius)
                    if (index == 0) ringPath.moveTo(p.x, p.y) else ringPath.lineTo(p.x, p.y)
                }
                ringPath.close()
                drawPath(
                    ringPath,
                    color = outline,
                    style = Stroke(width = 1f),
                )
            }
            // Axis lines and the 100% reference ring.
            axis.forEach { angle ->
                drawLine(
                    color = outline,
                    start = center,
                    end = point(angle, 1f),
                    strokeWidth = 1f,
                )
            }
            // Value polygon.
            val valuesList = listOf(values.damage, values.winrate, values.frags)
            val valuePath = Path()
            valuesList.forEachIndexed { index, value ->
                val p = point(axis[index], (value.toFloat() / drawMax).coerceIn(0f, 1f))
                if (index == 0) valuePath.moveTo(p.x, p.y) else valuePath.lineTo(p.x, p.y)
            }
            valuePath.close()
            drawPath(valuePath, color = color.copy(alpha = 0.25f))
            drawPath(valuePath, color = color, style = Stroke(width = 3f))

            // Labels at each vertex.
            val labels = listOf(
                "Damage" to formatPercent(values.damage),
                "Winrate" to formatPercent(values.winrate),
                "Frags" to formatPercent(values.frags),
            )
            labels.forEachIndexed { index, (name, percent) ->
                val p = point(axis[index], 1.22f)
                val text = textMeasurer.measure("$name\n$percent", style = labelStyle)
                drawText(
                    textLayoutResult = text,
                    topLeft = Offset(p.x - text.size.width / 2f, p.y - text.size.height / 2f),
                    color = onSurface,
                )
            }
        }
    }
}
