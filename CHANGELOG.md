# Changelog

All notable changes to the FORGE Launcher project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.0] - 2025-08-16

### Added
- Initial release of FORGE Launcher
- Tauri-based desktop application with Vue.js frontend
- Automatic mod update functionality with user-configurable settings
- Progress notifications for mod updates
- Mod name extraction with fallback mechanism (meta.cpp → mod.cpp)
- Cross-platform support (Windows, macOS, Linux)
- CI/CD pipeline with GitHub Actions for automated building and testing
- Release workflow for automated publishing across all platforms

### Features
- **Mod Management**: Enhanced mod handling with intelligent name extraction
- **Auto-Updates**: Configurable automatic mod updates with progress tracking
- **Cross-Platform**: Native builds for Windows (.msi, .exe), macOS (.dmg), and Linux (.deb, .AppImage)
- **Modern UI**: Vue.js-based interface with consistent styling (12px border-radius)

### Technical Details
- Built with Tauri v2 framework
- Vue.js 3.5+ frontend with TypeScript support
- Vite build system with Bun package manager
- Rust backend with optimized release builds
- Dependencies include:
  - Server query functionality (`valve-server-query`)
  - HTTP client (`reqwest`) with streaming support
  - Archive handling (`zip`)
  - Cryptographic hashing (`sha2`)
  - Async runtime (`tokio`)

### Infrastructure
- GitHub Actions CI pipeline with matrix builds
- Automated testing and linting
- Cross-platform artifact generation
- Automated release creation and asset uploading
- Code signing support for distribution

### Documentation
- Comprehensive help text for auto-update features
- Installation instructions for all platforms
- Issue reporting guidelines

### Fixed
- Updated libwebkit2gtk dependency to version 4.1 for improved Linux compatibility
- Resolved build system configuration issues
- Improved code structure and readability in core launcher functions

---

## Development Notes

This project uses conventional commit messages for clear change tracking:
- `feat:` for new features
- `fix:` for bug fixes  
- `docs:` for documentation changes
- `refactor:` for code restructuring
- `style:` for UI/styling changes

## Installation

### Windows
- Download the `.msi` installer for a traditional installation experience
- Or download the `.exe` for a portable installation

### macOS  
- Download the `.dmg` file and drag the app to your Applications folder

### Linux
- Download the `.deb` package for Debian/Ubuntu-based distributions
- Or download the `.AppImage` for a portable Linux installation

## Contributing

Please see our [Issues](https://github.com/InnovativeDevSolutions/launcher/issues) page to report bugs or request features.

---

*This changelog is automatically maintained. For detailed commit history, see the [Git log](https://github.com/InnovativeDevSolutions/launcher/commits).*
