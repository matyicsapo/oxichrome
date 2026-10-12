# oxichrome

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Manifest V3](https://img.shields.io/badge/chrome-Manifest%20V3-4285F4.svg)](https://developer.chrome.com/docs/extensions/develop/migrate/what-is-mv3)
[![Website](https://img.shields.io/badge/website-oxichrome.dev-ff6600.svg)](https://oxichrome.dev)

![oxichrome](https://raw.githubusercontent.com/0xsouravm/oxichrome/main/assets/og.png)

Write browser extensions entirely in Rust, compiled to WebAssembly. Chrome, Firefox, and Safari.

---

```rust
use leptos::prelude::*;
use oxichrome::prelude::*;

#[oxichrome::extension(
    name = "My Extension",
    version = "1.0.0",
    permissions = ["storage"]
)]
struct Extension;

#[oxichrome::background]
async fn start() {
    oxichrome::log!("Running!");
}

#[oxichrome::popup]
fn Popup() -> impl IntoView {
    view! { <p>"Hello from Rust."</p> }
}
```

```sh
cargo oxichrome build                  # Chrome/Edge/Brave
cargo oxichrome build --target firefox # Firefox
cargo oxichrome build --target safari  # Safari 16.4+
```

---

## What it does

- Five proc macros (`#[extension]`, `#[background]`, `#[on]`, `#[popup]`, `#[options_page]`) transform your Rust into wasm-bindgen exports
- Typed bindings to `chrome.storage`, `chrome.tabs`, `chrome.runtime`
- Leptos for reactive popup and options page UI
- `cargo oxichrome build` handles everything: compilation, wasm-bindgen, manifest generation, JS/HTML shim generation, static asset copying, optional wasm-opt
- **`--target firefox`** generates a Firefox-compatible Manifest V3 extension
- **`--target safari`** generates a Safari-compatible Manifest V3 web extension
- Separate `dist/chromium/`, `dist/firefox/`, and `dist/safari/` output directories

Zero JavaScript written by hand.

## Install

```sh
cargo install cargo-oxichrome
```

Requires:
- Rust (stable)
- `wasm32-unknown-unknown` target (auto-installed on first build)
- `wasm-bindgen-cli` (auto-installed with version matching on first build)

## Quick start

```sh
cargo oxichrome new my-extension
cd my-extension
cargo oxichrome build
```

`cargo oxichrome new` scaffolds a project with the `oxichrome` crate already in `[dependencies]`:

```toml
[dependencies]
oxichrome = { version = "0.2" }
wasm-bindgen = "0.2"
serde = { version = "1", features = ["derive"] }
```

**Chrome/Edge:** Load `dist/chromium/` in `chrome://extensions` with "Load unpacked".

**Firefox:** Build with `--target firefox`, then load `dist/firefox/manifest.json` in `about:debugging#/runtime/this-firefox`.

**Safari:** Build with `--target safari`, then follow the packaging steps below.

### Safari web extensions

The Safari target requires **Safari 16.4 or later** (including iOS/iPadOS 16.4 or
later) because the generated background service worker uses ES modules. It uses
Manifest V3 and the same WASM-enabled content security policy as Chromium.
Safari supports the `chrome.*` namespace used by oxichrome, so no API polyfill is
needed.

Build the extension resources on any supported development platform:

```sh
cargo oxichrome build --release --target safari
```

To package them as a Safari extension app, run Apple's tool on macOS with Xcode
installed:

```sh
xcrun safari-web-extension-packager dist/safari
```

On older Xcode versions, the tool is named `safari-web-extension-converter`:

```sh
xcrun safari-web-extension-converter dist/safari
```

Review any compatibility warnings, configure signing in the generated Xcode
project, and build and run the containing app. Enable the extension in Safari's
extension settings. For unsigned local macOS testing, enable "Allow Unsigned
Extensions" in Safari's Develop menu. Safari versions that support temporary
extension-folder installation can also load the generated folder directly; see
[Apple's running instructions](https://developer.apple.com/documentation/safariservices/running-your-safari-web-extension).

The build command generates web extension resources only; it does not run Xcode,
sign an app, or submit it to the App Store. Native packaging supports macOS and
iOS, but API availability and host-permission requirements differ by platform.
For example, Safari requires host permission for tabs access. Review
[Apple's compatibility guidance](https://developer.apple.com/documentation/safariservices/assessing-your-safari-web-extension-s-browser-compatibility)
and test your extension on each intended platform.

## CLI commands

| Command | Description |
|---------|-------------|
| `cargo oxichrome build` | Build for Chromium (default) |
| `cargo oxichrome build --release` | Optimized release build |
| `cargo oxichrome build --target firefox` | Build for Firefox |
| `cargo oxichrome build --target safari` | Build web extension resources for Safari 16.4+ |
| `cargo oxichrome clean` | Remove the `dist/` directory |
| `cargo oxichrome new <name>` | Scaffold a new extension project |

## Project structure

```
oxichrome/
├── oxichrome/            re-export crate (what users depend on)
├── oxichrome-core/       runtime: Chrome API bindings, error types, logging
├── oxichrome-macros/     proc macros
├── oxichrome-build/      source parsing, manifest/shim generation
├── oxichrome-cli/        the cargo oxichrome command
└── examples/
    ├── counter-extension/
    └── color-picker/
```

## Proc macros

### `#[oxichrome::extension(...)]`

Defines extension metadata. Applied to a struct.

```rust
#[oxichrome::extension(
    name = "My Extension",
    version = "1.0.0",
    description = "Optional description",
    permissions = ["storage", "tabs"]
)]
struct MyExtension;
```

### `#[oxichrome::background]`

Marks an async function as the background service worker entry point.

```rust
#[oxichrome::background]
async fn start() {
    oxichrome::log!("Started.");
}
```

### `#[oxichrome::on(namespace::event)]`

Registers an async function as a Chrome event listener.

```rust
#[oxichrome::on(runtime::on_installed)]
async fn on_install(details: oxichrome::__private::wasm_bindgen::JsValue) {
    oxichrome::log!("Installed: {:?}", details);
}
```

### `#[oxichrome::popup]`

Marks a Leptos component as the popup UI.

```rust
#[oxichrome::popup]
fn Popup() -> impl IntoView {
    view! { <p>"Hello."</p> }
}
```

### `#[oxichrome::options_page]`

Marks a Leptos component as the options page UI.

```rust
#[oxichrome::options_page]
fn Options() -> impl IntoView {
    view! { <h1>"Settings"</h1> }
}
```

## Chrome APIs

```rust
// Storage
let val: Option<i32> = oxichrome::storage::get("key").await?;
oxichrome::storage::set("key", &42).await?;
oxichrome::storage::remove("key").await?;

// Tabs
let tabs: Vec<Tab> = oxichrome::tabs::query(&query).await?;
let tab: Tab = oxichrome::tabs::create(&props).await?;
oxichrome::tabs::send_message(tab_id, &msg).await?;

// Runtime
let url = oxichrome::runtime::get_url("icon.png");
oxichrome::runtime::send_message(&msg).await?;
```

## Build output

```
dist/
├── chromium/                  # default target
│   ├── manifest.json
│   ├── background.js
│   ├── popup.html / popup.js
│   ├── options.html / options.js
│   └── wasm/
│       ├── crate_name.js
│       └── crate_name_bg.wasm
├── firefox/                   # --target firefox
│   ├── manifest.json          # background scripts, gecko ID
│   └── ...                    # same files as chromium/
└── safari/                    # --target safari
    ├── manifest.json          # module service worker, ready for Safari packaging
    └── ...                    # same files as chromium/
```

## License

MIT
