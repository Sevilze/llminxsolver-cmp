package com.llminxsolver.theme

import androidx.compose.material3.ColorScheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.Color
import java.io.File

private data class MatugenColors(
    val primary: Color,
    val onPrimary: Color,
    val primaryContainer: Color,
    val onPrimaryContainer: Color,
    val secondary: Color,
    val onSecondary: Color,
    val secondaryContainer: Color,
    val onSecondaryContainer: Color,
    val tertiary: Color,
    val onTertiary: Color,
    val tertiaryContainer: Color,
    val onTertiaryContainer: Color,
    val error: Color,
    val onError: Color,
    val errorContainer: Color,
    val onErrorContainer: Color,
    val background: Color,
    val onBackground: Color,
    val surface: Color,
    val onSurface: Color,
    val surfaceVariant: Color,
    val onSurfaceVariant: Color,
    val outline: Color,
    val outlineVariant: Color,
    val inverseSurface: Color,
    val inverseOnSurface: Color,
    val inversePrimary: Color,
    val surfaceTint: Color
)

private fun parseHexColor(hex: String): Color? = try {
    val cleanHex = hex.removePrefix("#")
    val colorInt = cleanHex.toLong(16)
    when (cleanHex.length) {
        6 -> Color(0xFF000000 or colorInt)
        8 -> Color(colorInt)
        else -> null
    }
} catch (_: Exception) {
    null
}

private fun parseMatugenColorsFromJson(json: String, isDark: Boolean): MatugenColors? {
    return try {
        val schemeKey = if (isDark) "dark" else "light"
        val schemePattern = """"$schemeKey"\s*:\s*\{([^}]+)\}""".toRegex()
        val schemeMatch = schemePattern.find(json) ?: return null
        val schemeContent = schemeMatch.groupValues[1]

        fun extractColor(key: String): Color {
            val pattern = """"$key"\s*:\s*"([^"]+)"""".toRegex()
            val match = pattern.find(schemeContent)
            return match?.groupValues?.get(1)?.let { parseHexColor(it) } ?: Color.Magenta
        }

        MatugenColors(
            primary = extractColor("primary"),
            onPrimary = extractColor("on_primary"),
            primaryContainer = extractColor("primary_container"),
            onPrimaryContainer = extractColor("on_primary_container"),
            secondary = extractColor("secondary"),
            onSecondary = extractColor("on_secondary"),
            secondaryContainer = extractColor("secondary_container"),
            onSecondaryContainer = extractColor("on_secondary_container"),
            tertiary = extractColor("tertiary"),
            onTertiary = extractColor("on_tertiary"),
            tertiaryContainer = extractColor("tertiary_container"),
            onTertiaryContainer = extractColor("on_tertiary_container"),
            error = extractColor("error"),
            onError = extractColor("on_error"),
            errorContainer = extractColor("error_container"),
            onErrorContainer = extractColor("on_error_container"),
            background = extractColor("background"),
            onBackground = extractColor("on_background"),
            surface = extractColor("surface"),
            onSurface = extractColor("on_surface"),
            surfaceVariant = extractColor("surface_variant"),
            onSurfaceVariant = extractColor("on_surface_variant"),
            outline = extractColor("outline"),
            outlineVariant = extractColor("outline_variant"),
            inverseSurface = extractColor("inverse_surface"),
            inverseOnSurface = extractColor("inverse_on_surface"),
            inversePrimary = extractColor("inverse_primary"),
            surfaceTint = extractColor("primary")
        )
    } catch (_: Exception) {
        null
    }
}

private fun matugenColorsToColorScheme(colors: MatugenColors, isDark: Boolean): ColorScheme =
    if (isDark) {
        darkColorScheme(
            primary = colors.primary,
            onPrimary = colors.onPrimary,
            primaryContainer = colors.primaryContainer,
            onPrimaryContainer = colors.onPrimaryContainer,
            secondary = colors.secondary,
            onSecondary = colors.onSecondary,
            secondaryContainer = colors.secondaryContainer,
            onSecondaryContainer = colors.onSecondaryContainer,
            tertiary = colors.tertiary,
            onTertiary = colors.onTertiary,
            tertiaryContainer = colors.tertiaryContainer,
            onTertiaryContainer = colors.onTertiaryContainer,
            error = colors.error,
            onError = colors.onError,
            errorContainer = colors.errorContainer,
            onErrorContainer = colors.onErrorContainer,
            background = colors.background,
            onBackground = colors.onBackground,
            surface = colors.surface,
            onSurface = colors.onSurface,
            surfaceVariant = colors.surfaceVariant,
            onSurfaceVariant = colors.onSurfaceVariant,
            outline = colors.outline,
            outlineVariant = colors.outlineVariant,
            inverseSurface = colors.inverseSurface,
            inverseOnSurface = colors.inverseOnSurface,
            inversePrimary = colors.inversePrimary,
            surfaceTint = colors.surfaceTint
        )
    } else {
        lightColorScheme(
            primary = colors.primary,
            onPrimary = colors.onPrimary,
            primaryContainer = colors.primaryContainer,
            onPrimaryContainer = colors.onPrimaryContainer,
            secondary = colors.secondary,
            onSecondary = colors.onSecondary,
            secondaryContainer = colors.secondaryContainer,
            onSecondaryContainer = colors.onSecondaryContainer,
            tertiary = colors.tertiary,
            onTertiary = colors.onTertiary,
            tertiaryContainer = colors.tertiaryContainer,
            onTertiaryContainer = colors.onTertiaryContainer,
            error = colors.error,
            onError = colors.onError,
            errorContainer = colors.errorContainer,
            onErrorContainer = colors.onErrorContainer,
            background = colors.background,
            onBackground = colors.onBackground,
            surface = colors.surface,
            onSurface = colors.onSurface,
            surfaceVariant = colors.surfaceVariant,
            onSurfaceVariant = colors.onSurfaceVariant,
            outline = colors.outline,
            outlineVariant = colors.outlineVariant,
            inverseSurface = colors.inverseSurface,
            inverseOnSurface = colors.inverseOnSurface,
            inversePrimary = colors.inversePrimary,
            surfaceTint = colors.surfaceTint
        )
    }

@Composable
actual fun getDynamicColorScheme(darkTheme: Boolean): ColorScheme? = remember(darkTheme) {
    try {
        val homeDir = System.getProperty("user.home")
        val colorsFile = File(homeDir, ".cache/matugen/colors.json")
        if (colorsFile.exists()) {
            val json = colorsFile.readText()
            parseMatugenColorsFromJson(json, darkTheme)?.let {
                matugenColorsToColorScheme(it, darkTheme)
            }
        } else {
            null
        }
    } catch (_: Exception) {
        null
    }
}
