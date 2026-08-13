package com.wowsinfo.libwowsinfo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInHorizontally
import androidx.compose.animation.slideOutHorizontally
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.wowsinfo.libwowsinfo.core.Core
import com.wowsinfo.libwowsinfo.ui.SearchScreen
import com.wowsinfo.libwowsinfo.ui.WoWsInfoTheme
import com.wowsinfo.libwowsinfo.ui.player.PlayerScreen
import com.wowsinfo.libwowsinfo.ui.player.ShipDetailScreen

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
                    WoWsInfoNavHost(core)
                }
            }
        }
    }
}

@Composable
private fun WoWsInfoNavHost(core: Core) {
    val navController = rememberNavController()
    NavHost(
        navController = navController,
        startDestination = "search",
        modifier = Modifier.safeDrawingPadding(),
        enterTransition = {
            slideInHorizontally(initialOffsetX = { it }, animationSpec = tween(280)) +
                fadeIn(animationSpec = tween(280))
        },
        exitTransition = { fadeOut(animationSpec = tween(200)) },
        popEnterTransition = {
            slideInHorizontally(initialOffsetX = { -it / 4 }, animationSpec = tween(280)) +
                fadeIn(animationSpec = tween(280))
        },
        popExitTransition = {
            slideOutHorizontally(targetOffsetX = { it }, animationSpec = tween(280)) +
                fadeOut(animationSpec = tween(280))
        },
    ) {
        composable("search") {
            val viewModel by core.viewModel.collectAsState()
            SearchScreen(
                core = core,
                viewModel = viewModel,
                onPlayerSelected = { navController.navigate("player") },
            )
        }
        composable("player") {
            val viewModel by core.viewModel.collectAsState()
            PlayerScreen(
                core = core,
                viewModel = viewModel,
                onBack = { navController.popBackStack() },
                onShipClick = { ship -> navController.navigate("ship/${ship.shipId}") },
            )
        }
        composable("ship/{shipId}") { backStackEntry ->
            val shipId = backStackEntry.arguments?.getString("shipId")
            val viewModel by core.viewModel.collectAsState()
            val ship = viewModel.player?.ships?.firstOrNull { it.shipId.toString() == shipId }
            if (ship != null) {
                ShipDetailScreen(ship, onBack = { navController.popBackStack() })
            }
        }
    }
}
