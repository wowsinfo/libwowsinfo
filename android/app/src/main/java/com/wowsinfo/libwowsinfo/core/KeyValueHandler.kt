package com.wowsinfo.libwowsinfo.core

import com.wowsinfo.libwowsinfo.KeyValueOperation
import com.wowsinfo.libwowsinfo.KeyValueResponse
import com.wowsinfo.libwowsinfo.KeyValueResult
import com.wowsinfo.libwowsinfo.Value
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class KeyValueHandler(private val delegate: KeyValueDataStoreDelegate) {

    suspend fun handle(operation: KeyValueOperation): KeyValueResult =
        withContext(Dispatchers.IO) {
            when (operation) {
                is KeyValueOperation.Get -> get(operation)
                is KeyValueOperation.Set -> set(operation)
                is KeyValueOperation.Delete -> delete(operation)
                is KeyValueOperation.Exists -> exists(operation)
                is KeyValueOperation.ListKeys -> listKeys(operation)
            }
        }

    private suspend fun get(operation: KeyValueOperation.Get): KeyValueResult =
        KeyValueResult.Ok(KeyValueResponse.Get(delegate.get(operation.key).toValue()))

    private suspend fun set(operation: KeyValueOperation.Set): KeyValueResult =
        KeyValueResult.Ok(
            KeyValueResponse.Set(delegate.set(operation.key, operation.value.toByteArray()).toValue()),
        )

    private suspend fun delete(operation: KeyValueOperation.Delete): KeyValueResult =
        KeyValueResult.Ok(KeyValueResponse.Delete(delegate.delete(operation.key).toValue()))

    private suspend fun exists(operation: KeyValueOperation.Exists): KeyValueResult =
        KeyValueResult.Ok(KeyValueResponse.Exists(delegate.exists(operation.key)))

    private suspend fun listKeys(operation: KeyValueOperation.ListKeys): KeyValueResult =
        KeyValueResult.Ok(KeyValueResponse.ListKeys(delegate.listKeys(operation.prefix), 0UL))

    private fun List<UByte>.toByteArray(): ByteArray =
        ByteArray(size) { index -> this[index].toByte() }

    private fun ByteArray?.toValue(): Value =
        this?.takeIf { it.isNotEmpty() }
            ?.let { Value.Bytes(it.toList().map(Byte::toUByte)) }
            ?: Value.None
}
