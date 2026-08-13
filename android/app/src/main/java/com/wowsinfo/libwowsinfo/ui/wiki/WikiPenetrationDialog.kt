package com.wowsinfo.libwowsinfo.ui.wiki

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
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
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text(curve.shellName) },
        text = {
            Column(verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp)) {
                PenetrationChart(curve.points)
                Row(modifier = Modifier.fillMaxWidth()) {
                    LegendItem("Raw", chartColor(0), Modifier.weight(1f))
                    LegendItem("Belt", chartColor(1), Modifier.weight(1f))
                    LegendItem("Deck", chartColor(2), Modifier.weight(1f))
                }
            }
        },
        confirmButton = {
            TextButton(onClick = onDismiss) { Text("Close") }
        },
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
    if (points.size < 2) {
        Text("Not enough data", style = MaterialTheme.typography.bodySmall)
        return
    }
    val maxRange = points.maxOf { it.rangeM }.coerceAtLeast(1.0)
    val maxPen = points.maxOf { it.rawPenMm.coerceAtLeast(it.beltPenMm).coerceAtLeast(it.deckPenMm) }
        .coerceAtLeast(1.0)

    Canvas(modifier = Modifier.fillMaxWidth().height(260.dp)) {
        val left = 8f
        val top = 12f
        val right = size.width - 8f
        val bottom = size.height - 22f
        val chartWidth = right - left
        val chartHeight = bottom - top

        fun x(range: Double) = left + (range / maxRange * chartWidth).toFloat()
        fun y(pen: Double) = bottom - (pen / maxPen * chartHeight).toFloat()

        // Grid lines at 25% steps.
        for (i in 0..4) {
            val gy = top + chartHeight * i / 4
            drawLine(
                color = androidx.compose.ui.graphics.Color.White.copy(alpha = 0.12f),
                start = Offset(left, gy),
                end = Offset(right, gy),
                strokeWidth = 1f,
            )
        }

        fun line(select: (PenetrationPoint) -> Double, color: androidx.compose.ui.graphics.Color) {
            val path = Path()
            points.forEachIndexed { index, point ->
                val px = x(point.rangeM)
                val py = y(select(point))
                if (index == 0) path.moveTo(px, py) else path.lineTo(px, py)
            }
            drawPath(path, color, style = Stroke(width = 3f, cap = StrokeCap.Round))
        }

        line({ it.rawPenMm }, chartColor(0))
        line({ it.beltPenMm }, chartColor(1))
        line({ it.deckPenMm }, chartColor(2))

        // Axis labels.
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
            "${fmt(maxPen)} mm",
            4f,
            top + 10f,
            android.graphics.Paint().apply {
                color = android.graphics.Color.WHITE
                textSize = 12f * density
            },
        )
    }
}
