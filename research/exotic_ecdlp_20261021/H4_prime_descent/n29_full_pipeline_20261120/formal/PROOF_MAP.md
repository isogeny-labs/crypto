# Shifted-target proof map

`H4ShiftedTargetRecovery.recover_from_shift` proves that, in any module `G` over a ring `R`, the exact hypothesis `Q + t • G₀ = s • G₀` implies `Q = (s-t) • G₀`. This is the structural step used after a verified triple for the shifted public point. The candidate computes `s` from separately solved base logarithms modulo the prime subgroup order and replays the resulting original scalar against `Q`.

The theorem is built with Lean `v4.35.0-rc4` and Mathlib commit `021c46bdd042f67b538b9f9871d556a93713a9a9`. The build command was:

```sh
cd /Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
lake env lean /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-current-main-20261119/research/exotic_ecdlp_20261021/H4_prime_descent/n29_full_pipeline_20261120/formal/ShiftedTargetRecovery.lean
```

[`build-output.txt`](build-output.txt) records `#print axioms`: `propext` only. The theorem contains no `sorry`, `admit`, custom research axiom, or native-evaluation certificate. `lean4checker --fresh` was unavailable on this host.

The previous checked `H4PairIndex.pair_lookup_iff` and `H4CofactorProjection.additive_map_preserves_three_sum` formalize the index equivalence and relation transport. A Lean theorem about this exact binary elliptic curve and the concrete rank-536 relation matrix would require importing the encoded curve, its point law, all row certificates, and the matrix rank certificate. That bridge remains open. The checked Sage audit independently reconstructs the exact objects and verifies their equations and final scalar.
