package com.wowsinfo.libwowsinfo.core

import android.content.Context
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.first

private val Context.dataStore by preferencesDataStore(name = "wowsinfo")

/**
 * Preferences-backed key/value store. The core stores UTF-8 strings (JSON or
 * small numbers), so values round-trip losslessly as string preferences.
 */
class KeyValueDataStoreDelegate(private val context: Context) {

    suspend fun get(key: String): ByteArray? =
        context.dataStore.data.first()[stringPreferencesKey(key)]?.encodeToByteArray()

    suspend fun set(key: String, value: ByteArray): ByteArray? {
        var previous: ByteArray? = null
        context.dataStore.edit { prefs ->
            val prefKey = stringPreferencesKey(key)
            previous = prefs[prefKey]?.encodeToByteArray()
            prefs[prefKey] = value.decodeToString()
        }
        return previous
    }

    suspend fun delete(key: String): ByteArray? {
        var previous: ByteArray? = null
        context.dataStore.edit { prefs ->
            val prefKey = stringPreferencesKey(key)
            previous = prefs[prefKey]?.encodeToByteArray()
            prefs.remove(prefKey)
        }
        return previous
    }

    suspend fun exists(key: String): Boolean =
        context.dataStore.data.first().contains(stringPreferencesKey(key))

    suspend fun listKeys(prefix: String): List<String> =
        context.dataStore.data.first()
            .asMap()
            .keys
            .map { it.name }
            .filter { it.startsWith(prefix) }
}
