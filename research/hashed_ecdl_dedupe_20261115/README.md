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
4. **chains** (`encoding.rs`). `modular_multiply` by a canonical constant whose
   double-and-add chain has at most four additions (such as the 3x^2 of
   doubling) uses modular additions instead of two Fourier products.
5. **implied** (`network.rs`, `encoding.rs`, `census.rs`). A checked definition
   emits a Define step and a Check of the same function on exactly its inputs
   and outputs. Program evaluation computes those outputs with that function,
   so the Check holds there by construction and is skipped. The graph keeps the
   vertex, and every accepted assignment is still verified against the whole
   graph, so a disagreement would raise an integrity error, not a wrong count.
   Census rewiring (which can swap endpoints) clears the marks.
6. **auto** (`encoding.rs`, `reduction.rs`). `Multiplier::Auto`, now the
   default of `decomposition_network`, builds each product with whichever
   gadget (Fourier or schoolbook) has fewer graph ports for those operand
   shapes, decided once per shape in a scratch encoder. Schoolbook wins at the
   recorded widths; Fourier still wins at d = 131, so ECCp-131 keeps it.
   `decomposition_network_with_multiplier(.., Fourier, ..)` gives the
   Theorem 3 gadget throughout.
7. **zeroadd** (`encoding.rs`). A ripple-adder bit with two constant-zero
   inputs passes the third through instead of emitting a full adder.
8. **compressed** (`encoding.rs`, `reduction.rs`). Free points of
   `decomposition_network` are enumerated as (x, parity of y): w + 1 free bits
   instead of 2w. The rest of y is a defined witness (the root of x^3+ax+b with
   that parity) pinned by the unchanged validity constraints; for odd q, y and
   q - y differ in parity, so accepted points and counts are unchanged and
   visited assignments drop. `decomposition_network_with_multiplier` keeps the
   (x, y) layout.
9. **verify** (`encoding.rs`, `network.rs`). The encoder proves once that every
   vertex is a Check step's vertex read through exactly its edges' signals, or
   a copy/endpoint vertex on one signal; an accepted assignment then
   re-evaluates only the implied checks' vertices, not the whole graph.
10. **hashfirst** (`reduction.rs`). The hash reads only x and the parity bit, so
    it runs before the curve equation.
11. **final** (`encoding.rs`). `point_add` with a constant first point pins the
    tangent slope to the constant (3x^2+a)/(2y) instead of multiplying, and
    the same-x test reuses x2 - x1.

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
| step 4 + chains | 3.01–5.65 (median 4.03) | 5.06–5.69 (median 5.39) |
| step 5 + implied | 3.79–6.78 (median 5.03) | 6.85–7.75 (median 7.27) |
| step 6 + auto | 8.08–13.78 (median 9.96) | 11.19–15.07 (median 13.1) |
| step 7 + zeroadd | 12.32–19.10 (median 14.21) | 15.08–20.95 (median 18.62) |
| step 8 + compressed | 19.05–30.11 (median 23.74) | 20.57–34.07 (median 31.56) |
| step 9 + verify | 23.05–31.52 (median 27.37) | 29.55–35.96 (median 33.68) |
| step 10 + hashfirst | 24.77–41.68 (median 28.78) | 29.27–38.09 (median 36.72) |
| **step 11 final** | **26.24–44.81 (median 30.97)** | **30.66–39.49 (median 37.47)** |
| edges, before / final | 23.6–29.8x | 16.6–17.8x |

* Five seeded `reduction::solve` runs (p = 5, 7, 11, 13) recover the same scalar
  with the same relation count and pass `reduction::verify` at every recorded
  step; final uses 20.08–29.06x fewer charged steps than before. [pcc:unverified]
* The ECCp-131 decomposition network (bundled certified input) drops from
  1,531,849 to 226,444 edges and from 225,617 to 39,307 vertices with two
  slots; from 2,380,538 to 489,492 edges with three. Maximum degree there is
  unchanged (1032).

These are constant factors in what every backend contracts. The exponent is
unchanged: the `Functional` backend still visits 2^(free input bits)
assignments, and the recursive backend still needs at least 2^131070 edges,
against 2^18.0 for the two-slot ECCp-131 network now. Most of the 10x on the
recorded toy curves (steps 6 and 7) comes from the multiplier choice at small
widths, so it does not carry over to d = 131, where Fourier stays smaller. No ECCp-131 scalar is
computed.

## 100x was not reached

Final against main on the recorded curves: 26–45x fewer charged steps per
count (m = 2), 31–39x (m = 3), 9–30x fewer edges, 3–10x fewer vertices. At
p = 13 the two-slot network has 1,215 vertices: 541 definition checks, 441
copy-equality vertices (the degree-3 fan-out trees of the construction) and
215 other constraints, built from bit-level ripple adders and AND arrays.
Further large factors would need wider fused predicates or wide equality stars:
fewer, bigger steps and vertices for the same work, which under the
bounded-degree model is relabelling, not an improvement, so they were not made.

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
`negate_point`, absent). `chains`: commit e852145431. `zeroadd`: commit f2adf2d616.
`implied`, `auto`, `compressed`, `verify`, `hashfirst`: the working tree after
steps 5, 6, 8, 9 and 10 (counts only). `mult` and `modops` modes print the multiplier and
modular-operation sizes used to choose the step-4 threshold.
