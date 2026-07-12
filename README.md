# zed-phpstan

Zed integration for [`phpstan-language-server`](https://github.com/laraochan/phpstan-language-server).

> [!WARNING]
> This project is currently a prototype and may change significantly before
> its first stable release.

This extension only starts the standalone language server. For supported
PHPStan versions, configuration, diagnostics, development, and other details,
see the [`phpstan-language-server` README](https://github.com/laraochan/phpstan-language-server#readme).

## Install

Install and configure [`phpstan-language-server`](https://github.com/laraochan/phpstan-language-server)
by following its README. Make sure the installed `phpstan-language-server`
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

`phpstan-language-server` uses the workspace's `vendor/bin/phpstan` by
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

See the [`phpstan-language-server` README](https://github.com/laraochan/phpstan-language-server#readme)
for the supported initialization options and project requirements.

## Development

```sh
cargo check
```

Then install the development extension from Zed's Extensions view and select
this directory.

## License

This project is licensed under the [MIT License](LICENSE).
