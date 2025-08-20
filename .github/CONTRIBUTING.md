# Contributing to Launcher

Thank you for your interest in contributing to this project! We welcome contributions from everyone.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/launcher.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites
- [Rust](https://rustlang.org/) (latest stable version)
- [Bun](https://bun.sh/) (latest version)
- Platform-specific dependencies:
  - **Linux**: `libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
  - **Windows**: Visual Studio Build Tools or Visual Studio Community
  - **macOS**: Xcode Command Line Tools

### Installation
1. Install frontend dependencies: `bun install`
2. Install Rust dependencies: `cd src-tauri && cargo build`

### Running the Application
- Development mode: `bun run tauri dev`
- Build for production: `bun run tauri build`

## Making Changes

### Code Style
- **Rust**: Follow standard Rust formatting with `cargo fmt`
- **TypeScript/JavaScript**: We use the project's Prettier and ESLint configurations
- **Commits**: Use conventional commit format (e.g., `feat:`, `fix:`, `docs:`)

### Testing
- Run Rust tests: `cd src-tauri && cargo test`
- Run frontend tests: `bun test`
- Lint Rust code: `cd src-tauri && cargo clippy -- -D warnings`
- Format Rust code: `cd src-tauri && cargo fmt --check`

### Pull Request Process
1. Ensure your code follows the established code style
2. Add tests for new functionality
3. Update documentation if needed
4. Fill out the pull request template completely
5. Ensure all CI checks pass

## Issue Reporting

When reporting issues, please:
1. Use the appropriate issue template
2. Provide clear steps to reproduce
3. Include your environment details
4. Add screenshots or logs if helpful

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the project's coding standards

## Questions?

If you have questions, feel free to:
- Open a discussion on GitHub
- Create an issue with the "question" label
- Check existing documentation

We appreciate your contributions!
