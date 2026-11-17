# Quotient-coordinate calibration: protocol and native source preflight

**Current destination: `Isogeny-labs/crypto`. Source preflight only; target calibration,
inversion and anchor-free initialization have not executed.**

This relocates the experiment originally prepared in
[ghs-trapdoor PR #10](https://github.com/aburan28/ghs-trapdoor/pull/10).
[PROVENANCE.json](PROVENANCE.json) pins that repository's exact source commit,
input hashes and byte counts. The historical manifests in `fixtures/` are
copied byte for byte; no challenge was replaced, relabeled or evaluated on
the target. The original Python execution path is replaced by Rust for this
repository, with the original normal-Weil-function formulas and coefficient
ordering. This is a correctness replay, with no performance comparison.

The initial delivery to `aburan28/crypto` is now
[`isogeny-labs/crypto` PR #2569](https://github.com/isogeny-labs/crypto/pull/2569).
The unchanged repository ID, `1221029056`, confirms the transfer. GitHub access
is restored and the approved auto-merge is enabled, subject to the normal
conflict, CI and review gates.

The approved 67,540-byte raw-log archive is published in the artifact store.
A fresh download into an empty cache reproduced its SHA-256 exactly; see the
[publication receipt](reports/publication-20261026T1437Z/STATUS.md) and tracked
[artifact manifest](reports/run-20261025T1845Z/.artifacts.json). To hydrate it
with configured AWS access, run from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull \
  research/quotient_calibration_20261025/reports/run-20261025T1845Z/runs.tar.xz
```

The [initial approved-publication record](reports/publication-20261025T2144Z/STATUS.md)
and [repository follow-up](reports/repository-20261025T2314Z/STATUS.md) preserve
the earlier access failures. Original run records retain their historical state.

Read [PROTOCOL.md](PROTOCOL.md) for the full five-gate experiment: the fixed
source over F343, its order-373 subgroup, kernel 153, chart transforms,
divisor origin, certified inherited anchor, field-embedding requirements,
training frame, all 32 challenges, controls, exceptional-fiber treatment,
group checks, budgets and stop rules. The historical settings file retains
its original paths; they resolve in the pinned source repository. The active
native adaptation is stated at the beginning of the protocol.

The [native target continuation](target-native/README.md) now implements
section-space divisor arithmetic, target Kummer evaluation and separate
calibration/evaluator/reference interfaces. Its fixed historical checks
pass; its certified-anchor frame and challenge runs remain unexecuted
pending configured checkpoint access and a qualified EKS target image/route.
Same-branch checkpoint uploads were approved on 2026-10-30. See the
[current review report](reports/review-20261030T1540Z/REPORT.md) and historical
[preparation report](reports/preparation-20261026T1824Z/REPORT.md).

The [run report](reports/run-20261025T1845Z/REPORT.md),
[one-page PDF](reports/run-20261025T1845Z/REPORT.pdf) and
[validation record](reports/run-20261025T1845Z/VALIDATION.md) distinguish
the passing native preflight from the unrelated baseline library gate.

## Reproduce the source preflight

From the repository root, with Rust 1.99.1 (the recorded replay toolchain):

```sh
cargo test --release --locked \
  --manifest-path research/quotient_calibration_20261025/native/Cargo.toml
cargo clippy --release --locked \
  --manifest-path research/quotient_calibration_20261025/native/Cargo.toml \
  --all-targets -- -D warnings
cargo run --release --locked \
  --manifest-path research/quotient_calibration_20261025/native/Cargo.toml \
  -- --output /tmp/native-calibration-preflight.json
```

Use a fresh output path: the executable refuses to overwrite an existing file.
The reference manifest is SHA-256 checked before use; `--reference PATH` can
select another local copy of those exact bytes, not a replacement experiment.
The native program independently checks modulus irreducibility, the cubic
embedding, normal 2/3-Weil cocycles, source order, 32 source-point additions,
all 64 section values per pilot point, rank, all nine frame minors, vectors,
frame weights and the coordinate-sampling stream. It does not enumerate
source discrete logs to label challenges or read any target image table.
The exact normalized section-matrix digest must match
`52f1938e51d9baca3e9d1753a90516611468ed1472c08203decbdd0a41ab87fe`.
The even tensor seed `(0,0,0)` and kernel basis are frozen inputs; this replay
does not repeat the historical seed search or kernel census.

## Requirement status

| Requirement | Status and evidence |
| --- | --- |
| Relocate to the requested repository | This study and native package in `isogeny-labs/crypto`, PR #2569; original work is left intact. |
| Preserve the case and challenges | Verified exact historical manifest and settings hashes in PROVENANCE.json. |
| Re-run the source preflight natively | Rust replay checks all frozen source outputs; see the run receipt and report under `reports/`. |
| Fit and freeze the target matrix | Not implemented/executed here. Nine fitted frame pairs alone are not validation. |
| All 32 exact target images, inverse fibers, signs and group relations | Not executed; the original gates remain required. |
| Negative target controls, exception coverage, time/memory accounting | Specified, not executed. |
| Profile A initialization | Still depends on the certified inherited anchor; no replacement arbitrary generator is permitted. |
| Profile B initialization and global section certificate | Not implemented/executed. A Profile A pass cannot establish Profile B. |

## Evidence and interpretation

The native replay is a separate implementation check against historical
Python/FLINT data, not an independent-host validation of the target map.
The large historical input reports remain available by immutable commit URLs
in PROVENANCE.json. Their full bytes were fetched and their SHA-256 digests
checked during relocation; the native evaluator does not read those reports.
No large raw table is copied into Git merely to duplicate the upstream source.

The next discriminating step is to evaluate the nine certified target frame
images under the fixed charts and origin, freeze the matrix, then require
32/32 exact holdout matches before attempting polynomial inversion. The old
F7^12 inverse splitting field cannot contain the F7^18 source coefficients;
explicit compatible embeddings are still required. See the protocol for the
F7^108 support plan and its degree-at-most-three prerequisite.

This delivers the relocated design and source replay. It establishes no new
vulnerability, complete transfer, general map identity, cost advantage or
claim about deployed curves.
