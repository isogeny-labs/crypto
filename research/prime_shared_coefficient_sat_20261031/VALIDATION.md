# Validation

## Completed checks

| Check | Result |
|---|---|
| Rust 1.78 locked release tests | 25 passed, 0 failed; library and binary test targets exit zero |
| Rust 1.98 `cargo clippy --all-targets -- -D warnings` | exit zero |
| Rustfmt check | exit zero |
| Legacy m3/m5/m7 witness evaluation | exact variable/equation counts and zero failed equations |
| Legacy `a0+1` mutations | rejected in all three fixtures |
| Complete F17 all-target census | 18/18 exact projected coefficient-model-set matches; 6 SAT, 12 UNSAT, 6 models per implementation |
| Strict parser controls | unknown fields, trailing text, malformed decimals/order rejected |
| Exporter determinism | byte-identical repeated JSON/msolve/QF_FF output in unit tests |
| Boolean arithmetic and canonical range | all 289 F17 operand pairs and all 32 five-bit words pass exhaustive checks |
| CNF/CNF-XOR gate exactness | every truth-table row for positive and negated AND/XOR gates passes in both renderings |
| E1 parser/control construction | malformed/truncated/trailing/XOR-as-CNF controls reject; m3/m5/m7 fixed controls construct and replay |
| msolve output semantics | strict finite/empty/positive-dimensional parser; Fp root filtering, coordinate sign, variable reorder/name checks, unreduced printed linear-form normalization, full canonical replay and mutations pass |
| msolve fixed-input runner construction | all canonical fields are fixed write-once; exact full assignment and `a0+1` empty-set controls are represented; no study process launched |
| msolve 0.11.0 upstream suite | 71 passed, 0 failed for commit `cc044551f8e202d00691990fd3fd0b1226a88ea2` |
| msolve semantic tool smoke | `x^2+1` over F3 yields an algebraic-closure parametrization with no F3 root; adding `x^3-x` yields `[-1]` |
| cvc5 finite-field proof smoke | trust-free complete checking stops at `THEORY_FF`; Alethe export rejects `CONST_FINITE_FIELD`; cvc5 is not proof-bearing for E1 |
| Comparator input hash register | 12/12 source, protocol, witness and E0-reference hashes verify |
| Earlier SAT input hash register | 12/12 verify at its declared frozen source commit `8cc48bdeedbe6284a4df2ab0d0a049feb1b11523`; current-head `lib.rs` and `main.rs` intentionally differ because comparator modules/CLI were added later |
| Parent protocol reconciliation | frozen E1 files retained byte-for-byte; unrun variable-arity E3 and `0.85/0.95` thresholds retired in favor of fixed-`m=3` `e<0.5`, `a<1.5` complete-cost gates |
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
cargo test --release --locked --offline --all-targets
cargo fmt -- --check
cargo clippy --locked --all-targets --offline -- -D warnings
cargo run --release --locked --offline -- e0-panel
# From the repository root:
cargo test --release --locked --offline --lib
```

The final compiler checks used locally installed Rust 1.78 and 1.98
toolchains with build targets under `/tmp`; generated target files are not part
of the study tree.

The two E1 subgates have distinct source revisions. Verify and run the SAT
panel from `8cc48bdeedbe6284a4df2ab0d0a049feb1b11523` with `inputs.sha256`;
verify and run the later msolve comparator from its exact launch revision with
`e1-comparator-inputs.sha256`. Updating the older register to match current
CLI wiring would silently rewrite a frozen experiment and is not permitted.

## Environment and exclusions

Pinned local development builds now exist for msolve 0.11.0, cvc5 1.4.2,
CaDiCaL, `lrat-trim`, CryptoMiniSat and `cake_xlrup`. The SAT proof toolchains
passed a two-clause contradictory proof smoke, msolve passed its upstream
suite and a semantic base-field smoke, and cvc5's finite-field proof limitation
was reproduced. None saw a study instance. `perf` access, an authorized
checkpoint credential and a compatible isolated runner image remain absent.
Consequently:

- no solver benchmark or scientific timing run occurred;
- no SAT or UNSAT result from an external backend on a study instance is reported;
- canonical msolve output was accepted only in non-study semantic smokes; the
  18-target and fixed-control panel remains unrun;
- QF_FF artifacts remain syntax-shaped and deterministic, while cvc5 1.4.2 is
  proof-ineligible for finite-field UNSAT under the frozen policy;
- both frozen CNF/CNF-XOR and msolve panels remain E1 obligations;
- parent-protocol semantic integration, E2 power, detector and isolation gates
  remain open.

`scripts/checkpoint.sh status` recognizes the resume/spec state but reports the
last durable push as stale or absent. No solver panel may launch until an
authorized checkpoint path makes the gate exit zero. Credentials must be
installed through the repository's checkpoint mechanism and must not be
placed in reports or chat.

## Evidence boundary

The E0 census is exhaustive only for the fixed F17 arity-three panel. It checks
the semantic projection, not asymptotic scaling. The historical m5/m7 rows are
known-answer conformance checks, not unknown solve results. The enumerator and
group/interpolation oracle are algorithmically separate but same-author and
share basic finite-field/polynomial primitives; they are not an independent
external replication. The variable-arity performance draft was retired before
data because it did not implement the parent complete-cost bridge. No affected
performance data exist, and E1 success alone would not admit this
representation to the fixed-`m=3` screen.
