# Preregistration: closure degree at arity five and six

Registered 2026-11-01 before generation of any draw or execution of any new
closure scan.

## Question and scope

Prior native measurements held the Boolean chained-system closure degree at
four through `m <= 4`, `ell <= 6`, and at most 54 variables. They do not answer
whether the same closure survives the first feasible `m = 5,6` cells. This
study measures exactly three cells:

- `(m,n,ell) = (5,11,2)`, 43 variables;
- `(m,n,ell) = (5,13,3)`, 54 variables; and
- `(m,n,ell) = (6,11,2)`, 56 variables.

There are four natural exactly-unsatisfiable draws and two natural
satisfiable soundness controls per cell. The study also replays the measured
`(4,12,5,draw 24)` row as a binary/source identity control. No clock is a
metric, so A/A is inapplicable. Emitted seconds are diagnostic only and are
invalid after checkpoint restore.

The five registered arms are composite finite packages with one instance
each: one identity replay; one package containing all six SAT controls; and
one four-UNSAT package for each of the three cells. These packages retain
`1 + 6 + 4 + 4 + 4 = 19` closure subprocesses. Three separate generator
invocations precede them. The package design does not turn the 19 heterogeneous
subprocesses into 20 nominally balanced observations.

## Frozen instruments, environment, and inputs

Only these native Rust examples are scientific instruments:

```text
examples/dreg_ladder.rs
examples/dreg_closure.rs
examples/dreg_high_arity_analyze.rs
```

Before generation, the frozen protocol binds the registered source commit,
release-binary hashes for all three examples, analyzer source hash, compiler
and container digest, `Cargo.lock`, historical input and expected identity
row, and all transitive repository sources through that commit. The execution
ref may be a pushed descendant that adds the frozen protocol or committed draw
inputs, but the instrument and runner paths must be byte-identical to the
registered source commit. Every stage rechecks those identities and refuses a
dirty or mismatched instrument.

Closure processes run under a sanitized environment. Inherited
`KIC_F4_*`, `KIC_GF2_*`, `F4_F2_*`, tuning, tracing, and debug variables are
removed; only the explicitly frozen allowlist and values are supplied. The
same binary and environment are used for identity, SAT controls, and UNSAT
measurements. In particular, the registered limits include four Rayon
threads, a 32,000,000,000-byte address-space cap, the recorded row/column
limits, and `DREG_CKPT_SECONDS=300`.

Each cell uses seed `20261101`, `--ffd-max 3`, `--max-draws 4096`,
`--unsat 4`, and `--min-sat 2`. The generator walks one deterministic stream
until both quotas are present or the cap is reached. Extra UNSAT records seen
while waiting for SAT controls remain in the immutable stream but do not
replace the first four UNSAT panel members. The seed is never replaced because
a desired outcome is rare.

Generation is per-cell, restartable, and atomic. An existing file is skipped
only after the exact audit succeeds; a temporary or invalid file is retained
as evidence and never promoted as a committed input. The audit parses records
in numeric draw order and requires a unique, strictly increasing full prefix,
the exact `m`, `n`, `ell`, variable count, and draw range, consistent outcome
and solution-count fields, and the correct terminal both-quota condition. The
first four UNSAT and first two SAT records by draw index form the frozen panel.
A capped stream without both quotas leaves that cell unmeasured.

## Execution units

Build the exact frozen commit in the recorded container, then generate only:

```sh
cargo build --release --locked \
  --example dreg_ladder \
  --example dreg_closure \
  --example dreg_high_arity_analyze
for step in gen-m5-n11-l2 gen-m5-n13-l3 gen-m6-n11-l2; do
  research/dreg_high_arity_20261101/run.sh \
    target/release/examples/dreg_ladder \
    target/release/examples/dreg_closure \
    target/release/examples/dreg_high_arity_analyze "$step"
done
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure \
  target/release/examples/dreg_high_arity_analyze audit-draws
```

Commit and push each of the three audited draw files with its `.log` and
`.generation.json` custody sidecars before closure execution. Then submit one
detached job for each exact logical name:

```text
identity-m4-n12-l5-d24
control-m5-n11-l2-d<SAT_DRAW_1>
control-m5-n11-l2-d<SAT_DRAW_2>
control-m5-n13-l3-d<SAT_DRAW_1>
control-m5-n13-l3-d<SAT_DRAW_2>
control-m6-n11-l2-d<SAT_DRAW_1>
control-m6-n11-l2-d<SAT_DRAW_2>
unsat-m5-n11-l2-d<UNSAT_DRAW_1..4>
unsat-m5-n13-l3-d<UNSAT_DRAW_1..4>
unsat-m6-n11-l2-d<UNSAT_DRAW_1..4>
```

For each name, the invocation is:

```sh
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure \
  target/release/examples/dreg_high_arity_analyze \
  <exact-logical-name>
```

Each invocation starts exactly one closure subprocess, has a 24-hour process
timeout and 60-second `TERM` grace, and writes one new append-only attempt at
`$DREG_OUTPUT_ROOT/runs/<logical-name>/attempts/<NNNN>/`. Its stdout, stderr,
exit status, host facts, frozen identities, environment manifest, checkpoint
generation, and resume provenance are retained. Checkpoints are shared between
attempts of the same logical name and never across draws. Four unsuccessful
attempts make the subprocess terminal-unmeasured; they do not make it false.

After all terminal records are present, run the frozen classifier:

```sh
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure \
  target/release/examples/dreg_high_arity_analyze analyze
```

The runner uses a full repository checkout. On detached workers it requires an
absolute durable `DREG_OUTPUT_ROOT` exactly equal to `$EPHEMERAL_OUT`. If
`$EPHEMERAL_RESUME_DIR` is present, the runner validates and copies the old
output into an empty `$EPHEMERAL_OUT` before starting a new attempt. The old
tree is never modified. Resumed algebraic counters remain valid; all wall and
CPU timing fields are marked invalid.

Launch is blocked until the runner has the deployed ephemeral-stack identity:
current STS resolves to account `590183823895`, not stack account
`104186811093`, and `/ephemeral/config` is unavailable. No job is launched
under the wrong account. This operational block is not a scientific result.

## Exact per-subprocess classification

The native analyzer validates every attempt and selects a terminal record only
under the frozen rules. It requires exactly one JSON object for a successful
closure attempt, exact cell and draw identity, and no contradictory fields.

- `closure <= 4`: a complete degree-3 or degree-4 refutation. An early
  degree-3 refutation is complete evidence and does not require a degree-4 row.
- `above 4`: no refutation, `measured_to = 4`, and a present, completed,
  saturated degree-4 row.
- `unmeasured`: timeout, OOM, address-space or row/column cap, missing output,
  corrupt checkpoint, exhausted retry budget, or any incomplete scan.
- `invalid`: malformed or duplicate successful output, wrong cell/draw,
  contradictory algebraic fields, a refuted satisfiable control, or an
  algebraically different identity replay.

A SAT control is sound only when exactly one validated object scans exactly
degrees 3 and 4, both rows are complete and saturated, and neither XL nor
closure refutes it. The identity row must match the historical row in every
registered algebraic field after removal of explicitly non-scientific
provenance and diagnostic timing fields.

## Package decision rules

- **bounded-four at this cell** only if all four UNSAT subprocesses are
  measured and refute by degree four and all six shared SAT controls are sound;
- **draw-dependent** if all four UNSAT subprocesses complete, contain both
  outcomes, and all controls are sound;
- **not universally four at this cell** if at least one complete UNSAT
  subprocess saturates degree four without refuting and the controls are
  sound, even when another UNSAT subprocess is merely unmeasured; and
- **unmeasured/invalid** otherwise, with every missing, censored, or invalid
  subprocess listed rather than averaged away. An invalid record remains an
  instrument failure; an unmeasured row cannot erase a valid counterexample.

These outcomes do not close the broader lead. A bounded-four result advances
to growing `ell` and `n`, matched external F4, extraction, yield/rank, and
complete cost. An above-four result rejects only a universal degree-four claim
at that exact cell. Degree five, other presentations, GG14, FGHR, symmetric
systems, topology, and exact extraction remain live. A timeout, OOM, cap,
interruption, incomplete scan, or infrastructure failure is never negative
mathematical evidence.

## Evidence and export

Retain immutable draw JSONL/logs and generation-provenance sidecars, every
append-only attempt, checkpoints,
source and binary hashes, compiler/container identity, sanitized environment,
host facts, protocol hash, commit, resume provenance, and every failure. The
native analyzer emits a deterministic summary retaining all 19 subprocesses
and every attempt. The `experiment.export/v1` binds the frozen protocol hash
and has five logical package rows with `sample_size.instances = 1`; each row
links its complete subprocess evidence. Resumed rows explicitly invalidate
wall and CPU time.

The final report includes a source-linked Markdown narrative, quantitative
SVG, Typst source, inspected PDF, and independent review record. No review,
report, or atlas disposition may call a timeout, OOM, cap, partial scan, or
launcher-account mismatch a disproof.
