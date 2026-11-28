# resgcd m = 2 at t = 12…14 under the NTT cost model — result

Follows `RESULT_RESGCD_NTT_COSTMODEL_20261128.md`. Binary `c5220353`
(size-aware NTT dispatch, isogeny-labs/crypto#5353). Every claim is
[pcc:unverified]. Claude Code, local container, one thread, 30 trials per
point, CLI seed 20261400. Data: `t_scaling_m2_costmodel_18bit_t12_14.jsonl`;
driver `sweep_m2_high_t.sh` (`TS="12 13 14" BITS=18`).

## Curve

t = 14 needs p ≥ 4·2^14. The 16-bit roster curve has p = 65353 and is
rejected. At 17 bits no auxiliary curve with a point of order 2^14 was found
(aux seeds 0–2), because the Hasse interval (~1450 wide) usually holds no
multiple of 16384. So all three rungs run on the 18-bit roster curve
`icv1-fp18-t175-45770ebc`, which also makes the t-slope same-curve.

## Per-call F_p operations (relation counts 30 / 30 in every cell)

| t | \|V\| | classical | NTT, classical gcd | half (Kar.) | half + NTT | half+NTT / classical |
|--:|--:|--:|--:|--:|--:|--:|
| 12 | 4096 | 47,432,618 | 46,244,152 | 39,546,647 | 20,956,613 | 0.442 |
| 13 | 8192 | 185,260,778 | 179,491,122 | 122,188,835 | 51,366,169 | 0.277 |
| 14 | 16384 | 733,059,620 | 709,854,329 | 379,529,345 | 131,219,382 | 0.179 |

Split of the 30-call totals at t = 14: product tree 1.37G (Karatsuba) against
0.67G (NTT); gcd 20.10G (classical), 9.50G (half, Karatsuba) and 2.74G
(half, NTT). The half-GCD on NTT takes 7.3× fewer multiplications than
classical Euclid on the gcd alone at degree 16384.

Growth per unit t: classical ×3.91, ×3.96 (|V|², gcd-dominated);
half + NTT ×2.45, ×2.55 (about |V|^{1.3} over this range, approaching the
|V|·log²|V| of a fast gcd). The 16-bit curve gave 0.46× and 0.30× at
t = 12, 13 (binary `4b0e7305`), within a few percent of this curve.

## Reading

At m = 2 the fast arithmetic now dominates the solver cost: 5.6× fewer
multiplications per call at t = 14 and growing. This is per-system work; the
m = 2 relation search still needs about p/|V| trials per relation and Θ(|V|)
work per trial at best, so total cost stays at least linear in p, against
rho's √p. The constant-factor verdict on the PKM chain is unchanged.
