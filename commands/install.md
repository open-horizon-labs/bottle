# /bottle:install

Install a bottle (stable or edge).

## Usage

Run `bottle install` in the terminal and follow the prompts.

```bash
bottle install [stable|edge]
```

## Options

- `stable` (default) - Production-ready Open Horizon Labs stack
- `edge` - Latest features, may have rough edges

## If bottle is not installed

First install the bottle CLI:

```bash
cargo install bottle
# or
brew install open-horizon-labs/tap/bottle
```

Then run `bottle install`.
