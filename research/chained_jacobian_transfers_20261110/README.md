# Chained Jacobian transfers

Current continuation: [report](REPORT_20261111.md), [PDF](report.pdf),
[small-prime supplement](SUPPLEMENT_20261111.md), [supplement PDF](small-prime-report.pdf),
[persistent queue](QUEUE.md), [sources](SOURCES_20261111.md),
[GitHub #3904](https://github.com/isogeny-labs/crypto/issues/3904).

`native/` is the execution path for new development. It is a standalone,
dependency-free Rust crate for public F101 fixtures and model-tagged quadratic
splittings. Its default API uses a fixed field. The opt-in `small-prime`
feature adds checked odd primes≤4096, with enumeration capped at101; see
the supplement and external-source input manifest. The historical
Python files, protocol and 2026-11-10 receipt are retained evidence; their
old status paragraphs do not describe the continuation. Do not extend or use
those Python paths for new comparisons.

```sh
cargo test --release --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml -- --nocapture
cargo clippy --locked --offline --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml --all-targets -- -D warnings
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/ChainTransfer.lean
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/PolynomialIdentities.lean
```

The cross-repository reference is
`cryptanalysis:experiments/chained-transfer-reference/`. The comparison uses
pre-existing BigUint arithmetic after a faithful monic chart change; both
implementations use Cantor and the runs are by this producer. This is
differential development evidence, not signed independent verification.
All current claim standings must be read through `tools/pcc/pcc.py standing`.
[pcc:unverified]

The fixed fixture now has full rational source maps, a genuine Richelot hop,
its sign-corrected dual, nonzero and zero endpoint controls, and a complete
finite-group deduplication against an elliptic shortcut. Its target model is
isomorphic to the source. A second preselected fixture has nonisomorphic
Jacobians over F101 (rational2-torsion4 vs16, each order9216), but its complete
96-point elliptic endpoint also matches a direct2-isogeny after doubling.
Its preserved prime subgroup has order3. A predeclared second Richelot hop
now reaches a third Jacobian with rational2-torsion8; the reverse-kernel
intersection is trivial and all full-domain dual checks pass. The three
counts4,16,8 distinguish the three Jacobians over F101. The report gives the precise scope
and open geometric proof bridge. No performance run or speed claim is present.
[pcc:unverified]

Pinned oracle baseline: [cryptanalysis PR #1469](https://github.com/isogeny-labs/cryptanalysis/pull/1469),
merged; [successor PR #1471](https://github.com/isogeny-labs/cryptanalysis/pull/1471)
pins b280efc with27 passing tests/1 ignored benchmark. The third model's
independent BigUint development checks pass; its immutable replay is tracked
separately from those retained checkpoints. [pcc:unverified]

Benchmark/integration blockers have a
[named record](../../problems/chained-transfer-admission-and-ledger-sync.md).
Root-library checks and the new focused CI are tracked in the session log and
PR. A source checkpoint does not waive their gates.

The next bounded prerequisite is implemented in the sibling reference:
[cubic arbitrary-class norm](CUBIC_NORM_20261111.md),
[diagram and PDF](cubic-report.pdf). Whole-fiber cubic pullbacks preserve
full prime-order109/19 sources through genera4/13; an actual genus4 norm
handles closed support through degree4. All130 fixed norm witnesses pass a
separate root arithmetic replay. Genus13 recovery is restricted to the exact
pullback image. No computational advantage or prime-field descent is claimed.
The active queue now moves to balanced even-sextic arithmetic, then possible
Frobenius-stable non-split Richelot kernels. [pcc:unverified]
