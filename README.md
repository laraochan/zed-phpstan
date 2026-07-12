# zed-phpstan

Zed integration for [`phpstan-language-server`](https://github.com/laraochan/phpstan-language-server).

This extension only starts the standalone language server. For supported
PHPStan versions, configuration, diagnostics, development, and other details,
see the [`phpstan-language-server` README](https://github.com/laraochan/phpstan-language-server#readme).

## Install

Install `phpstan-language-server` and make it available on Zed's `PATH`:

```sh
git clone https://github.com/laraochan/phpstan-language-server
cargo install --path phpstan-language-server
```

Install this extension from Zed's Extensions view, or use **Install Dev
Extension** while developing it locally.

The opened project must have PHPStan installed as `vendor/bin/phpstan`; follow
the language server's documentation for project setup.

## Development

```sh
cargo check
```

Then install the development extension from Zed's Extensions view and select
this directory.
