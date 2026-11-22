# Exact finite sign overlap

`TwistShiftOverlap.lean` checks three declarations:

| Claim | Lean declaration | Scope |
| --- | --- | --- |
| Indicator expansion | mask_expansion | Every commutative ring |
| Mixed-sign indicator | sign_mask | Integers with both inputs in {−1,1} |
| Finite translated overlap | finite_overlap | Any finite type, a permutation, and a ±1 integer function |

The final identity is 4 times the mixed-sign indicator sum equals the size of
the finite type minus its sign correlation. It uses exactly the stated ±1 and
permutation hypotheses. Legendre-symbol, projective genus-two smoothness and
Weil-bound bridges are **not formalized**; neither subgroup maps nor complexity
claims follow from this theorem alone. The written classical interpretation
is in [the pre-data protocol](../execution-v5/PROTOCOL.md).

Lean 4.34.1 and mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned.
The replay builds the actual module and a fresh native axiom auditor. Three
theorems pass with only propext, Classical.choice and Quot.sound; no sorry,
admit, custom axiom or trusted native_decide evaluation. Initial missing-import
and overloaded-lemma development errors remain in separate receipts.

The native identity issuer's overlap-mask mode gives
**IDC1ha81899274c038263**, with 32 issuer probes, 64 domain-separated checker
probes and false-plus-one rejection. It supplements the exact Lean proof;
it does not prove the missing geometric/Legendre/Weil bridges.

```sh
bash research/twist_routes_1_3_20261021/lean-v4/replay.sh /absolute/fresh/receipt-dir
```

The large compiled auditor is in the maintained Setec artifact manifest;
hydrate it with the artifact tool or rebuild it using the pinned replay.
