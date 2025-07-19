# Rust Playground

A collection of Rust learning exercises and experiments organized by different learning resources.

## Project Structure

```
rust_playground/
├── src/
│   └── main.rs          # Main application entry point
├── rust-handson/        # Rust hands-on tutorial exercises
│   ├── 3-21.rs
│   ├── 3-22.rs
│   ├── 3-23.rs
│   ├── 3-24.rs
│   ├── 3-25.rs
│   ├── 3-26.rs
│   └── 3-27.rs
├── Cargo.toml           # Project configuration
├── rustfmt.toml         # Rust formatter configuration
└── .vscode/
    └── settings.json    # VSCode settings for auto-formatting
```

## Naming Convention

To avoid binary name conflicts when adding multiple learning resources, we use prefixed binary names:

- `handson-*` - For rust-handson tutorial exercises
- `book-*` - For The Rust Programming Language book (example)
- `rustlings-*` - For Rustlings exercises (example)
- `aoc-*` - For Advent of Code solutions (example)

## Running the Code

### Execute specific exercises

```bash
# Run a specific exercise from rust-handson
cargo run --bin handson-3-21
cargo run --bin handson-3-22
# ... and so on

# Build only (without running)
cargo build --bin handson-3-21

# Build in release mode
cargo build --release --bin handson-3-21
```

### List all available binaries

```bash
cargo build --bins
```

## Adding New Learning Resources

This project is organized to accommodate multiple learning resources. To add exercises from a new resource:

1. Create a new directory for your learning resource:
   ```bash
   mkdir your-resource-name
   ```

2. Add your Rust files to the new directory:
   ```bash
   cp your-exercise.rs your-resource-name/
   ```

3. Update `Cargo.toml` to include the new binaries with appropriate prefix:
   ```toml
   [[bin]]
   name = "prefix-exercise-name"
   path = "your-resource-name/your-exercise.rs"
   ```

### Example: Adding a new resource

If you're adding exercises from "The Rust Programming Language" book:

```bash
# Create directory
mkdir rust-book

# Add exercise file
touch rust-book/chapter-1.rs
```

Then in `Cargo.toml`:
```toml
[[bin]]
name = "book-chapter-1"
path = "rust-book/chapter-1.rs"
```

### Example Directory Structure for Multiple Resources

```
rust_playground/
├── rust-handson/        # Current tutorial (prefix: handson-)
├── rust-book/           # The Rust Programming Language book (prefix: book-)
├── rustlings/           # Rustlings exercises (prefix: rustlings-)
├── advent-of-code/      # Advent of Code solutions (prefix: aoc-)
└── custom-projects/     # Your own experiments (prefix: custom-)
```

## Development Setup

### Auto-formatting

This project is configured with automatic code formatting on save in VSCode:

- Formatter: `rustfmt` via `rust-analyzer`
- Format on save: Enabled
- Configuration: See `rustfmt.toml`

### Requirements

- Rust toolchain (install via [rustup](https://rustup.rs/))
- VSCode with rust-analyzer extension (for IDE features)

## Contributing

When adding new exercises:

1. Choose appropriate prefix for your learning resource
2. Follow the existing naming convention (prefix-identifier)
3. Place files in the appropriate learning resource directory
4. Update `Cargo.toml` with the new binary entry
5. Ensure code is properly formatted using `cargo fmt`

## License

This is a personal learning repository. Feel free to use it as a reference for your own Rust learning journey.