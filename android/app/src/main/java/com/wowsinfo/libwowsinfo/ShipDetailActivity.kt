package com.wowsinfo.libwowsinfo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.player.ShipDetailScreen

class ShipDetailActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val core = (application as WoWsInfoApp).core
        val shipId = intent.getStringExtra(EXTRA_SHIP_ID)?.toULongOrNull()
        setContent {
            WoWsInfoTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    val viewModel by core.viewModel.collectAsState()
                    val ship = viewModel.player?.ships?.firstOrNull { it.shipId == shipId }
                    if (ship != null) {
                        Box(Modifier.fillMaxSize().safeDrawingPadding()) {
                            ShipDetailScreen(ship, onBack = { finish() })
                        }
                    } else {
                        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                            CircularProgressIndicator()
                        }
                    }
                }
            }
        }
    }

    companion object {
        const val EXTRA_SHIP_ID = "ship_id"
    }
}
