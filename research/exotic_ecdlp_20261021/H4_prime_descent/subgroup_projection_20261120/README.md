# H4 subgroup projection diagnostic — 2026-11-20 UTC

The fixed `n=19` curve has group order `524448 = 864·607`, with prime
subgroup order 607. **None of the 119 original lifted x-subspace points
lies directly in that subgroup.** Multiplication by the cofactor 864
produces 112 distinct nonidentity subgroup points; their ordered pair
sums already cover all 607 subgroup elements. This changes the base
construction and the accounting for any DLP-subgroup experiment.

| Exact count | Value |
| --- | ---: |
| Original lifted points | 119 |
| Direct 607-subgroup members | 0 |
| Points cleared to identity | 1 |
| Distinct nonidentity cleared points | 112 |
| Duplicate cleared images among the 119 inputs | 6 |
| Cleared-base ordered pair entries | 12,544 |
| Distinct cleared-base pair sums | 607 of 607 |
| Distinct cleared-base three-sum targets | 607 of 607 |
| Planted target ordered triples after clearing | 2,380 |
| Ordinary target ordered triples after clearing | 2,430 |

[`protocol.json`](protocol.json) froze the exact preceding result hashes,
curve, subgroup order, cofactor, stages, and 90-second cap before execution.
The [runner](run.py) reconstructs the curve and all lifts from recorded
integer coordinates. It checks direct membership using `[607]P=O`,
clears by `[864]`, enumerates the 607-point subgroup from one nonidentity
cleared point, and counts pair and triple sums. The two earlier target
points are cleared by the same map before lookup. All 607 subgroup
elements appear as pair sums, so three-sum coverage is also complete.

The [independent audit](audit.py) maps every cleared point to its scalar
in the enumerated 607-point subgroup, then recomputes pair and triple
sums modulo 607. It also tests the original 119 points against that
enumerated subgroup. It reproduces all counts and the two target
multiplicities. [Result](result.json), [audit receipt](audit.json), and
[checked Sage runtime](sage-runtime-info.json) retain the evidence.

One local diagnostic run charged 0.172 s for lifting, 0.055 s for direct
membership checks, 0.012 s for cofactor clearing and subgroup checks,
0.087 s for the pair table, 0.664 s for full three-sum enumeration, and
0.00149 s for both recorded target lookups. Peak RSS was 249,479,168
bytes. These are stage observations on an ordinary host, not isolated
single-target timing comparisons.

The structural reason original relations survive projection is proved in
[`H4CofactorProjection.additive_map_preserves_three_sum`](formal/CofactorProjection.lean):
any additive map sends an exact three-point relation to the corresponding
relation among mapped points. For this curve the map is `[864]`.
`lean CofactorProjection.lean` under pinned Lean `v4.36.0` passed;
[`#print axioms`](formal/build-output.txt) reports none. The written
proof applies additivity twice, then substitutes the original relation.

**Proof map:** the Lean theorem covers the additive-transport implication
with its homomorphism hypothesis explicit. The exact curve, subgroup,
factor-base, and coverage counts are independently replayed finite
computations. Their full Lean formalization remains partial: it needs a
Lean definition of this binary elliptic group, a checked `[864]` map
bridge, and cardinality certificates for the 119 input lifts and 607
subgroup elements. `lean4checker --fresh` is unavailable on this host.

The 112 cleared points form a different base from the original x-subspace
base: their x-coordinates need not lie in that subspace. An original
`S_4` relation can be projected, but the previous 41.2555% full-curve
coverage fraction is not the subgroup yield of the projected base.
The present 607-order subgroup is small; this exact coverage does not
measure relation collection, factor-base logarithms, final linear
algebra, or a matched same-target reference.

Reproduce from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/subgroup_projection_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/subgroup_projection_20261120/run.py
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/subgroup_projection_20261120/audit.py
cd research/exotic_ecdlp_20261021/H4_prime_descent/subgroup_projection_20261120/formal && lean CofactorProjection.lean
```

The next scaling experiment should choose a curve with a large prime DLP
subgroup, freeze its usable base construction and target law, and compare
full relation yield and cost with a same-target reference.
