package com.wowsinfo.libwowsinfo.core

import android.util.Log
import com.wowsinfo.libwowsinfo.Duration
import com.wowsinfo.libwowsinfo.Instant
import com.wowsinfo.libwowsinfo.TimeRequest
import com.wowsinfo.libwowsinfo.TimeResponse
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

class TimeHandler {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main.immediate)
    private val activeTimers = mutableMapOf<ULong, Job>()

    fun handle(
        request: TimeRequest,
        requestId: UInt,
        resolve: suspend (UInt, ByteArray) -> Unit,
    ) {
        Log.d(TAG, "handle: $request")
        when (request) {
            is TimeRequest.Now -> {
                val now = System.currentTimeMillis()
                val response = TimeResponse.Now(
                    Instant(
                        seconds = (now / 1000).toULong(),
                        nanos = ((now % 1000) * 1_000_000).toUInt(),
                    ),
                )
                scope.launch { resolve(requestId, response.bincodeSerialize()) }
            }

            is TimeRequest.NotifyAt -> {
                val targetMs = request.instant.seconds.toLong() * 1000 +
                    request.instant.nanos.toLong() / 1_000_000
                val delayMs = (targetMs - System.currentTimeMillis()).coerceAtLeast(0)
                val timerId = request.id.value
                activeTimers[timerId] = scope.launch {
                    delay(delayMs)
                    activeTimers.remove(timerId)
                    resolve(requestId, TimeResponse.InstantArrived(request.id).bincodeSerialize())
                }
            }

            is TimeRequest.NotifyAfter -> {
                val delayMs = request.duration.nanos / 1_000_000u
                val timerId = request.id.value
                activeTimers[timerId] = scope.launch {
                    delay(delayMs.toLong())
                    activeTimers.remove(timerId)
                    resolve(requestId, TimeResponse.DurationElapsed(request.id).bincodeSerialize())
                }
            }

            is TimeRequest.Clear -> {
                val timerId = request.id.value
                activeTimers.remove(timerId)?.cancel()
                scope.launch {
                    resolve(requestId, TimeResponse.Cleared(request.id).bincodeSerialize())
                }
            }
        }
    }

    companion object {
        private const val TAG = "TimeHandler"
    }
}
