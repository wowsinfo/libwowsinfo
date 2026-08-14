package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.nativeCanvas
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.PenCurveView
import com.wowsinfo.libwowsinfo.PenetrationPoint
import com.wowsinfo.libwowsinfo.ui.chartColor
import java.util.Locale
import kotlin.math.roundToInt

private fun fmt(value: Double): String = String.format(Locale.US, "%.1f", value)

/**
 * Drag-based AP penetration over range (wows-toolkit ballistic model). Shows
 * the raw penetration and the effective belt/deck values for every AP shell
 * of the main battery.
 */
@Composable
fun WikiPenetrationDialog(
    curves: List<PenCurveView>,
    onDismiss: () -> Unit,
) {
    val curve = curves.firstOrNull() ?: run {
        onDismiss()
        return
    }
    val maxRange = curve.points.maxOfOrNull { it.rangeM } ?: 1.0
    var sliderRangeM by remember { mutableStateOf(maxRange) }
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text(curve.shellName) },
        text = {
            Column(
                modifier = Modifier.verticalScroll(rememberScrollState()),
                verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp),
            ) {
                PenetrationChart(curve.points)
                Row(modifier = Modifier.fillMaxWidth()) {
                    LegendItem("Raw", chartColor(0), Modifier.weight(1f))
                    LegendItem("Belt", chartColor(1), Modifier.weight(1f))
                    LegendItem("Deck", chartColor(2), Modifier.weight(1f))
                }
                LineChart(
                    points = curve.points,
                    series = listOf(Triple("Flight time", chartColor(3), { it.timeS })),
                    unit = "s",
                    height = 160,
                )
                LineChart(
                    points = curve.points,
                    series = listOf(Triple("Impact angle", chartColor(4), { it.impactAngleDeg })),
                    unit = "°",
                    height = 160,
                )
                val sample = sampleAt(curve.points, sliderRangeM)
                if (sample != null) {
                    Slider(
                        value = sliderRangeM.toFloat(),
                        onValueChange = { sliderRangeM = it.toDouble() },
                        valueRange = 0f..maxRange.toFloat(),
                    )
                    Text(
                        text = buildString {
                            append("Range ${fmt(sliderRangeM / 1000)} km · ")
                            append("Pen ${sample.rawPenMm.roundToInt()} / ")
                            append("${sample.beltPenMm.roundToInt()} / ")
                            append("${sample.deckPenMm.roundToInt()} mm · ")
                            append("Flight ${fmt(sample.timeS)} s · ")
                            append("Angle ${fmt(sample.impactAngleDeg)}°")
                        },
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) { Text("Close") }
        },
    )
}

private fun sampleAt(points: List<PenetrationPoint>, rangeM: Double): PenetrationPoint? {
    if (points.isEmpty()) return null
    if (rangeM <= points.first().rangeM) return points.first()
    val last = points.last()
    if (rangeM >= last.rangeM) return last
    val index = points.indexOfFirst { it.rangeM >= rangeM }
    if (index <= 0) return points.first()
    val hi = points[index]
    val lo = points[index - 1]
    val span = (hi.rangeM - lo.rangeM).coerceAtLeast(1e-9)
    val t = ((rangeM - lo.rangeM) / span).toFloat()
    fun lerp(a: Double, b: Double) = a + (b - a) * t
    return PenetrationPoint(
        rangeM = rangeM,
        velocity = lerp(lo.velocity, hi.velocity),
        timeS = lerp(lo.timeS, hi.timeS),
        rawPenMm = lerp(lo.rawPenMm, hi.rawPenMm),
        beltPenMm = lerp(lo.beltPenMm, hi.beltPenMm),
        deckPenMm = lerp(lo.deckPenMm, hi.deckPenMm),
        impactAngleDeg = lerp(lo.impactAngleDeg, hi.impactAngleDeg),
    )
}

@Composable
private fun LegendItem(label: String, color: androidx.compose.ui.graphics.Color, modifier: Modifier = Modifier) {
    Row(modifier = modifier, verticalAlignment = Alignment.CenterVertically) {
        Canvas(modifier = Modifier.padding(end = 6.dp).width(24.dp).height(4.dp)) {
            drawLine(color, Offset(0f, size.height / 2), Offset(size.width, size.height / 2), strokeWidth = 3f)
        }
        Text(text = label, style = MaterialTheme.typography.labelSmall, color = color)
    }
}

@Composable
private fun PenetrationChart(points: List<PenetrationPoint>) {
    LineChart(
        points = points,
        series = listOf(
            Triple("Raw", chartColor(0), { it.rawPenMm }),
            Triple("Belt", chartColor(1), { it.beltPenMm }),
            Triple("Deck", chartColor(2), { it.deckPenMm }),
        ),
        unit = "mm",
        height = 260,
    )
}

@Composable
private fun LineChart(
    points: List<PenetrationPoint>,
    series: List<Triple<String, androidx.compose.ui.graphics.Color, (PenetrationPoint) -> Double>>,
    unit: String,
    height: Int,
) {
    if (points.size < 2) {
        Text("Not enough data", style = MaterialTheme.typography.bodySmall)
        return
    }
    val maxRange = points.maxOf { it.rangeM }.coerceAtLeast(1.0)
    val maxY = series.flatMap { triple -> points.map(triple.third) }.maxOrNull()
        ?.coerceAtLeast(1.0) ?: 1.0

    Canvas(modifier = Modifier.fillMaxWidth().height(height.dp)) {
        val left = 8f
        val top = 12f
        val right = size.width - 8f
        val bottom = size.height - 22f
        val chartWidth = right - left
        val chartHeight = bottom - top

        fun x(range: Double) = left + (range / maxRange * chartWidth).toFloat()
        fun y(value: Double) = bottom - (value / maxY * chartHeight).toFloat()

        for (i in 0..4) {
            val gy = top + chartHeight * i / 4
            drawLine(
                color = androidx.compose.ui.graphics.Color.White.copy(alpha = 0.12f),
                start = Offset(left, gy),
                end = Offset(right, gy),
                strokeWidth = 1f,
            )
        }

        series.forEach { (_, color, select) ->
            val path = Path()
            points.forEachIndexed { index, point ->
                val px = x(point.rangeM)
                val py = y(select(point))
                if (index == 0) path.moveTo(px, py) else path.lineTo(px, py)
            }
            drawPath(path, color, style = Stroke(width = 3f, cap = StrokeCap.Round))
        }

        drawContext.canvas.nativeCanvas.drawText(
            "0",
            left,
            bottom + 16f,
            android.graphics.Paint().apply {
                color = android.graphics.Color.WHITE
                textSize = 12f * density
            },
        )
        drawContext.canvas.nativeCanvas.drawText(
            "${fmt(maxRange / 1000)} km",
            right - 34f * density,
            bottom + 16f,
            android.graphics.Paint().apply {
                color = android.graphics.Color.WHITE
                textSize = 12f * density
            },
        )
        drawContext.canvas.nativeCanvas.drawText(
            "${fmt(maxY)} $unit",
            4f,
            top + 10f,
            android.graphics.Paint().apply {
                color = android.graphics.Color.WHITE
                textSize = 12f * density
            },
        )
    }
}
