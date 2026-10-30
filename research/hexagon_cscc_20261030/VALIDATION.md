# Software validation and evidence boundaries

These are development correctness checks on public fixtures. Compiler/test
durations in raw logs are not benchmark evidence. No paid experiment, EKS
job, curve claim, speedup claim or formal-proof promotion is recorded.

## Passed checks

| Command / control | Observed result | Raw output |
| --- | --- | --- |
| `cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml` | 40 library tests, 3 existing runner tests and 1 new factoring CLI test pass | `checks/tensor-tests.txt.gz` |
| Tensor `cargo clippy --locked --all-targets -- -D warnings` | Pass | `checks/tensor-clippy.txt` |
| `cargo +1.78.0 check --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets` | Pass on the declared minimum toolchain | `checks/tensor-msrv.txt` |
| `cargo test --release --locked --manifest-path tools/factor-base/Cargo.toml` | 22 pass, 1 pre-existing fixture writer ignored | `checks/factory-tests.txt.gz` |
| Factory strict Clippy | Pass | `checks/factory-clippy.txt` |
| `cargo test --locked --manifest-path tools/complexity-catalog/Cargo.toml` | 5 pass: complete inventory, source metadata/coverage rejection, path resolution and two finite construction controls | `checks/catalog-tests.txt.gz` |
| Catalog strict Clippy | Pass | `checks/catalog-clippy.txt` |
| Factory `verify` on all six frozen manifests | All replay; same native implementation | `checks/frozen-archives.txt` |
| Factory `replay` on frozen binary evaluation receipt | Semantic digest `d54ee493f3d99a33616de283b55a9c0dd93f48c01bb90278d84a0064aa5dde07` reproduced | `checks/frozen-replay.txt` |
| Report rendering | Seven current Typst pages; all text bounds within pages; every page inspected and no clipped source entries | `report.typ`, `report.pdf`, `integration.svg` |

The primary development compiler is Rust 1.99.1 on Linux x86_64. Formatting
and `git diff --check` are checked separately before publication. Verbatim logs
with trailing blank lines are stored losslessly as deterministic gzip. Source hashes
are in `sources.sha256`. This hash list binds selected files; it is not a
general mathematical or execution-proof certificate.

## Specific properties covered

- Divisor counts match a separately written trial-division reference for
  every prefix on four tiny integers, including primes, a square and a
  composite. Prefix order, zero padding and exclusion of divisor one are
  exercised. Complete factorization includes prime powers, and minimum-factor
  boundary conventions match Mathlib. The CLI emits no success JSON on a
  resource failure, zero factorization or malformed negative input.
- Weighted controls reproduce a triangle Max-Cut profile and an even-parity
  Hamming-weight profile. Zero-variable constants, inconsistent constraints,
  duplicate variable indices, coefficient-bound overflow and exhausted work
  have distinct checked outcomes.
- Prime recipes are compared with explicit finite-field formulas/norm-equation
  enumeration. Wrong exact orders, a square quadratic parameter, zero coset
  parameters, exceeded construction caps and an empty subgroup intersection
  are rejected. Existing archive/replay is exercised for every new family;
  decimal/hex coset encodings canonicalize to the same archive identity.
- The source catalog covers all 22 versioned subject entries, including all
  three v2 records. Missing/duplicate versions, invalid or mismatched PDF
  digests, unsupported proof promotion and broken integration pointers fail
  its checks. The two finite mathematical controls do not claim the source
  papers' infinite-family bounds.

## Broad root-library check

The required `cargo test --release --lib` initially failed because sparse
path dependencies and 31 archived `include_str!`/`include_bytes!` inputs were
off disk. The tracked inputs were materialized without changing their bytes.
Those failures are preserved in `checks/root-sparse-path-error.txt` and
`checks/root-sparse-includes-error.txt`; they are not reported as passing runs.

The follow-up command is:

```sh
CARGO_PROFILE_RELEASE_OPT_LEVEL=1 CARGO_PROFILE_RELEASE_DEBUG=0 cargo test --release --lib -j 1 -- --test-threads=2
```

It runs the full test set, with reduced compilation optimization and two test
threads to fit the development workspace. The initial compiled panel contains
5,553 tests and is still running at this source checkpoint. Sparse runtime
inputs also caused failures; 39 additional unchanged tracked inputs have been
materialized for follow-up checks. This broad panel is not reported passed.
Final observations and the integration checks are recorded separately below.
No performance inference uses this build.

## Errors retained and corrected

The first factoring test used an inferred signed integer type; an explicit
unsigned conversion fixed it. The factory initially called a test-only field
power helper; production prime checks now use the existing integer modular
power implementation. The independent torus control first used the wrong
multiple of 101, then exposed a genuinely empty subgroup intersection; the
positive shifted fixture and explicit empty negative control preserve both
cases. A catalog pointer to `src/ml_dataset.rs` was corrected to the existing
directory. Raw failed attempts are retained under `checks/`, alongside final
passing outputs. None is interpreted as a mathematical rejection.

Conductor was not installed (`conductor check` returned command-not-found).
The behavior and missing reservation are recorded here under the repository's
documented unavailable-hook behavior in `docs/CONDUCTOR.md`; no reservation or
independent review is claimed. A transient execution-transport disconnect
recovered; completed output files were inspected before deciding what to rerun.

## Sibling and formal-check limits

Cryptanalysis's four new tensor source/test files match this change byte-for-byte;
eight pre-existing differences in other tensor files remain untouched. Its
component checks are retained in `cryptanalysis:docs/ic/hexagon-cscc-20261030/`.
Crypto Autoresearcher ingests 22 Hexagon notes plus the direct PKM review and
regenerates its indices. Its full ledger validator refuses a sparse checkout;
the bypass was not used and full-corpus validation remains unpassed locally.

No source Lean development, general contraction proof, new asymptotic theorem,
subexponential weighted implementation or fast Subset Sum/3SUM engine was
independently verified or completed here. Those source claims and implementation
gaps remain explicit in the catalog and existing tensor audit problem record.

## Integration with current upstream

Merged main `3b34d504b` into the feature source. The two module export
conflicts retain both additions, the general audit retains both history
paragraphs, and both complete parent journals remain ordered subsequences.
No frozen evidence conflict was resolved or rewritten. Independent main
changes are not claimed as part of this implementation.

The combined tensor package passes 75 tests: 50 library, eight runner and
17 integration tests. All-target strict Clippy, formatting and Rust 1.78
checks pass (`checks/tensor-integrated-*`). Catalog formatting, strict Clippy
and all five tests pass against the combined tree (`checks/catalog-integrated-*`).
The standalone factory's source, manifest and lock are unchanged by the merge;
its earlier 22-test and archive replay outputs remain applicable. Selected
current source hashes are refreshed in `sources.sha256`.

The broad root run remains tied to initial source checkpoint `333a4d55b` in a
separate detached worktree. It is not represented as a check of the subsequently
merged main. Its runtime-input failures and any follow-up observations are
retained separately; no complete root pass is claimed here.

Cryptanalysis integrates both its bindings package (88 tests) and the newly
landed suite dependency (34 tests), with both Clippy/MSRV checks passing.
The new four files are identical across those two copies and crypto; existing
other differences remain. Its PR is https://github.com/isogeny-labs/cryptanalysis/pull/1320.
Knowledge intake is https://github.com/isogeny-labs/crypto-autoresearcher/pull/2767.

The three committed structured prime examples also build through the native
CLI batch interface and verify through the archive interface; outputs and the
raw batch receipt are retained as `checks/prime-example-cli*`. Diagnostic wall
times in that receipt are unqualified and support no performance claim.

## Broad-check publication checkpoint

`checks/root-library-progress.json` binds an incomplete raw-output snapshot
(`root-library-progress.txt.gz`) to source `333a4d55b` and records that the
5,553-test broad panel is still running. It is not a successful full-suite
receipt and is not substituted for validation of the later integrated source.

The first follow-up passes the five CPU-emulator tests and exposes 13 further
missing archive inputs. After restoring 87 unchanged tracked files, the second
passes 13 and exposes five more missing inputs; restoring those files makes
all remaining five pass. Thus all 18 originally observed runtime-input failures
have a successful unchanged-source follow-up. Raw failed follow-ups, passing
outputs and materialized path lists are retained as `checks/crypto-root-*`.
No source, assertion, timeout or limit was changed to obtain these passes.
The complete original broad run remains pending at this publication checkpoint.
