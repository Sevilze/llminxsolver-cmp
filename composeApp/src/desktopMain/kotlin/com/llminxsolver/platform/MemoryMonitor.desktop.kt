package com.llminxsolver.platform

import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.util.Timer
import java.util.TimerTask

actual class MemoryMonitor actual constructor() {
    private var timer: Timer? = null
    private var isMonitoring = false
    private val osName = System.getProperty("os.name").lowercase()
    private val pid = ProcessHandle.current().pid()

    actual fun getMemoryInfo(): MemoryInfo {
        val systemInfo = when {
            osName.contains("linux") -> getLinuxMemoryInfo()
            osName.contains("mac") || osName.contains("darwin") -> getMacMemoryInfo()
            osName.contains("win") -> getWindowsMemoryInfo()
            else -> getFallbackMemoryInfo()
        }

        val appMemory = when {
            osName.contains("linux") -> getLinuxAppMemory()
            osName.contains("mac") || osName.contains("darwin") -> getMacAppMemory()
            osName.contains("win") -> getWindowsAppMemory()
            else -> getJvmAppMemory()
        }

        return systemInfo.copy(appMemoryBytes = appMemory)
    }

    private fun getLinuxMemoryInfo(): MemoryInfo {
        try {
            val meminfo = File("/proc/meminfo")
            if (!meminfo.exists()) return getFallbackMemoryInfo()

            var memTotal = 0L
            var memAvailable = 0L
            var memFree = 0L
            var buffers = 0L
            var cached = 0L

            meminfo.useLines { lines ->
                lines.forEach { line ->
                    val parts = line.split(":")
                    if (parts.size == 2) {
                        val key = parts[0].trim()
                        val value = parts[1].trim().split(" ")[0].toLongOrNull() ?: 0L
                        when (key) {
                            "MemTotal" -> memTotal = value * 1024
                            "MemAvailable" -> memAvailable = value * 1024
                            "MemFree" -> memFree = value * 1024
                            "Buffers" -> buffers = value * 1024
                            "Cached" -> cached = value * 1024
                        }
                    }
                }
            }

            val available = if (memAvailable > 0) memAvailable else memFree + buffers + cached
            val used = memTotal - available

            return MemoryInfo(
                usedMemoryBytes = used,
                totalMemoryBytes = memTotal,
                usagePercent = if (memTotal > 0) used.toFloat() / memTotal else 0f
            )
        } catch (e: Exception) {
            return getFallbackMemoryInfo()
        }
    }

    private fun getLinuxAppMemory(): Long {
        try {
            val statusFile = File("/proc/self/status")
            if (!statusFile.exists()) return getJvmAppMemory()

            statusFile.useLines { lines ->
                lines.forEach { line ->
                    if (line.startsWith("VmRSS:")) {
                        val value = line.substringAfter(":").trim().split(" ")[0].toLongOrNull()
                        if (value != null) return value * 1024
                    }
                }
            }
        } catch (e: Exception) {
            // Fall through
        }
        return getJvmAppMemory()
    }

    private fun getMacMemoryInfo(): MemoryInfo {
        try {
            val pageSize = runCommand(listOf("pagesize"))?.toLongOrNull() ?: 4096L

            val sysctlOutput = runCommand(listOf("sysctl", "-n", "hw.memsize"))
            val totalMem = sysctlOutput?.toLongOrNull() ?: return getFallbackMemoryInfo()

            val vmStatOutput = runCommand(listOf("vm_stat")) ?: return getFallbackMemoryInfo()

            var pagesFree = 0L
            var pagesInactive = 0L
            var pagesSpeculative = 0L
            var pagesPurgeable = 0L

            vmStatOutput.lines().forEach { line ->
                when {
                    line.startsWith("Pages free:") ->
                        pagesFree = extractNumber(line)

                    line.startsWith("Pages inactive:") ->
                        pagesInactive = extractNumber(line)

                    line.startsWith("Pages speculative:") ->
                        pagesSpeculative = extractNumber(line)

                    line.startsWith("Pages purgeable:") ->
                        pagesPurgeable = extractNumber(line)
                }
            }

            val availableBytes =
                (pagesFree + pagesInactive + pagesSpeculative + pagesPurgeable) * pageSize
            val usedBytes = totalMem - availableBytes

            return MemoryInfo(
                usedMemoryBytes = usedBytes,
                totalMemoryBytes = totalMem,
                usagePercent = if (totalMem > 0) usedBytes.toFloat() / totalMem else 0f
            )
        } catch (e: Exception) {
            return getFallbackMemoryInfo()
        }
    }

    private fun getMacAppMemory(): Long {
        try {
            val psOutput = runCommand(listOf("ps", "-o", "rss=", "-p", pid.toString()))
            val rssKb = psOutput?.trim()?.toLongOrNull()
            if (rssKb != null) return rssKb * 1024
        } catch (e: Exception) {
            // Fall through
        }
        return getJvmAppMemory()
    }

    private fun getWindowsMemoryInfo(): MemoryInfo {
        try {
            val wmicOutput = runCommand(
                listOf(
                    "wmic",
                    "OS",
                    "get",
                    "FreePhysicalMemory,TotalVisibleMemorySize",
                    "/VALUE"
                )
            ) ?: return getFallbackMemoryInfo()

            var totalKb = 0L
            var freeKb = 0L

            wmicOutput.lines().forEach { line ->
                when {
                    line.startsWith("TotalVisibleMemorySize=") ->
                        totalKb = line.substringAfter("=").trim().toLongOrNull() ?: 0L

                    line.startsWith("FreePhysicalMemory=") ->
                        freeKb = line.substringAfter("=").trim().toLongOrNull() ?: 0L
                }
            }

            val totalBytes = totalKb * 1024
            val usedBytes = (totalKb - freeKb) * 1024

            return MemoryInfo(
                usedMemoryBytes = usedBytes,
                totalMemoryBytes = totalBytes,
                usagePercent = if (totalBytes > 0) usedBytes.toFloat() / totalBytes else 0f
            )
        } catch (e: Exception) {
            return getFallbackMemoryInfo()
        }
    }

    private fun getWindowsAppMemory(): Long {
        try {
            val wmicOutput = runCommand(
                listOf(
                    "wmic",
                    "process",
                    "where",
                    "processid=$pid",
                    "get",
                    "WorkingSetSize",
                    "/VALUE"
                )
            )
            wmicOutput?.lines()?.forEach { line ->
                if (line.startsWith("WorkingSetSize=")) {
                    val bytes = line.substringAfter("=").trim().toLongOrNull()
                    if (bytes != null) return bytes
                }
            }
        } catch (e: Exception) {
            // Fall through
        }
        return getJvmAppMemory()
    }

    private fun getJvmAppMemory(): Long {
        val runtime = Runtime.getRuntime()
        return runtime.totalMemory() - runtime.freeMemory()
    }

    private fun getFallbackMemoryInfo(): MemoryInfo {
        val runtime = Runtime.getRuntime()
        val used = runtime.totalMemory() - runtime.freeMemory()
        val max = runtime.maxMemory()
        return MemoryInfo(
            usedMemoryBytes = used,
            totalMemoryBytes = max,
            usagePercent = used.toFloat() / max
        )
    }

    private fun runCommand(command: List<String>): String? = try {
        val process = ProcessBuilder(command)
            .redirectErrorStream(true)
            .start()
        val reader = BufferedReader(InputStreamReader(process.inputStream))
        val output = reader.readText()
        process.waitFor()
        reader.close()
        output.trim()
    } catch (e: Exception) {
        null
    }

    private fun extractNumber(line: String): Long =
        line.replace(Regex("[^0-9]"), "").toLongOrNull() ?: 0L

    actual fun startMonitoring(intervalMs: Long, onUpdate: (MemoryInfo) -> Unit) {
        if (isMonitoring) return
        isMonitoring = true

        timer = Timer("MemoryMonitor", true).apply {
            scheduleAtFixedRate(
                object : TimerTask() {
                    override fun run() {
                        if (isMonitoring) {
                            onUpdate(getMemoryInfo())
                        }
                    }
                },
                0L,
                intervalMs
            )
        }
    }

    actual fun stopMonitoring() {
        isMonitoring = false
        timer?.cancel()
        timer = null
    }
}
