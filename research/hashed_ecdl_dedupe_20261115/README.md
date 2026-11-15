# hashed-ecdl-tensor: smaller decomposition networks, same language

Constant-factor reductions to `crates/hashed-ecdl-tensor`, measured against
`origin/main` 23cf569058 (*before*) on the same instances with the same tool,
after each cumulative step. After merging `main` b8cd452f3f (which adds the
`Multiplier` choice, Fourier power tables and treewidth tooling, Fourier stays
the default), the tool built against that `main` reproduces every `*_before`
record byte for byte, and built against the merged branch reproduces every
`*_final` record byte for byte; so the comparison holds against current `main`.

## The steps

1. **dedupe** (`encoding.rs`, `network.rs`, `reduction.rs`). `canonical(w)` and
   `valid_point(P)` are emitted once per (signals, modulus/curve, semantic
   region); `modular_add`/`modular_subtract`/`modular_multiply` on the same
   operand signals return the word already built (*memo only*). The derived
   running sum starts at A1 instead of adding O + A1 (*skip only*).
2. **fold** (`encoding.rs`). A definition whose inputs are all constants
   returns constant words; an all-constant constraint that holds is dropped (one
   that fails is kept, so an empty language stays empty); `and`/`or` with a
   constant operand and `gated_equal` with a constant selector simplify. When
   folding would change evaluation (function error, output too wide) the
   unfolded definition is built as before.
3. **valid** (`encoding.rs`, `reduction.rs`). `point_add` records its result as
   a valid point instead of re-checking it: every case pins the result to the
   identity, a valid input, or the chord/tangent point of the unique slope.
   `negate_point` records (x, -y) of a valid point as valid.
4. **final** (`encoding.rs`). `modular_multiply` by a canonical constant whose
   double-and-add chain has at most four additions (such as the 3x^2 of
   doubling) uses modular additions instead of two Fourier products.

New unit tests: memo emits nothing twice; `point_add`'s graph accepts exactly
one of all 2^(1+2w) output values, a + b, for every input pair on two tiny
curves (deleting the output-y constraint makes it fail); `modular_multiply` by
every constant c < 13 matches x*c mod 13 for every 4-bit x.

## Results (`runs/report.pdf`, numbers from `runs/report.json`)

| vs main | m = 2 (24 counts) | m = 3 (12 counts) |
|---|---|---|
| counts equal brute force, same visited assignments | all, every step | all, every step |
| step 1 dedupe: charged steps before / after | 1.54–2.01 (median 1.77) | 2.49–2.87 (median 2.69) |
| step 2 + fold | 1.99–2.96 (median 2.39) | 3.13–3.57 (median 3.36) |
| step 3 + valid | 2.24–3.88 (median 2.86) | 3.47–4.05 (median 3.76) |
| step 4 final | 3.01–5.65 (median 4.03) | 5.06–5.69 (median 5.39) |
| edges, final / before | 0.12 | 0.17 |

* Five seeded `reduction::solve` runs (p = 5, 7, 11, 13) recover the same scalar
  with the same relation count and pass `reduction::verify` at every recorded
  step; final uses 3.53–5.56x fewer charged steps than before. [pcc:unverified]
* The ECCp-131 decomposition network (bundled certified input) drops from
  1,531,849 to 253,318 edges and from 225,617 to 43,540 vertices with two
  slots; from 2,380,538 to 517,805 edges with three. Maximum degree is
  unchanged (132 at small sizes, 1032 at d = 131).

These are constant factors in what every backend contracts. The exponent is
unchanged: the `Functional` backend still visits 2^(free input bits)
assignments, and the recursive backend still needs at least 2^131070 edges,
against 2^18.0 for the two-slot ECCp-131 network now. No ECCp-131 scalar is
computed.

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
$B counts 2 5 7 11 13 > runs/counts2_final.jsonl
$B counts 3 5 7       > runs/counts3_final.jsonl
for a in "5 1 2" "5 2 1" "7 1 3" "11 1 5" "13 3 7"; do $B solve $a; done > runs/solves_final.jsonl
$B sizes 5 11 17 37 67 131 257 521 1031 > runs/sizes_final.jsonl
$B size-json ../../crates/hashed-ecdl-tensor/examples/eccp131-certified-hash.json > runs/eccp131_final.json
$B report runs > runs/report.json
python3 runs/plot.py        # layout only: draws recorded values
typst compile runs/report.typ runs/report.pdf
```

`before`: the same commands with `crates/hashed-ecdl-tensor/src` at 23cf569058.
`skip_only`/`memo_only`: only `reduction.rs`, or only `encoding.rs` and
`network.rs`, from step 1. `dedupe`: the first commit of this change. `fold`
and `valid`: the working tree after steps 2 and 3 (step 4's
`multiply_by_small_constant`, and for `fold` also `proven_valid` /
`negate_point`, absent). `mult` and `modops` modes print the multiplier and
modular-operation sizes used to choose the step-4 threshold.
