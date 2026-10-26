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

The repair closes the `verify_external_assignment` method after its existing
Boolean tail expression, completes `SolveReport` with the same `field_ops: None`
used by adjacent constructors and closes the returned struct and function,
closes the `GhsFp6Explicit` enum variant, and removes duplicated alternatives
from two match arms while preserving their full variant sets. No arithmetic,
search, measurement, or result logic is intentionally changed. The touched
workload file's imports are sorted as Rustfmt requests.

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

Local validation so far:

- A read-only `rustfmt --edition 2021 --emit stdout --config skip_children=true`
  sweep parsed all 947 tracked `.rs` files under `src/`; zero files failed
  parsing. This checks syntax, not types or tests.
- `rustfmt --check --edition 2021` passed on all three touched source files.
- `git diff --check` passed.
- The complete release-library test is being rerun after the derive repair and
  tracked fixture hydration. A parser sweep alone is not a test pass.

A successful parse is only the first gate. The library test, affected tests,
and any new compiler errors must be checked before calling the branch green.
