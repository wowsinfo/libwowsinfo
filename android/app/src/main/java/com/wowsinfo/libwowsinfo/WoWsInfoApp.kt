package com.wowsinfo.libwowsinfo

import android.app.Application
import kotlin.ExperimentalUnsignedTypes
import com.wowsinfo.libwowsinfo.core.Core
import com.wowsinfo.libwowsinfo.core.HttpHandler
import com.wowsinfo.libwowsinfo.core.KeyValueDataStoreDelegate
import com.wowsinfo.libwowsinfo.core.KeyValueHandler
import com.wowsinfo.libwowsinfo.core.TimeHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class WoWsInfoApp : Application() {
    val core: Core by lazy {
        Core(
            httpHandler = HttpHandler(),
            keyValueHandler = KeyValueHandler(KeyValueDataStoreDelegate(this)),
            timeHandler = TimeHandler(),
        )
    }

    private val appScope = CoroutineScope(SupervisorJob() + Dispatchers.Default)

    override fun onCreate() {
        super.onCreate()
        loadLocalData()
    }

    /** Load the bundled zst assets once at startup and keep the wiki data in memory. */
    @OptIn(ExperimentalUnsignedTypes::class)
    private fun loadLocalData() {
        appScope.launch {
            val ships = withContext(Dispatchers.IO) {
                assets.open("wowsinfo.zst").use { it.readBytes().toUByteArray().toList() }
            }
            val lang = withContext(Dispatchers.IO) {
                assets.open("lang.zst").use { it.readBytes().toUByteArray().toList() }
            }
            core.update(Event.SetLocalData(ships, lang))
            core.update(Event.LoadLocalWarships)
        }
    }
}
