# zed-phpstan

PHPStan diagnostics for PHP files in [Zed](https://zed.dev/), powered by the
standalone [`phpstan-language-server`](https://github.com/laraochan/phpstan-language-server).

## Requirements

Install the LSP server and make it available on Zed's `PATH`:

```sh
git clone https://github.com/laraochan/phpstan-language-server
cargo install --path phpstan-language-server
```

The opened project must also install PHPStan as `vendor/bin/phpstan`:

```sh
composer require --dev phpstan/phpstan
```

PHPStan 1.12.27+ or 2.1.17+ is required by the language server. The project
may use `phpstan.neon`, `phpstan.neon.dist`, or another configuration file.

## Architecture

This extension only locates and starts `phpstan-language-server`. The server
handles LSP over stdio, keeps unsaved buffers in memory, and invokes the
workspace's PHPStan in editor mode. This preserves Composer autoloading,
PHPStan extensions, configuration, and the project's PHP runtime.

The server is diagnostics-only. A PHP language server such as Phpactor can be
used alongside it for completion, navigation, and symbols.

## Development

Install the dev extension from Zed's Extensions view, then select this
directory. The Rust extension is built for WebAssembly by Zed.
