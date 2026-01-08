package com.llminxsolver.util

import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.toAwtImage
import java.io.ByteArrayOutputStream
import java.io.File
import javax.imageio.ImageIO

actual fun getDownloadDirectory(): String {
    val userHome = System.getProperty("user.home")
    val xdgDownload = System.getenv("XDG_DOWNLOAD_DIR")
    return when {
        !xdgDownload.isNullOrBlank() && File(xdgDownload).isDirectory -> xdgDownload
        else -> File(userHome, "Downloads").absolutePath
    }
}

actual fun exportCsv(filename: String, content: String): Boolean = try {
    val downloadDir = File(getDownloadDirectory())
    if (!downloadDir.exists()) downloadDir.mkdirs()
    val file = File(downloadDir, filename)
    file.writeText(content, Charsets.UTF_8)
    true
} catch (e: Exception) {
    e.printStackTrace()
    false
}

actual fun exportPng(filename: String, pngBytes: ByteArray): Boolean = try {
    val downloadDir = File(getDownloadDirectory())
    if (!downloadDir.exists()) downloadDir.mkdirs()
    val file = File(downloadDir, filename)
    file.writeBytes(pngBytes)
    true
} catch (e: Exception) {
    e.printStackTrace()
    false
}

actual fun imageBitmapToPng(bitmap: ImageBitmap): ByteArray {
    val awtImage = bitmap.toAwtImage()
    val outputStream = ByteArrayOutputStream()
    ImageIO.write(awtImage, "png", outputStream)
    return outputStream.toByteArray()
}
