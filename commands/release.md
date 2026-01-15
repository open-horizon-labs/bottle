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
2. Checks git status is clean
3. Bumps manifest version to today's date
4. Commits the change
5. Creates git tag (e.g., `stable-2026.01.15`)
6. Pushes commit and tag

## Full Release Workflow

To release a bottle with updated tool versions:

```bash
# 1. Update manifest with latest upstream versions
./scripts/update-manifest.sh

# 2. Review changes
git diff bottles/stable/manifest.json

# 3. Release (commits, tags, pushes)
bottle release stable -m "Update tool versions"
```

### Preview version changes without applying

```bash
./scripts/update-manifest.sh --dry-run
```

## Releasing the bottle CLI itself

To release a new version of the `bottle` binary:

```bash
# Auto-increment patch version
./scripts/release.sh

# Or specify version explicitly
./scripts/release.sh 0.2.0
```

This updates Cargo.toml, plugin.json, runs tests, commits, tags, and pushes. GitHub Actions then builds binaries and updates homebrew.
