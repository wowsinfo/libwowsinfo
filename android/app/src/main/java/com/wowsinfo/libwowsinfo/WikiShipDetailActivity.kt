package com.wowsinfo.libwowsinfo

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
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
                    LaunchedEffect(viewModel.localShip, viewModel.localDataReady, shipId) {
                        if (viewModel.localDataReady &&
                            shipId != null &&
                            viewModel.localShip?.shipId != shipId
                        ) {
                            core.update(Event.LoadLocalShipWiki(shipId))
                        }
                    }
                    val ship = viewModel.localShip?.takeIf { shipId == null || it.shipId == shipId }
                    if (ship == null) {
                        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                            CircularProgressIndicator()
                            androidx.compose.material3.Text(
                                text = if (viewModel.localDataReady) "Loading ship…" else "Loading game data…",
                                style = MaterialTheme.typography.bodySmall,
                                modifier = Modifier.padding(top = 12.dp),
                            )
                        }
                    } else {
                        androidx.compose.runtime.CompositionLocalProvider(
                            com.wowsinfo.libwowsinfo.ui.LocalUnits provides viewModel.units,
                        ) {
                            WikiShipDetailScreen(
                                ship = ship,
                                compare = viewModel.localCompare,
                                onBack = { finish() },
                                onShipClick = { id ->
                                    startActivity(
                                        Intent(this, WikiShipDetailActivity::class.java)
                                            .putExtra(EXTRA_SHIP_ID, id.toString()),
                                    )
                                },
                                onSelectModule = { slot, index ->
                                    core.update(Event.SelectLocalShipModule(slot, index))
                                },
                                onCompare = { shipIds ->
                                    core.update(Event.LoadLocalCompare(shipIds))
                                },
                                onToggleSkill = { key ->
                                    core.update(Event.ToggleLocalSkill(key))
                                },
                            onToggleUpgrade = { key ->
                                core.update(Event.ToggleLocalUpgrade(key))
                            },
                            onToggleFlag = { key ->
                                core.update(Event.ToggleLocalFlag(key))
                            },
                            onSetHp = { fraction ->
                                core.update(Event.SetLocalHp(fraction))
                            },
                            onSetSpotted = { spotted ->
                                core.update(Event.SetLocalSpotted(spotted))
                            },
                            modifier = Modifier.fillMaxSize().safeDrawingPadding(),
                        )
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
