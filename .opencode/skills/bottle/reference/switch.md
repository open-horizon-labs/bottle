# bottle-switch

Switch to a different bottle.

## Usage

```
bottle-switch <bottle-name> [-y]
```

- `bottle-name` - Name of bottle to switch to (required)
- `-y` - Skip confirmation prompt

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
   bottle switch <bottle-name> [-y]
   ```

   Pass through the output to the user.

## Examples

```
bottle-switch edge
-> bottle switch edge

bottle-switch stable -y
-> bottle switch stable -y
```
