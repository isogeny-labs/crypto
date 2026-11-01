# Validation

## Completed checks

| Check | Result |
|---|---|
| Rust 1.78 locked release tests | 6 passed, 0 failed; library, binary and doc tests exit zero |
| Rust 1.98 `cargo clippy --all-targets -- -D warnings` | exit zero |
| Rustfmt check | exit zero |
| Legacy m3/m5/m7 witness evaluation | exact variable/equation counts and zero failed equations |
| Legacy `a0+1` mutations | rejected in all three fixtures |
| Complete F17 all-target census | 18/18 exact projected coefficient-model-set matches; 6 SAT, 12 UNSAT, 6 models per implementation |
| Strict parser controls | unknown fields, trailing text, malformed decimals/order rejected |
| Exporter determinism | byte-identical repeated JSON/msolve/QF_FF output in unit tests |
| CLI m3/t4 smoke | JSON has 33 variables, 35 equations and 6 projected variables; msolve emits 37 lines; QF_FF emits 33 declarations, 35 assertions, `check-sat` and `get-value` |
| Repository release-library gate | completed in 1,673.72 s: 5,796 passed, 12 failed, 128 ignored; see exact failure boundary below |

The Rust 1.78 invocation emits the repository's pre-existing parent-manifest
warning about `lints.rust.unexpected_cfgs.check-cfg`; the tests still exit zero.

The repository-wide gate did not exit zero. Nine unchanged Cairn/network tests
failed when the managed sandbox refused their process/network operation with
`EPERM`; two unchanged ecbench publication tests could not find an expected
sidecar; and the unchanged
`cryptanalysis::ic_wide::tests::run_f2m4_is_the_one_word_pipeline` assertion
observed `13130` instead of `3757`. All 12 failing test sources are outside
this standalone study and are unmodified by this branch. This result is
reported as a failed broad gate, not converted into a pass.

## Commands

```bash
cargo test --release --locked --offline
cargo fmt -- --check
cargo clippy --all-targets --offline -- -D warnings
cargo run --release --locked --offline -- e0-panel
# From the repository root:
cargo test --release --locked --offline --lib
```

The final two compiler checks used locally installed Rust 1.78 and 1.98
toolchains with build targets under `/tmp`; generated target files are not part
of the study tree.

## Environment and exclusions

`msolve`, `cvc5`, `cadical`, `cryptominisat` and `perf` were not installed on
the current host. No AWS identity or authorized isolated runner was configured.
Consequently:

- no solver benchmark or scientific timing run occurred;
- no SAT or UNSAT result from an external backend is reported;
- QF_FF and msolve artifacts were syntax-shaped and determinism-tested, but not
  accepted by their external parsers in E0;
- CNF/CNF-XOR exporters and proof checkers remain E1 obligations;
- E2 power, detector and isolation gates remain open.

## Evidence boundary

The E0 census is exhaustive only for the fixed F17 arity-three panel. It checks
the semantic projection, not asymptotic scaling. The historical m5/m7 rows are
known-answer conformance checks, not unknown solve results. The enumerator and
group/interpolation oracle are algorithmically separate but same-author and
share basic finite-field/polynomial primitives; they are not an independent
external replication. The protocol was kept draft; no affected performance
data exist.
