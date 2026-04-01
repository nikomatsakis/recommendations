---
name: serde
description: Serde serialization and deserialization guidance
activation: default
---

# Serde

## Derive usage

Use `#[derive(Serialize, Deserialize)]` on structs and enums. Prefer derive over manual impls unless you need custom logic.

## Common attributes

- `#[serde(rename_all = "camelCase")]` for consistent field naming
- `#[serde(default)]` for optional fields with defaults
- `#[serde(skip_serializing_if = "Option::is_none")]` to omit None fields
- `#[serde(flatten)]` to inline nested struct fields
- `#[serde(untagged)]` for enums without type tags

## Format selection

- `serde_json` for JSON
- `toml` for config files
- `serde_yaml` for YAML
- `bincode` or `postcard` for compact binary

## Error handling

Use `serde_json::from_str::<T>(s)?` with `?` propagation. Serde errors implement `std::error::Error`.
