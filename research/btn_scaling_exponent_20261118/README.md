# Scaling exponent and the Bochum v0.2 cost at 120–500 bits

Does the hashed-ECDL tensor method, as implemented in
`crates/hashed-ecdl-tensor` (crypto `main` 833a35d21e, with the reductions of
#4356), scale subexponentially, and what would Theorem 2 have to deliver at
120–500-bit fields? Report: `runs/report.pdf`; numbers: `runs/report.json`.

## Findings

* **Measured exponent.** Exact two-slot counts on prime-order curves from 3 to
  17 bits, 30 counts, all equal to an independent O(p) count. log2 charged
  steps grows 1.17 per field bit (R² 0.995) and visited assignments grow
  1.00 per bit, against 0.5 for Pollard rho. One decomposition query costs
  about 2^(1.17 b) steps: exponential, with more than twice rho's exponent.
  [pcc:unverified]
* **The recursion never engages.** The recursive backend (Appendix D.4, in
  `contraction/recursive.rs`) needs λ = ceil(log2 2s) ≥ 131072. At 120–500 bits
  the networks have about 1.3×10^5 to 1.1×10^6 edges (λ = 18–22); one modular
  product alone, a strict lower bound, is 10,312–41,420 edges.
* **Remark 2 with the implemented solver.** At least 2^190 (120 bits, m = 2)
  up to 2^1181 (500 bits, m = 3) operations, against 2^60–2^250 for rho.
* **If Theorem 2 held.** With m = 2, Remark 2's O(m² b r^(1/2)) solver calls
  alone exceed rho, so no solver helps. With m = 3, the hidden constant in
  2^(O(s/√log s)) would have to be about 10^-4 or smaller at every size here.
  The proof gives no saving below 2^(2^17) edges (the manuscript's own
  Limitations paragraph).

## Method and limits

Sizes at 120–500 bits come from one modular product built exactly at each
width and the exact ECCp-131 network (from
`research/hashed_ecdl_dedupe_20261115`). No curve at these sizes can be
certified here, so network sizes are estimates scaled from ECCp-131, with one
product as the strict lower bound. Rho is analytic, not measured. Step counts
are Meter charges, not time, so no `benchmarks/` or leaderboard row is added.
Theorem D.1 itself is not tested.

## Reproduce

```sh
export CARGO_TARGET_DIR=/tmp/scaling-target
cargo build --release --manifest-path native/Cargo.toml
B=$CARGO_TARGET_DIR/release/btn-scaling
$B scaling 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 > runs/scaling.jsonl
$B gadget 120 128 131 160 192 224 256 320 384 448 500 > runs/gadget.jsonl
cp ../hashed_ecdl_dedupe_20261115/runs/eccp131_final.json runs/eccp131_network.json
$B report runs > runs/report.json
python3 runs/plot.py        # layout only
typst compile runs/report.typ runs/report.pdf
```
