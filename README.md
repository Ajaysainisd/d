# d

**One command system for every development project.**

`d up`, `d build web`, `d test`, `d logs` — same commands across Flutter, Docker, Node, and more.

```bash
d run web          # Flutter → flutter run -d chrome
d up               # Docker → docker compose up
d dev              # Node → npm run dev
d build ios        # Flutter → flutter build ios
d test             # whatever test runner the project uses
d doctor           # check your dev environment
```

## How it works

d auto-detects your project type by looking for known files (`pubspec.yaml`, `package.json`, `docker-compose.yml`, etc.) and delegates to the right underlying tool. No configuration needed for common projects.

Add a `d.yaml` only when you need to override:

```yaml
commands:
  run:
    web:
      command: flutter run -d chrome --web-port=8080
```

## Install

Requires Rust. [Install Rust](https://rustup.rs) if you don't have it.

```bash
git clone https://github.com/ajay39/d.git
cd d
cargo build --release
cp target/release/d-cli /usr/local/bin/d
```

Or run directly during development:

```bash
cargo run -- <command>
# examples:
cargo run -- doctor
cargo run -- --dry-run build ios
```

## Commands

```
d up               Start services
d down             Stop services
d run [target]     Run a target (web, ios, android, etc.)
d build [target]   Build a target
d test             Run tests
d lint             Lint code
d format           Format code
d doctor           Health checks
d logs             View logs
d shell            Open a shell
d migrate          Run migrations
d clean            Clean artifacts
d restart          Restart services
d install          Install dependencies
d dev              Start dev server
d release          Create release build
```

### Options

```
--dry-run                Show what would execute without running
--project-type <TYPE>    Force a project type (flutter, docker, node)
--parallel               Run workspace commands concurrently
--log-level <LEVEL>      info, warn, error, debug
--no-hooks               Skip pre/post hooks
--env <ENV>              Environment (dev, staging, production)
```

## Supported platforms

### Flutter
Detected via `pubspec.yaml`.
| Verb | Targets | Underlying command |
|------|---------|-------------------|
| `run` | web, ios, android, macos, linux, windows | `flutter run` |
| `build` | ios, android, web, macos, linux, windows | `flutter build {target}` |
| `test` | — | `flutter test` |
| `clean` | — | `flutter clean` |
| `format` | — | `dart format .` |
| `lint` | — | `dart analyze` |

### Docker
Detected via `docker-compose.yml`, `compose.yaml`, or `compose.yml`.
| Verb | Targets | Underlying command |
|------|---------|-------------------|
| `up` | — | `docker compose up` |
| `up` detached | — | `docker compose up -d` |
| `down` | — | `docker compose down` |
| `restart` | — | `docker compose restart` |
| `logs` | — | `docker compose logs` |
| `build` | — | `docker compose build` |
| `shell` | `<service>` | `docker compose exec {target} sh` |

### Node
Detected via `package.json`. Auto-detects npm, pnpm, or yarn.
| Verb | Targets | Underlying command |
|------|---------|-------------------|
| `install` | — | `npm install` |
| `dev` | — | `npm run dev` |
| `build` | — | `npm run build` |
| `test` | — | `npm test` |
| `lint` | — | `npm run lint` |
| `format` | — | `npm run format` |
| `start` | — | `npm start` |

## Configuration (d.yaml)

Optional. Placed in your project root.

```yaml
name: myapp
type: flutter           # optional, auto-detected

commands:
  run:
    web:
      command: flutter run -d chrome --web-port=8080
  build:
    ios:
      release: flutter build ios --release
      debug: flutter build ios --debug

hooks:
  pre-run: echo "Starting..."
  post-run: echo "Done!"
  pre-build: ./scripts/build-check.sh
```

### Hooks

| Hook | Runs |
|------|------|
| `pre-up` / `post-up` | Before/after `d up` |
| `pre-down` / `post-down` | Before/after `d down` |
| `pre-run` / `post-run` | Before/after `d run` |
| `pre-build` / `post-build` | Before/after `d build` |
| `pre-test` / `post-test` | Before/after `d test` |

Pre-hooks that fail (non-zero exit) block the main command. Use `--no-hooks` to skip.

## Workspaces

For monorepos. Create a `workspace.yaml` at the root:

```yaml
workspace:
  - backend
  - mobile
  - website
```

Or in `d.yaml`:

```yaml
name: myapp
workspace:
  - backend
  - mobile
  - website
```

Running `d up` at the workspace root starts every project.

## Command resolution priority

1. CLI arguments (`--project-type`, `--env`)
2. `d.yaml` project config (user overrides)
3. Workspace config
4. Platform defaults (from detection)
5. Built-in defaults (`doctor`, `help`)

## Doctor

```bash
d doctor
```

Checks for required tools on your PATH and ensures they're working. Platform-specific checks run based on detected project type.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Failure |
| 2 | Invalid command |
| 3 | Dependency missing |

## Development

### Build

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### Project structure

```
crates/
├── d-cli/       Binary entry point (clap CLI)
├── d-core/      Engine: detection, resolution, config, hooks, doctor
├── d-flutter/   Flutter platform plugin
├── d-docker/    Docker platform plugin
└── d-node/      Node.js platform plugin
```

### Adding a new platform

1. Create a new crate `crates/d-<name>/`
2. Implement the `Platform` trait from `d-core`:
   - `name()` — platform identifier
   - `detect(dir)` — return 0.0–1.0 confidence
   - `commands()` — return `Vec<CommandDef>`
   - `doctor_checks()` — return platform-specific health checks
3. Register in `d-cli/src/main.rs` by adding `d_<name>::platform()` to the platforms vec
4. Add the crate to `Cargo.toml`

## License

MIT
