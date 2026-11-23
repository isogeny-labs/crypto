# Triple-sum proof map

The central theorem `H4TripleSumBound.card_orderedThreeSums_le` applies to any type `G` with `AddCommMonoid G` and `DecidableEq G` and any finite base `B : Finset G`. `orderedThreeSums B` is the image of all ordered triples in `B³` under addition. The conclusion is `|(B+B+B)| ≤ Nat.choose (B.card+2) 3`, with repeated summands allowed. The proof maps every ordered triple into the symmetric cube of the subtype of points in `B`, then bounds its image by the cardinality of that symmetric cube. It uses Mathlib's `Sym.card_sym_eq_choose` for the exact count.

`add_mem_threeSums` proves the bridge from any concrete three-summand relation to the symmetric-cube image. `orderedThreeSums_subset` lifts that bridge to the complete ordered image. `card_threeSums_le` counts the symmetric cube. `card_orderedThreeSums_le_1072` specializes the general theorem under the exact hypothesis `B.card=1072` and yields the numerical bound 205,895,824.

The formal module was checked with Lean `v4.35.0-rc4` and Mathlib commit `021c46bdd042f67b538b9f9871d556a93713a9a9`. The exact command was:

```sh
cd /Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
lake env lean /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-current-main-20261119/research/exotic_ecdlp_20261021/H4_prime_descent/n29_instance1_yield_20261120/formal/TripleSumBound.lean
```

[`build-output.txt`](build-output.txt) records all five `#print axioms` results: `propext`, `Classical.choice`, and `Quot.sound` only. The module contains no `sorry`, `admit`, `sorryAx`, custom research axioms, or native-evaluation certificate. `lean4checker --fresh` was not available in this environment.

The exact H4 curve, subgroup, and `B.card=1072` premise are reconstructed and checked by the independent Sage audit; their data are not imported into Lean. Consequently the formal theorem proves the bound conditionally on that audited base-size premise. The 138/256 sample yield and the occupancy calculation are empirical and model results, respectively; neither is a Lean theorem.
