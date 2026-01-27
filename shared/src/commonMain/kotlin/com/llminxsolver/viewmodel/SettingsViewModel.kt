package com.llminxsolver.viewmodel

import com.llminxsolver.data.DynamicColorMode
import com.llminxsolver.data.ParallelConfig
import com.llminxsolver.data.SchemeType
import com.llminxsolver.data.ThemeMode
import com.llminxsolver.data.createPlatformSettingsRepository
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

class SettingsViewModel(private val scope: CoroutineScope) {
    private val settingsRepository = createPlatformSettingsRepository()

    private val _megaminxColorScheme = MutableStateFlow(MegaminxColorScheme())
    val megaminxColorScheme: StateFlow<MegaminxColorScheme> = _megaminxColorScheme.asStateFlow()

    private val _skipDeletionWarning = MutableStateFlow(false)
    val skipDeletionWarning: StateFlow<Boolean> = _skipDeletionWarning.asStateFlow()

    private val _wallpaperPath = MutableStateFlow<String?>(null)
    val wallpaperPath: StateFlow<String?> = _wallpaperPath.asStateFlow()

    private val _dynamicColorMode = MutableStateFlow(DynamicColorMode.BuiltIn)
    val dynamicColorMode: StateFlow<DynamicColorMode> = _dynamicColorMode.asStateFlow()

    private val _schemeType = MutableStateFlow(SchemeType.TonalSpot)
    val schemeType: StateFlow<SchemeType> = _schemeType.asStateFlow()

    private val _themeMode = MutableStateFlow(ThemeMode.System)
    val themeMode: StateFlow<ThemeMode> = _themeMode.asStateFlow()

    private val _isLoaded = MutableStateFlow(false)
    val isLoaded: StateFlow<Boolean> = _isLoaded.asStateFlow()

    fun collectSettings(onConfigUpdate: (Int, Int, Int, Int) -> Unit) {
        settingsRepository.settings
            .onEach { settings ->
                _megaminxColorScheme.value = settings.megaminxColorScheme
                _skipDeletionWarning.value = settings.skipDeletionWarning
                _wallpaperPath.value = settings.wallpaperPath
                _dynamicColorMode.value = settings.dynamicColorMode
                _schemeType.value = settings.schemeType
                _themeMode.value = settings.themeMode
                _isLoaded.value = true
                onConfigUpdate(
                    settings.memoryBudgetMb,
                    settings.tableGenThreads,
                    settings.searchThreads,
                    settings.defaultPruningDepth
                )
            }
            .launchIn(scope)
    }

    fun setMegaminxColorScheme(scheme: MegaminxColorScheme) {
        _megaminxColorScheme.value = scheme
        scope.launch {
            settingsRepository.updateSettings { it.copy(megaminxColorScheme = scheme) }
        }
    }

    fun setSkipDeletionWarning(skip: Boolean) {
        _skipDeletionWarning.value = skip
        scope.launch {
            settingsRepository.updateSettings { it.copy(skipDeletionWarning = skip) }
        }
    }

    fun setWallpaperPath(path: String?) {
        _wallpaperPath.value = path
        scope.launch {
            settingsRepository.updateSettings { it.copy(wallpaperPath = path) }
        }
    }

    fun setDynamicColorMode(mode: DynamicColorMode) {
        _dynamicColorMode.value = mode
        scope.launch {
            settingsRepository.updateSettings { it.copy(dynamicColorMode = mode) }
        }
    }

    fun setSchemeType(type: SchemeType) {
        _schemeType.value = type
        scope.launch {
            settingsRepository.updateSettings { it.copy(schemeType = type) }
        }
    }

    fun setThemeMode(mode: ThemeMode) {
        _themeMode.value = mode
        scope.launch {
            settingsRepository.updateSettings { it.copy(themeMode = mode) }
        }
    }

    fun saveParallelConfig(config: ParallelConfig) {
        scope.launch {
            settingsRepository.updateSettings {
                it.copy(
                    memoryBudgetMb = config.memoryBudgetMb,
                    tableGenThreads = config.tableGenThreads,
                    searchThreads = config.searchThreads
                )
            }
        }
    }

    fun savePruningDepth(depth: Int) {
        scope.launch {
            settingsRepository.updateSettings {
                it.copy(defaultPruningDepth = depth)
            }
        }
    }
}
