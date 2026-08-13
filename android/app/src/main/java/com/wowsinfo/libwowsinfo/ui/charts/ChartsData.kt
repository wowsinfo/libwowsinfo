package com.wowsinfo.libwowsinfo.ui.charts

import com.wowsinfo.libwowsinfo.PlayerStatistics
import com.wowsinfo.libwowsinfo.ShipStatLine

/**
 * Chart data computed from a player's ship list, mirroring the Rust core's
 * `charts` module so the UI and the tested logic stay in sync.
 */

/** Stats-vs-average radar axes as percentages (100 = at average). */
data class RadarValues(val damage: Double, val winrate: Double, val frags: Double)

/** Battles-weighted per-class averages. */
data class ClassAverage(
    val className: String,
    val battles: Long,
    val avgDmg: Double,
    val avgWinrate: Double,
    val avgFrags: Double,
    val avgXp: Double,
    val survival: Double,
    val accuracy: Double,
)

/** One slice of the game-mode distribution. */
data class ModeBattles(val mode: String, val battles: Long)

private fun radarAxis(
    ships: List<ShipStatLine>,
    actual: (ShipStatLine) -> Double,
    expected: (ShipStatLine) -> Double,
): Double? {
    var battles = 0L
    var sum = 0.0
    for (ship in ships) {
        val expectedValue = expected(ship)
        if (ship.battles <= 0 || expectedValue <= 0.0) continue
        battles += ship.battles
        sum += actual(ship) / expectedValue * 100.0 * ship.battles
    }
    return if (battles == 0L) null else sum / battles
}

/** Battles-weighted player stats vs expected averages (null when unknown). */
fun playerRadar(ships: List<ShipStatLine>): RadarValues? {
    val damage = radarAxis(ships, { it.avgDmg }, { it.expectedDmg })
        ?: return null
    val winrate = radarAxis(ships, { it.avgWinrate }, { it.expectedWinrate })
        ?: return null
    val frags = radarAxis(ships, { it.avgFrags }, { it.expectedFrags })
        ?: return null
    return RadarValues(damage, winrate, frags)
}

/** Stats-vs-expected radar for a single ship (null when data is missing). */
fun shipRadar(ship: ShipStatLine): RadarValues? {
    val damage = if (ship.expectedDmg > 0.0) ship.avgDmg / ship.expectedDmg * 100.0 else return null
    val winrate =
        if (ship.expectedWinrate > 0.0) ship.avgWinrate / ship.expectedWinrate * 100.0 else return null
    val frags = if (ship.expectedFrags > 0.0) ship.avgFrags / ship.expectedFrags * 100.0 else return null
    return RadarValues(damage, winrate, frags)
}

/** Battles-weighted averages grouped by ship class, most played first. */
fun perClassAverages(ships: List<ShipStatLine>): List<ClassAverage> {
    data class Totals(
        var battles: Long = 0,
        var dmg: Double = 0.0,
        var winrate: Double = 0.0,
        var frags: Double = 0.0,
        var xp: Double = 0.0,
        var survival: Double = 0.0,
        var accuracy: Double = 0.0,
    )
    val totals = LinkedHashMap<String, Totals>()
    for (ship in ships) {
        if (ship.battles <= 0) continue
        val pvp = ship.statistics.pvp
        val battles = ship.battles.toDouble()
        val total = totals.getOrPut(ship.type) { Totals() }
        total.battles += ship.battles
        total.dmg += ship.avgDmg * battles
        total.winrate += ship.avgWinrate * battles
        total.frags += ship.avgFrags * battles
        total.xp += (pvp?.xp?.toDouble()?.div(battles) ?: 0.0) * battles
        total.survival +=
            (pvp?.let { it.survivedBattles.toDouble() / battles * 100.0 } ?: 0.0) * battles
        total.accuracy +=
            (pvp?.mainBattery?.let {
                if (it.shots > 0) it.hits.toDouble() / it.shots * 100.0 else 0.0
            } ?: 0.0) * battles
    }
    return totals
        .map { (className, total) ->
            val battles = total.battles.toDouble()
            ClassAverage(
                className = className,
                battles = total.battles,
                avgDmg = total.dmg / battles,
                avgWinrate = total.winrate / battles,
                avgFrags = total.frags / battles,
                avgXp = total.xp / battles,
                survival = total.survival / battles,
                accuracy = total.accuracy / battles,
            )
        }
        .sortedByDescending { it.battles }
}

/** Battles split across PvP / Solo / Div2 / Div3 / PvE / Rank (played only). */
fun modeDistribution(stats: PlayerStatistics): List<ModeBattles> =
    listOf(
        "PvP" to stats.pvp,
        "Solo" to stats.solo,
        "Div2" to stats.div2,
        "Div3" to stats.div3,
        "PvE" to stats.pve,
        "Rank" to stats.rankSolo,
    ).mapNotNull { (mode, pvp) ->
        pvp?.takeIf { it.battles > 0 }?.let { ModeBattles(mode, it.battles) }
    }
