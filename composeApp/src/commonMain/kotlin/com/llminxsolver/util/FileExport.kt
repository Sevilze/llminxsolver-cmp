package com.llminxsolver.util

import androidx.compose.ui.graphics.ImageBitmap

expect fun getDownloadDirectory(): String

expect fun exportCsv(filename: String, content: String): Boolean

expect fun exportPng(filename: String, pngBytes: ByteArray): Boolean

expect fun imageBitmapToPng(bitmap: ImageBitmap): ByteArray
