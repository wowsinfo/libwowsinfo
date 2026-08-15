package com.wowsinfo.libwowsinfo.ui

import androidx.compose.runtime.staticCompositionLocalOf
import com.wowsinfo.libwowsinfo.LocalizedUnits

/** Display-ready unit suffixes for the current language (resolved from lang.zst). */
val LocalUnits = staticCompositionLocalOf {
    LocalizedUnits(" knots", " s", " km", " m", " m/s", " mm", " kg")
}
