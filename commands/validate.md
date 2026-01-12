# /bottle:validate

Validate a bottle manifest (curator command).

## Usage

```bash
bottle validate <bottle>
```

## Examples

```bash
bottle validate stable
bottle validate edge
```

## Checks

- All tools in manifest have valid tool definitions
- All plugins referenced exist
- Version formats are valid
- Prerequisites are documented
