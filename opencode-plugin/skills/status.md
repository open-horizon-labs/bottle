# bottle-status

Show current bottle status and installed tools.

## Usage

```
bottle-status [--check-updates]
```

- `--check-updates` - Also check for available updates

## Execution

1. **Check if bottle binary exists:**
   ```bash
   command -v bottle
   ```

2. **If bottle not found**, tell the user:
   ```
   The bottle CLI is not installed.

   Install with Homebrew:
     brew install oh-labs/tap/bottle

   Or with Cargo:
     cargo install bottle

   Then run this command again.
   ```

3. **If bottle exists**, run:
   ```bash
   bottle status [--check-updates]
   ```

   Pass through the output to the user.

## Examples

```
bottle-status
-> bottle status

bottle-status --check-updates
-> bottle status --check-updates
```
