# /bottle:update

Update to the latest bottle snapshot.

## Usage

```
/bottle:update [-y]
```

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
   bottle update [-y]
   ```

   Pass through the output to the user.

## Examples

```
/bottle:update
→ bottle update

/bottle:update -y
→ bottle update -y
```
