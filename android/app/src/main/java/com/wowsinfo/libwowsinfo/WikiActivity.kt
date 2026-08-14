package com.wowsinfo.libwowsinfo

import android.os.Bundle
import android.content.Intent
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
                        core.update(Event.LoadLocalWarships)
                        core.update(Event.LoadWiki(WikiDataset.COLLECTIONS))
                        core.update(Event.LoadWiki(WikiDataset.COLLECTIONCARDS))
                        core.update(Event.LoadWiki(WikiDataset.MAPS))
                    }
                    WikiScreen(
                        viewModel = viewModel,
                        onBack = { finish() },
                        onShipClick = { shipId ->
                            startActivity(
                                Intent(this, WikiShipDetailActivity::class.java)
                                    .putExtra(WikiShipDetailActivity.EXTRA_SHIP_ID, shipId.toString()),
                            )
                        },
                        modifier = Modifier.fillMaxSize().safeDrawingPadding(),
                    )
                }
            }
        }
    }
}
