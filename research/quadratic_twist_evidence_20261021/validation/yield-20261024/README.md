# Relation-yield and conditional-degree proof validation

Run id `yield-20261024`, started2026-10-24T19:12:12Z, new managed workspace.
Starting published head `171285079f285b65f1b2443d3bc8c22d5d307d6e`;
proof plan committed before execution at `65bea0a7ef8dad84453c30d61bdc9efedee28354`.
Sources were developed after that protocol commit; their exact bytes are bound
by `INPUTS.sha256`, not mislabelled as already present in the starting head.

| Check | Verified result | Evidence |
| --- | --- | --- |
| Final Lean build | passes all ten modules | `proof-attempt-10/` |
| Unmodified repository gate |127 declarations, zero failing/extra trusted | `proof-gate-final.log`, `.exit`, `lean-gate-final/` |
| Named research statements |106 total, including38 new statements | three new source modules and their `#print axioms` |
| Actual F5 controls | one versus three raw zeros, zero nonzero twist rows modulo3 | `YieldFixture` statements in the gate |
| Source-bound cleared-cost IDC1 |32+16 points, false-plus-one and false source hash rejected | `certificates/`, `cert-issue.log` |
| Full new native CI command | passes exact-byte replay and controls | `ci-replay-final.log`, `.exit`, archived `runs/ci-final/` |
| Isolated backend suite |22 passed, zero failed/ignored | archived `runs/ci-final/backend-tests.log` |
| Existing workflow metadata suite | four passed | `workflow-tests.log`, `.exit` |
| Final full release library suite |4,479 passed, zero failed,119 existing ignores | `cargo-lib-final.log.xz`, `.exit` |
| Shell syntax | passes `bash -n run-yield.sh` | final shell source / command |
| Report | four final pages rendered and inspected | `pdf-build-inspected.log`, `.exit`, `pdf-inspection.txt` |
| Shared Conductor | unavailable, exit127; no reservation/replacement | `conductor.log`, `.exit` |
| Additional external Lean checker | unavailable; no external replay claimed | `extra-checker.log` |

`RelationYield` proves the entire accepted coefficient set is identical under
an injective additive map, instantiates on the actual twist point map, and
proves actual coefficient span/rank equality over any field. Its full-group
bijection is a separate uniform-sampling theorem. `RelationCost` proves explicit
conditional budget/overhead/degree-model implications, not an actual curve
regularity bound. The F5 correctness control and synthetic rational-budget
examples remain separate. There is no performance run or admitted IC/rho point.

## Commands, sources and pins

```sh
tools/lean/proof-gate.sh --out research/quadratic_twist_evidence_20261021/validation/yield-20261024/lean-gate-final research/quadratic_twist_evidence_20261021/lean
bash research/quadratic_twist_evidence_20261021/run-yield.sh /tmp/twist-yield-ci-final
cargo test --release --locked --lib
python3 -m unittest scripts.test_workflows_parse
```

Lean4.34.1/Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` remain pinned.
The proof command uses `ELAN_HOME=/workspace/twist-tooling/elan` and
`MATHLIB_CACHE_DIR=/workspace/twist-tooling/cache/mathlib`; the gate is not patched.
Only `propext`, `Classical.choice`, `Quot.sound` occur in the accepted audit.
There are21 generated helpers beyond the106 named research theorems.
Unsuccessful elaborations' provisional `sorryAx` output is not a proof receipt.

Rust1.99.1 uses `RUSTUP_HOME=/workspace/twist-tooling/rustup`,
`CARGO_HOME=/workspace/twist-tooling/cargo` and `CARGO_BUILD_JOBS=2`.
The native certificate backend is the existing repository code; no field or
curve implementation changes. IDC1 issue seed20261024 has32 points and extra
seed2381 has16. The bound statement is only the polynomial part of clearing
budget denominators. Lean separately proves the positive-denominator inequality
and the other structural results. All old source-bound certificates remain exact.

`INPUTS.sha256` binds ten Lean sources, pins, native crate/lock/backend inputs,
the protocol, root Cargo inputs and registry. `OUTPUTS.sha256` binds final gate,
native certificate/control archive and report assets. `git-provenance.txt`
records unchanged root source/Cargo trees and old frozen study sources.
The exact native harness/workflow and final Lake configuration are retained
as source snapshots. No Python curve arithmetic, research harness or verifier
is introduced: Python only runs an existing YAML metadata suite, the lead-atlas
metadata query, and report layout of recorded Lean-certified integers.

## Unsuccessful/intermediate attempts retained

- `proof-attempt-01/` fails because `variables` was used as a parameter name,
  which is a reserved Lean token. `02/` passes the initial yield/cost/fixture
  statements. `03/` fails a new full-group count extension because the indexed
  finite-function enumeration needed `DecidableEq` on its index. `04/` passes
  that correction. `05/` through `07/` retain successful intermediate ratio,
  span/rank and overhead extensions. `08/` and `09/` retain a failed Pascal
  recurrence rewrite in the strict-degree proof. `10/` passes the final source.
  Every snapshot ends `.lean.txt`, so failed source is not discovered as a
  standalone proof target. No placeholder or custom axiom is in the final source.
- The first full Rust suite could not compile because29 tracked fixture files
  were absent from the initial sparse checkout. After compile-time fixtures
  were materialized, `cargo-lib-02` ran all tests:4470 passed,9 failed,119 ignored.
  The nine failures were missing runtime evidence/chain-set files, not source
  arithmetic regressions. The repository's existing sparse-checkout helper then
  materialized tracked references, without editing them or patching tests.
  The final full run passes4,479 tests with119 existing ignores. All three
  transcripts remain as `.log.xz`, with their original exit files.
- The first cache command failed before Lake execution because stdout was
  redirected to the wrong relative validation path. `cache-setup-01.log` is a
  retrospective diagnostic note of that shell error. `cache-setup-02.log` is
  the actual corrected cache transcript; all8908 cached modules decompressed.
- The first metadata-only atlas query lacked a tracked orchestration dependency
  in its sparse checkout. After that dependency was materialized, the exact
  `--match` rerun passes. Both outputs/exits are retained. No atlas disposition
  is changed. `atlas-handoff.json` supplies existing lead IDs for the next review.
- The minimal Rust installation initially lacked rustfmt; the component was
  installed and the final native source formatted. Compiler/build and final
  exact-byte replay receipts remain. No failed certificate arithmetic is hidden.
- Initial PDF inspection found an overflowing dependency pin and a long proof-map
  cell. Their layout was corrected; build transcripts and final inspection are
  retained. Formulae and proof claims did not change during those layout fixes.

## Raw tree and report inspection

`runs.tar.xz` contains the fresh native CI tree: exact certificate and corrupted
payloads, arithmetic counterexample, source-hash rejection, build/test logs,
input/output digests and tool versions. It contains no compiled executable.
Extract/check from this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c RUNS.sha256
```

The inner original output manifest retains its absolute `/tmp/twist-yield-ci-final`
paths. `RUNS.sha256` checks the portable extracted tree. Deterministic packing is
`tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs | xz -6 -T1`.
Individual proof/source/PDF receipt paths remain loose because the report and
proof map reference them. No original constituent exceeds10MiB, and no older
committed tree is repacked. Large generated cost graphs, registry, leaderboard
and conflicted claim files stay unchanged.

All four final PDF pages were rendered with `pdftoppm` and inspected. The
dependency-pin/table layout corrections and the final natural-number-domain
annotation were inspected after rendering; unchanged pages were also checked
for identical image bytes. Editable Markdown, TeX, diagram and plot
sources accompany the PDF. The graph lays out the integers1 and3 from the
kernel-checked fixture, with the zero-nonzero-row control stated explicitly.

## Acceptance boundary

The requested structural and conditional theorems are verified in their stated
domains. Actual useful-yield improvement on small factor bases, lower actual
regularity, a regularity/solving-degree/work bridge, a compatible main-DLP
reduction and a measured pipeline advantage remain unestablished. No task is
represented as running on those gaps. Existing general geometry, shared
Conductor, CI/access/review, catalog integration and merge-conflict holds remain.
