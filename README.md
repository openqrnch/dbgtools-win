# dbgtools-win
A set of platform-specific functions that can be useful when debugging userland
Windows applications and libraries.

# Usage
This crate is meant to be used in development environments only.  Its functions
are tightly tied to other development tools, and it may add significant
overhead to your code.

The recommended way to use `dbgtools` is to add it as an optional feature so
that it is only compiled into your project when it's needed for debugging:

In `Cargo.toml`:

```toml
[dependencies]
dbgtools-win = { version = "0.2", optional = true }
```

In code:

```rust
// Wait for a debugger to connect and break as soon as it has
#[cfg(feature="dbgtools-win")]
dbgtools_win::debugger::wait_for_then_break();
```

Build using:

```
cargo build --features dbgtools-win
```

# See also
- [verboten](https://crates.io/crates/verboten) - A simple Windows service
  wrapper for msvsmon, the remote debugging server for Visual Studio.

