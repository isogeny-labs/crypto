# hashed-ecdl-tensor: emit each check once, start the sum at A1

A constant-factor change to `crates/hashed-ecdl-tensor`, measured here against
`origin/main` (23cf569058) on the same instances with the same tool.

## The change

1. **Encoder reuse** (`src/encoding.rs`). `canonical(w)` and `valid_point(P)`
   are emitted once per (signals, modulus or curve, semantic region), and
   `modular_add` / `modular_subtract` / `modular_multiply` on the same operand
   signals return the word already built. Before, `point_add` re-validated both
   inputs and every `modular_multiply` re-checked both operands. A repeated
   check is the same constraint on the same signals, and each modular output
   and its auxiliary witnesses are uniquely determined by the inputs, so the
   language and every count are unchanged.
2. **No O + A1** (`src/reduction.rs`). The derived-last-slot network started its
   running sum at a constant identity and added A1 through a full addition
   network; it now starts the sum at A1.

## Results (`runs/report.pdf`, numbers from `runs/report.json`)

| | m = 2 (24 counts) | m = 3 (12 counts) |
|---|---|---|
| counts equal brute force, before and after | all | all |
| visited assignments | unchanged | unchanged |
| charged steps, before / after | 1.54–2.01 (median 1.77) | 2.49–2.87 (median 2.69) |
| edges, after / before | 0.46–0.48 | 0.52 |
| skip O+A1 alone (median step ratio) | 1.48 | 2.11 |
| encoder reuse alone (median step ratio) | 1.34 | 1.46 |

* Five seeded `reduction::solve` runs (p = 5, 7, 11, 13) recover the same scalar
  with the same relation count, pass `reduction::verify` before and after, and
  use 1.76–2.05x fewer charged steps. [pcc:unverified]
* The ECCp-131 decomposition network (bundled certified input) drops from
  1,531,849 to 707,701 edges with two slots and from 2,380,538 to 1,210,277
  with three. Maximum degree is unchanged (132) at every recorded size.

This is a constant factor in what every backend contracts. It does not change
the exponent: the `Functional` backend still visits 2^(free input bits)
assignments, and the recursive backend still needs at least 2^131070 edges,
against 2^19.4 for the two-slot ECCp-131 network after this change. No
ECCp-131 scalar is computed.

## Units and gates

Step counts are the crate's `Meter` charges, deterministic diagnostics and not
calibrated time. No wall-clock time is reported, so no timing-isolation path
applies and no `benchmarks/` or leaderboard row is added: the validation gate
admits none without a replay certificate, validator report and matched rho
reference.

## Reproduce

```sh
export CARGO_TARGET_DIR=/tmp/dedupe-target
cargo build --release --manifest-path native/Cargo.toml   # against the crate in this tree
B=$CARGO_TARGET_DIR/release/dedupe-measure
$B counts 2 5 7 11 13 > runs/counts2_after.jsonl
$B counts 3 5 7       > runs/counts3_after.jsonl
for a in "5 1 2" "5 2 1" "7 1 3" "11 1 5" "13 3 7"; do $B solve $a; done > runs/solves_after.jsonl
$B sizes 5 11 17 37 67 131 257 521 1031 > runs/sizes_after.jsonl
$B size-json ../../crates/hashed-ecdl-tensor/examples/eccp131-certified-hash.json > runs/eccp131_after.json
# *_before: the same commands with crates/hashed-ecdl-tensor/src checked out at 23cf569058.
# *_skip_only: only reduction.rs changed; *_memo_only: only encoding.rs and network.rs changed.
$B report runs > runs/report.json
python3 runs/plot.py        # layout only: draws recorded values
typst compile runs/report.typ runs/report.pdf
```

The `before` and `after` binaries were built from the same tool source. The
`after` records were regenerated from the final tree (after `cargo fmt` and the
added unit test) and are byte-identical to the committed ones; the `file` field
of the ECCp-131 records holds the path as run from this directory.
