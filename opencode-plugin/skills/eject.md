# bottle-eject

Eject from bottle management (keep tools, manage manually).

## Usage

```
bottle-eject [-y]
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
   bottle eject [-y]
   ```

   Pass through the output to the user.

## What ejecting does

- Removes bottle state file (`~/.bottle/state.json`)
- Keeps all installed tools
- You manage tool updates manually going forward

## Examples

```
bottle-eject
-> bottle eject

bottle-eject -y
-> bottle eject -y
```
