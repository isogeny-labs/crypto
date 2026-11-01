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

## Sources checked for this contract

- `research/factor_base_arity_review_20261024/README.md` identifies these
  three cells as the first feasible native `m >= 5` closure discriminator and
  keeps presentation, scale, and extraction questions separate.
- `research/dreg_closure_driver_20261016/PREREGISTRATION.md` establishes the
  exact distinction between a completed degree-four saturation and a killed,
  unmeasured draw, together with natural satisfiable controls.
- `examples/dreg_closure.rs`, `examples/dreg_ladder.rs`, and the canonical
  `experiment.protocol/v1` / `experiment.export/v1` schemas in
  `crypto-autoresearcher` were inspected for the executable and evidence
  contracts. They require an independently reproducible draw stream,
  append-only subprocess evidence, and a schema-valid export rather than a
  narrative-only result.
- `examples/dreg_high_arity_analyze.rs`,
  `examples/dreg_high_arity_report.rs`, and this study's `run.sh` were
  cross-checked for the exact resource/deterministic-failure taxonomy,
  shared-control conditioning, retained 19-row evidence, and five-package
  publication mapping. The documentation below registers those executable
  contracts without treating incomplete validation as a scientific result.

## Frozen instruments, environment, and inputs

Only these native Rust examples are scientific instruments:

```text
examples/dreg_ladder.rs
examples/dreg_closure.rs
examples/dreg_high_arity_analyze.rs
```

Before generation, the frozen protocol binds the registered source commit,
release-binary hashes for the three scientific examples and dedicated report
example, analyzer and report source hashes, compiler and container digest,
`Cargo.lock`, historical input and expected identity row, and the exact source
paths enumerated by the runner and
`sources.sha256`. The runner compares `Cargo.toml`, `Cargo.lock`, `build.rs`,
root `src/**`, all four examples, the runner, and the source manifest to the
registered commit. Cargo path-dependency trees require a full checkout but
are not separately claimed as enumerated source coverage; the byte-identical
release binaries bind the code that is actually executed.

The execution ref may be a pushed descendant that adds the frozen protocol or
committed draw inputs, but the instrument, report, and runner paths must remain
byte-identical to the registered source commit. Every invocation requires a
fully qualified `DREG_EXECUTION_REF=refs/heads/...`, checks its live
`git ls-remote origin` object ID equals `HEAD`, and then checks that the
registered source commit is an ancestor. A local or stale remote-tracking ref
is insufficient. Every stage also refuses a dirty or mismatched instrument.

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
as evidence and never promoted as a committed input. Structural
self-consistency is not enough: the frozen native ladder audit independently
replays the seed-derived RNG stream and recomputes every scientific field,
including basis, target, exact solution count, and outcome, comparing all
fields except diagnostic timing. It rejects blank, malformed, extra, missing,
duplicate, reordered, or skipped records. Each `.generation.json` sidecar
binds the installed JSONL and generator log by path, SHA-256, byte length, and
record count and retains the exact audit argv. The first four UNSAT and first
two SAT records by draw index form the frozen panel. A capped stream without
both quotas leaves that cell unmeasured.

## Execution units

Build the exact frozen commit in the recorded container, then generate only:

```sh
cargo build --release --locked \
  --example dreg_ladder \
  --example dreg_closure \
  --example dreg_high_arity_analyze \
  --example dreg_high_arity_report
for step in gen-m5-n11-l2 gen-m5-n13-l3 gen-m6-n11-l2; do
  research/dreg_high_arity_20261101/run.sh \
    target/release/examples/dreg_ladder \
    target/release/examples/dreg_closure \
    target/release/examples/dreg_high_arity_analyze \
    target/release/examples/dreg_high_arity_report "$step"
done
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure \
  target/release/examples/dreg_high_arity_analyze \
  target/release/examples/dreg_high_arity_report audit-draws
```

Commit and push each of the three replay-audited draw files with its `.log`
and `.generation.json` custody sidecars before closure execution. Then submit
one detached job for each exact logical name in exactly the displayed order:

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
  target/release/examples/dreg_high_arity_report \
  <exact-logical-name>
```

Each invocation starts exactly one closure subprocess, has a 24-hour process
timeout and 60-second `TERM` grace, and writes one new append-only attempt at
`$DREG_OUTPUT_ROOT/runs/<logical-name>/attempts/<NNNN>/`. Its stdout, stderr,
exit status, host facts, frozen identities, environment manifest, checkpoint
generation, and resume provenance are retained. Metadata, status, and
finished records are installed atomically. Every attempt directory has an
atomic `allocated.json` with schema `dreg.high-arity-allocation/v1`. A final
directory containing only that marker and recognized setup temporaries is
reused and does not consume one of the four attempts; metadata or process
evidence commits the allocation to the retry count. An orphan setup temporary
without process evidence is retryable, not invalid evidence. Checkpoints are
shared between attempts of the same logical name and never across draws.
Committed generations live only at
`checkpoints/<logical>/gen-<20 decimal digits>/checkpoint-manifest.json`.
The shell preflight and native store both require the manifest to cover exactly
every ordinary generation file, with unique safe basenames and a hashed
`catalog.json`, and require its `spec_sha256` to equal the catalog's system
identity; an omitted, extra, or misbound state file invalidates that generation.
Four recognized infrastructure/resource attempts make the subprocess
terminal-unmeasured; they do not make it false. An assertion, panic, or other
unrecognized nonzero exit is a deterministic instrument failure and is
invalid, not resource censoring.

The identity and six SAT controls must reach terminal records before the
twelve candidate scans. A malformed or contradictory terminal control stops
execution. A resource-unmeasured identity or SAT control does **not** suppress
candidate scans: the candidate rows are still pursued and retained, while all
cell-package conclusions remain conditional and therefore unmeasured until
the shared gate passes.

After all terminal records are present, run the frozen classifier:

```sh
research/dreg_high_arity_20261101/run.sh \
  target/release/examples/dreg_ladder \
  target/release/examples/dreg_closure \
  target/release/examples/dreg_high_arity_analyze \
  target/release/examples/dreg_high_arity_report analyze
```

The runner uses a full repository checkout. On detached workers it requires an
absolute durable `DREG_OUTPUT_ROOT` exactly equal to `$EPHEMERAL_OUT`. If
`$EPHEMERAL_RESUME_DIR` is present, the runner builds a deterministic
SHA-256 manifest of the prior evidence tree, copies it into an initially empty
`$EPHEMERAL_OUT`, recomputes the destination manifest, and requires exact
agreement before starting a new attempt. It additionally requires
`DREG_RESUME_ANCHOR` to name an absolute ordinary manifest, outside both old
and new roots and outside the repository, that checkpoint transport retained
independently before transfer. The recomputed immutable-source manifest must be
byte-identical to that anchor; otherwise import fails before attempt allocation.
A v3 import record retains descriptors for the copy, immutable, and anchor
manifests. The old tree is never modified. Resumed algebraic counters remain
valid; all wall and CPU timing fields are marked invalid.

After every detached attempt stops and before checkpoint transport snapshots
the output tree, invoke the same frozen runner and binaries with step
`seal-resume-anchor` and an absolute `DREG_RESUME_ANCHOR_OUT` outside
`DREG_OUTPUT_ROOT` and the repository. This validates current bindings and
create-new installs the exact immutable manifest. The transport retains that
sidecar separately and supplies its restored path as `DREG_RESUME_ANCHOR`; an
anchor recomputed only after an incomplete source arrives is inadmissible.

The checked-in `launch-request.json` is a blocked prelaunch record, not an
authorization to spend. The whole campaign has one USD 20.00 ledger covering
builds, queue/idle time, all successful, failed, interrupted, and Spot-restart
attempts, checkpoints, storage, and transfer. It permits at most one
16-vCPU/64-GiB-or-smaller worker at a time, one build attempt, three generator
attempts, and 76 closure attempts: one first attempt plus at most three retries
for each of the 19 subprocesses. Its original absolute deadline is
`2026-12-01T00:00:00Z`. Any unknown or insufficient account-budget or lane
headroom, projected ceiling overrun, or expired deadline stops submission and
does not reset on a retry.

Launch is blocked until the runner has the deployed ephemeral-stack identity:
current STS resolves to account `590183823895`, not stack account
`104186811093`, and `/ephemeral/config` is unavailable. The 2026-11-01 cost
inspection also recorded both USD 25 daily AWS budget thresholds above their
limits, so no headroom is established. The request therefore retains no
backend submission command and cannot launch.

The three exact generator commands and exact analyzer command are already in
the request. After generation, but before closure spending, replace the empty
closure-command list with the exact 19 draw-specific commands in registered
order and commit and push the replay-audited draw files and custody sidecars.
Also record the exact backend submission command and immutable request
identity. Before any cloud build or job, commit and push the exact code,
frozen protocol, inputs or hashed manifests, configuration, seeds, and launch
request to the focused experiment branch; run the request's
`check-experiment-git` argv against that pushed HEAD; and retain and bind its
publication receipt to the image and request. No other account, region, pool,
provider, or renamed retry may bypass a stop. These operational blocks are not
scientific results and leave every affected lead open.

## Exact per-subprocess classification

The native analyzer validates every attempt and selects a terminal record only
under the frozen rules. It requires exactly one JSON object for a successful
closure attempt, exact cell and draw identity, and no contradictory fields.
It also cross-binds that object's `checkpoint_segments` to the selected
attempt's validated restore metadata: a fresh store must report exactly one
segment and a restored store at least two. Missing or inconsistent segment
provenance invalidates the claimed-success record.

- `closure <= 4`: a complete degree-3 or degree-4 refutation. An early
  degree-3 refutation is complete evidence and does not require a degree-4 row.
- `above 4`: no refutation, `measured_to = 4`, and a present, completed,
  saturated degree-4 row.
- `unmeasured`: an explicitly recognized timeout (`124`, receipt label
  `timeout`), corrupt/malformed/wrong-system checkpoint (`125`,
  `checkpoint_invalid`), SIGKILL/OOM or timeout-kill (`137`, `signal_kill`),
  checkpoint `TERM` (`143`, `signal_term`), exact Rust allocator-failure address-space
  event (`134` plus the registered stderr line, `address_space`), row/column cap,
  corrupt checkpoint, infrastructure interruption, exhausted
  infrastructure-only retry budget, or incomplete scan whose non-success
  status identifies that censoring. Other signals are deterministic failures,
  not presumed resource events.
- `invalid`: a claimed-success attempt with missing, malformed, duplicate,
  wrong-cell/draw, missing-required-degree, or contradictory output; an
  assertion, panic, or unrecognized nonzero exit; a refuted satisfiable
  control; or an algebraically different identity replay.

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

Retain immutable draw JSONL/logs and hash-binding generation sidecars, native
replay receipts, every append-only attempt, checkpoints, source and binary
hashes, compiler/container identity, sanitized environment, host facts,
protocol hash, pushed execution ref and commit, v3 resume-import
copy/immutable/external-anchor manifests, and every failure. The native
analyzer emits `summary.json`, a
deterministic summary retaining all 19 subprocesses and every attempt. It does
not currently emit an `experiment.export/v1` bundle or report files.

Invalid resume-import custody is retained as invalid instrument evidence and
must still produce an invalid/no-conclusion publication once frozen provenance
is sound. If the frozen protocol, source, or binary-manifest provenance itself
is unavailable or invalid, the publisher cannot authenticate the analyzer and
must refuse publication rather than render an untrusted conclusion.

The preregistered publication contract is specified in
[REPORT_EXPORT_CONTRACT.md](REPORT_EXPORT_CONTRACT.md). Before protocol
freeze, the dedicated native `dreg_high_arity_report` example must pass the
registered contract tests, be source- and binary-hashed, and be added to the
frozen publication path. Its present implementation accepts the immutable
analyzer summary and frozen protocol, validates their exact
19-row/five-package structure, and may translate but must not reclassify them.
Immediately before publication the runner reruns the frozen analyzer over the
retained evidence and requires the complete summary to match byte-for-byte;
the publisher separately rederives the row, cell, package, and global grammar.
The export binds the frozen protocol and summary hashes and has five logical
package rows with
`sample_size.instances = 1`; each row links its complete subprocess evidence.
Resumed rows explicitly invalidate wall and CPU time.

The final report includes a source-linked Markdown narrative, quantitative
SVG, Typst source, inspected PDF, and independent review record under the
sibling `reports/reviews` directory rather than inside the strict five-file
publication bundle or its raw-evidence traversal. No review,
report, or atlas disposition may call a timeout, OOM, cap, partial scan, or
launcher-account mismatch a disproof.
