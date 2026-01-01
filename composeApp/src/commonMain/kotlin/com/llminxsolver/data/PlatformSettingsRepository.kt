package com.llminxsolver.data

import kotlinx.coroutines.flow.Flow

interface PlatformSettingsRepository {
    val settings: Flow<AppSettings>
    suspend fun updateSettings(transform: (AppSettings) -> AppSettings)
}

expect fun createPlatformSettingsRepository(): PlatformSettingsRepository
