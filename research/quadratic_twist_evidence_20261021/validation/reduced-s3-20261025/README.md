# Reduced S3 validation

Run `reduced-s3-20261025`, preregistered at commit2ee8f6a21 before execution;
starting head115963f0bc4f7c61a8d4a46d55a05fbcadfa075b. This is a fresh native
experiment/proof continuation, not another CI-only status update.

| Requirement | Executed result | Receipt |
| --- | --- | --- |
| Full declared finite workload |98 size2 cases,188 comparisons,1,384 targets; C++/Rust independently agree on every polynomial, root/point lift, row/rank, affine/homogeneous matrix and declared count | native canonical cases/audit; `runs/replay-final/` |
| Matched-base controls | both transport directions reproduce rank and degree profiles; fixed base-field maps checked on points | case roles `matched`, `pulled_back_twist_base`; independent native audit |
| Semantic corruptions | false remainder, false degree, false rank and missing case reject101, no receipt; modified case hashes are refreshed so rejection exceeds checksum checking | `runs/replay-final/controls/` |
| Exact native CI command | fresh producer/auditor/certificate outputs byte-match all frozen new outputs | `runs/replay-final/`, `replay-final.log` |
| New Lean module |15 named theorems,22 audited declarations; division, ideal equality, constrained roots, actual coefficients/leading forms and cutoff | `lean-gate-final/...lean-reduced/` |
| Full unchanged scoped gate |185 previous +22 new =207 audited declarations,0 failing/extra trusted;159 named research theorems total | `proof-gate-final.log`, `lean-gate-final/` |
| Polynomial identity | IDC1h77b3276833a18411,32 issue+16 replay points; false identity/source controls reject | native `certificate/` |
| Fresh mandatory library suite |4,479 pass,0 failures,119 pre-existing ignores | `cargo-lib.log.xz` |
| Isolated backend |22 tests pass | `runs/replay-final/backend-tests.log` |
| Workflow metadata |4 tests pass | `workflow-tests.log` |
| New follow-up card | draft validates; no execution/lease/funding claimed | `backlog-validate.log` |
| Report | two pages rendered and visually inspected; optimized PDF text exactly matches LaTeX PDF | `pdf-inspection.txt`, `pdf-build-final.log`; report Makefile |
| Shared coordination | actual checks exit127; no reservation/replacement claimed | `conductor.log`, `backlog-conductor.log` |

`INPUTS.sha256` binds the protocol, new native/Lean sources and the125 original
case/index pins. `OUTPUTS.sha256` binds canonical new records, certificates,
reports and validation receipts. Original frozen INPUTS/OUTPUTS files and all
four already published payload hashes remain unchanged and verify separately.
The old Mathlib/Lake/Cargo/library sources and registry are not rewritten.

## Exact reproduction

```sh
bash research/quadratic_twist_evidence_20261021/run-reduced-s3.sh FRESH_DIRECTORY
tools/lean/proof-gate.sh --out FRESH_GATE research/quadratic_twist_evidence_20261021/lean research/quadratic_twist_evidence_20261021/lean-reduced
cargo test --release --locked --lib --quiet
python3 -m unittest scripts.test_workflows_parse
target/release/backlog validate XP-20261025-reduced-s3-mutant-reinsertion
```

Lean4.34.1/Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 and Rust1.99.1
are pinned. Supported writable `MATHLIB_CACHE_DIR` avoids the read-only default
cache; `CARGO_HOME`, `RUSTUP_HOME`, `ELAN_HOME` select the installed workspace
toolchains. Network permission is used for dependency access and the library's
existing loopback tests. The inherited proxy/CA remain enabled. No gate/test
is patched, disabled or skipped; no external lean4checker is claimed.

Every native producer/auditor command has a600-second cap; none was hit.
Test/build elapsed times are transcript diagnostics, not research timings.
There is no new benchmark/leaderboard row, runtime ratio or external-fastest
claim. Affine profiles start at the maximum input degree; homogeneous profiles
start at0, with null caps retained. The first-target displayed fixture has
only quadratic inputs so degree0/1 contains no input multiples.

## Failed attempts and presentation

The first C++ build included the old renamed main and failed its return-type
warning. Its exact source/log are retained; a verbatim helper-only snapshot
avoids importing that main without modifying the frozen original or weakening
compiler flags. The first new-project build raced dependency setup and failed
proxy access before proof checking; it is retained as infrastructure failure.
Subsequent elaboration failures and exact `.lean.txt` source snapshots are
retained, including provisional sorryAx diagnostics that are never accepted.
Final gate receipts have zero such failures.

The first frozen-input check caught an edit to the older pinned workflow.
That failed check is retained. The old workflow was restored byte-for-byte;
the new experiment has its own workflow, and all original hashes then pass.

The first report build lacked a TeX symbol package; failed source/log remain.
The final report uses standard fonts and size-optimized PDF rewriting, with text
equality checked by the Makefile, and both final pages visually inspected.
All new selected PDF/archive assets are below the repository's64KiB data
threshold; compact JSON outputs remain below1MiB per file. No new external
upload or credential reuse occurs. Earlier reports stay frozen.

Raw native/proof failure records are packed deterministically as `runs.tar.xz`.
Unchanged duplicate case files live once in the loose canonical `native/`
records rather than being copied five more times into the archive. Exact
changed control payloads remain in the archive; `CASE_OVERLAYS.json` names the
duplicates and the deliberately missing case. Reconstruct the complete raw
tree, retaining mutations/deletion, then check every original raw file hash:

```sh
bash research/quadratic_twist_evidence_20261021/validation/reduced-s3-20261025/restore-runs.sh FRESH_DIRECTORY
```

This is byte hydration only, not mathematical replay. All original raw-tree
hashes remain in `RUNS.sha256`, and rehydration verifies them. The archive is
54,908 bytes; no splitting or external upload is used. Compiled binaries/build
caches are omitted, with binary hash/compiler versions recorded instead.
The same native sources/seed inputs reproduce all checked result bytes.

The next adaptive-row experiment is a validated draft card, not a running job.
The existing single four-hour automation resumes the queue. Current-head CI,
review, geometric/full-work and integration/merge gates remain open.
