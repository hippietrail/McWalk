# McWalk

A Rust crate that provides firmlink-aware directory walking for macOS by wrapping Swift's `FileManager` APIs.

## Why McWalk?

On macOS, APFS firmlinks are a feature that transparently redirects certain directory paths (e.g., `/System/Volumes/Data` -> `/`). Standard directory walking implementations in C, Rust, and other languages are not aware of these firmlinks and can end up traversing the same directories multiple times or missing files.

Swift's `FileManager` is one of the few APIs that properly handles firmlinks, and `McWalk` makes this functionality available to Rust programs via an FFI bridge.

## Features

- ✅ Firmlink-aware directory traversal on macOS
- ✅ Recursive directory walking
- ✅ Skips hidden files and package contents by default
- ✅ Safe Rust API with callback-based iteration
- ✅ Uses Swift's native `FileManager.enumerator` for maximum compatibility

## Requirements

- macOS
- Swift compiler (`swiftc`)
- Rust 1.70+ (with `2021` edition)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mcwalk = "0.1.0"
```

Basic usage:

```rust
use mcwalk::walk;

fn main() {
    let path = "/System/Volumes/Data"; // A path that crosses firmlinks

    walk(path, |file_path| {
        println!("{}", file_path.display());
    }).unwrap();
}
```

## How It Works

1. **Swift Layer**: `src/McWalkCore.swift` uses Swift's `FileManager.enumerator` to walk directories with firmlink awareness
2. **FFI Bridge**: The Swift code exposes a C-compatible function using `@_cdecl`
3. **Rust Wrapper**: `src/lib.rs` provides a safe Rust API that uses channels to communicate with the Swift callback
4. **Build System**: `build.rs` compiles the Swift code into a static library and links it with Rust

## Limitations

- macOS only (requires Swift and Foundation framework)
- Not thread-safe (uses global static for FFI callback)
- Callback-based API (no iterator implementation yet)

## Building

```bash
cargo build
```

## Running the Example

```bash
cargo run --example main
```

## Testing

```bash
cargo test
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! This is a relatively simple FFI bridge, so there's room for improvement in areas like:
- Thread-safe implementation
- Iterator-based API
- Configuration options (file filtering, depth limits, etc.)
- Better error handling
