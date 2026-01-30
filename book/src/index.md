# Symposium Recommendations

This repository contains the recommended agent mods for [Symposium](https://github.com/symposium-dev/symposium).

Symposium fetches these recommendations on startup to suggest relevant mods for your workspace. Recommendations are matched based on workspace characteristics like the presence of specific files or dependencies.

## Fetch URL

Symposium fetches the combined recommendations file from:

```
https://symposium-dev.github.io/recommendations/recommendations.toml
```

## How It Works

1. When Symposium starts, it fetches the recommendations file
2. Each recommendation has optional conditions (e.g., "when Cargo.toml exists")
3. Symposium evaluates conditions against your workspace
4. Matching mods are suggested for installation

## Repository Structure

```
recommendations/
├── sparkle.toml           # Single-file recommendation
├── cargo.toml
├── rust-analyzer.toml
└── my-mod/                # Directory-based recommendation
    └── config.toml
```

Each recommendation is either:
- A `.toml` file directly in `recommendations/`
- A directory containing `config.toml`

CI validates each file and publishes a concatenated `recommendations.toml` to this site.
