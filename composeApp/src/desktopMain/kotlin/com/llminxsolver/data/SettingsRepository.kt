package com.llminxsolver.data

import com.llminxsolver.theme.MegaminxColorScheme
import java.io.File
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
private data class SettingsJson(
    val memoryBudgetMb: Int = 512,
    val tableGenThreads: Int = 4,
    val searchThreads: Int = 4,
    val skipDeletionWarning: Boolean = false,
    val useDynamicColors: Boolean = true,
    val wallpaperPath: String? = null,
    val dynamicColorMode: String = "BuiltIn",
    val schemeType: String = "TonalSpot",
    val themeMode: String = "System",
    val megaminxUFace: String = "#FFE1E100",
    val megaminxFFace: String = "#FFC80000",
    val megaminxLFace: String = "#FFE16400",
    val megaminxBlFace: String = "#FF00C800",
    val megaminxBrFace: String = "#FFFF9696",
    val megaminxRFace: String = "#FF000096"
)

class JsonSettingsRepository : PlatformSettingsRepository {
    private val json = Json {
        prettyPrint = true
        ignoreUnknownKeys = true
    }
    private val settingsFile: File
    private val _settings = MutableStateFlow(AppSettings.Default)
    override val settings: Flow<AppSettings> = _settings.asStateFlow()

    init {
        val userHome = System.getProperty("user.home")
        val dataDir = File(userHome, ".config/llminx-solver")
        dataDir.mkdirs()
        settingsFile = File(dataDir, "settings.json")
        loadSettings()
    }

    private fun loadSettings() {
        try {
            if (settingsFile.exists()) {
                val jsonStr = settingsFile.readText()
                val loaded = json.decodeFromString<SettingsJson>(jsonStr)
                _settings.value = loaded.toAppSettings()
            } else {
                saveSettings(_settings.value)
            }
        } catch (e: Exception) {
            _settings.value = AppSettings.Default
            saveSettings(_settings.value)
        }
    }

    private fun saveSettings(settings: AppSettings) {
        try {
            val jsonData = settings.toSettingsJson()
            settingsFile.writeText(json.encodeToString(jsonData))
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }

    override suspend fun updateSettings(transform: (AppSettings) -> AppSettings) {
        val newSettings = transform(_settings.value)
        _settings.value = newSettings
        saveSettings(newSettings)
    }

    private fun SettingsJson.toAppSettings(): AppSettings {
        val colorScheme = MegaminxColorScheme(
            uFace = hexStringToColor(megaminxUFace)
                ?: MegaminxColorScheme().uFace,
            fFace = hexStringToColor(megaminxFFace)
                ?: MegaminxColorScheme().fFace,
            lFace = hexStringToColor(megaminxLFace)
                ?: MegaminxColorScheme().lFace,
            blFace = hexStringToColor(megaminxBlFace)
                ?: MegaminxColorScheme().blFace,
            brFace = hexStringToColor(megaminxBrFace)
                ?: MegaminxColorScheme().brFace,
            rFace = hexStringToColor(megaminxRFace)
                ?: MegaminxColorScheme().rFace
        )
        return AppSettings(
            memoryBudgetMb = memoryBudgetMb,
            tableGenThreads = tableGenThreads,
            searchThreads = searchThreads,
            skipDeletionWarning = skipDeletionWarning,
            megaminxColorScheme = colorScheme,
            useDynamicColors = useDynamicColors,
            wallpaperPath = wallpaperPath,
            dynamicColorMode = DynamicColorMode.entries.find {
                it.name == dynamicColorMode
            } ?: DynamicColorMode.BuiltIn,
            schemeType = SchemeType.entries.find { it.name == schemeType } ?: SchemeType.TonalSpot,
            themeMode = ThemeMode.entries.find { it.name == themeMode } ?: ThemeMode.System
        )
    }

    private fun AppSettings.toSettingsJson(): SettingsJson = SettingsJson(
        memoryBudgetMb = memoryBudgetMb,
        tableGenThreads = tableGenThreads,
        searchThreads = searchThreads,
        skipDeletionWarning = skipDeletionWarning,
        useDynamicColors = useDynamicColors,
        wallpaperPath = wallpaperPath,
        dynamicColorMode = dynamicColorMode.name,
        schemeType = schemeType.name,
        themeMode = themeMode.name,
        megaminxUFace = megaminxColorScheme.uFace.toHexString(),
        megaminxFFace = megaminxColorScheme.fFace.toHexString(),
        megaminxLFace = megaminxColorScheme.lFace.toHexString(),
        megaminxBlFace = megaminxColorScheme.blFace.toHexString(),
        megaminxBrFace = megaminxColorScheme.brFace.toHexString(),
        megaminxRFace = megaminxColorScheme.rFace.toHexString()
    )
}

actual fun createPlatformSettingsRepository(): PlatformSettingsRepository = JsonSettingsRepository()
