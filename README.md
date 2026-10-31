# SigmaOS browser_handoff

A Rust state model for browser choice and URL handoff. It represents default/custom browser selection, reports handoff outcomes, stores URLs deferred when no browser is configured, and provides host extraction helpers.

This crate does not spawn a browser or invoke operating-system URL handlers. Deferred URLs are held in memory only. `SigmaOS-browser_handoff` is a distinct repository from `SigmaOS-browser-handoff` despite their similar names.

## Build

The manifest references sibling [`SigmaOS-kernel`](../SigmaOS-kernel). From the SigmaOS checkout:

```sh
cargo check --manifest-path SigmaOS-browser_handoff/Cargo.toml --lib
```

## License

GPL-3.0-or-later. See [`Cargo.toml`](Cargo.toml).
