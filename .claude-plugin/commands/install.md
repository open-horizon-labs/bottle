# /bottle:install

Install a bottle (curated tool stack).

## Usage

```
/bottle:install [bottle-name]
```

- `bottle-name` - Name of bottle to install (default: `stable`)

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
   bottle install <bottle-name>
   ```

   Pass through the output to the user.

## Examples

```
/bottle:install
→ bottle install stable

/bottle:install edge
→ bottle install edge
```
