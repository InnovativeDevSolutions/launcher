# Arma 3 Launcher

A modern, feature-rich launcher for Arma 3 built with Tauri, Vue 3, and TypeScript. This launcher provides comprehensive mod management, server querying, and automated mod updating capabilities.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.x-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3.x-green.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)
![Rust](https://img.shields.io/badge/Rust-2021-red.svg)

## Features

### 🎮 Game Management
- **Launch Arma 3** with selected mods and custom parameters
- **Server Query System** with real-time status checking
- **BattlEye Toggle** for enabling/disabling anti-cheat
- **Custom Startup Parameters** for advanced launch options

### 🔧 Mod Management
- **Automatic Mod Scanning** for both local and Steam Workshop mods
- **Mod Enable/Disable** with persistent state management
- **Mod Information Display** including size, type, and modification date
- **Extension Detection** for identifying mod capabilities (DLLs)
- **Bulk Operations** for enabling/disabling all mods at once

### 📦 Automatic Mod Updates (Forge Framework)
- **Custom Forge Mod Support** for framework-specific modifications
- **Google Cloud Storage Integration** for reliable mod distribution
- **Remote Version Checking** against cloud-hosted Forge mod repositories
- **Automatic Download & Installation** of Forge framework mod updates
- **ZIP Extraction** with intelligent folder structure handling
- **Registry Tracking** for installed Forge mod versions and metadata
- **Batch Updates** for updating multiple Forge mods simultaneously
- **Configurable Auto-Updates** with toggle control in Options panel
- **Startup Update Checking** when auto-updates are enabled
- **Manual Override** always available regardless of auto-update setting

> **Note**: Auto-update functionality is specifically designed for custom Forge framework mods hosted on Google Cloud Storage. Steam Workshop mods are managed separately through Steam's own update mechanisms.

### 🎯 Mod Presets
- **Save & Load Presets** for different mod configurations
- **Named Presets** for easy identification (e.g., "PvP Setup", "Co-op Mission")
- **Preset Management** with creation, loading, and deletion capabilities

### ⚙️ Configuration Management
- **Persistent Settings** stored in app data directory
- **Directory Path Management** for Arma 3 and Steam library locations
- **Server Configuration** with address and password storage
- **Startup Parameters** for configuration-specific launch options
- **Auto-save Functionality** for seamless user experience

## Architecture

### Frontend (Vue 3 + TypeScript)
- **Modern Vue 3** with Composition API and `<script setup>` syntax
- **TypeScript Integration** for type-safe development
- **Reactive State Management** using Vue's provide/inject system
- **Component-based Architecture** with modular view components
- **Responsive UI** with custom CSS styling and animations

### Backend (Rust + Tauri)
- **Tauri Framework** for secure desktop application development
- **Async Command System** for non-blocking operations
- **File System Integration** for mod scanning and management
- **Network Operations** for server queries and mod downloads
- **Cross-platform Compatibility** with Windows-optimized builds

### Key Components
- **PlayView**: Main launch interface with server status and controls
- **ModsView**: Comprehensive mod management with list views and controls
- **ModUpdater**: Automated update system with progress tracking
- **OptionsView**: Configuration panel for directories and settings
- **FeedView**: Game feed and news display

## Technical Stack

### Dependencies
- **Frontend**: Vue 3, TypeScript, Vite
- **Backend**: Tauri 2.x, tokio, serde, reqwest
- **Mod Management**: walkdir, zip, sha2
- **Server Query**: valve-server-query
- **Utilities**: chrono, uuid, libloading

### File Structure
```
launcher/
├── src/                    # Vue frontend source
│   ├── components/         # Vue components
│   │   ├── FeedView.vue    # Feed interface
│   │   ├── PlayView.vue    # Main launch interface
│   │   ├── ModsView.vue    # Mod management
│   │   ├── ModUpdater.vue  # Update system UI
│   │   └── OptionsView.vue # Settings panel
│   ├── App.vue            # Root component
│   └── main.ts            # Application entry point
├── src-tauri/             # Rust backend source
│   ├── src/
│   │   └── launcher.rs     # Server communication
│   │   ├── lib.rs          # Main library and commands
│   │   ├── mod_updater.rs  # Update management
│   │   ├── mods.rs         # Mod detection system
│   └── Cargo.toml         # Rust dependencies
└── package.json           # Node.js dependencies
```

## Installation & Development

### Prerequisites
- **Node.js** (v16 or higher)
- **Rust** (latest stable)
- **Arma 3** installation
- **Steam** (for Workshop mod support)

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd launcher
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

### Configuration

On first launch, configure the following in the Options panel:
- **Arma 3 Directory**: Path to your Arma 3 installation
- **Steam Library Path**: Path to Steam library for Workshop mods
- **Server Settings**: Default server address and password
- **Automatic Mod Updates**: Toggle to enable/disable automatic update checking on startup

## Usage

### Basic Operation
1. **Set up directories** in the Options view
2. **Select mods** in the Mods view
3. **Configure server** settings in the Play view
4. **Launch Arma 3** with your selected configuration

### Mod Updates
1. **Check for updates** using the Update System
2. **Review available updates** for your installed mods
3. **Apply updates** individually or in batch
4. **Monitor progress** through the update interface

#### Auto-Update Configuration
- **Enable Auto-Updates**: Navigate to Options → Update Settings and toggle "Automatic Mod Updates"
- **Behavior When Enabled**:
  - Updates are checked automatically 3 seconds after app startup
  - Available updates are applied automatically after a 2-second delay
  - Progress is shown in the Update System interface
- **Behavior When Disabled**:
  - No automatic checking occurs on startup
  - Manual "Check for Updates" button remains available
  - Users maintain full control over when updates are applied
- **Settings Persistence**: Auto-update preference is saved to `settings.json` alongside other configuration options

### Preset Management
1. **Configure your mods** for a specific scenario
2. **Save as preset** with a descriptive name
3. **Load presets** quickly for different gameplay modes
4. **Manage presets** through the preset interface

## Build Configuration

The application is optimized for release builds with:
- **Link Time Optimization (LTO)** enabled
- **Single codegen unit** for maximum optimization
- **Panic abort strategy** for smaller binaries
- **Symbol stripping** for reduced file size
- **Result**: ~12MB final executable

## Development

### Recommended IDE Setup
- **VS Code** with extensions:
  - [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Code Style
- **Vue 3 Composition API** with `<script setup>`
- **TypeScript strict mode** enabled
- **Rust 2021 edition** with modern async/await patterns
- **Modular architecture** with clear separation of concerns

## Contributing

Contributions are welcome! Please ensure:
- Code follows existing style conventions
- TypeScript types are properly defined
- Rust code follows clippy recommendations
- UI changes maintain design consistency

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- **Tauri Team** for the excellent desktop framework
- **Vue.js Team** for the reactive frontend framework
- **Rust Community** for the robust backend ecosystem
- **Arma 3 Community** for continued support and feedback
