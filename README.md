# FightGrid

**Every Strike. Every Round. Every Bracket.**

FightGrid is a Rust-powered tournament bracketing app built for martial arts—especially Filipino Eskrima. It focuses on clean fight scheduling, smooth progression through pools and elimination brackets, and clear visibility into every round.

## Goals
- Make it effortless to build and update brackets mid-event
- Track every strike, round, and result with low latency
- Support Eskrima-specific scoring and rulesets
- Keep the CLI fast and reliable; grow into a cross-platform UI later

## Getting Started
1) Ensure you have Rust installed (`https://rustup.rs`).
2) Build and run the CLI:
```bash
cargo run -- --help
```

## Project Structure
- `src/main.rs` – entrypoint for the FightGrid CLI
- `Cargo.toml` – crate metadata and dependencies

## Next Steps
- Define core data model (fighters, divisions, pools, brackets)
- Implement bracket generation and progression rules for Eskrima formats
- Add persistence layer (local file/db) and export options
- Build tests for bracket logic and tie-break scenarios

## License
MIT