package com.wowsinfo.libwowsinfo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.wowsinfo.libwowsinfo.core.Core
import com.wowsinfo.libwowsinfo.ui.SearchScreen
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.player.PlayerScreen

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
                    WoWsInfoScreen(core)
                }
            }
        }
    }
}

@Composable
private fun WoWsInfoScreen(core: Core) {
    val viewModel by core.viewModel.collectAsState()
    var backToSearch by rememberSaveable { mutableStateOf(false) }

    val showPlayer = viewModel.phase is Phase.Player && !backToSearch
    if (showPlayer) {
        BackHandler { backToSearch = true }
        PlayerScreen(
            core = core,
            viewModel = viewModel,
            onBack = { backToSearch = true },
            modifier = Modifier.safeDrawingPadding(),
        )
    } else {
        SearchScreen(
            core = core,
            viewModel = viewModel,
            onPlayerSelected = { backToSearch = false },
            modifier = Modifier.safeDrawingPadding(),
        )
    }
}
