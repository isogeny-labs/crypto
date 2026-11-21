# n35 packed-coordinate key proof map

`H4PackedPointKey35.or_eq_add` proves that for `x < 2^35`, the exact bitwise expression `x ||| (y <<< 35)` equals `x + 2^35*y`. `H4PackedPointKey35.key_injective` proves that equal keys imply equal coordinate pairs when both low coordinates lie in `[0,2^35)`. `H4PackedPointKey35.key_lt_sentinel` proves that when both coordinates lie in that range, their key is below `2^70`, leaving the runner's identity sentinel distinct. These theorems retain the bitwise operation and exact width used by the n35 runner.

The runner uses Sage's `to_integer()` to encode each binary-field coordinate. The independent audit checks this encoding against the polynomial-bit representation for all 4,096 base points; it independently reconstructs the 8,321,091 pair sums and all 32 target outcomes. Lean does not import Sage's finite-field type, elliptic-curve point law, or concrete cardinality certificate. The bridge that every coordinate's `to_integer()` is its canonical 35-bit polynomial representation remains an implementation/API obligation; the audit supplies finite-instance checks. The exact sum count and timing are audited computations, not Lean theorems.

The module imports only Lean `Init`, with pinned toolchain `leanprover/lean4:v4.35.0-rc4`; it has no external dependency. The build command from the crypto worktree root was:

```sh
/Users/adamburan/.elan/toolchains/leanprover--lean4---v4.35.0-rc4/bin/lean research/exotic_ecdlp_20261021/H4_prime_descent/n35_packed_pair_20261120/formal/PackedPointKey35.lean
```

[build-output.txt](build-output.txt) records `#print axioms` for all three declarations: `propext` and `Quot.sound` only. The source has no `sorry`, `admit`, custom axioms, or native evaluation. `lean4checker --fresh` was unavailable on this host.
