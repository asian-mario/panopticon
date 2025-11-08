# panopticon (v0.1) — minimal skeleton

This workspace contains an initial skeleton for Panopticon v0.1 (Rust + Bevy). It includes:

- a minimal Bevy app scaffold with a Clock and Tick event
- a `validate` CLI binary that loads YAML sample data and performs basic checks
- sample `game/` content and minimal JSON schemas
- `Makefile.toml` tasks for common flows

How to try locally:

1. Install Rust and cargo-make (optional):

```powershell
cargo install cargo-make
```

2. Run the dev app:

```powershell
cargo run
```

3. Run the validator:

```powershell
cargo run --bin validate
```

Notes
- The project is a starting point and intentionally minimal. See `.github/instructions/v0.1.md` for the full design doc.
