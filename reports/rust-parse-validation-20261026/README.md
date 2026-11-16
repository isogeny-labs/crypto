# Release-library parser repairs on `crypto` main

The required `cargo test --release --locked --offline --lib -- --test-threads=1`
check could not reach tests on `main` commit
`03a001c9b2cd880284a2c34b6b45cc8437905bae`. Three source files had
unclosed Rust syntax. Sequential compiler runs exposed the next error only
after the previous file was repaired in a separate local branch. The original
files in that commit have SHA-256 values:

| Source | SHA-256 | Compiler evidence |
| --- | --- | --- |
| `src/cryptanalysis/binary_semaev_chain_sat.rs` | `b4ac996629bb17005d691d4cd5f179143c4fab51ada2e207a185f547980d9b71` | [first failure](01-chain-sat.log) |
| `src/cryptanalysis/ecbench/methods.rs` | `52f499e441b301811c26a0ce01f54286434b31e682bf316e7e6e99bc127e2ddd` | [second failure](02-ecbench-methods.log) |
| `src/cryptanalysis/ecbench/workload.rs` | `98359cda4b008f394b521b1c2478353f623872713dd7c155388b5cd35aae28ff` | [third failure](03-ecbench-workload.log) |

The initial parser repair closes the `verify_external_assignment` method after its existing
Boolean tail expression, completes `SolveReport` with the same `field_ops: None`
used by adjacent constructors and closes the returned struct and function,
closes the `GhsFp6Explicit` enum variant, and removes duplicated alternatives
from two match arms while preserving their full variant sets. Those syntax
edits do not intentionally change arithmetic, search, measurement, or result
logic. The touched workload file's imports are sorted as Rustfmt requests.

The first compile after those syntax repairs exposed two `E0119` errors:
`FrobeniusOrbitChoiceWires` had two stacked derive attributes that each
generated `Clone` and `Debug`. The redundant `#[derive(Clone, Debug)]` was
removed; the existing `#[derive(Clone, Debug, PartialEq, Eq)]` remains. That
compile also reported 40 missing `include_str!`/`include_bytes!` inputs caused
by the sparse checkout. All 32 unique inputs are tracked at the base commit
(about 96 MB total) and were hydrated unchanged. The mixed compiler log is
[preserved](04-sparse-and-derive.log); its missing-file errors are checkout
artifacts, while `E0119` is a real source error.

Relevant prior sources were the three original repository files and their
adjacent constructors, plus the official Rust Reference on
[function bodies](https://doc.rust-lang.org/reference/items/functions.html),
[enum variants](https://doc.rust-lang.org/reference/items/enumerations.html),
and [match alternatives](https://doc.rust-lang.org/stable/reference/expressions/match-expr.html).
The reference confirms the required block, variant, and or-pattern delimiters;
the repository context determines the preserved fields and variant sets.

Runtime logs numbered 05–12 are stored as deterministic, lossless gzip files
so the exact test output remains intact without generated whitespace in the
patch. [The raw-log manifest](raw-log-manifest.json) records each uncompressed
SHA-256 and byte count; `gzip -dc FILE.log.gz` reads a log.
The 79,877-byte `10-broader-run-before-fixture-recheck.log.gz` is held by the
repository artifact store; its tracked [pointer](.artifacts.json) preserves the
gzip SHA-256 and size. Run
`python3 tools/artifacts/artifacts.py pull reports/rust-parse-validation-20261026`
to hydrate it before reading or decompressing it.

Local validation so far:

- A read-only `rustfmt --edition 2021 --emit stdout --config skip_children=true`
  sweep parsed all 947 tracked `.rs` files under `src/`; zero files failed
  parsing. This checks syntax, not types or tests.
- `rustfmt --check --edition 2021` passed on all four touched source files.
- `git diff --check` passed.
- The hydrated release-library binary reached 1,910 passing tests, 16 failures,
  and 47 ignored tests before the unchanged
  `jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
  occupied the run for over ten minutes. A previous full-suite run recorded
  the same long blocker. Because the suite could no longer pass, this attempt
  was interrupted with SIGINT; exit 101 and the partial raw output are
  [preserved](05-release-lib-interrupted.log.gz). This is **not** a full-suite
  verdict or pass.
- Each of the 16 failures was rerun alone with `--exact --nocapture`. Twelve
  passed after their tracked runtime fixtures were hydrated in this sparse
  checkout; the remaining four are recorded in the
  [focused recheck](06-focused-failure-recheck.json). Three share a method
  registration collision: `ic.hyperelliptic` first resolves to the historical
  genus-2 declaration, which does not accept `branches` or `d_step_in`.
  The fourth reports a legacy comparison id for explicit pricing weight zero,
  contrary to its own test and the fact that the weight enters the identity.

The source repair now selects the hyperelliptic thread declaration only when
its required thread-specific parameters are present. It retains the historical
genus-2 defaults and the committed `ECM1ha6ff9c61b9eb` method id, and rejects
either variant on the other's workload. Explicit weight zero no longer receives
a legacy id; an absent weight still does.
The first post-fix `cargo test --release --locked --offline --lib` build ran
for its 600-second watchdog and exited 124 while compiling the library,
without a compiler diagnostic; its [stdout](07-first-postfix-compile-timeout.log.gz)
is retained. The second, 1,800-second bounded release build completed in
13m25s; the new historical genus-2 method-id regression passed on the rebuilt
binary, with 5,015 tests filtered out. Its [log](08-postfix-focused-build.log.gz)
is retained. Each of the four remaining failures from the original full run
then passed on this same rebuilt binary with `--exact --nocapture
--test-threads=1`; the [summary](09-rebuilt-focused-summary.json) links the
four raw outputs. These tests validate the affected routing and comparison
cases, not the hyperelliptic algorithm.

A broader run of the same rebuilt binary used `--test-threads=1 --skip
jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
It completed 4,887 passes, 121 intentional ignores, seven failures, and the
one explicit skip in 879.07 seconds; its [raw log](10-broader-run-before-fixture-recheck.log.gz)
is preserved. All seven failures were the CryptoPro-B tests reading
`research/cryptopro_b_chain_sweep_20261006/frozen_inputs/chains.constants.json`.
That 205,232-byte file is tracked at the base commit but omitted from this
sparse checkout; every failure reported `No such file or directory` for that
path. After hydrating the tracked blob unchanged (SHA-256
`683f92d0e56f5606f78b6c685f747a7b67106a7b5e5883e81869c8d6199d7336`),
all seven passed individually on the same binary. The
[recheck summary](11-cryptopro-fixture-recheck-summary.json) links the exact
outputs. The separately bounded long `jv_isogeny_walk` case then passed on
the same binary in 185.91 seconds with exit code 0; its
[raw output](12-long-jv-exact.log.gz) is losslessly preserved with SHA-256
`cf57bcb58d5f388c600c90bfe0cd16e1c20c3e6c26147d207c58ebec0f37a2f2`.
These combined runs cover every nonignored library test, but no single
unqualified full-suite pass is claimed.

For this routing decision, the relevant repository sources are the historical
genus-2 [spec](../../research/ecbench_jacobians_transfers_20261025/specs/genus2.json)
and [plan](../../research/ecbench_jacobians_transfers_20261025/sessions/genus2/plan.json),
the [hyperelliptic IC note](../../research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md),
and `docs/ecbench/README.md` §method identities. Rust's official
[`Iterator::find` documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
explains why the first duplicate declaration masked the later one. The
underlying Gaudry/ADH algorithm, historical measurements, and the note's
36-run smoke claim are **not** revalidated by this routing fix. The original
Gaudry paper's IACR PDF was inaccessible through this host (HTTP 403), so this
report makes no new claim about its algorithm or novelty.

A successful parse, the affected tests, and the combined broad and exact runs
are limited gates. Hosted current-head CI remains open before an unqualified
green verdict.
