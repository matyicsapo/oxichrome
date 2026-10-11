# oxichrome-build

Source parsing with `syn` and build artifact generation (manifest.json, JS shims, HTML shells) for oxichrome.

This is an internal crate. You probably want [oxichrome](https://crates.io/crates/oxichrome) or [cargo-oxichrome](https://crates.io/crates/cargo-oxichrome) instead.

## Icons and actions

Configure extension icons and the toolbar action in the extension attribute:

```rust
#[oxichrome::extension(
	name = "Focus Block Rs",
	version = "0.1.0",
	permissions = ["storage"],
	action = {
		default_title = "Focus Block",
		default_icon = {
			"16" = "icons/icon16.png",
			"32" = "icons/icon32.png",
			"48" = "icons/icon48.png",
			"128" = "icons/icon128.png",
		},
	},
	icons = {
		"16" = "icons/icon16.png",
		"32" = "icons/icon32.png",
		"48" = "icons/icon48.png",
		"128" = "icons/icon128.png",
	},
)]
struct FocusBlock;
```

Paths are preserved exactly in the generated manifest and are relative to the
extension output directory. Place these example files in `static/icons/`; the CLI
copies the contents of `static/` into the output directory.

An `#[oxichrome::popup]` function automatically adds `default_popup = "popup.html"`
to the action without discarding its other settings. An explicit `default_popup`
takes precedence. Actions also work without a popup. `default_icon` accepts either
a size-to-path map or a single string path.
