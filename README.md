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

### 3. Solving

Press the Solve button to start the search. The Status Bar displays the current search depth, progress percentage, and estimated time remaining. Solutions appear in real-time as they are found. Press Cancel to interrupt the search at any time.

---

### 4. Solutions Panel

Found solutions are displayed with their move sequence notation, FTM and FFTM move counts, and MCC score. The MCC score estimates algorithm speed based on finger simulation, lower scores indicate faster algorithms.

---

## Credits

This project includes code and logic adapted from the following open-source projects:

- **[jazzthief81/llminxsolver](https://github.com/jazzthief81/llminxsolver)**: The original Megaminx last layer solver. The Rust implementation is adapted from this Java-based solver, preserving the IDA\* search algorithm and pruning table strategies.

- **[trangium/BatchSolver](https://github.com/trangium/trangium.github.io/tree/master/BatchSolver)**: The MCC calculation logic and finger simulation model are adapted from this project's algorithm scoring system.
