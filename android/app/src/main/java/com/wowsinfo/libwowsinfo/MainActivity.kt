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
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import com.wowsinfo.libwowsinfo.ui.SearchScreen
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val core = (application as WoWsInfoApp).core
        setContent {
            WoWsInfoTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    val viewModel by core.viewModel.collectAsState()
                    SearchScreen(
                        core = core,
                        viewModel = viewModel,
                        onPlayerSelected = { accountId ->
                            core.update(Event.SelectPlayer(accountId))
                            startActivity(
                                Intent(this, PlayerActivity::class.java)
                                    .putExtra(PlayerActivity.EXTRA_ACCOUNT_ID, accountId.toString()),
                            )
                        },
                        onClanSelected = { clanId ->
                            core.update(Event.SelectClan(clanId))
                            startActivity(
                                Intent(this, ClanActivity::class.java)
                                    .putExtra(ClanActivity.EXTRA_CLAN_ID, clanId.toString()),
                            )
                        },
                        onRealtime = {
                            startActivity(Intent(this, RealtimeActivity::class.java))
                        },
                        onWiki = {
                            startActivity(Intent(this, WikiActivity::class.java))
                        },
                        modifier = Modifier.safeDrawingPadding(),
                    )
                }
            }
        }
    }
}
