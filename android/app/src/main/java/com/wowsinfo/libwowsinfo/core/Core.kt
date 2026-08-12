package com.wowsinfo.libwowsinfo.core

import android.util.Log
import com.wowsinfo.libwowsinfo.Config
import com.wowsinfo.libwowsinfo.CoreFfi
import com.wowsinfo.libwowsinfo.Effect
import com.wowsinfo.libwowsinfo.Event
import com.wowsinfo.libwowsinfo.Request
import com.wowsinfo.libwowsinfo.Requests
import com.wowsinfo.libwowsinfo.Server
import com.wowsinfo.libwowsinfo.ViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * Owns the Rust core and runs the effect loop. UI events are serialised and
 * handed across the JNI bridge; every effect the core requests is dispatched
 * to a handler and resolved back through `CoreFfi.resolve`.
 */
class Core(
    private val httpHandler: HttpHandler,
    private val keyValueHandler: KeyValueHandler,
    private val timeHandler: TimeHandler,
) {
    private val coreFfi = CoreFfi()
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)

    private val _viewModel = MutableStateFlow(getViewModel())
    val viewModel: StateFlow<ViewModel> = _viewModel.asStateFlow()

    init {
        update(Event.Init(Config(Server.ASIA, "en", "")))
    }

    fun update(event: Event) {
        Log.d(TAG, "update: $event")
        scope.launch {
            handleEffects(coreFfi.update(event.bincodeSerialize()))
        }
    }

    private suspend fun handleEffects(bytes: ByteArray) {
        if (bytes.isEmpty()) {
            Log.d(TAG, "handleEffects: no effects")
            return
        }
        Requests.bincodeDeserialize(bytes).value.forEach { request ->
            processRequest(request)
        }
    }

    private suspend fun processRequest(request: Request) {
        Log.d(TAG, "processRequest: $request")
        when (val effect = request.effect) {
            is Effect.Http ->
                resolveAndHandleEffects(request.id, httpHandler.request(effect.value).bincodeSerialize())

            is Effect.KeyValue ->
                resolveAndHandleEffects(request.id, keyValueHandler.handle(effect.value).bincodeSerialize())

            is Effect.Time ->
                timeHandler.handle(effect.value, request.id, ::resolveAndHandleEffects)

            is Effect.Render ->
                render()
        }
    }

    private suspend fun resolveAndHandleEffects(requestId: UInt, data: ByteArray) {
        Log.d(TAG, "resolveAndHandleEffects: $requestId")
        handleEffects(coreFfi.resolve(requestId, data))
    }

    private fun render() {
        _viewModel.value = getViewModel().also {
            Log.d(TAG, "render: $it")
        }
    }

    private fun getViewModel(): ViewModel = ViewModel.bincodeDeserialize(coreFfi.view())

    companion object {
        private const val TAG = "Core"
    }
}
