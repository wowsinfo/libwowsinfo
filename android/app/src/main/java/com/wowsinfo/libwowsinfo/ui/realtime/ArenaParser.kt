package com.wowsinfo.libwowsinfo.ui.realtime

import org.json.JSONObject

/** One vehicle from `tempArenaInfo.json` (the RS companion's live match). */
data class ArenaPlayer(
    val name: String,
    val shipId: Long,
    val relation: Int,
    val id: Long,
)

/** Parsed `tempArenaInfo.json` document, mirroring the Rust `arena` module. */
data class ArenaInfo(
    val mapName: String,
    val gameMode: Int,
    val playersPerTeam: Int,
    val teamsCount: Int,
    val duration: Int,
    val name: String,
    val scenario: String,
    val gameType: String,
    val dateTime: String,
    val players: List<ArenaPlayer>,
)

/** Bots/AI ships use `:`-prefixed names and are skipped for stat lookups. */
fun isBot(name: String): Boolean = name.startsWith(":")

/** Parse the arena file served by WoWs-RS on the LAN. */
fun parseArena(text: String): ArenaInfo? = runCatching {
    val json = JSONObject(text)
    if (json.isNull("vehicles")) return null
    val vehicles = json.getJSONArray("vehicles")
    val players = (0 until vehicles.length()).map { index ->
        val v = vehicles.getJSONObject(index)
        ArenaPlayer(
            name = v.optString("name", ""),
            shipId = v.optLong("shipId", 0),
            relation = v.optInt("relation", 2),
            id = v.optLong("id", 0),
        )
    }
    ArenaInfo(
        mapName = json.optString("mapName", ""),
        gameMode = json.optInt("gameMode", 0),
        playersPerTeam = json.optInt("playersPerTeam", 0),
        teamsCount = json.optInt("teamsCount", 0),
        duration = json.optInt("duration", 0),
        name = json.optString("name", ""),
        scenario = json.optString("scenario", ""),
        gameType = json.optString("gameType", ""),
        dateTime = json.optString("dateTime", ""),
        players = players,
    )
}.getOrNull()

/** Ally/enemy split mirroring the RS client (`relation < 2` is friendly). */
fun splitTeams(players: List<ArenaPlayer>): Pair<List<ArenaPlayer>, List<ArenaPlayer>> =
    players.partition { it.relation < 2 }
