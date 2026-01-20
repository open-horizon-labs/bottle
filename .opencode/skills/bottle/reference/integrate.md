# bottle-integrate

Add or remove platform integrations (Claude Code, Codex).

## Usage

```
bottle-integrate [--list]
bottle-integrate <platform>
bottle-integrate --remove <platform>
```

- `--list` - List available and installed integrations
- `platform` - Platform to integrate: `claude_code`, `opencode`, `codex`
- `--remove` - Remove the integration instead of adding it

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
   bottle integrate [platform] [--list] [--remove]
   ```

   Pass through the output to the user.

## Platforms

- **claude_code** - Claude Code plugin integration
- **opencode** - OpenCode plugin (this one!)
- **codex** - Codex skill integration

## Examples

```
bottle-integrate --list
-> bottle integrate --list

bottle-integrate claude_code
-> bottle integrate claude_code

bottle-integrate --remove codex
-> bottle integrate --remove codex
```
