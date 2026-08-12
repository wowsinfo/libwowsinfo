package com.wowsinfo.libwowsinfo.core

import android.util.Log
import com.novi.serde.Bytes
import com.wowsinfo.libwowsinfo.HttpError
import com.wowsinfo.libwowsinfo.HttpHeader
import com.wowsinfo.libwowsinfo.HttpRequest
import com.wowsinfo.libwowsinfo.HttpResponse
import com.wowsinfo.libwowsinfo.HttpResult
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import java.net.SocketTimeoutException
import java.net.UnknownHostException
import java.util.concurrent.TimeUnit

class HttpHandler {
    private val client = OkHttpClient.Builder()
        .connectTimeout(15, TimeUnit.SECONDS)
        .readTimeout(30, TimeUnit.SECONDS)
        .writeTimeout(30, TimeUnit.SECONDS)
        .build()

    suspend fun request(op: HttpRequest): HttpResult = withContext(Dispatchers.IO) {
        Log.d(TAG, "${op.method} ${op.url}")
        try {
            val body = when {
                op.body.content.isNotEmpty() -> op.body.content.toRequestBody()
                op.method.uppercase() in BODY_REQUIRED_METHODS -> ByteArray(0).toRequestBody()
                else -> null
            }
            val request = Request.Builder()
                .url(op.url)
                .method(op.method, body)
                .apply { op.headers.forEach { addHeader(it.name, it.value) } }
                .build()

            client.newCall(request).execute().use { response ->
                val headers = response.headers.toList().map { (name, value) ->
                    HttpHeader(name, value)
                }
                val responseBody = response.body?.bytes() ?: ByteArray(0)
                HttpResult.Ok(HttpResponse(response.code.toUShort(), headers, Bytes(responseBody)))
            }
        } catch (e: SocketTimeoutException) {
            Log.d(TAG, "timeout: ${op.url}")
            HttpResult.Err(HttpError.Timeout)
        } catch (e: UnknownHostException) {
            Log.d(TAG, "unknown host: ${op.url}")
            HttpResult.Err(HttpError.Io("Unknown host: ${e.message}"))
        } catch (e: IllegalArgumentException) {
            Log.w(TAG, "invalid URL ${op.url}: ${e.message}")
            HttpResult.Err(HttpError.Url(e.message ?: "Invalid URL"))
        } catch (e: Exception) {
            Log.w(TAG, "request failed for ${op.url}: ${e.message}")
            HttpResult.Err(HttpError.Io(e.message ?: "IO error"))
        }
    }

    companion object {
        private const val TAG = "HttpHandler"
        private val BODY_REQUIRED_METHODS = setOf("POST", "PUT", "PATCH")
    }
}
