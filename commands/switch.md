# /bottle:switch

Switch to a different bottle.

## Usage

```bash
bottle switch <bottle>
```

## Examples

```bash
bottle switch edge    # Switch from stable to edge
bottle switch stable  # Switch from edge to stable
```

## What it does

1. Shows current vs target bottle differences
2. Uninstalls current bottle's plugins
3. Installs new bottle's plugins
4. Updates tool versions to match

## If bottle is not installed

First install the bottle CLI:

```bash
cargo install bottle
```
