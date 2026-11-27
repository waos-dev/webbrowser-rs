# webbrowser for WaOS

This fork adds WaOS to webbrowser 1.2.7: on a WaOS app (a WASI component, `target_os = "wasi"`) `open`, `open_browser` and `open_browser_with_options` open a link through WaOS's opener, where upstream compiles its browser (`web-sys`) backend. The change is `src/waos.rs`, its switches in `src/lib.rs` and the target dependencies in `Cargo.toml`; everything else is upstream's. It is kept for whoever ports an app to WaOS, and is not offered upstream.

## Using it

```toml
[patch.crates-io]
webbrowser = { git = "https://github.com/waos-dev/webbrowser-rs", branch = "waos" }
```

The fork asks for the published `wasdk` (0.8.0). An app that takes `wasdk` from WaOS's repository names it in the same `[patch.crates-io]`, so the fork builds with that one too.

## What it does

`Browser::Default` opens the link with the app the user's system picks for its scheme, through `wasdk::opener` (`open_link`), waiting for it with `wasdk::block_on`. Any other `Browser` returns `ErrorKind::NotFound`, and `BrowserOptions::dry_run` checks whether an opener is available without opening the link. If WaOS has no opener or no app handles the scheme, the error is `ErrorKind::NotFound`; if the user refused the scheme, `ErrorKind::PermissionDenied`; any other failure is `ErrorKind::Other`.
