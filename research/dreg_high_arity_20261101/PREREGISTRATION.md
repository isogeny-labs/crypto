# Preregistration: closure degree at arity five and six

Registered 2026-11-01 before generation of any draw or execution of any new
closure scan.

## Question and scope

Prior native measurements held the Boolean chained-system closure degree at
four through `m <= 4`, `ell <= 6`, and at most 54 variables.  They do not
answer whether the same closure survives the first feasible `m = 5,6` cells.
This study measures exactly three such cells:

- `(m,n,ell) = (5,11,2)`, 43 variables;
- `(m,n,ell) = (5,13,3)`, 54 variables; and
- `(m,n,ell) = (6,11,2)`, 56 variables.

There are four natural exactly-unsatisfiable draws and two natural
satisfiable soundness controls per cell.  The study also replays the already
measured `(4,12,5,draw 24)` row as a binary/source identity control.  No
clock is a metric, so A/A is inapplicable.  The emitted seconds are retained
only to diagnose interrupted work and are invalid after checkpoint restore.

## Frozen instruments and inputs

Only the native Rust examples are scientific instruments:

```text
examples/dreg_ladder.rs
examples/dreg_closure.rs
```

`sources.sha256` binds them, `Cargo.lock`, and the historical identity input
and expected row.  Before freezing, `protocol.json` receives the exact commit,
release-binary hashes, and analysis-script hash.  Draw generation uses seed
`20261101`, `--ffd-max 3`, `--max-draws 4096`, and a separate invocation per
cell.  The seed is never replaced because a desired outcome is rare.

The draw gate requires at least four non-satisfiable records whose exact
solution count is zero and two records whose exact solution count is positive.
The first four and first two by committed draw index are used.  A cell that
does not pass this gate is recorded as unmeasured.

## Execution

Build, then generate only:

```sh
cargo build --release --locked --example dreg_ladder --example dreg_closure
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure gen
```

After the exact draw files are committed and pushed:

```sh
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure audit-draws
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure identity
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure controls
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure measure
```

Every closure process runs alone with four Rayon threads, a 32,000,000,000
byte address-space limit, 400,000 columns, a 24-hour process timeout, and a
separate checkpoint directory.  `DREG_CKPT_SECONDS=900`; a resumed scan keeps
its exact algebraic counters but loses all wall-time interpretation.
Detached workers set `DREG_OUTPUT_ROOT=$EPHEMERAL_OUT`, so their run records
and checkpoint generations survive collection or Spot interruption while the
draw inputs remain the exact committed files.  A worker whose durable output
root is absent does not launch.

## Decision rules

Per draw:

- `closure <= 4`: a complete scan refutes at degree 3 or 4;
- `above 4`: degree 4 is completely built and saturated without refuting;
- `unmeasured`: timeout, OOM, cap, missing output, corrupt checkpoint, or
  incomplete degree-4 scan; and
- `invalid`: any satisfiable control is refuted or the identity replay differs
  in an algebraic field.

Per cell:

- **bounded-four at this cell** only if all four unsatisfiable draws are
  measured and refute by degree four, and both satisfiable controls survive;
- **not universally four at this cell** if any complete unsatisfiable draw
  saturates degree four without refuting, with both controls sound;
- **draw-dependent** if measured draws mix the two outcomes; and
- **unmeasured/invalid** otherwise.

These outcomes do not close the broader lead.  A bounded-four result advances
to growing `ell`, `n`, matched external F4, and complete cost.  An above-four
result rejects only a universal degree-four claim at that exact cell; degree
five, other presentations, GG14, FGHR, symmetric systems, and topology remain
live.  A cap or resource failure is never negative scientific evidence.

## Evidence and export

Retain draw JSONL/logs, per-draw JSONL/logs/status records, checkpoints,
source and binary hashes, host facts, protocol hash, commit, and every
failure.  The export must use `experiment.export/v1`, bind the frozen protocol
hash, list all planned/attempted/completed/interrupted/resumed rows, and mark
wall time invalid for every resumed row.  The report must include a
source-linked Markdown narrative, quantitative SVG, Typst source, inspected
PDF, and independent review record.  No review or atlas disposition may call
a timeout, OOM, cap, or partial scan a disproof.

