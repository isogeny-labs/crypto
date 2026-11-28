# Selected-tower field ledger replay

The clean full-checkout release build pins source commit
`a1c7d2699bd01690416274300da4b1ace3fa92e6`, Rust 1.99.1, binary
SHA-256 `2b06842604a3afc2f75931d25e5c6d472a6f8a822f7dd8478e7a5e6842ead025`,
and `BUILD.json` SHA-256
`a2ea69ea4d96c3d5b83c8e93b8911ba115738e95aeb432db8f0e9357c12ad5ab`.
`verify.py --checkout <clean full checkout at that commit> --binary <rebuilt ic>`
checks the source and binary pins, six original held-out preflight maps, all
new raw hashes and operation counters, the exact p=5569 F4 one-trial replay,
and parity of the factor-base and oracle's separate tower construction.

The six maps retained their prior abscissa counts, workload/target identity,
group operations and every earlier native counter. The p=5569 F4 replay also
retained its 17 abscissae and 415 additions, 538 doublings and two scalar
multiplications in each construction. Its new factor-base and oracle ledgers
agree: 4567 selector direct multiplications, 69 exponentiation
multiplications, 1098 RHS multiplications, 2072 curve-check
multiplications, 61 direct Vélu/chain multiplications and 207 Vélu
inversion-exponentiation multiplications. The six held-out map receipts have
their individual counts in `MAP_REPLAY.json`. All seven diagnostic processes
ran on a contended host and stopped after one relation query; their timing is
not a qualified comparison.

The counters are **separate primitive classes**. Selector direct `mul` calls
use `u128` remainder as the F4 oracle's `mulm` does. `PrimeCurve::rhs` and
curve checks use its optimized `mulmod`; `residual_walk::pow_mod` uses its own
small-product path. Tonelli–Shanks multiplications are already counted
separately. Jacobi uses division/remainder, and additions, reductions and
other positive field work remain unpriced. One `ns_per_fp_mul` price must not
be applied indiscriminately to all these buckets. The v4 cold binary and
store remain unchanged; this source needs a new frozen amendment and cold
store before it can contribute to promotion evidence.
