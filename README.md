# Checksum Check

**Version 1.3.0**

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
- **Vite v7.2.6** - Modern build tool and development server offering fast HMR and optimized builds
- **Vitest v4.0.15** - Fast unit test framework for testing

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

## Testing

This application includes comprehensive test coverage with **96 total tests** validating all core functionality.

### Test Statistics
- **Frontend Tests**: 62 tests (utils, formats, component rendering)
- **Backend Tests**: 34 tests (12 unit tests + 22 integration tests)
- **Test Coverage**: All hash algorithms validated against known cryptographic values

### Setup

First, install test dependencies:
```bash
npm install
```

### Running Tests

#### Quick Test (All Tests)
```bash
# Windows
run-tests.bat

# Linux/macOS
chmod +x run-tests.sh
./run-tests.sh
```

#### Frontend Tests (React/TypeScript)
```bash
# Run all frontend tests
npm test
```

# Run tests with coverage report
npm run test:coverage

# Run tests with UI
npm run test:ui
```

The frontend tests include:
- **Unit Tests**: Utility functions (file size formatting, date formatting)
- **Component Tests**: App component functionality, user interactions, state management
- **Integration Tests**: File selection, hash calculation, clipboard operations, theme toggling

#### Backend Tests (Rust)
```bash
# Navigate to the Rust backend directory
cd src-tauri

# Run all Rust tests
cargo test

# Run tests quietly
cargo test --quiet

# Run specific test
cargo test test_calculate_empty_file
```

The backend tests include:
- **Unit Tests** (12): Core hash calculation functions, file metadata extraction, async operations
- **Integration Tests** (22): All hash algorithms (MD5, SHA1, SHA256, SHA512), file I/O operations
- **Edge Cases**: Empty files, large files, binary files, unicode content, hash determinism

### Test Coverage

The test suite validates:
- ✅ All four hash algorithms (MD5, SHA1, SHA256, SHA512) produce correct outputs
- ✅ Known hash values from cryptographic standards (empty file, "quick brown fox", etc.)
- ✅ File metadata extraction (size, creation date, modification date)
- ✅ File size formatting with appropriate units (B, KB, MB, GB, TB)
- ✅ Date formatting from Unix timestamps
- ✅ Hash format validation (lowercase hex, correct lengths)
- ✅ UI rendering and accessibility
- ✅ Theme initialization (dark mode by default)
- ✅ Binary and unicode file support
- ✅ Large file processing (1MB+ files)
- ✅ Hash determinism and consistency
- ✅ Edge cases (empty files, special characters, whitespace)



### Known Test Values

For verification purposes, here are the expected hash values for common test inputs:

**Empty file:**
- MD5: `d41d8cd98f00b204e9800998ecf8427e`
- SHA1: `da39a3ee5e6b4b0d3255bfef95601890afd80709`
- SHA256: `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- SHA512: `cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e`

**"The quick brown fox jumps over the lazy dog":**
- MD5: `9e107d9d372bb6826bd81d3542a419d6`
- SHA1: `2fd4e1c67a2d28fced849ee1bb76e7391b93eb12`
- SHA256: `d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592`
- SHA512: `07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6`

## Development

### Project Structure
```
ChecksumCheck/
├── src/                    # Frontend React source
│   ├── tests/             # Frontend test files (62 tests)
│   │   ├── App.test.tsx   # Component tests (13 tests)
│   │   ├── utils.test.ts  # Utility tests (11 tests)
│   │   └── file-formats.test.ts  # Format tests (38 tests)
│   ├── App.tsx            # Main React component
│   └── main.tsx           # React entry point
├── src-tauri/             # Backend Rust source
│   ├── src/
│   │   └── main.rs        # Rust main file with unit tests (12 tests)
│   └── tests/
│       └── integration_test.rs  # Integration tests (22 tests)
├── run-tests.bat          # Windows test runner
├── package.json           # Frontend dependencies
└── vitest.config.ts       # Test configuration
```

### Adding New Tests

**Frontend (TypeScript/React):**
1. Create test files in `src/tests/` with `.test.ts` or `.test.tsx` extension
2. Use Vitest and React Testing Library APIs
3. Run `npm test` to execute
4. Total frontend tests: 62

**Backend (Rust):**
1. Add unit tests in `src-tauri/src/main.rs` under `#[cfg(test)] mod tests`
2. Add integration tests in `src-tauri/tests/` directory
3. Run `cargo test` to execute
4. Total backend tests: 34 (12 unit + 22 integration)

## License

MIT License - See [LICENSE](LICENSE) file for details
