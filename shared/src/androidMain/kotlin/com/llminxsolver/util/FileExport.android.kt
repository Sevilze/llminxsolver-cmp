package com.llminxsolver.util

import android.content.Context
import android.graphics.Bitmap
import android.os.Environment
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asAndroidBitmap
import java.io.ByteArrayOutputStream
import java.io.File

actual fun getDownloadDirectory(): String =
    Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS).absolutePath

actual fun exportCsv(filename: String, content: String): Boolean = try {
    val downloadDir = File(getDownloadDirectory())
    if (!downloadDir.exists()) downloadDir.mkdirs()
    val file = File(downloadDir, filename)
    file.writeText(content, Charsets.UTF_8)
    true
} catch (e: Exception) {
    appContext?.let { context ->
        try {
            val file = File(context.getExternalFilesDir(Environment.DIRECTORY_DOWNLOADS), filename)
            file.writeText(content, Charsets.UTF_8)
            true
        } catch (_: Exception) {
            false
        }
    } ?: false
}

actual fun exportPng(filename: String, pngBytes: ByteArray): Boolean = try {
    val downloadDir = File(getDownloadDirectory())
    if (!downloadDir.exists()) downloadDir.mkdirs()
    val file = File(downloadDir, filename)
    file.writeBytes(pngBytes)
    true
} catch (e: Exception) {
    appContext?.let { context ->
        try {
            val file = File(context.getExternalFilesDir(Environment.DIRECTORY_DOWNLOADS), filename)
            file.writeBytes(pngBytes)
            true
        } catch (_: Exception) {
            false
        }
    } ?: false
}

actual fun imageBitmapToPng(bitmap: ImageBitmap): ByteArray {
    val androidBitmap = bitmap.asAndroidBitmap()
    val outputStream = ByteArrayOutputStream()
    androidBitmap.compress(Bitmap.CompressFormat.PNG, 100, outputStream)
    return outputStream.toByteArray()
}

private var appContext: Context? = null

fun initializeFileExport(context: Context) {
    appContext = context.applicationContext
}
