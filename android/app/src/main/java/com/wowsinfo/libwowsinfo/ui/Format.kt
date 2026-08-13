package com.wowsinfo.libwowsinfo.ui

import android.graphics.Color
import androidx.compose.ui.graphics.Color as ComposeColor
import com.wowsinfo.libwowsinfo.Server
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

val Server.displayName: String
    get() = when (this) {
        Server.RU -> "RU"
        Server.EU -> "EU"
        Server.COM -> "NA"
        Server.ASIA -> "ASIA"
    }

fun formatPercent(value: Double): String = String.format(Locale.US, "%.2f%%", value)

fun formatNumber(value: Long): String = String.format(Locale.US, "%,d", value)

fun formatNumber(value: Double): String = String.format(Locale.US, "%,.0f", value)

fun formatRating(value: Double): String = String.format(Locale.US, "%,.0f", value)

fun formatDecimal(value: Double): String = String.format(Locale.US, "%.2f", value)

fun parseRatingColor(hex: String): ComposeColor =
    runCatching { ComposeColor(Color.parseColor(hex)) }.getOrDefault(ComposeColor.Unspecified)

fun formatEpochDate(seconds: Long?): String {
    if (seconds == null || seconds <= 0) {
        return "Unknown"
    }
    return SimpleDateFormat("yyyy-MM-dd", Locale.US).format(Date(seconds * 1000))
}
