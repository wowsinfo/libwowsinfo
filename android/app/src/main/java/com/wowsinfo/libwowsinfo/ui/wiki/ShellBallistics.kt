package com.wowsinfo.libwowsinfo.ui.wiki

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
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.drawText
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.unit.dp
import com.wowsinfo.libwowsinfo.ShellInfo
import com.wowsinfo.libwowsinfo.ui.SectionTitle
import com.wowsinfo.libwowsinfo.ui.chartColor
import kotlin.math.PI
import kotlin.math.cos
import kotlin.math.pow
import kotlin.math.sin
import kotlin.math.sqrt

/** One simulated shell flight sample. */
data class BallisticPoint(val rangeKm: Double, val velocity: Double, val time: Double)

/**
 * Shell flight simulation mirroring the Rust `combat::ballistics` module:
 * International Standard Atmosphere density, quadratic drag and RK4
 * integration. Drives the shell velocity chart (AP penetration proxy).
 */
object ShellBallistics {
    private const val P0 = 101_325.0
    private const val L = 0.0065
    private const val T0 = 288.15
    private const val G = 9.8
    private const val M_AIR = 0.028_964_4
    private const val R_GAS = 8.314_47
    private const val DRAG = 0.25

    fun airDensity(y: Double): Double {
        val t = T0 - L * y
        if (t <= 0.0) return 0.0
        return M_AIR * P0 * (t / T0).pow(G * M_AIR / (R_GAS * L)) / (R_GAS * t)
    }

    private fun dragCoeff(massKg: Double, caliberM: Double): Double =
        PI / 8.0 * DRAG * caliberM * caliberM / massKg

    fun simulate(massKg: Double, caliberM: Double, muzzle: Double, maxKm: Double): List<BallisticPoint> {
        val k = dragCoeff(massKg, caliberM)
        val dt = 0.02
        // Fire at 45 degrees so the shell arcs over the full gun range.
        val angle = Math.toRadians(45.0)
        var vx = muzzle * cos(angle)
        var vy = muzzle * sin(angle)
        var x = 0.0
        var y = 0.0
        var time = 0.0
        val points = mutableListOf<BallisticPoint>()
        val maxM = maxKm * 1000.0
        var nextMark = 0.5
        while (x < maxM && y >= 0.0 && time < 120.0) {
            val rho = airDensity(y)
            val speed = sqrt(vx * vx + vy * vy)
            val ax = -k * rho * vx * speed
            val ay = -k * rho * vy * speed - G
            vx += ax * dt
            vy += ay * dt
            x += vx * dt
            y += vy * dt
            time += dt
            if (x >= nextMark * 1000.0 && nextMark <= maxKm + 0.001) {
                points.add(
                    BallisticPoint(
                        rangeKm = nextMark,
                        velocity = sqrt(vx * vx + vy * vy),
                        time = time,
                    ),
                )
                nextMark += 0.5
            }
        }
        return points
    }

    /** Caliber (mm) parsed from a shell name like "152 mm AP Mk35". */
    fun caliberOf(shellName: String): Double =
        Regex("""([\d.]+)\s*mm""").find(shellName)?.groupValues?.get(1)?.toDoubleOrNull() ?: 0.0
}

/** Line chart of the AP shell's remaining velocity over range. */
@Composable
fun ShellBallisticsSection(shell: ShellInfo, maxRangeKm: Double, modifier: Modifier = Modifier) {
    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(6.dp)) {
        SectionTitle("Shell ballistics")
        Text(
            "AP remaining velocity over range (drag + gravity)",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        val caliber = ShellBallistics.caliberOf(shell.name)
        val points = if (shell.bulletMass > 0.0 && shell.bulletSpeed > 0.0 && caliber > 0.0) {
            ShellBallistics.simulate(shell.bulletMass, caliber / 1000.0, shell.bulletSpeed, maxRangeKm)
        } else {
            emptyList()
        }
        if (points.size < 2) {
            Text(
                "Not enough shell data.",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        } else {
            val color = chartColor(1)
            val onSurface = MaterialTheme.colorScheme.onSurface
            val textMeasurer = rememberTextMeasurer()
            Canvas(modifier = Modifier.fillMaxWidth().height(160.dp)) {
                val maxVelocity = points.maxOf { it.velocity }
                val minVelocity = points.minOf { it.velocity }
                val range = (maxVelocity - minVelocity).coerceAtLeast(1.0)
                val stepX = size.width / (points.size - 1)
                val path = Path()
                points.forEachIndexed { index, point ->
                    val x = index * stepX
                    val y = size.height - ((point.velocity - minVelocity) / range * size.height).toFloat()
                    if (index == 0) path.moveTo(x, y) else path.lineTo(x, y)
                }
                drawPath(path, color = color, style = Stroke(width = 3f, cap = StrokeCap.Round))
                val end = textMeasurer.measure(
                    "${points.last().rangeKm} km · ${points.last().velocity.toInt()} m/s",
                )
                drawText(
                    textLayoutResult = end,
                    topLeft = Offset(size.width - end.size.width, size.height - end.size.height),
                    color = onSurface,
                )
            }
        }
    }
}
