package com.llminxsolver.data

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.intPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import com.llminxsolver.theme.MegaminxColorScheme
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private val Context.dataStore: DataStore<Preferences> by preferencesDataStore(
    name = "llminx_settings"
)

private var appContext: Context? = null

fun initDataStore(context: Context) {
    appContext = context.applicationContext
}

private object PreferencesKeys {
    val MEMORY_BUDGET_MB = intPreferencesKey("memory_budget_mb")
    val TABLE_GEN_THREADS = intPreferencesKey("table_gen_threads")
    val SEARCH_THREADS = intPreferencesKey("search_threads")
    val DEFAULT_PRUNING_DEPTH = intPreferencesKey("default_pruning_depth")
    val SKIP_DELETION_WARNING = booleanPreferencesKey("skip_deletion_warning")
    val USE_DYNAMIC_COLORS = booleanPreferencesKey("use_dynamic_colors")
    val DYNAMIC_COLOR_MODE = stringPreferencesKey("dynamic_color_mode")
    val SCHEME_TYPE = stringPreferencesKey("scheme_type")
    val THEME_MODE = stringPreferencesKey("theme_mode")

    val MEGAMINX_U_FACE = stringPreferencesKey("megaminx_u_face")
    val MEGAMINX_F_FACE = stringPreferencesKey("megaminx_f_face")
    val MEGAMINX_L_FACE = stringPreferencesKey("megaminx_l_face")
    val MEGAMINX_BL_FACE = stringPreferencesKey("megaminx_bl_face")
    val MEGAMINX_BR_FACE = stringPreferencesKey("megaminx_br_face")
    val MEGAMINX_R_FACE = stringPreferencesKey("megaminx_r_face")
}

actual fun createPlatformSettingsRepository(): PlatformSettingsRepository =
    DataStoreSettingsRepository()

class DataStoreSettingsRepository : PlatformSettingsRepository {
    private val dataStore: DataStore<Preferences>
        get() = appContext?.dataStore
            ?: throw IllegalStateException("DataStore not initialized. Call initDataStore first.")

    override val settings: Flow<AppSettings> = dataStore.data.map { prefs ->
        val colorScheme = MegaminxColorScheme(
            uFace = prefs[PreferencesKeys.MEGAMINX_U_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().uFace,
            fFace = prefs[PreferencesKeys.MEGAMINX_F_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().fFace,
            lFace = prefs[PreferencesKeys.MEGAMINX_L_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().lFace,
            blFace = prefs[PreferencesKeys.MEGAMINX_BL_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().blFace,
            brFace = prefs[PreferencesKeys.MEGAMINX_BR_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().brFace,
            rFace = prefs[PreferencesKeys.MEGAMINX_R_FACE]
                ?.let { hexStringToColor(it) }
                ?: MegaminxColorScheme().rFace
        )

        AppSettings(
            memoryBudgetMb = prefs[PreferencesKeys.MEMORY_BUDGET_MB]
                ?: AppSettings.Default.memoryBudgetMb,
            tableGenThreads = prefs[PreferencesKeys.TABLE_GEN_THREADS]
                ?: AppSettings.Default.tableGenThreads,
            searchThreads = prefs[PreferencesKeys.SEARCH_THREADS]
                ?: AppSettings.Default.searchThreads,
            defaultPruningDepth = prefs[PreferencesKeys.DEFAULT_PRUNING_DEPTH]
                ?: AppSettings.Default.defaultPruningDepth,
            skipDeletionWarning = prefs[PreferencesKeys.SKIP_DELETION_WARNING]
                ?: AppSettings.Default.skipDeletionWarning,
            megaminxColorScheme = colorScheme,
            useDynamicColors = prefs[PreferencesKeys.USE_DYNAMIC_COLORS]
                ?: AppSettings.Default.useDynamicColors,
            dynamicColorMode = prefs[PreferencesKeys.DYNAMIC_COLOR_MODE]
                ?.let { DynamicColorMode.entries.find { mode -> mode.name == it } }
                ?: DynamicColorMode.BuiltIn,
            schemeType = prefs[PreferencesKeys.SCHEME_TYPE]
                ?.let { SchemeType.entries.find { type -> type.name == it } }
                ?: SchemeType.TonalSpot,
            themeMode = prefs[PreferencesKeys.THEME_MODE]
                ?.let { ThemeMode.entries.find { mode -> mode.name == it } }
                ?: ThemeMode.System
        )
    }

    override suspend fun updateSettings(transform: (AppSettings) -> AppSettings) {
        dataStore.edit { prefs ->
            val colorScheme = MegaminxColorScheme(
                uFace = prefs[PreferencesKeys.MEGAMINX_U_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().uFace,
                fFace = prefs[PreferencesKeys.MEGAMINX_F_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().fFace,
                lFace = prefs[PreferencesKeys.MEGAMINX_L_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().lFace,
                blFace = prefs[PreferencesKeys.MEGAMINX_BL_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().blFace,
                brFace = prefs[PreferencesKeys.MEGAMINX_BR_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().brFace,
                rFace = prefs[PreferencesKeys.MEGAMINX_R_FACE]
                    ?.let { hexStringToColor(it) }
                    ?: MegaminxColorScheme().rFace
            )

            val currentSettings = AppSettings(
                memoryBudgetMb = prefs[PreferencesKeys.MEMORY_BUDGET_MB]
                    ?: AppSettings.Default.memoryBudgetMb,
                tableGenThreads = prefs[PreferencesKeys.TABLE_GEN_THREADS]
                    ?: AppSettings.Default.tableGenThreads,
                searchThreads = prefs[PreferencesKeys.SEARCH_THREADS]
                    ?: AppSettings.Default.searchThreads,
                defaultPruningDepth = prefs[PreferencesKeys.DEFAULT_PRUNING_DEPTH]
                    ?: AppSettings.Default.defaultPruningDepth,
                skipDeletionWarning = prefs[PreferencesKeys.SKIP_DELETION_WARNING]
                    ?: AppSettings.Default.skipDeletionWarning,
                megaminxColorScheme = colorScheme,
                useDynamicColors = prefs[PreferencesKeys.USE_DYNAMIC_COLORS]
                    ?: AppSettings.Default.useDynamicColors,
                dynamicColorMode = prefs[PreferencesKeys.DYNAMIC_COLOR_MODE]
                    ?.let { DynamicColorMode.entries.find { mode -> mode.name == it } }
                    ?: DynamicColorMode.BuiltIn,
                schemeType = prefs[PreferencesKeys.SCHEME_TYPE]
                    ?.let { SchemeType.entries.find { type -> type.name == it } }
                    ?: SchemeType.TonalSpot,
                themeMode = prefs[PreferencesKeys.THEME_MODE]
                    ?.let { ThemeMode.entries.find { mode -> mode.name == it } }
                    ?: ThemeMode.System
            )

            val newSettings = transform(currentSettings)

            prefs[PreferencesKeys.MEMORY_BUDGET_MB] = newSettings.memoryBudgetMb
            prefs[PreferencesKeys.TABLE_GEN_THREADS] = newSettings.tableGenThreads
            prefs[PreferencesKeys.SEARCH_THREADS] = newSettings.searchThreads
            prefs[PreferencesKeys.DEFAULT_PRUNING_DEPTH] = newSettings.defaultPruningDepth
            prefs[PreferencesKeys.SKIP_DELETION_WARNING] = newSettings.skipDeletionWarning
            prefs[PreferencesKeys.USE_DYNAMIC_COLORS] = newSettings.useDynamicColors
            prefs[PreferencesKeys.DYNAMIC_COLOR_MODE] = newSettings.dynamicColorMode.name
            prefs[PreferencesKeys.SCHEME_TYPE] = newSettings.schemeType.name
            prefs[PreferencesKeys.THEME_MODE] = newSettings.themeMode.name
            prefs[PreferencesKeys.MEGAMINX_U_FACE] =
                newSettings.megaminxColorScheme.uFace.toHexString()
            prefs[PreferencesKeys.MEGAMINX_F_FACE] =
                newSettings.megaminxColorScheme.fFace.toHexString()
            prefs[PreferencesKeys.MEGAMINX_L_FACE] =
                newSettings.megaminxColorScheme.lFace.toHexString()
            prefs[PreferencesKeys.MEGAMINX_BL_FACE] =
                newSettings.megaminxColorScheme.blFace.toHexString()
            prefs[PreferencesKeys.MEGAMINX_BR_FACE] =
                newSettings.megaminxColorScheme.brFace.toHexString()
            prefs[PreferencesKeys.MEGAMINX_R_FACE] =
                newSettings.megaminxColorScheme.rFace.toHexString()
        }
    }
}
