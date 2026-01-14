# bottle-list

List available bottles (curated and bespoke).

## Usage

```
bottle-list
```

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
   bottle list
   ```

   Pass through the output to the user.

## Output

Shows:
- **Curated bottles** (stable, edge) from GitHub
- **Bespoke bottles** from `~/.bottle/bottles/`
- Current active bottle (if installed)

## Examples

```
bottle-list
-> bottle list
```
