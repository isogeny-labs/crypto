# Accepted native selector correctness path

Attempt-02 uses `run.sh` for native compilation and invocations. Node in
`custody.mjs` checks source byte ranges and file custody only; it computes no
field, curve, relation or research result. The Rust executable emits the
compact research record directly. No Python runs in this path.

The normal native call exited zero. Both deliberate corruptions exited 101
through the ordinary checking path: an off-fibre root failed the terminal
equation, and a changed norm-one lift failed its trace check. All source,
compiler and executable hashes were unchanged. Captured logs are all below
64 KiB; none was truncated. These are same-team bounded correctness
observations, `[pcc:unverified]`, not independent signed validation or a cost
measurement.

## Files and evidence

* `PROTOCOL.md`: frozen amendment and predetermined controls.
* `production.rs`: unchanged selected production constructor and arithmetic.
* `source-manifest.json`: declared exact source ranges; Node rechecks each
  selected byte against the pinned production files.
* `qualifier.rs`: independent u128-remainder checking arithmetic and standard
  odd-power trace recurrence, plus two explicit corruption modes.
* `run.sh`, `custody.mjs`: accepted shell execution and source/custody I/O.
* `evidence/attempt-02/normal.stdout`: complete compact raw Rust JSON, 3,829 bytes.
* `evidence/attempt-02/mutated-{root,lift}.{stdout,stderr,exit}`: expected failure
  controls with their original errors.
* `evidence/attempt-02/{pre-build,pre-execution,post-execution}.json` and
  `source-byte-check.json`: provenance and source-range receipts.

The source revision is `bd20b95dce44a91d7f74a1367853e724f280a3b6`.
The selected `production.rs` hash is
`c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c`.
No production arithmetic or counter is stubbed. This standalone compilation
is not a full root-library build or a run of the production modules' tests.

## Native output

For p=2147483647 and E:y²=x³−3x+21, all 8,190 production roots at k=1..12 and
the separate k=0 mathematical control passed cardinality, terminal-zero,
nonzero-derivative, norm-one, Frobenius, exact-order and trace-doubling checks.
Production's rejection of k=0 is retained; it did not construct that control.
Every positive-depth recurrence set equals the production set.

| k | Selector coordinates | Curve columns modulo point negation | Signed affine points |
|---:|---:|---:|---:|
| 0 (separate control) | 1 | 1 | 2 |
| 1 | 2 | 1 | 2 |
| 2 | 4 | 2 | 4 |
| 3 | 8 | 4 | 8 |
| 4 | 16 | 8 | 16 |
| 5 | 32 | 15 | 30 |
| 6 | 64 | 34 | 68 |
| 7 | 128 | 72 | 144 |
| 8 | 256 | 116 | 232 |
| 9 | 512 | 263 | 526 |
| 10 | 1,024 | 516 | 1,032 |
| 11 | 2,048 | 1,001 | 2,002 |
| 12 | 4,096 | 2,062 | 4,124 |

No selected coordinate has y=0 on the chosen curve. Only the k=0 control has
x=0. The counts apply to these exact fibres and are not a half-density theorem.
There is no group-order certification, relation finder, timing comparison,
rank measurement, or complete discrete-logarithm solve in this qualification.

## Reproduction and custody

For an ordinary correctness replay, compile the two Rust files with a
compatible native compiler and use new output locations:

```sh
rustc --edition=2024 --crate-name pplus_selector_qualifier -C opt-level=2 qualifier.rs -o /tmp/pplus-selector-qualifier
/tmp/pplus-selector-qualifier
/tmp/pplus-selector-qualifier --mutate-first-root
/tmp/pplus-selector-qualifier --mutate-norm-one-lift
```

The last two commands are required to fail. A provenance-qualified replay
must preserve source/executable hashes and write a fresh attempt directory.
The original `run.sh` deliberately refuses to overwrite `evidence/attempt-02`
or `build/`. It expects exact upstream production snapshots at `../upstream/`,
as documented by `source-manifest.json`; the pinned Git blobs can reconstruct
those snapshots. `PPLUS_RUSTC` chooses an explicit compiler.

Accepted raw JSON SHA-256:
`8cd8cb2de7d879a15a1146b34e752558d264e5d70f43cf8ec9acb3ff60d3c380`.

Accepted executable SHA-256:
`b49afd1c9c04aaa95911566b29fd0cb5be174d08d78b0701dcbf97b2f0c98371`.

The rejected attempt-01 and its oversized raw log remain unchanged; see
`../ATTEMPT01_CUSTODY.md`. Its Python wrappers are historical provenance,
not an accepted reproduction path.
