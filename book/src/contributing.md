# Contributing

To add a recommendation for your mod:

1. Create a new file in `recommendations/` (e.g., `recommendations/your-mod.toml`)
2. Add your recommendation in the format shown below
3. Submit a pull request

CI will validate your recommendation format automatically.

## Recommendation Format

Each file contains a single `[recommendation]` block:

```toml
[recommendation]
source.cargo = { crate = "example-mod" }
when.file-exists = "Cargo.toml"
```

## Sources

| Source | Description | Example |
|--------|-------------|---------|
| `source.cargo` | Install from crates.io | `{ crate = "my-mod" }` |
| `source.npx` | Install from npm | `{ package = "@scope/my-mod" }` |
| `source.pipx` | Install from PyPI | `{ package = "my-mod" }` |

### Additional Options

Arguments can be passed to the installed binary:

```toml
source.cargo = { crate = "my-mod", args = ["--flag", "value"] }
```

## Conditions

| Condition | Description |
|-----------|-------------|
| `when.file-exists` | Recommend when a file exists |
| `when.files-exist` | Recommend when all listed files exist |
| `when.using-crate` | Recommend when a Rust crate is a dependency |
| `when.using-crates` | Recommend when all listed crates are dependencies |
| `when.any` | Recommend when any nested condition matches |
| `when.all` | Recommend when all nested conditions match |

Mods without conditions are always recommended.

### Examples

Always recommended:
```toml
[recommendation]
source.cargo = { crate = "always-useful" }
```

Recommended for Rust projects:
```toml
[recommendation]
source.cargo = { crate = "rust-helper" }
when.file-exists = "Cargo.toml"
```

Recommended when using a specific crate:
```toml
[recommendation]
source.cargo = { crate = "tokio-debug" }
when.using-crate = "tokio"
```

Recommended for either Rust or Node projects:
```toml
[recommendation]
source.cargo = { crate = "polyglot-tool" }
when.any = [
    { file-exists = "Cargo.toml" },
    { file-exists = "package.json" },
]
```
