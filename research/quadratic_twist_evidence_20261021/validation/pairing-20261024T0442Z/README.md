# Pairing-angle validation and retained attempts

Run id `pairing-20261024T0442Z`, 2026-10-24. Starting published head
`ad66884e6be033ec8264ccb52500ef5e98f2c44b`; protocol-only commit
`dcd89832fc6169dd71dad048cc59a70f694a66bc` preceded execution. New implementation
sources were uncommitted during execution; their exact bytes are identified
by `INPUTS.sha256`, not by mislabelling the starting Git head as their revision.

| Check | Actual outcome | Receipt |
| --- | --- | --- |
| C++ strict compile and full sweep | pass; 942 models, 274 shared cases and 4,616 auxiliary points | `native-attempt-02/`; final replay in archive |
| Independent Rust replay | pass; coefficient pairs, exponent inversion, linear Miller, first legal offsets and Gaussian ranks | `audit-run-final.log`; frozen `../../native/pairing-20261024T0442Z/audit.json` |
| Exact final CI command | pass; results/audit/certificate bytes, all controls and backend tests | `ci-replay-final.log`, `.exit`; archived `runs/ci-final/` |
| Unmodified study proof gate | pass; 68 named theorems, 85 audited declarations, zero failing/trusted | `proof-gate-final.log`, `.exit`, `lean-gate-final/` |
| Full release library suite | 4,479 passed, zero failed, 119 existing ignores | `cargo-lib.log.xz`, `.exit` |
| Isolated native backend | 22 passed, zero failed/ignored | archived `runs/ci-final/backend-tests.log` |
| Workflow metadata parser | four tests pass | `workflow-tests.log`, `.exit` |
| Norm polynomial certificate | 32+16 replay points; false-plus-one and false hash rejected | frozen `certificate/`, final raw issue/check logs |
| Four corrupted result payloads | each exit101, no audit output; specific expected diagnostics | archived `runs/ci-final/controls/` |
| Report | four pages built and inspected | `pdf-build.log`, `.exit`, `pdf-inspection.txt` |
| Extra external Lean checker | unavailable; no external replay claimed | `extra-checker.log` |
| Actual shared Conductor | unavailable, exit127; no reservation/replacement claimed | `conductor.log`, `.exit` |

## Exact commands and provenance

The final shell harness is retained as `run-pairing-source.sh.txt` and active
at `../../run-pairing.sh`. It compiles C++ with
`-std=c++17 -O2 -Wall -Wextra -Werror -lcrypto`, builds the isolated Rust crate
with `cargo build --release --locked`, caps both full native replays at 600
seconds, compares exact frozen JSON bytes, checks the norm certificate, runs
all four corrupted payloads, then runs the 22 native backend tests.
The source workflow is retained as `workflow-source.yml.txt`; the active
workflow uploads the fresh pairing output directory even on failure.

Final commands from the repository root:

```sh
bash research/quadratic_twist_evidence_20261021/run-pairing.sh /tmp/twist-pairing-ci-final
tools/lean/proof-gate.sh --out research/quadratic_twist_evidence_20261021/validation/pairing-20261024T0442Z/lean-gate-final research/quadratic_twist_evidence_20261021/lean
cargo test --release --locked --lib
python3 -m unittest scripts.test_workflows_parse
```

The proof command uses `ELAN_HOME=/workspace/twist-tooling/elan` and supported
`MATHLIB_CACHE_DIR=/workspace/twist-tooling/cache/mathlib`. Rust uses
`RUSTUP_HOME=/workspace/twist-tooling/rustup`,
`CARGO_HOME=/workspace/twist-tooling/cargo`, `CARGO_BUILD_JOBS=2`; versions are
retained in the final environment receipt. Library tests run with networking
enabled for the existing localhost socket tests. No test is disabled or skipped
to obtain a pass; the 119 ignores pre-exist this round. Python runs only the
existing workflow YAML metadata suite and lays out recorded report data.

`INPUTS.sha256` binds new sources, prior included native arithmetic, all seven
Lean modules, pinned Cargo/Lake inputs, the registry and the existing identity
backend. `OUTPUTS.sha256` binds frozen results, audit, certificate, final proof
receipts, raw archive and report assets. `git-provenance.txt` binds the starting
root Cargo/source trees and verifies unchanged frozen study sources. Seeds are
430000+n for the eleven IC fixtures, 20261024 for IDC1 issue points and 2381
for the 16 extra certificate points. The run records contain every chosen
point/offset; all finite population loops are deterministic.

The 11 IC certificates check projected factor-base field equations and full
rank over F11. This validates the recorded solution, not every random-search
attempt counter. No timing series, CPU isolation, matched rho, external-fastest
comparison or performance measurement was requested or admitted in this round.

## Failed and intermediate attempts

- `native-attempt-01/` retains the first source and failed strict compilation:
  two helper names did not exist in the included frozen backend. The failure
  occurred before the workload. `native-attempt-02/` retains the corrected
  source, passing compile, full sweep log and exits.
- `proof-attempt-01/` and `02/` retain kernel/elaboration failures around
  elliptic scalar evaluation; `03/` first checks the 121-pair table. Further
  sources/logs `04/` through `06/` retain failed explicit twist/Frobenius and
  generic character elaborations; provisional axiom printouts are not passes.
  `07/` passes all added statements with two redundant-tactic warnings;
  `08/` retains the cleaned final source and the passing gate build log.
  All failed sources use `.lean.txt`, outside proof-gate target discovery.
- Auditor builds/replays `01`, `02` and final are retained. The final revision
  adds explicit native control-count fields; earlier successful audit JSONs
  remain as `audit-attempt-01.json` and `audit-attempt-02.json`. The initial
  auditor build did not capture a source snapshot; `audit-source-02.rs.txt`
  and `audit-source-final.rs.txt` retain the later exact sources. The final
  hashed full replay is authoritative for current claims.
- The first and final full harness runs both pass. The final changes add the
  independent control-count receipt and the specific count-mismatch diagnostic.
  Both raw trees are retained. No reported timeout occurred.

## Raw archive and inspection

`runs.tar.xz` contains both raw harness trees, exact corrupted JSON payloads,
specific diagnostics/exits, input/output manifests, environment/tool versions,
certificate issue/check logs and backend tests. Compiled executables are
excluded; the sources and build commands regenerate them. Extract with:

```sh
cd research/quadratic_twist_evidence_20261021/validation/pairing-20261024T0442Z
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

The original inner manifests preserve their absolute `/tmp/twist-pairing-ci-*`
output paths. `RUNS.sha256` checks the portable extracted tree from this
directory. Packing is deterministic:

```sh
tar --sort=name --mtime='2026-10-24 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs |
  xz -6 -T1 > runs.tar.xz
```

Individual proof/failed-source/PDF receipt paths remain loose because review
and proof maps name them. None of the original uncompressed constituent files
exceeds the repository's 10MiB rule. Earlier committed evidence is not repacked.

`mov-crossref.json` retains the DOI response used to verify the primary paper's
title, authors and year. `../../reports/pairing-20261024T0442Z/` retains editable
Markdown/TeX, diagram/graph sources and every rendered copy. All four final PDF
pages were rendered with `pdftoppm` and inspected for content and legibility.

General geometric pairing correctness/nondegeneracy, uniform DLP reduction,
general normalization, exceptional S3/S4 lifting, universal trace-zero and actual
shared coordination remain open. Existing conflicts, complete-check access,
review and exact-head CI are separate merge gates. No inherited SEA/global
gate or external service failure is patched; no frozen evidence is rewritten.
