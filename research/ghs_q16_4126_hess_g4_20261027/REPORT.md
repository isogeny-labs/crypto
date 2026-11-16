# Complete direct Hess genus-four controls for four residual sources

27 October 2026. This is a **known Hess construction**, now exercised end to end on all four exact residual sources from the quartic search. Each run supplies a subgroup-preserving transfer to its own genus-four hyperelliptic Jacobian, smooth-divisor relation collection, a finite-field linear solve, target recovery, and a same-source rho comparison. Every measured transfer cost exceeds rho. The [original order-4126 protocol](PROTOCOL.md), [variant script manifest](variants.json), [exact models and maps](EXACT_MODELS.md), [Magma attack template](attack.magma), [four-run benchmark summary](benchmark_summary.json), [512-trial rho template](rho.magma), and [independent audit](audit.py) make the result replayable.

| `#E(K)`, selected `r` | `#Jac(C)(k)`, projector | factor-base size | attempts / smooth rows / rank | full transfer | same-target rho / target | transfer / rho |
| ---: | :--- | ---: | :--- | ---: | ---: | ---: |
| 4006, 2003 | 88132, 44 | 21 | 211 / 22 / 22 | 0.350 s | 0.000684 s | 512× |
| 4054, 2027 | 56756, 28 | 13 | 506 / 14 / 14 | 0.450 s | 0.000645 s | 698× |
| 4078, 2039 | 57092, 28 | 13 | 471 / 18 / 14 | 0.450 s | 0.000645 s | 698× |
| 4126, 2063 | 57764, 28 | 13 | 431 / 15 / 14 | 0.430 s | 0.000625 s | 688× |

The rho entries are each `512` target-verified trials on the **same source subgroup** as that row. Transfer time includes cover construction, subgroup map evaluation, factor-base setup, relations, linear algebra, and source verification; it excludes the initial field and source-object setup, as does the rho timer. The source-specific scripts and full relation rows are retained in the corresponding `attack_<order>.magma` and `attack_<order>_receipt.json` files; order 4126 uses the unsuffixed names. These are maps to Hess genus-four curves, **not maps to the new plane quartics**.

## Exact order-4126 worked example

Let `K=GF(2)[w]/(w^12+w^7+w^6+w^5+w^3+w+1)` and `k=GF(2)[a]/(a^4+a+1)`. The source over `K` is

```text
E: Y²+XY = X³ + w^5 X² + (w^5+w^4+w^3+w^2+w+1).
#E(K)=4126=2·2063.
G=(62,1499), Q=(2633,1798), Q=[7]G
```

Coordinates use the polynomial-basis integer encoding implemented by `fromint` in the script. In particular, the encoded constant `63` equals `w^5+w^4+w^3+w^2+w+1`. Magma's `WeilDescent(E,k,1)` produced the following reproducible genus-four hyperelliptic model:

```text
C: y² + (a^8 x^4 + a^3 x² + x + a^4)y
   = a^11 x^9 + a^13 x^8 + a x^5 + a^10 x³
     + a x² + a^12 x + a^12.
#Jac(C)(k)=57764=28·2063.
```

The returned divisor-class homomorphism `h` evaluated to these reduced Mumford triples:

```text
h(G) = (x^4+a^6 x³+a^13 x²+x+a^8,
        a² x³+x²+a^6 x+a^9, 4)
h(Q) = (x^4+a^4 x²+a^5 x+a^10,
        a^12 x³+a^13 x²+a² x+a^12, 4).
```

The script checked `h(G)≠0`, `[2063]h(G)=0`, `[2063]h(Q)=0`, and ultimately `h(Q)=[7]h(G)`. It verified the recovered scalar independently as `Q=[7]G` on the source. The [exact-model ledger](EXACT_MODELS.md) gives the corresponding curve and evaluated Mumford images for orders 4006, 4054, and 4078 as well. Magma's alternate plane-curve output for 4126 has degree nine and passed nonprincipal, order-2063, and target-relation checks in the [plane receipt](plane_pilot_receipt.json). The hyperelliptic routes were used for the full runs.

## Complete relation and decomposition run

The degree-nine model is outside the documented gate `q≥d!` for Magma's plane-curve line index calculus (`16<9!`). Instead, the attack used the genus-four hyperelliptic model's reduced Mumford divisors. Its factor base consists of the 13 rational affine points on `C` (the fourteenth rational point is at infinity). For deterministic pairs `(u,v)`, it formed `R=u h(G)+v h(Q)`, factored `R[1]`, retained only rationally split divisors, and reconstructed every retained `R` from its factor-base points in `Jac(C)(k)` before adding a matrix row.

For the prime subgroup, multiplication by the cofactor 28 projects each factor-base class into the order-2063 subgroup. Because `28·1842≡1 (mod 2063)`, this projection does not annihilate `h(G)`. If `t_i=log_{h(G)}(28 F_i)` and `x=log_{h(G)} h(Q)`, every checked relation becomes

```text
Σ_i n_i t_i − 28v x = 28u (mod 2063).
```

At attempt 431 in the 4126 run, the script had retained 15 smooth relations, including the complete `(u,v)`, factor-base row, and right-hand side for each in the receipt. The 15×14 matrix had rank 14 over `GF(2063)`. The other three runs likewise reached full rank with the row counts in the table. [Independent Python elimination](audit.py) of all four saved systems returned `x=7` for each. Magma then checked every target equality in both the relevant Jacobian and original elliptic subgroup. [AttackRelations.lean](AttackRelations.lean) checks the abstract projection identity and finite cofactor arithmetic; the finite-field maps, smooth factorizations, ranks, and recovered scalars remain external Magma/Python checks.

## Measured same-target cost

All timings below are Magma `Realtime` measurements on its calculator backend. The rho baseline used the **same** `E`, `G`, `Q`, and `r=2063`; 512 salted four-way Pollard-rho trials each independently verified the source scalar. Its operation counts include every walk step and failed restart, but exclude initialization scalar multiplications; wall time includes them. These are complete calculator-backend cost measurements, **not EKS-qualified performance measurements** under the repository's benchmark policy. The configured AWS caller in this environment reports account `590183823895`, rather than the required Iso Lab account `104186811093`; no EKS job or L2/S3 admission receipt was obtained.

| Stage | Measured work or time |
| :--- | ---: |
| Hess cover construction | 0.190 s |
| image map evaluation | below displayed 0.001 s precision |
| full cover-to-verification attack, excluding source-object setup | 0.430 s |
| smooth-relation collection within that total | 431 attempts; 15 relations; 0.170 s |
| linear solve and source verification | below displayed 0.01 s precision each |
| total run including source-object setup | 0.470 s |
| source rho, 512 trials | 0.320 s total, 0.000625 s amortized per target |
| source rho walk operations | median 168; mean 189.42; range 15–828 |

The order-4126 one-target transfer run is 688 times the rho amortized wall time in this backend (`0.430/0.000625`). The other three ratios range from 512 to 698. The relation stage alone exceeds the rho baseline in every row. The small orders make these timing ratios workload-specific; they are measured rejections of a speedup for these exact controls, not asymptotic bounds. The complete attacks use split-divisor index-calculus relations, not Magma's plane line sieve.

## Prior work and rigorous boundary

[Hess's generalized GHS paper](https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem/315278965D8D277A85812D9498236A88) gives the generalized Artin–Schreier transfer and a nonvanishing result for the large cyclic input factor. [Magma's Weil-descent documentation](https://www.math.uzh.ch/sepp/magma-2.19.8-cr/html/text1383.htm) explicitly provides the curve and divisor-class map. The `c=1` factor orders `(1,3)` give genus `8−4−1+1=4`; this is within the published construction. The [Magma index-calculus documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1473) states the `q≥d!` gate for its line sieve. Thus neither the map nor this control constitutes a new GHS family.

An additional [complete `c=1` genus census](C1_BOUND.md) gives a rigorous finite bound for all 8,190 normalized ordinary sources: 30 have genus one, 510 genus three, and 7,650 genus four under this published choice. Of the 2,875 source models outside the **genus-three Hess isogeny closure**, 2,865 have a direct genus-four `c=1` image and ten are subfield genus-one exceptions. The census checks all equations and has three independent Magma genus controls; it does not assert subgroup survival or rho advantage for all 2,865. All four exact benchmarked sources lie in that genus-four part. Their complete runs show that a successful subgroup transfer alone does not imply a rho-beating attack.

## Reproduction

```sh
python3 research/ghs_q16_4126_hess_g4_20261027/build_variants.py
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py attack
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py rho
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py attack_4006
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py rho_4006
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py attack_4054
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py rho_4054
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py attack_4078
python3 research/ghs_q16_4126_hess_g4_20261027/run_magma.py rho_4078
python3 research/ghs_q16_4126_hess_g4_20261027/audit.py
lean research/ghs_q16_4126_hess_g4_20261027/AttackRelations.lean
sage -python research/ghs_q16_4126_hess_g4_20261027/c1_audit.py
lean research/ghs_q16_4126_hess_g4_20261027/C1Counts.lean
```
