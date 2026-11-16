# Checked-root comparison: exact validation and retained failures

Run `known-roots-20261025` starts at7fe697ae9b5d7f460e5d23ac160e22d1f94eba9c.
Protocol17a757495 and compiled source/proof freeze3ad11f812 precede the first
new native execution. That execution rejects the invalid ascending-base premise.
Correction83941917e preserves base-column order and sorts only the query roots;
it is committed before the successful paired run. Full commit hashes, exact
sources, toolchains, host metadata and command ledger are in the raw archive.

| Check | Executed outcome | Restored raw evidence |
| --- | --- | --- |
| `bash research/quadratic_twist_evidence_20261021/run-known-roots.sh FRESH_DIR` | PASS;80 paired setups/800 separate verified source recoveries; exact canonical result/audit/certificate bytes | `runs/final/` |
| Separate native arithmetic and exact request replay | PASS; all32 components; roots, witnesses, degrees, queries, rows, ranks, logs and scalars; unchanged scan reference | canonical `native/known-roots-20261025/audit.json` |
| Old independent batch-RREF/witness semantic checker | PASS;100 distinct complete solver certificates | `runs/final/audit.log` |
| Nine rehashed semantic corruptions | each exit101/no receipt | `runs/final/controls/`, precise changed case/index bytes and logs |
| Source-bound monic-membership IDC1 | IDC1he9f02ada0008638c;32+16 native points; false identity/source rejected | `runs/final/certificate/`, `certificate.log` |
| Unmodified scoped Lean proof gate | PASS;182 public named theorems /238 audited declarations; zero failing/extra trusted | `runs/lean-gate-final/`, `runs/local/proof-gate-final.log` |
| Fresh root `cargo test --release --locked --lib --quiet` |4479 passed,0 failures,119 pre-existing ignores | `runs/local/cargo-lib-final.log` |
| Isolated auditor backend tests |22 passed | `runs/final/backend-tests.log` |
| Workflow YAML metadata tests |4 passed | `runs/local/workflow-tests.log` |
| Both affected backlog cards | PASS; drafts, no routing-state promotion | `runs/local/backlog-validate-final.log` |
| Full curve-name metadata linter |0 problems | `runs/local/naming-full-final.log` |
| Current-main official artifact policy | no new/changed selected files | `artifact-policy-check.json` |
| Report build, no overfull/reference errors, PDF text equality | PASS; both final pages rendered and inspected | `runs/local/report-build-02.log`, report sources/PDF |

Every native command has a600-second cap; none is reached. Existing local
resources only, two build workers; no paid provisioning or uploads. Build/test
elapsed times are diagnostics, not isolated research timings. The32 counters
are differently weighted, sometimes nested algorithm requests. Field tables,
sorting, allocation, hashing, traversal, exponentiation and audit remain unpriced.
The six new public Lean theorems plus one generated definition helper use only
propext/Classical.choice/Quot.sound. Lean4.34.1 and Mathlib
d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned. No trusted native evaluation,
custom claim axiom or external lean4checker replay is claimed.

## Frozen references and failures

Thirty exact JSON inputs (ten reduced, ten adaptive, ten phase-demand cases)
are bound by `native/known-roots-20261025/source-inputs.sha256`. Each newly
executed scan reference equals its frozen phase record after dropping the
three added zero counters. Both native implementations execute the candidate
and unchanged matrix algorithm; the complete candidate certificates equal
the original adaptive ones by actual target x coordinate. No scalar answer
is passed into query or recovery. `INPUTS.sha256` binds final sources, Cargo,
protocol/correction, Lean and existing certificate/hash backends.

`runs/failed/producer-attempt-01/` retains a pre-execution C++ build failure
(wrong helper name), exact source and compiler log. The first actual native
execution rejects `distinct ascending roots` at3ad11f812, exits1 and produces
no accepted case. Its full source/log and empty output directory remain in
`runs/failed/native/`; the auditor was not started. The transported controls
have valid ordered base columns with nonascending x-coordinates. The committed
correction sorts a separate query list, never the base columns or logs.
The first successful and fresh final runs reproduce the same canonical results.

The first report optimization moved Graphviz's `fi` ligature into another text
order, and the text-equality check failed. Exact diagram/report source, before/
after text and log are in `runs/failed/`. A linear diagram with unambiguous
labels passes text equality; both final pages were then rendered and inspected.
Draft-card schema/placeholder mistakes are preserved in `runs/failed/card-attempt-01/`
and corrected without changing their draft/main-ancestry/Cairn gates.

The current-main profile tool could not probe because gawk is absent, exit2;
its exact source hash and diagnostic are retained. It supplies no profile or
cost evidence. Conductor exits127 and no shared task lease/replacement is
claimed. Neither access failure is presented as a mathematical obstruction.
No calibrated S, speed ratio, rho/external-fastest or leaderboard promotion
is claimed. All14 old identity sources/records and prior frozen rounds remain.

## Restore the exact raw tree

```
bash research/quadratic_twist_evidence_20261021/validation/known-roots-20261025/restore-runs.sh FRESH_DIRECTORY
```

Canonical case JSONs stay loose because the replay and verifier name their
paths. The archive dehydrates only byte-identical copies of those cases in
the final/control trees; every changed control case and rehashed index stays
exact. Restoration first copies canonical records, then overlays the archive
and checks **208 original file hashes** in `RUNS.sha256`. No previous frozen
file is modified. The compiled producer is omitted, with its hash/size and
exact build retained. `OUTPUTS.sha256` binds the canonical output directory.

`runs.tar.xz` is deterministic. Extract it without hydration and repack:

```
tar --sort=name --mtime='2026-10-25 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1 > repacked.tar.xz
```

Full restoration and byte-identical repacking are checked before publication.
The archive is34,768 bytes; the inspected PDF56,331 bytes, below the64KiB data
threshold. All case JSONs are below1MiB. No credential reuse/upload occurred.
Current-head CI access, review, conflicts, broader mathematical and shared
coordination holds remain. The next signed-cache draft is not a running job.
