# LLMinx Solver - Compose Multiplatform

[![Linux Release](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-linux.yml/badge.svg)](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-linux.yml) [![macOS Release](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-macos.yml/badge.svg)](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-macos.yml) [![Windows Release](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-windows.yml/badge.svg)](https://github.com/Sevilze/llminxsolver-cmp/actions/workflows/release-windows.yml)

llminxsolver-cmp is a Megaminx Last Layer solver written in **Rust** with a **Compose Multiplatform** GUI, designed to run on both Mobile and Desktop platforms with a heavy focus on **Material 3 Expressive** theme. The solver uses an IDA\* search algorithm with precomputed pruning tables for finding optimal solutions.

The Rust solver library is exposed to Kotlin via [Mozilla's UniFFI](https://github.com/mozilla/uniffi-rs), providing non-blocking solve operations with callback-based progress reporting and thread-safe cancellation support.

## Installation

### Arch Linux (AUR)

```bash
paru -S llminxsolver-bin
```

Or using yay:

```bash
yay -S llminxsolver-bin
```

### Debian / Ubuntu

Download the `.deb` file from the [Releases](https://github.com/Sevilze/llminxsolver-cmp/releases) page and install it:

```bash
sudo dpkg -i llminxsolver_*.deb
sudo apt-get install -f  # Fix dependencies if needed
```

### Fedora / Red Hat

Download the `.rpm` file from the [Releases](https://github.com/Sevilze/llminxsolver-cmp/releases) page and install it:

```bash
sudo rpm -i llminxsolver-*.rpm
```

### Windows

Download the `.exe` installer from the [Releases](https://github.com/Sevilze/llminxsolver-cmp/releases) page and run it to install the application.

### macOS

Download the `.dmg` file from the [Releases](https://github.com/Sevilze/llminxsolver-cmp/releases) page, open it, and drag the application to your Applications folder.

### NixOS / Nix

This project provides a Nix flake for reproducible builds and easy execution.

#### Prerequisites

- Install Nix with flakes enabled (see [Nix installation guide](https://nixos.org/download/)).

#### Using the Cachix Binary Cache

To avoid compiling the application and its dependencies from source, use the project's Cachix binary cache. This will fetch the pre-built application binary directly.

1. Install Cachix:

    ```bash
    nix profile install nixpkgs#cachix
    ```

2. Use the cache:

    ```bash
    cachix use llminxsolver-bin
    ```

#### Running the Application

Once the cache is configured, you can run the application directly from GitHub:

```bash
nix run github:Sevilze/llminxsolver-cmp
```

#### Building from Source

To build the desktop application locally:

```bash
nix build
```

The resulting binary will be placed in `./result/bin/llminxsolver`.

#### Development Shell

For development, you can enter a Nix shell with all necessary build tools (Rust, JDK, Gradle, etc.):

```bash
nix develop
```

## Usage Guide

The application is divided into several key components, each handling a specific aspect of the workflow.

---

### 1. Megaminx Viewer

The Megaminx Viewer displays the last layer face with 5 corners and 5 edges. This is where you set up the puzzle state before solving.

**Interactions:**

- Drag between stickers of the same piece type to swap pieces.
- Drag between stickers on the same corner to rotate its orientation.
- Drag between stickers on the same edge to flip its orientation.

**Ignore Options:**

Skip certain piece aspects during solving. You can ignore corner positions, edge positions, corner orientations, or edge orientations independently.

---

### 2. Search Configuration

The Search Configuration panel allows you to customize how the solver searches for solutions.

**Allowed Faces:**

Select which face moves the solver is allowed to use. The current supported move sets are RU, RUF, RUL, RUFL, RUFLbL, RUbL, RUbR, and RUD.

**Metric:**

Choose between FTM (Face Turn Metric), which counts consecutive same-face moves as one, and FFTM (Fifth Turn Metric), which counts 72 and 144 degree turns separately.

**Search Depth:**

Enable depth limiting and set a maximum search depth between 1 and 30 moves.

---

### 3. Running the Solver

Press the **Solve** button to initialize the search. You can press **Cancel** to interrupt the search at any time.

The Status Bar provides real-time feedback during the search, showing the current IDA* depth, progress percentage, and estimated time remaining for the current depth. A memory indicator displays system and application memory usage, color-coded based on pressure. When searching with multiple move sets simultaneously, the status bar shows a collapsible multi-mode view where you can expand to see individual progress for each mode.

---

### 4. Solution Panels

The application provides two panels for viewing and managing solutions. Click any solution in either panel to copy it to clipboard.

**Scored Solutions:**

The scored solutions panel displays solutions ranked by MCC score, which estimates execution speed based on finger simulation where lower values indicate faster algorithms. Solutions can also be sorted by move count using the column headers. Adjust the slider to control how many top solutions are displayed.

**Raw Solutions:**

The raw solutions panel is a collapsible view showing all solutions in discovery order. Solutions are loaded on-demand as you scroll using paginated lazy loading, which keeps memory usage low even when thousands of solutions are found.

**Export:**

Both panels support exporting solutions to XLSX format. The scored export includes MCC scores, move counts, and algorithms for the displayed subset, while the raw export includes all solutions from the search. Both export options embed a PNG image of the current puzzle state.

---

### 5. Settings & Configuration

Access settings by clicking the gear icon in the top bar. The settings dialog contains three tabs:

#### Storage Tab

Manage pruning table storage:

- **Storage Usage**: Shows total used space and available disk space.
- **Skip Deletion Warning**: Toggle to bypass confirmation dialogs when deleting tables.
- **Pruning Tables**: List of generated pruning tables with their sizes. You can freely delete any tables which will be regenerated on the next solver initialization.

#### Memory Tab

Configure parallel solver resources:

- **Memory Budget**: Set the maximum RAM allocation for the solver (64 MB to 50% of system RAM).
- **Table Gen Threads**: Number of threads for generating pruning tables.
- **Search Threads**: Number of threads for the IDA* search algorithm.
- **Presets**: Quick buttons to apply Desktop or Mobile configurations.

#### Graphics Tab

Customize visual appearance:

- **Dynamic Color Mode** (Android only): Choose between "BuiltIn" (system native) or "Matugen" (custom algorithm) for wallpaper-based theming.
- **Wallpaper Path** (Desktop only): Specify an image file path for color extraction.
- **Color Scheme**: Select from multiple Material You schemes (Tonal Spot, Content, Expressive, Fidelity, Fruit Salad, Monochrome, Neutral, Rainbow, Vibrant).
- **Theme Mode**: Choose System, Light, or Dark theme.
- **Face Colors**: Customize the color of each Megaminx face (U, F, L, bL, bR, R) using hex input or an RGBA color picker dialog.

---

## Credits

This project includes code and logic adapted from the following open-source projects:

- **[jazzthief81/llminxsolver](https://github.com/jazzthief81/llminxsolver)**: The original Megaminx last layer solver. The Rust implementation is adapted from this Java-based solver, preserving the IDA\* search algorithm and pruning table strategies.

- **[trangium/BatchSolver](https://github.com/trangium/trangium.github.io/tree/master/BatchSolver)**: The MCC calculation logic and finger simulation model are adapted from this project's algorithm scoring system.
