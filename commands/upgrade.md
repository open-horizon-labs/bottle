# /bottle:upgrade

Bump a tool version in a bottle manifest (curator command).

## Usage

```bash
bottle upgrade <bottle> <tool> <version>
```

## Examples

```bash
bottle upgrade stable superego 0.9.0
bottle upgrade edge wm 0.2.0
```

## What it does

1. Updates manifest.json with new version
2. Runs validation
3. Reminds curator to update plugins if needed
