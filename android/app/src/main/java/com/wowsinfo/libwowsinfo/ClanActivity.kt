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
import com.wowsinfo.libwowsinfo.ui.player.ClanScreen

class ClanActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val core = (application as WoWsInfoApp).core
        val clanId = intent.getStringExtra(EXTRA_CLAN_ID)?.toULongOrNull()
        setContent {
            WoWsInfoTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    val viewModel by core.viewModel.collectAsState()
                    LaunchedEffect(viewModel.selectedClan, clanId) {
                        if (clanId != null && viewModel.selectedClan?.clanId != clanId) {
                            core.update(Event.SelectClan(clanId))
                        }
                    }
                    val clan = viewModel.selectedClan?.takeIf { clanId == null || it.clanId == clanId }
                    if (clan == null) {
                        Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                            CircularProgressIndicator()
                        }
                    } else {
                        ClanScreen(
                            clan = clan,
                            onBack = { finish() },
                            onMemberClick = { member ->
                                core.update(Event.SelectPlayer(member.accountId))
                                startActivity(
                                    Intent(this, PlayerActivity::class.java)
                                        .putExtra(
                                            PlayerActivity.EXTRA_ACCOUNT_ID,
                                            member.accountId.toString(),
                                        ),
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
        const val EXTRA_CLAN_ID = "clan_id"
    }
}
