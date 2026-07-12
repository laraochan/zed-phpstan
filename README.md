# zed-phpstan

Zed integration for [`phpstan-diagnostics-lsp`](https://github.com/laraochan/phpstan-diagnostics-lsp).

> [!WARNING]
> This project is currently a prototype and may change significantly before
> its first stable release.

This extension only starts the standalone language server. For supported
PHPStan versions, configuration, diagnostics, development, and other details,
see the [`phpstan-diagnostics-lsp` README](https://github.com/laraochan/phpstan-diagnostics-lsp#readme).

## Install

Install and configure [`phpstan-diagnostics-lsp`](https://github.com/laraochan/phpstan-diagnostics-lsp)
by following its README. Make sure the installed `phpstan-diagnostics-lsp`
command is available on Zed's `PATH`.

Install this extension from Zed's Extensions view, or use **Install Dev
Extension** while developing it locally.

## Zed settings

Use Intelephense for PHP language features and PHPStan for diagnostics:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "phpstan"]
    }
  }
}
```

`phpstan-diagnostics-lsp` uses the workspace's `vendor/bin/phpstan` by
default. To override its paths or memory limit, add its initialization options:

```json
{
  "lsp": {
    "phpstan": {
      "initialization_options": {
        "phpstanPath": "tools/phpstan",
        "phpstanConfigPath": "phpstan.neon.dist",
        "memoryLimit": "1G"
      }
    }
  }
}
```

See the [`phpstan-diagnostics-lsp` README](https://github.com/laraochan/phpstan-diagnostics-lsp#readme)
for the supported initialization options and project requirements.

## Development

```sh
cargo check
```

Then install the development extension from Zed's Extensions view and select
this directory.

## License

This project is licensed under the [MIT License](LICENSE).
