#!/bin/bash
set -e

cd "$(dirname "$0")"

trap 'if [ $? -ne 0 ]; then error_footer; fi' EXIT

readonly CYAN='\033[0;36m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly RED='\033[0;31m'
readonly DIM='\033[2m'
readonly RESET='\033[0m'

readonly CHECK="${GREEN}✓${RESET}"
readonly ARROW="${CYAN}→${RESET}"
readonly WARN="${YELLOW}!${RESET}"
readonly ERROR="${RED}✗${RESET}"

BINDGEN_OUT="shared/src/commonMain/kotlin/uniffi"
JNI_LIBS="shared/src/androidMain/jniLibs"
DESKTOP_LIBS="shared/src/desktopMain/resources"
ANDROID_TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android")
ICON_SRC="desktopApp/icons/manimicon.png"
RES_DIR="androidApp/src/main/res"

progress_step() {
    local current="$1"
    local total="$2"
    local msg="$3"
    echo -e "${DIM}[$current/$total]${RESET} ${ARROW} ${msg}"
}

success() {
    echo -e "  ${CHECK} ${1}"
}

step() {
    echo -e "  ${ARROW} ${1}"
}

warn() {
    echo -e "  ${WARN} ${YELLOW}${1}${RESET}"
}

fail() {
    echo -e "  ${ERROR} ${RED}${1}${RESET}"
}

icon_done() {
    local size="$1"
    local name="$2"
    echo -e "    ${CHECK} ${DIM}${size}${RESET} ${name}"
}

header() {
    local title="$1"
    echo ""
    echo -e "${GREEN}▸ ${title}${RESET}"
    echo ""
}

footer() {
    echo ""
    echo -e "${GREEN}▸ Command terminated successfully.${RESET}"
}

error_footer() {
    echo ""
    echo -e "${RED}▸ Command failed.${RESET}"
}

ensure_cargo_ndk() {
    if ! command -v cargo-ndk &> /dev/null; then
        step "Installing cargo-ndk..."
        cargo install cargo-ndk
        success "cargo-ndk installed"
    fi
}

add_rust_targets() {
    step "Adding Rust targets..."
    for target in "${ANDROID_TARGETS[@]}"; do
        rustup target add "$target" 2>/dev/null || true
    done
    success "Targets ready"
}

strip_bindings() {
    local file="$1"
    sed -i '/^\/\/ This template implements a class for working with a Rust struct via a handle$/,/^\/\/ \[1\] https:\/\/stackoverflow.com\/questions\/24376768\/can-java-finalize-an-object-when-it-is-still-in-scope\/24380219$/d' "$file"
    sed -i '/^\/\/$/d' "$file"
    sed -i ':a;N;$!ba;s/\n\n\n\n/\n\n/g' "$file"
}

generate_bindings() {
    progress_step "1" "3" "Generating UniFFI bindings"
    step "Building release library..."
    cargo build -p llminxsolver-uniffi --release
    success "Library built"

    step "Generating Kotlin bindings..."
    mkdir -p "$BINDGEN_OUT"
    cargo run --bin uniffi-bindgen generate \
        --library target/release/libllminxsolver_uniffi.so \
        --language kotlin \
        --out-dir "$BINDGEN_OUT" || warn "Run on target platform for correct bindings"

    local generated="$BINDGEN_OUT/uniffi/llminxsolver/llminxsolver.kt"
    if [ -f "$generated" ]; then
        step "Stripping template comments..."
        strip_bindings "$generated"
        success "Bindings generated"
    fi
}

generate_bindings_profiler() {
    progress_step "1" "2" "Generating profiler bindings"
    export RUSTFLAGS="-C force-frame-pointers=yes"
    step "Building profiling library..."
    cargo build -p llminxsolver-uniffi --profile profiling
    success "Library built"

    step "Generating Kotlin bindings..."
    mkdir -p "$BINDGEN_OUT"
    cargo run --bin uniffi-bindgen generate \
        --library target/profiling/libllminxsolver_uniffi.so \
        --language kotlin \
        --out-dir "$BINDGEN_OUT" || warn "Run on target platform for correct bindings"

    local generated="$BINDGEN_OUT/uniffi/llminxsolver/llminxsolver.kt"
    if [ -f "$generated" ]; then
        step "Stripping template comments..."
        strip_bindings "$generated"
        success "Bindings generated"
    fi
}

build_android() {
    progress_step "2" "3" "Building Android libraries"
    ensure_cargo_ndk
    add_rust_targets

    step "Creating output directories..."
    mkdir -p "$JNI_LIBS/arm64-v8a"
    mkdir -p "$JNI_LIBS/armeabi-v7a"
    mkdir -p "$JNI_LIBS/x86_64"
    success "Directories ready"

    step "Building for arm64-v8a, armeabi-v7a, x86_64..."
    cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 \
        -o "$JNI_LIBS" \
        build -p llminxsolver-uniffi --release
    success "Android libraries built"
}

build_desktop() {
    progress_step "3" "3" "Building Desktop library"
    step "Building release library..."
    cargo build -p llminxsolver-uniffi --release
    success "Library built"

    step "Copying to resources..."
    mkdir -p "$DESKTOP_LIBS"
    case "$(uname -s)" in
        Linux*)     cp target/release/libllminxsolver_uniffi.so "$DESKTOP_LIBS/" 2>/dev/null || true ;;
        Darwin*)    cp target/release/libllminxsolver_uniffi.dylib "$DESKTOP_LIBS/" 2>/dev/null || true ;;
        MINGW*|MSYS*|CYGWIN*) cp target/release/llminxsolver_uniffi.dll "$DESKTOP_LIBS/" 2>/dev/null || true ;;
    esac
    success "Library ready"
}

build_desktop_profiler() {
    progress_step "2" "2" "Building Desktop profiler"
    export RUSTFLAGS="-C force-frame-pointers=yes"
    step "Building profiling library..."
    cargo build -p llminxsolver-uniffi --profile profiling
    success "Library built"

    step "Copying to resources..."
    mkdir -p "$DESKTOP_LIBS"
    case "$(uname -s)" in
        Linux*)
            cp target/profiling/libllminxsolver_uniffi.so "$DESKTOP_LIBS/" 2>/dev/null || true
            if readelf -S target/profiling/libllminxsolver_uniffi.so 2>/dev/null | grep -q .debug_info; then
                success "Library ready (debug symbols verified)"
            else
                warn "Library ready (debug symbols not found)"
            fi
            ;;
        Darwin*)    cp target/profiling/libllminxsolver_uniffi.dylib "$DESKTOP_LIBS/" 2>/dev/null || true ; success "Library ready" ;;
        MINGW*|MSYS*|CYGWIN*) cp target/profiling/llminxsolver_uniffi.dll "$DESKTOP_LIBS/" 2>/dev/null || true ; success "Library ready" ;;
    esac
}

process_icon() {
    local size="$1"
    local output="$2"
    local type="$3"
    local extent="$4"
    mkdir -p "$(dirname "$output")"
    if [[ "$type" == "Legacy" ]]; then
        magick "$ICON_SRC" -filter Lanczos -sharpen 0x0.5 -quality 100 -resize "$size" "$output"
    else
        magick "$ICON_SRC" -filter Lanczos -sharpen 0x0.5 -quality 100 -resize "$size" -gravity center -background transparent -extent "$extent" "$output"
    fi
}

generate_android_icons() {
    if [ ! -f "$ICON_SRC" ]; then
        fail "Source file not found: $ICON_SRC"
        return 1
    fi

    if ! command -v magick &> /dev/null; then
        fail "ImageMagick not found. Install with: sudo pacman -S imagemagick"
        return 1
    fi

    progress_step "1" "2" "Generating Legacy launcher icons"
    step "Processing mdpi..."
    process_icon "48x48"   "$RES_DIR/mipmap-mdpi/ic_launcher.png"       "Legacy"
    icon_done "48x48" "ic_launcher.png"
    process_icon "48x48"   "$RES_DIR/mipmap-mdpi/ic_launcher_round.png" "Legacy"
    icon_done "48x48" "ic_launcher_round.png"

    step "Processing hdpi..."
    process_icon "72x72"   "$RES_DIR/mipmap-hdpi/ic_launcher.png"       "Legacy"
    icon_done "72x72" "ic_launcher.png"
    process_icon "72x72"   "$RES_DIR/mipmap-hdpi/ic_launcher_round.png" "Legacy"
    icon_done "72x72" "ic_launcher_round.png"

    step "Processing xhdpi..."
    process_icon "96x96"   "$RES_DIR/mipmap-xhdpi/ic_launcher.png"      "Legacy"
    icon_done "96x96" "ic_launcher.png"
    process_icon "96x96"   "$RES_DIR/mipmap-xhdpi/ic_launcher_round.png" "Legacy"
    icon_done "96x96" "ic_launcher_round.png"

    step "Processing xxhdpi..."
    process_icon "144x144" "$RES_DIR/mipmap-xxhdpi/ic_launcher.png"     "Legacy"
    icon_done "144x144" "ic_launcher.png"
    process_icon "144x144" "$RES_DIR/mipmap-xxhdpi/ic_launcher_round.png" "Legacy"
    icon_done "144x144" "ic_launcher_round.png"

    step "Processing xxxhdpi..."
    process_icon "192x192" "$RES_DIR/mipmap-xxxhdpi/ic_launcher.png"    "Legacy"
    icon_done "192x192" "ic_launcher.png"
    process_icon "192x192" "$RES_DIR/mipmap-xxxhdpi/ic_launcher_round.png" "Legacy"
    icon_done "192x192" "ic_launcher_round.png"
    success "Legacy icons complete"

    progress_step "2" "2" "Generating Adaptive icon foregrounds"
    step "Processing mdpi..."
    process_icon "72x72"   "$RES_DIR/mipmap-mdpi/ic_launcher_foreground.png"    "Adaptive" "108x108"
    icon_done "72x72" "ic_launcher_foreground.png"

    step "Processing hdpi..."
    process_icon "108x108" "$RES_DIR/mipmap-hdpi/ic_launcher_foreground.png"    "Adaptive" "162x162"
    icon_done "108x108" "ic_launcher_foreground.png"

    step "Processing xhdpi..."
    process_icon "144x144" "$RES_DIR/mipmap-xhdpi/ic_launcher_foreground.png"   "Adaptive" "216x216"
    icon_done "144x144" "ic_launcher_foreground.png"

    step "Processing xxhdpi..."
    process_icon "216x216" "$RES_DIR/mipmap-xxhdpi/ic_launcher_foreground.png"  "Adaptive" "324x324"
    icon_done "216x216" "ic_launcher_foreground.png"

    step "Processing xxxhdpi..."
    process_icon "288x288" "$RES_DIR/mipmap-xxxhdpi/ic_launcher_foreground.png" "Adaptive" "432x432"
    icon_done "288x288" "ic_launcher_foreground.png"
    success "Adaptive icons complete"
}

show_usage() {
    echo ""
    echo -e "${CYAN}Usage:${RESET} $0 ${YELLOW}<command>${RESET}"
    echo ""
    echo -e "${CYAN}Commands:${RESET}"
    echo -e "  ${YELLOW}bindings${RESET}          Generate UniFFI Kotlin bindings"
    echo -e "  ${YELLOW}bindings-profiler${RESET} Generate bindings with profiling symbols"
    echo -e "  ${YELLOW}android${RESET}           Build Android libraries (arm64, armv7, x86_64)"
    echo -e "  ${YELLOW}desktop${RESET}           Build Desktop library for current platform"
    echo -e "  ${YELLOW}desktop-profiler${RESET}  Build Desktop library with profiling enabled"
    echo -e "  ${YELLOW}icons${RESET}             Generate Android launcher icons"
    echo -e "  ${YELLOW}profiler${RESET}          Run bindings-profiler + desktop-profiler"
    echo -e "  ${YELLOW}all${RESET}               Run bindings + android + desktop (default)"
}

case "${1:-all}" in
    bindings)          generate_bindings ;;
    bindings-profiler) generate_bindings_profiler ;;
    android)           build_android ;;
    desktop)           build_desktop ;;
    desktop-profiler)  build_desktop_profiler ;;
    icons)             generate_android_icons ;;
    all)
        header "LLMinx Solver Native Build"
        generate_bindings
        build_android
        build_desktop
        footer
        ;;
    profiler)
        header "LLMinx Profiler Build"
        generate_bindings_profiler
        build_desktop_profiler
        echo ""
        echo -e "${YELLOW}Profile with:${RESET}"
        echo "  ./scripts/profile-cpu.sh ./gradlew :desktopApp:runDistributable"
        echo ""
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
