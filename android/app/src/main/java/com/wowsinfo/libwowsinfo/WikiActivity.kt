package com.wowsinfo.libwowsinfo

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
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.wiki.WikiScreen

class WikiActivity : ComponentActivity() {
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
                    LaunchedEffect(Unit) {
                        core.update(Event.LoadWarship)
                        core.update(Event.LoadWiki(WikiDataset.CONSUMABLES))
                        core.update(Event.LoadWiki(WikiDataset.COMMANDERSKILLS))
                        core.update(Event.LoadWiki(WikiDataset.COLLECTIONS))
                    }
                    WikiScreen(
                        viewModel = viewModel,
                        onBack = { finish() },
                        modifier = Modifier.fillMaxSize().safeDrawingPadding(),
                    )
                }
            }
        }
    }
}
