package com.llminxsolver.theme

import android.app.WallpaperManager
import android.content.Context
import android.graphics.Bitmap
import android.graphics.drawable.BitmapDrawable
import android.os.Build
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import com.llminxsolver.data.DynamicColorMode
import com.llminxsolver.data.SchemeType
import java.io.File
import java.io.FileOutputStream

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

private fun getWallpaperBitmap(context: Context): Bitmap? {
    val wallpaperManager = WallpaperManager.getInstance(context)

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
        try {
            val fd = wallpaperManager.getWallpaperFile(WallpaperManager.FLAG_SYSTEM)
            if (fd != null) {
                val bitmap = android.graphics.BitmapFactory.decodeFileDescriptor(fd.fileDescriptor)
                fd.close()
                if (bitmap != null) return bitmap
            }
        } catch (_: SecurityException) {
        } catch (_: Exception) {
        }
    }

    return try {
        val drawable = wallpaperManager.drawable ?: return null

        when (drawable) {
            is BitmapDrawable -> drawable.bitmap

            else -> {
                val width = drawable.intrinsicWidth.takeIf { it > 0 } ?: 512
                val height = drawable.intrinsicHeight.takeIf { it > 0 } ?: 512
                Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888).also { bmp ->
                    val canvas = android.graphics.Canvas(bmp)
                    drawable.setBounds(0, 0, canvas.width, canvas.height)
                    drawable.draw(canvas)
                }
            }
        }
    } catch (_: Exception) {
        null
    }
}

private fun getWallpaperImagePath(context: Context): String? {
    return try {
        val bitmap = getWallpaperBitmap(context) ?: return null

        val scaled = if (bitmap.width > 256 || bitmap.height > 256) {
            val scale = 256f / maxOf(bitmap.width, bitmap.height)
            Bitmap.createScaledBitmap(
                bitmap,
                (bitmap.width * scale).toInt(),
                (bitmap.height * scale).toInt(),
                true
            )
        } else {
            bitmap
        }

        val cacheFile = File(context.cacheDir, "wallpaper_theme.png")
        FileOutputStream(cacheFile).use { out ->
            scaled.compress(Bitmap.CompressFormat.PNG, 100, out)
        }

        if (scaled !== bitmap) {
            scaled.recycle()
        }

        cacheFile.absolutePath
    } catch (_: Exception) {
        null
    }
}

private fun getWallpaperColorsImagePath(context: Context): String? {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O_MR1) return null

    return try {
        val wallpaperManager = WallpaperManager.getInstance(context)
        val colors = wallpaperManager.getWallpaperColors(WallpaperManager.FLAG_SYSTEM)
        val seedArgb = colors?.primaryColor?.toArgb() ?: return null

        val seedBitmap = Bitmap.createBitmap(2, 2, Bitmap.Config.ARGB_8888)
        seedBitmap.eraseColor(seedArgb)

        val cacheFile = File(context.cacheDir, "wallpaper_theme_seed.png")
        FileOutputStream(cacheFile).use { out ->
            seedBitmap.compress(Bitmap.CompressFormat.PNG, 100, out)
        }
        seedBitmap.recycle()

        cacheFile.absolutePath
    } catch (_: Exception) {
        null
    }
}

private fun nativeThemeColorsToScheme(
    colors: uniffi.llminxsolver.ThemeColors,
    isDark: Boolean
): ColorScheme {
    fun parseColor(hex: String, fallback: Color): Color = parseHexColor(hex) ?: fallback

    return if (isDark) {
        darkColorScheme(
            primary = parseColor(colors.primary, Color.Magenta),
            onPrimary = parseColor(colors.onPrimary, Color.White),
            primaryContainer = parseColor(colors.primaryContainer, Color.Magenta),
            onPrimaryContainer = parseColor(colors.onPrimaryContainer, Color.White),
            secondary = parseColor(colors.secondary, Color.Magenta),
            onSecondary = parseColor(colors.onSecondary, Color.White),
            secondaryContainer = parseColor(colors.secondaryContainer, Color.Magenta),
            onSecondaryContainer = parseColor(colors.onSecondaryContainer, Color.White),
            tertiary = parseColor(colors.tertiary, Color.Magenta),
            onTertiary = parseColor(colors.onTertiary, Color.White),
            tertiaryContainer = parseColor(colors.tertiaryContainer, Color.Magenta),
            onTertiaryContainer = parseColor(colors.onTertiaryContainer, Color.White),
            error = parseColor(colors.error, Color.Red),
            onError = parseColor(colors.onError, Color.White),
            errorContainer = parseColor(colors.errorContainer, Color.Red),
            onErrorContainer = parseColor(colors.onErrorContainer, Color.White),
            background = parseColor(colors.background, Color.Black),
            onBackground = parseColor(colors.onBackground, Color.White),
            surface = parseColor(colors.surface, Color.Black),
            onSurface = parseColor(colors.onSurface, Color.White),
            surfaceVariant = parseColor(colors.surfaceVariant, Color.DarkGray),
            onSurfaceVariant = parseColor(colors.onSurfaceVariant, Color.White),
            outline = parseColor(colors.outline, Color.Gray),
            outlineVariant = parseColor(colors.outlineVariant, Color.DarkGray),
            inverseSurface = parseColor(colors.inverseSurface, Color.White),
            inverseOnSurface = parseColor(colors.inverseOnSurface, Color.Black),
            inversePrimary = parseColor(colors.inversePrimary, Color.Magenta),
            surfaceTint = parseColor(colors.surfaceTint, Color.Magenta),
            surfaceDim = parseColor(colors.surfaceDim, Color.DarkGray),
            surfaceBright = parseColor(colors.surfaceBright, Color.Gray),
            surfaceContainerLowest = parseColor(colors.surfaceContainerLowest, Color.Black),
            surfaceContainerLow = parseColor(colors.surfaceContainerLow, Color.DarkGray),
            surfaceContainer = parseColor(colors.surfaceContainer, Color.DarkGray),
            surfaceContainerHigh = parseColor(colors.surfaceContainerHigh, Color.Gray),
            surfaceContainerHighest = parseColor(colors.surfaceContainerHighest, Color.Gray)
        )
    } else {
        lightColorScheme(
            primary = parseColor(colors.primary, Color.Magenta),
            onPrimary = parseColor(colors.onPrimary, Color.White),
            primaryContainer = parseColor(colors.primaryContainer, Color.Magenta),
            onPrimaryContainer = parseColor(colors.onPrimaryContainer, Color.Black),
            secondary = parseColor(colors.secondary, Color.Magenta),
            onSecondary = parseColor(colors.onSecondary, Color.Black),
            secondaryContainer = parseColor(colors.secondaryContainer, Color.Magenta),
            onSecondaryContainer = parseColor(colors.onSecondaryContainer, Color.Black),
            tertiary = parseColor(colors.tertiary, Color.Magenta),
            onTertiary = parseColor(colors.onTertiary, Color.Black),
            tertiaryContainer = parseColor(colors.tertiaryContainer, Color.Magenta),
            onTertiaryContainer = parseColor(colors.onTertiaryContainer, Color.Black),
            error = parseColor(colors.error, Color.Red),
            onError = parseColor(colors.onError, Color.White),
            errorContainer = parseColor(colors.errorContainer, Color.Red),
            onErrorContainer = parseColor(colors.onErrorContainer, Color.Black),
            background = parseColor(colors.background, Color.White),
            onBackground = parseColor(colors.onBackground, Color.Black),
            surface = parseColor(colors.surface, Color.White),
            onSurface = parseColor(colors.onSurface, Color.Black),
            surfaceVariant = parseColor(colors.surfaceVariant, Color.LightGray),
            onSurfaceVariant = parseColor(colors.onSurfaceVariant, Color.Black),
            outline = parseColor(colors.outline, Color.Gray),
            outlineVariant = parseColor(colors.outlineVariant, Color.LightGray),
            inverseSurface = parseColor(colors.inverseSurface, Color.Black),
            inverseOnSurface = parseColor(colors.inverseOnSurface, Color.White),
            inversePrimary = parseColor(colors.inversePrimary, Color.Magenta),
            surfaceTint = parseColor(colors.surfaceTint, Color.Magenta),
            surfaceDim = parseColor(colors.surfaceDim, Color.LightGray),
            surfaceBright = parseColor(colors.surfaceBright, Color.White),
            surfaceContainerLowest = parseColor(colors.surfaceContainerLowest, Color.White),
            surfaceContainerLow = parseColor(colors.surfaceContainerLow, Color.White),
            surfaceContainer = parseColor(colors.surfaceContainer, Color.LightGray),
            surfaceContainerHigh = parseColor(colors.surfaceContainerHigh, Color.LightGray),
            surfaceContainerHighest = parseColor(colors.surfaceContainerHighest, Color.Gray)
        )
    }
}

@Composable
actual fun getDynamicColorScheme(
    darkTheme: Boolean,
    wallpaperPath: String?,
    dynamicColorMode: DynamicColorMode,
    schemeType: SchemeType
): ColorScheme? {
    val context = LocalContext.current

    if (dynamicColorMode == DynamicColorMode.BuiltIn) {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        } else {
            null
        }
    }

    val imagePath = wallpaperPath
        ?: getWallpaperColorsImagePath(context)
        ?: getWallpaperImagePath(context)

    return remember(darkTheme, imagePath, schemeType) {
        try {
            if (imagePath != null) {
                uniffi.llminxsolver.generateThemeFromImage(
                    imagePath,
                    darkTheme,
                    schemeType.toNative()
                )?.let {
                    return@remember nativeThemeColorsToScheme(it, darkTheme)
                }
            }
            null
        } catch (_: Exception) {
            null
        }
    }
}
