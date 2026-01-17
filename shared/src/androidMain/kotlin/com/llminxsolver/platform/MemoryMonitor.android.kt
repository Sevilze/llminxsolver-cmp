package com.llminxsolver.platform

import android.app.ActivityManager
import android.content.Context
import android.os.Debug
import android.os.Handler
import android.os.Looper

actual class MemoryMonitor actual constructor() {
    private var handler: Handler? = null
    private var runnable: Runnable? = null
    private var isMonitoring = false

    actual fun getMemoryInfo(): MemoryInfo {
        val context = appContext ?: return MemoryInfo(0, 0, 0f, 0)
        val activityManager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
        val memInfo = ActivityManager.MemoryInfo()
        activityManager.getMemoryInfo(memInfo)

        val used = memInfo.totalMem - memInfo.availMem

        val appMemory = getAppMemory()

        return MemoryInfo(
            usedMemoryBytes = used,
            totalMemoryBytes = memInfo.totalMem,
            usagePercent = used.toFloat() / memInfo.totalMem,
            appMemoryBytes = appMemory
        )
    }

    private fun getAppMemory(): Long = try {
        val memoryInfo = Debug.MemoryInfo()
        Debug.getMemoryInfo(memoryInfo)
        memoryInfo.totalPss * 1024L
    } catch (e: Exception) {
        val runtime = Runtime.getRuntime()
        runtime.totalMemory() - runtime.freeMemory()
    }

    actual fun startMonitoring(intervalMs: Long, onUpdate: (MemoryInfo) -> Unit) {
        if (isMonitoring) return
        isMonitoring = true

        handler = Handler(Looper.getMainLooper())
        runnable = object : Runnable {
            override fun run() {
                if (isMonitoring) {
                    onUpdate(getMemoryInfo())
                    handler?.postDelayed(this, intervalMs)
                }
            }
        }
        runnable?.let { handler?.post(it) }
    }

    actual fun stopMonitoring() {
        isMonitoring = false
        runnable?.let { handler?.removeCallbacks(it) }
        handler = null
        runnable = null
    }

    companion object {
        private var appContext: Context? = null

        fun initialize(context: Context) {
            appContext = context.applicationContext
        }
    }
}
