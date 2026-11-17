# Phase-demand replay, controls and proof validation

Run `phase-demand-20261025` starts at
`fda3018794b3e840fc7656a17fa9195cfaf13652`. The main protocol is
committed at `d25dd8cfe`; the exact29-counter contract and compiled native
source at `ac0915b5e` precede the first producer execution. Full commit pins
are retained in the raw archive. Source `INPUTS.sha256` binds the final
producer, helper snapshot, independent auditor, Cargo inputs, protocols,
Lean source and existing backend. `OUTPUTS.sha256` binds every canonical
native result, independent receipt and frozen input-pin list.

## Checks performed

| Command/check | Outcome | Raw record after restoring the archive |
| --- | --- | --- |
| `bash research/quadratic_twist_evidence_20261021/run-phase-demand.sh FRESH_DIR` | PASS;40 setups,400 separate targets and400 source scalar checks; exact case/audit bytes | `runs/final/producer.log`, `audit.log`, empty `cases-diff.log` |
| Independent arithmetic, incremental demand replay and old batch-RREF/witness audit | PASS;100 distinct solver certificates; all29 vectors and phase boundaries checked | canonical `native/phase-demand-20261025/audit.json` |
| Rehashed false setup total, matrix demand, omitted failed query, base log, scalar, source binding | each exit101, no receipt | `runs/final/controls/`; exact changed JSON and failure log |
| Unmodified scoped repository proof gate | PASS;185+22+19+5=231 audited declarations /176 public research theorems; zero failing/extra trusted | `runs/proof-gate-final/`, `runs/local/proof-gate-final.log` |
| `cargo test --release --locked --lib --quiet` | PASS;4479 passed,119 existing ignores | `runs/local/cargo-lib-final.log` |
| New auditor crate `cargo test --release --locked` | PASS;22 backend tests | `runs/final/backend-tests.log` |
| `python3 -m unittest scripts.test_workflows_parse` | PASS;4 metadata parser tests | `runs/local/workflow-tests.log` |
| `backlog validate XP-20261025-known-factor-base-roots` | PASS; unexecuted draft only | `runs/local/backlog-validate.log` |
| Full curve-name metadata linter | PASS;0 problems | `runs/local/naming-full.log` |
| Current-main official artifact policy on added/changed paths | PASS; no selected file needs the store | `artifact-policy-check.json` |
| Report `make check`, PDF text equality and inspection of both rendered pages | PASS | `runs/local/report-build-02.log`, source-linked report/graph/PDF |

Every native producer/auditor has a600-second cap; no cap was reached.
The code ran on the existing Linux x86-64 cloud workspace with no isolation
grade earned for timing. CPU, memory and disk snapshots are in `runs/local/`.
Build/test elapsed times in logs are diagnostics, never research timings.
Counts are29 explicitly unpriced request components; nested counters are not
summed into a time/GAE figure. Fixed field-table setup, hashing, allocation,
sorting, traversal, internal exponentiation and independent audit remain
unpriced. No claim or row is added to benchmarks/leaderboards.

## Immutable inputs and failed attempts

All20 F43 policy source JSONs (ten preceding reduced cases and ten adaptive
cases) are pinned in `native/phase-demand-20261025/source-inputs.sha256`.
The new helper is a verbatim prefix of the frozen `native/adaptive-s3.cpp`
before `int main`; the frozen source is unchanged. Each query actually
executes the new counter-instrumented solver and compares its complete result
with its frozen reduced certificate by actual x coordinate; no scalar lookup
is returned to the recovery routine. Independent Rust reproduces the whole
certificate and ledger with separate arithmetic, and also invokes the old
batch semantic audit. The first and fresh final runs have identical results.

After the first successful native audit, the only native diagnostic change
replaced a full-object `assert_eq` dump with the same equality predicate and
a short error message. It changes no comparison, algorithm or counter. The
pre-execution source remains committed, and both successful run records are
retained. Six controls are exercised by the final exact CI command.

The first Lean build used a division-lemma name unavailable in Lean4.34.1.
`runs/failed/PhaseRecovery-attempt01.lean.txt` and `lean-build-01.log`
retain that actual source and failure. The corrected `Int.emod_add_mul_ediv`
build is `runs/local/lean-build-02.log`; the separate full scoped proof gate
contains zero sorry/custom/native trusted axioms. Only propext,
Classical.choice and Quot.sound occur. Toolchain and Mathlib remain pinned
to Lean4.34.1 / d13f23b723b8a846827a245b89c10fc7d3f11612.
The five new theorems concern additive groups and actual F43 recovery;
no new polynomial identity or IDC1 source is introduced. All14 earlier
identity records remain unchanged. External lean4checker was unavailable;
no external checker is claimed.

The first report's PDF optimization moved Graphviz's `fi` ligature into a
different text order. The failed text-equality check, exact diagram source
and log remain in `runs/failed/`. The diagram labels now say “Checked”, and
the final two-page PDF has identical extracted text before/after optimization.
Both final pages were rendered and visually inspected. The new PDF and raw
archive stay below64KiB, every case JSON stays below1MiB, and no external
upload or credential reuse occurred.

## Restore and audit the raw tree

The raw tree is a deterministic `runs.tar.xz`, with
`runs.tar.xz.sha256`. Exact canonical cases must stay loose because the
replay and source bindings name those repository paths. The archive omits
only byte-identical duplicates of those JSONs; it retains all precise changed
corruption payloads, logs, metadata and failed sources. `RUNS.sha256` binds
the entire original raw tree before duplicate dehydration. Restore it with:

```
bash research/quadratic_twist_evidence_20261021/validation/phase-demand-20261025/restore-runs.sh FRESH_DIRECTORY
```

The script hydrates each canonical final/control case tree, overlays the
archive and checks every original file hash. No file in the repository or
an earlier frozen run is altered. The compiled producer binary is omitted;
its hash and byte count are retained in `runs/local/producer-binary.sha256`
and its build/source inputs are retained. Rebuild it using the replay script.

Extracting the archive without hydration and repacking gives the same bytes:

```
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > repacked.tar.xz
```

Full restoration checks all138 original file hashes, and deterministic
repacking was also checked before publication. The lead-atlas match and its exact commit pin
are in `runs/local/`. The pre-edit shared Conductor check still exited127;
no reservation or replacement local coordinator is claimed. Current PR CI,
reviews, integration and broader scientific completion gates remain open.
