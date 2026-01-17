# JNA - Required for UniFFI native library bindings
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
-keepclassmembers class * extends com.sun.jna.Structure {
    public *;
}

# UniFFI generated bindings
-keep class uniffi.** { *; }
-keepclassmembers class uniffi.** { *; }

# Keep callback interfaces for JNA/UniFFI
-keep interface uniffi.llminxsolver.SolverCallback { *; }
-keep class * implements uniffi.llminxsolver.SolverCallback { *; }

# Keep all native method names
-keepclasseswithmembernames class * {
    native <methods>;
}

# Kotlin coroutines
-keepnames class kotlinx.coroutines.internal.MainDispatcherFactory {}
-keepnames class kotlinx.coroutines.CoroutineExceptionHandler {}
-keepclassmembers class kotlinx.coroutines.** {
    volatile <fields>;
}

# Kotlin serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt
-keepclassmembers class kotlinx.serialization.json.** {
    *** Companion;
}
-keepclasseswithmembers class kotlinx.serialization.json.** {
    kotlinx.serialization.KSerializer serializer(...);
}
-keep,includedescriptorclasses class com.llminxsolver.**$$serializer { *; }
-keepclassmembers class com.llminxsolver.** {
    *** Companion;
}
-keepclasseswithmembers class com.llminxsolver.** {
    kotlinx.serialization.KSerializer serializer(...);
}

# Compose
-keep class androidx.compose.** { *; }
-dontwarn androidx.compose.**

# Keep data classes used by the app
-keep class com.llminxsolver.data.** { *; }
-keep class com.llminxsolver.viewmodel.** { *; }

# Don't warn about JNA internal classes
-dontwarn com.sun.jna.platform.**
-dontwarn java.awt.**

# Keep R8 full mode compatibility
-keepattributes SourceFile,LineNumberTable
-renamesourcefileattribute SourceFile

# Kotlin Result - Required for DataStore with coroutines
-keep class kotlin.Result { *; }
-keepclassmembers class kotlin.Result {
    *;
}

# DataStore Preferences
-keep class androidx.datastore.** { *; }
-keepclassmembers class androidx.datastore.preferences.** { *; }
-keep class * extends com.google.protobuf.GeneratedMessageLite { *; }

# Keep all FfiConverter objects from UniFFI (they use reflection for serialization)
-keep class uniffi.llminxsolver.FfiConverter* { *; }
-keep class uniffi.llminxsolver.*Converter* { *; }

# Keep UniFFI data classes with all their fields
-keep class uniffi.llminxsolver.ThemeColors { *; }
-keep class uniffi.llminxsolver.SchemeType { *; }
-keep class uniffi.llminxsolver.** { *; }
-keepclassmembers class uniffi.llminxsolver.** { *; }
