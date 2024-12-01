# Checksum Check

A modern, cross-platform desktop application for calculating and verifying file checksums. Built with Tauri 2.0, React, and Rust.

## Features

- Calculate multiple hash types simultaneously:
  - MD5
  - SHA1
  - SHA256
  - SHA512
- Drag-and-drop file support
- File information display (size, creation date, modification date)
- One-click hash copying
- System tray integration
- Dark/Light theme support
- Modern, responsive UI built with TailwindCSS and DaisyUI
- Cross-platform support (Windows, macOS, Linux)
- Local-only processing for security

## Technology Stack

### Frontend
- **React v18.2.0** - Core UI library for building component-based user interfaces
- **TypeScript v5.2.2** - JavaScript superset adding static typing and enhanced developer tooling
- **TailwindCSS v3.3.6** - Utility-first CSS framework for rapid UI development
- **DaisyUI v4.4.19** - Component library for TailwindCSS providing pre-built UI elements and theming
- **Vite v5.0.10** - Modern build tool and development server offering fast HMR and optimized builds

### Backend
- **Tauri v2.0.0** - Framework for building lightweight, secure desktop applications
- **Rust** - Systems programming language for high-performance, safe backend operations
- **System-level hash computation libraries** - Native Rust crates for efficient checksum calculations

### Key Dependencies
- `@tauri-apps/api` v2.0.0 - Core Tauri API for frontend-backend communication
- `@tauri-apps/plugin-dialog` v2.0.1 - File system dialog functionality
- Various Rust crates for cryptographic functions

## Security

All checksum calculations are performed locally on your device. No data is sent to external servers, ensuring complete privacy and security of your files.

## License

MIT License - See [LICENSE](LICENSE) file for details
