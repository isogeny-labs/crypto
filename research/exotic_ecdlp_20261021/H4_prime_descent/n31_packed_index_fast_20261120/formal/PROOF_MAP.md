# Packed point-key proof map

`H4PackedPointKey.or_eq_add` proves that for natural coordinates `x < 2^31`, the exact bitwise expression `x ||| (y <<< 31)` equals `x + 2^31*y`. `H4PackedPointKey.key_injective` proves that equality of two such packed keys forces equality of both coordinates when each low coordinate is below `2^31`. `H4PackedPointKey.key_lt_sentinel` proves that when both coordinates are below `2^31`, the key is below `2^62`, the runner's reserved identity sentinel. These are structural statements about the actual bit-packing rule, with no curve-specific conclusion assumed.

The runner represents each binary-field coordinate by `int(e.to_integer())`. It checks all 2,090 base-point coordinates against the independent polynomial-bit encoding; the four completed-arm audits check the exact 2,116,611 group-sum count and all 32 target witnesses. The Lean module does not import Sage's finite-field encoding or the elliptic-curve point law. The bridge that `to_integer()` is a canonical value in `[0,2^31)` for every curve point remains an implementation/API obligation, supported here by the field representation, base checks, exact sum count, and witness replay. No finite-instance cardinality or timing is asserted as a Lean theorem.

The module builds with Lean `v4.35.0-rc4` and Mathlib commit `021c46bdd042f67b538b9f9871d556a93713a9a9`. Command from `/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal`:

```sh
lake env lean /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-current-main-20261119/research/exotic_ecdlp_20261021/H4_prime_descent/n31_packed_index_fast_20261120/formal/PackedPointKey.lean
```

[build-output.txt](build-output.txt) records `#print axioms` for all three theorems: the standard Lean foundations `propext`, `Classical.choice`, and `Quot.sound` occur as listed. The source has no `sorry`, `admit`, custom axioms, or native-evaluation certificate. `lean4checker --fresh` was unavailable on this host.
