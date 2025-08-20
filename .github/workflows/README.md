# GitHub Workflows for FORGE Launcher

This directory contains GitHub Actions workflows specifically for building, testing, and releasing the FORGE Launcher application.

## Workflows

### 1. CI Workflow (`ci.yml`)

**Purpose**: Continuous Integration - runs on every push and pull request to main/develop branches.

**What it does**:
- Runs tests for both frontend (Vue.js) and backend (Rust)
- Checks code formatting and linting
- Verifies builds work on all platforms (Windows, macOS, Linux)
- Builds debug versions for verification

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

### 2. Release Workflow (`release.yml`)

**Purpose**: Builds and publishes releases with cross-platform binaries.

**What it does**:
- Builds production versions for:
  - Windows (x64): `.msi` and `.exe` installers
  - macOS (Intel x64 & Apple Silicon ARM64): `.dmg` files
  - Linux (x64): `.deb` packages and `.AppImage` files
- Creates GitHub releases with all artifacts
- Automatically generates release notes

**Triggers**:
- Push tags starting with `v` (e.g., `v1.0.0`, `v2.1.3`)
- Manual workflow dispatch with version input

## How to Create a Release

### Option 1: Using Git Tags (Recommended)

1. Ensure your code is ready for release
2. Update version numbers in:
   - `package.json`
   - `src-tauri/Cargo.toml`
   - `src-tauri/tauri.conf.json`
3. Create and push a tag from the launcher directory:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. The release workflow will automatically trigger and create the release

### Option 2: Manual Workflow Dispatch

1. Go to the GitHub Actions tab in your repository
2. Select the "Release" workflow
3. Click "Run workflow"
4. Enter the version (e.g., `v1.0.0`)
5. Click "Run workflow"

## Release Artifacts

The workflow will generate the following files:

### Windows
- `launcher-{version}-x64.msi` - MSI installer
- `launcher-{version}-x64.exe` - NSIS installer

### macOS
- `launcher-{version}-aarch64.dmg` - ARM64/Apple Silicon
- `launcher-{version}-x64.dmg` - Intel x64

### Linux
- `launcher-{version}-x64.deb` - Debian package
- `launcher-{version}-x64.AppImage` - Portable application

## Environment Variables and Secrets

The workflows may use these GitHub secrets (configure in repository settings):

- `GITHUB_TOKEN` - Automatically provided by GitHub
- `TAURI_SIGNING_PRIVATE_KEY` - (Optional) For code signing
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - (Optional) Password for signing key

## Project Structure

The workflows are configured for the launcher directory structure:
```
launcher/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── README.md
├── src-tauri/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── ... (other frontend files)
```

## Troubleshooting

### Common Issues

1. **Build fails on specific platform**: Check the platform-specific dependencies in the workflow
2. **Version mismatch**: Ensure version numbers are consistent across all config files
3. **Permission denied**: Verify the `GITHUB_TOKEN` has write permissions to contents

### Debug Steps

1. Check the Actions tab for detailed logs
2. Look for platform-specific errors in the matrix builds
3. Verify all dependencies are properly installed
4. Test builds locally using the same commands as in the workflow

## Local Testing

To test builds locally before releasing:

```bash
# From the launcher directory
# Install frontend dependencies
bun install

# Build frontend
bun run build

# Build Tauri app
bun run tauri build
```

## Workflow Maintenance

- Keep GitHub Actions up to date by checking for newer versions
- Monitor Rust and Bun versions for compatibility
- Update system dependencies as needed
- Review and update the build matrix for new platforms

## Security Notes

- The workflows run with minimal permissions
- Code signing is optional but recommended for production releases
- All artifacts are scanned by GitHub's security systems
- Consider adding dependency scanning and vulnerability checks

## Path Configuration

All paths in the workflows are relative to the launcher directory root:
- Rust workspace: `./src-tauri -> target`
- Frontend dependencies: `bun install` (uses package.json in root)
- Build output: `src-tauri/target/release/bundle/...`
