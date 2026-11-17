# Residual cubic Weil-restriction quartics over GF(16)

27 October 2026. The [frozen protocol](PROTOCOL.md), [30,000-draw result](results.json), [independent replay](verification.json), and [per-candidate prior-work ledger](prior_work.json) preserve the exact experiment. The screen used the same 13 catalog-positive, cofactor-two source orders outside the certified genus-three Hess isogeny closure throughout. It found four smooth plane quartics in matching Jacobian isogeny classes. Orders 4006, 4054, and 4078 were absent from the preceding 2,048-draw screen; order 4126 was already represented there.

This is an exact-equation and catalog-model finding. It does **not** establish a new vulnerable family: the source-to-quartic maps have not been constructed, and no complete transfer attack has been costed for these quartics.

## Exact models

Let `K=GF(2)[w]/(w^12+w^7+w^6+w^5+w^3+w+1)` and `k=GF(2)[a]/(a^4+a+1)`. Encoded field elements use Sage's polynomial-basis integer representation. Every source below has `E: y²+xy=x³+32*x²+b` over `K`. A quartic vector gives coefficients of `x^i*y^j*z^(4-i-j)` in descending `i`, then descending `j`.

| `#E(K)=#Jac(C)(k)` | `b` | `r` | `G`, `Q=[7]G` | quartic coefficients | `(#C(k),#C(k²),#C(K))` | LMFDB class |
| ---: | ---: | ---: | :--- | :--- | :--- | :--- |
| 4006 | 198 | 2003 | `(199,2783)`, `(738,3018)` | `[1,14,11,0,8,3,6,8,7,1,15,9,15,12,3]` | `(17,257,3824)` | [3.16.a_a_adn](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_adn) |
| 4054 | 32 | 2027 | `(33,349)`, `(2098,1182)` | `[1,15,8,7,1,6,10,2,10,7,10,3,12,3,12]` | `(17,257,3968)` | [3.16.a_a_abr](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_abr) |
| 4078 | 44 | 2039 | `(45,1367)`, `(47,3257)` | `[1,4,10,9,0,6,9,5,3,8,15,8,2,12,2]` | `(17,257,4040)` | [3.16.a_a_at](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_at) |
| 4126 | 63 | 2063 | `(62,1499)`, `(2633,1798)` | `[1,3,13,3,5,4,5,4,15,11,15,7,4,9,1]` | `(17,257,4184)` | [3.16.a_a_bd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bd) |

The independent verifier confirms geometric smoothness and all three point counts using alternate projective charts. It checks the displayed elliptic orders, `[r]G=O`, `Q=[7]G`, and the matching Jacobian orders. Rational point-difference classes reach the order-`r` subgroup on each quartic. This last check confirms possible factor-base elements, not a source-to-Jacobian map.

## Bounded prior-work check

All four Jacobian isogeny classes were already present in the queried LMFDB catalog. Its stored models for these classes are all hyperelliptic: respectively 52, 100, 184, and 140. None of the three first-seen **orders** appeared in the earlier local 2,048-draw quartic screen. Exact-label publication searches for the three class labels found no paper identifying these equations; a bounded search cannot prove global publication novelty. The 4126 class and a different quartic equation were already in the earlier local screen. [Hess's generalized construction](https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem/315278965D8D277A85812D9498236A88) and the [published isogeny extension](https://research-information.bris.ac.uk/en/publications/extending-the-ghs-weil-descent-attack/) precede these computations.

## Search accounting and claim gate

The fixed draw produced 2,854 curves with `#C(k)=17`, 156 also with `#C(k²)=257`, and 151 geometrically smooth survivors. Eight had one of the 13 target orders, representing the four distinct orders above. Their reciprocal Weil polynomials are `1+c3*T³+4096*T⁶`, with `c3=-91,-43,-19,29`, respectively, so `#Jac(C)(k)=4097+c3`. These are exact identities for the verified witnesses, not estimates of attack work.

| Order | equation first seen in this screen? | new eligible family? | subgroup transfer to this quartic | complete GHS cost vs same-source rho |
| ---: | :--- | :--- | :--- | :--- |
| 4006 | yes | unestablished | missing | unknown |
| 4054 | yes | unestablished | missing | unknown |
| 4078 | yes | unestablished | missing | unknown |
| 4126 | yes; class previously represented | unestablished | missing | unknown |

An attack claim requires an evaluable homomorphism `Phi` with `Phi(G)≠0` and `Phi(Q)=[7]Phi(G)`, followed by target recovery through relation collection/decomposition and a full charged cost below rho on the same source subgroup. [Direct Hess genus-four maps and complete split-divisor runs](../ghs_q16_4126_hess_g4_20261027/REPORT.md) now exist for **all four source equations** in the table. They are separate, previously published constructions; none supplies a map to the quartic here. Their full measured transfer costs are 512–698 times the same-source rho baselines, so they do not yield a speedup for these exact controls.

## Reproduction

```sh
sage -python research/ghs_q16_residual_quartic_search_20261027/search.py
sage -python research/ghs_q16_residual_quartic_search_20261027/verify.py
python3 research/ghs_q16_residual_quartic_search_20261027/audit.py
lean research/ghs_q16_residual_quartic_search_20261027/QuarticCounts.lean
```

The search elapsed time is a discovery cost, not a discrete-log attack cost.
