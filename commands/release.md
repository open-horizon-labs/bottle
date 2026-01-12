# /bottle:release

Tag and publish a bottle update (curator command).

## Usage

```bash
bottle release <bottle> [-m <message>]
```

## Examples

```bash
bottle release stable -m "Bump superego to 0.9.0"
```

## What it does

1. Runs validation
2. Checks git status
3. Creates git tag (e.g., `stable-2026.01.15`)
4. Pushes tag
