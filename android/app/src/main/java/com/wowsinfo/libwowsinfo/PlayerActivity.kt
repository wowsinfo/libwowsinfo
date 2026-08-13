package com.wowsinfo.libwowsinfo

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import com.wowsinfo.libwowsinfo.Phase
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.player.PlayerScreen

class PlayerActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val core = (application as WoWsInfoApp).core
        val accountId = intent.getStringExtra(EXTRA_ACCOUNT_ID)?.toULongOrNull()
        setContent {
            WoWsInfoTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    val viewModel by core.viewModel.collectAsState()
                    LaunchedEffect(viewModel.player, viewModel.phase, accountId) {
                        // Reload the player after process death; on the normal
                        // flow the load is already in flight, so this is a no-op.
                        if (viewModel.player == null &&
                            viewModel.phase !is Phase.LoadingPlayer &&
                            accountId != null
                        ) {
                            core.update(Event.SelectPlayer(accountId))
                        }
                    }
                    PlayerScreen(
                        core = core,
                        viewModel = viewModel,
                        onBack = { finish() },
                        onShipClick = { ship ->
                            startActivity(
                                Intent(this, ShipDetailActivity::class.java)
                                    .putExtra(
                                        ShipDetailActivity.EXTRA_SHIP_ID,
                                        ship.shipId.toString(),
                                    ),
                            )
                        },
                        modifier = Modifier.safeDrawingPadding(),
                    )
                }
            }
        }
    }

    companion object {
        const val EXTRA_ACCOUNT_ID = "account_id"
    }
}
