package com.wowsinfo.libwowsinfo.ui

import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

/**
 * Palette from the original app (`value/colour.ts`): GREY[100]/GREY[900]
 * backgrounds, white/black surfaces and a BLUE[500] primary tint.
 */
private val LightColors = lightColorScheme(
    primary = Color(0xFF2196F3),
    background = Color(0xFFF5F5F5),
    surface = Color(0xFFFFFFFF),
    surfaceVariant = Color(0xFFE0E0E0),
    error = Color(0xFFF44336),
    onSurfaceVariant = Color(0xFF616161),
)

private val DarkColors = darkColorScheme(
    primary = Color(0xFF2196F3),
    background = Color(0xFF212121),
    surface = Color(0xFF000000),
    surfaceVariant = Color(0xFF424242),
    error = Color(0xFFF44336),
    onSurfaceVariant = Color(0xFFBDBDBD),
)

@Composable
fun WoWsInfoTheme(content: @Composable () -> Unit) {
    val context = LocalContext.current
    val dark = isSystemInDarkTheme()
    val colorScheme = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        // Material You expressive colours from the wallpaper palette.
        if (dark) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
    } else {
        if (dark) DarkColors else LightColors
    }
    MaterialTheme(
        colorScheme = colorScheme,
        content = content,
    )
}

/** Themed section heading, like `SectionTitle.tsx` in the original app. */
@Composable
fun SectionTitle(title: String, modifier: Modifier = Modifier) {
    Text(
        text = title,
        style = MaterialTheme.typography.titleMedium,
        fontWeight = FontWeight.Bold,
        color = MaterialTheme.colorScheme.primary,
        modifier = modifier.padding(top = 8.dp, bottom = 4.dp),
    )
}
