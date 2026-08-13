package com.wowsinfo.libwowsinfo

import android.content.Intent
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
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.wiki.WikiShipDetailScreen

class WikiShipDetailActivity : ComponentActivity() {
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
                    LaunchedEffect(viewModel.selectedShipWiki, shipId) {
                        if (shipId != null && viewModel.selectedShipWiki?.shipId != shipId) {
                            core.update(Event.LoadShipWiki(shipId))
                        }
                    }
                    val ship = viewModel.selectedShipWiki?.takeIf { shipId == null || it.shipId == shipId }
                    if (ship == null) {
                        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                            CircularProgressIndicator()
                        }
                    } else {
                        val similar = viewModel.warship.values
                            .filter { it.tier == ship.tier && it.type == ship.type && it.shipId != ship.shipId }
                            .sortedBy { it.name }
                        WikiShipDetailScreen(
                            ship = ship,
                            similarShips = similar,
                            onBack = { finish() },
                            onShipClick = { id ->
                                startActivity(
                                    Intent(this, WikiShipDetailActivity::class.java)
                                        .putExtra(EXTRA_SHIP_ID, id.toString()),
                                )
                            },
                            modifier = Modifier.fillMaxSize().safeDrawingPadding(),
                        )
                    }
                }
            }
        }
    }

    companion object {
        const val EXTRA_SHIP_ID = "ship_id"
    }
}
