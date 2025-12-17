# CLAUDE.md

**Note**: This project uses [bd (beads)](https://github.com/steveyegge/beads)
for issue tracking. Use `bd` commands instead of markdown TODOs.
See AGENTS.md for workflow details.

This file provides guidance to Claude Code when working with this repository.

## Project Overview

feast is a Tauri v2 desktop application with Svelte 5 frontend and SQLite database.

## Build & Test Commands

```bash
# Development
pnpm install          # Install frontend dependencies
pnpm tauri dev        # Run in development mode
pnpm tauri build      # Build for production

# Testing
pnpm test             # Run Vitest frontend tests
pnpm test:watch       # Run tests in watch mode
pnpm test:rust        # Run Rust tests
pnpm test:all         # Run all tests

# Linting & Type Checking
pnpm check            # Type-check Svelte templates
pnpm lint             # Lint frontend code
cd src-tauri && cargo clippy  # Lint Rust code
```

## Technology Stack

- **Frontend**: Svelte 5 + TypeScript + Tailwind CSS v4
- **Backend**: Rust (Tauri v2)
- **Database**: SQLite via sqlx with compile-time query checking
- **Testing**: Vitest (frontend) + cargo test (Rust)

## Architecture

### Data Flow

- **IPC**: Svelte <-> Tauri commands for native operations
- **State**: Svelte stores for UI state, Rust for persistent data
- **Database**: SQLite via sqlx

### Key Directory Structure

```
src/lib/
  components/     - Svelte components
  stores/         - Svelte stores
  tauri/          - Tauri IPC wrappers
  types/          - TypeScript interfaces

src-tauri/src/
  commands/       - Tauri command handlers
  db/             - Database operations
  error/          - Error types
```

## Code Style

### TypeScript/Svelte

- Use TypeScript strict mode
- Prefer Svelte stores for state management
- Use Tailwind utility classes
- Component files: PascalCase
- Utility files: camelCase

### Rust

- Follow Rust 2021 edition idioms
- Use `Result<T, E>` for error handling
- Tauri commands use snake_case
- Keep Tauri commands thin - delegate to lib modules

## Critical Notes

- **Tauri plugins require capabilities**: Plugins must be configured in `src-tauri/capabilities/default.json`
- **`$lib` alias requires both tsconfig.json AND vite.config.ts**
- **Always run `pnpm check` before marking frontend work complete**
