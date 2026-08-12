package com.wowsinfo.libwowsinfo

import android.app.Application
import com.wowsinfo.libwowsinfo.core.Core
import com.wowsinfo.libwowsinfo.core.HttpHandler
import com.wowsinfo.libwowsinfo.core.KeyValueDataStoreDelegate
import com.wowsinfo.libwowsinfo.core.KeyValueHandler
import com.wowsinfo.libwowsinfo.core.TimeHandler

class WoWsInfoApp : Application() {
    val core: Core by lazy {
        Core(
            httpHandler = HttpHandler(),
            keyValueHandler = KeyValueHandler(KeyValueDataStoreDelegate(this)),
            timeHandler = TimeHandler(),
        )
    }
}
