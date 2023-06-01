<!-- ![tauri-plugin-unarchiver](banner.png) -->

# Tauri Plugin UnArchiver

A lightweight archive extraction utility.

## Install

_This plugin requires a Rust version of at least **1.64**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-unarchiver = { git = "https://github.com/ZanzyTHEbar/tauri-plugin-unarchiver", branch = "main" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

> Note: Since most JavaScript package managers are unable to install packages from git monorepos we provide read-only mirrors of each plugin. This makes installation option 2 more ergonomic to use.

```sh
pnpm add https://github.com/ZanzyTHEbar/tauri-plugin-unarchiver
# or
npm add https://github.com/ZanzyTHEbar/tauri-plugin-unarchiver
# or
yarn add https://github.com/ZanzyTHEbar/tauri-plugin-unarchiver
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_unarchiver::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```js
import { unarchive } from "tauri-plugin-unarchiver-api";

// Destination is optional, if not provided it will be extracted to the same directory as the archive
await unarchive("/path/to/file", "/path/to/destination");
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License

Code: (c) 2015 - Present - DaOfficialWizard.

MIT or MIT/Apache 2.0 where applicable.
