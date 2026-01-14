# bottle-create

Create a new bespoke bottle.

## Usage

```
bottle-create <name> [--from <source>]
```

- `name` - Name for the new bottle (required)
- `--from <source>` - Copy manifest from an existing bottle

## Execution

1. **Check if bottle binary exists:**
   ```bash
   command -v bottle
   ```

2. **If bottle not found**, tell the user:
   ```
   The bottle CLI is not installed.

   Install with Homebrew:
     brew install open-horizon-labs/homebrew-tap/bottle

   Or with Cargo:
     cargo install bottle

   Then run this command again.
   ```

3. **If bottle exists**, run:
   ```bash
   bottle create <name> [--from <source>]
   ```

   Pass through the output to the user.

## What it creates

- New bottle at `~/.bottle/bottles/<name>/`
- Manifest file with tool versions
- Can be customized independently of curated bottles

## Examples

```
bottle-create mystack
-> bottle create mystack

bottle-create mystack --from stable
-> bottle create mystack --from stable
```
