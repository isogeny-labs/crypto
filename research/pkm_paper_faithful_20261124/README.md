# PKM §3.4 run as the paper defines it: `|V| = p^(1/m)` at five field sizes

Date: 2026-11-24. Branch `codex/pkm-map-sweep-20261123` (draft PR #5067).
Status: measured end-to-end runs plus an independent construction check. All
empirical statements are `[pcc:unverified]` until an independent replay.

This closes the "full relation/LA/descent" item that was left open for this
lane. It does **not** replace the degree-32 component sweeps; it asks the
narrower question those sweeps did not: does the paper's Algorithm 1, with the
§3.4 auxiliary-isogeny factor base at the paper's own density condition,
actually collect relations, reach a rank, and recover a scalar?

## 1. What the paper defines

[Petit–Kosters–Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)

**Algorithm 1** (§3.1):

1. fix `m`;
2. find a map `L` and a decomposition `L = L_n0 ∘ … ∘ L_1`;
3. `F = {(x, y) ∈ E(K) : L(x) = 0}`, `|F| ≈ deg L ≈ p^(1/m)`;
4. collect `deg L + Δ` relations:
   a. draw `a, b ∈ F_p`, compute `(X, Y) = aP + bQ`;
   b. solve
      `S_{m+1}(x_{1,1}, …, x_{m,1}, X) = 0`,
      `x_{i,j+1} = L_j(x_{i,j})`,
      `0 = L_{n0}(x_{i,n0})`   (system (4); rational `L_j` cleared of denominators);
   c. store the relation `Σ P_i = aP + bQ`;
5. linear algebra to recover `k`.

**§3.4 construction.** An auxiliary `E'/F_p` whose order has a divisor
`|H| = ∏ p_i ≈ p^(1/m)` with every `p_i` small; `H ⊂ E'(F_p)`; `V` is the set
of `x`-coordinates of a *coset* of `H`; `φ: E' → E'` is the isogeny with kernel
`H`, written as a chain `φ = φ_n0 ∘ … ∘ φ_1` of Vélu maps
`φ_j = (ξ_j/ψ_j², y·ω_j/ψ_j³)`; `L_j = ξ_j/ψ_j²` and
`L_n0 = ξ_n0/ψ_n0² − χ`, with `χ` the abscissa of a non-2-torsion image point.
Lemma 1 then gives `|{x : L(x) = 0}| = |H|`.

**§3.2 density.** Expected solutions per target `(deg L)^m/(m!·p)`; at
`(deg L)^m ≈ m!·p` that is `Θ(1)` and the relation search needs a constant
number of systems per relation.

**§3.5 precomputation.** Random-curve search: on average about `p^(1/m)`
curves are tried before one has a large enough smooth divisor.

The paper's own §4 experiment (Table 5) stops at the relation search: it
reports `nbsols`, the degree of regularity `D`, time and memory for `n1 ≤ 11`
(`p ≤ 3 173 683`), with `V = x(coset)` of an order-`2^n1` subgroup. It does not
run step 5.

## 2. What these runs do

Binary: `ic` SHA-256 `06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`
(the hash the earlier PKM protocols pin).

```
ic bench --bits B --family generic \
  --factor-base "prime-isogeny:t=T,seed=0,aux=ordinary" \
  --oracle "pkm-semaev:m=M,engine=f4,degree=8,mode=chain" \
  --targets walk --linalg incremental-gauss-full-rank-checked \
  --max-trials N --solver-budget-seconds 30 --json
```

`--targets walk` is an `r`-adding walk whose jumps are `[a_j]G + [b_j]Q` with
the coefficients tracked mod `r`: every target is `aP + bQ` for known `a, b`,
i.e. Algorithm 1 step 4a, with the guard that no target is decomposed twice.
The `--targets random` run below is step 4a literally (fresh `a, b` each trial).

The factor base is the paper's §3.4 object (`src/cryptanalysis/ic_framework/prime_tower.rs`,
`isogeny_tower`): search `E'/F_p` with `2^t | #E'`, take `Q` of exact order
`2^t`, `V = x(R' + ⟨Q⟩)` with `2R' ∉ ⟨Q⟩` (so a coset and its negative are
disjoint), build the `t` Vélu steps
`y_{j+1} = y_j + τ_j/(y_j − ξ_j) = (y_j² − ξ_j y_j + τ_j)/(y_j − ξ_j)`,
i.e. `ξ_j/ψ_j²` with `deg ξ_j = 2`, `ψ_j = y − ξ_j`, and the terminal
`χ = x(φ(R'))`. The base is every point of the *target* curve `E` whose
abscissa lies in `V`, one column per abscissa.

The roster curves were built with `p ≈ 2^(2t)`; `√p` is 28.8, 62.5, 127.2,
255.6, 511.7 at `t = 5, 6, 7, 8, 9` against `2^t = 32, 64, 128, 256, 512`.
So the paper's condition `|F| ≈ deg L ≈ p^(1/2)` holds by construction, not by
tuning after the fact.

## 3. Results

`pred = (2^t)^m/(m!·p)` is the paper's §3.2 formula; `predF = |F|^m/(m!·p)`
uses the measured base size. `obs` is `relations_found / targets_tried`.
`D` is the F4 solving degree; `aux` is the number of auxiliary curves tried.

| p | t | m | \|V\|=2^t | √p | \|F\| | cols | pred | predF | obs | targets | rows | rank | D | aux | wall |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 827 | 5 | 2 | 32 | 28.8 | 36 | 18 | 0.62 | 0.78 | 0.554 | 74 | 41 | 19 | 5 | 58 | 5 s |
| 3907 | 6 | 2 | 64 | 62.5 | 50 | 25 | 0.52 | 0.32 | 0.271 | 140 | 38 | 26 | 5 | 132 | 43 s |
| 16187 | 7 | 2 | 128 | 127.2 | 142 | 71 | 0.51 | 0.62 | 0.485 | 270 | 131 | 72 | 5 | 175 | 122 s |
| 16187 | 7 | 2 | 128 | 127.2 | 142 | 71 | 0.51 | 0.62 | 0.402 | 127 | 51 | 51 | 5 | 175 | 14 s |
| 65353 | 8 | 2 | 256 | 255.6 | 268 | 134 | 0.50 | 0.55 | 0.391 | 128 | 50 | 50 | 5 | 641 | 59 s |
| 261847 | 9 | 2 | 512 | 511.7 | 524 | 262 | 0.50 | 0.52 | 0.412 | 279 | 115 | 115 | 5 | 108 | 986 s |
| 3907 | 4 | 3 | 16 | 15.7 | 18 | 9 | 0.175 | 0.249 | 0.270 | 37 | 10 | 10 | 6 | 7 | 142 s |

Every row recovered the planted scalar and passed the harness's full-point
check. Rows 3 and 4 are the same instance solved twice — walk targets and
literal random `aP + bQ` — and both return `k = 9331`. Row 5 is the
`incremental-gauss` (target-pinned) completion at 16 bits; see §5.

Reading the table:

* **Density.** The paper's `(deg L)^m/(m!·p)` predicts 0.50–0.62 and the runs
  observe 0.27–0.55. Against the measured base size, `obs/predF` is 0.71, 0.85,
  0.78, 0.64, 0.71, 0.76, 0.79, 1.09 — one constant factor, `≈0.77 ± 0.13`, across
  sixteen-fold range in `p` and both `m`. The relation search is in the regime
  the paper's analysis describes, and the analytic constant is right to within
  the 1/4-to-4× that the heuristic claims.
* **Precomputation.** §3.5 predicts about `p^(1/m) = 2^t` curves tried.
  Measured 58, 132, 175, 641, 108 against 32, 64, 128, 256, 512 — same order,
  and the two outliers are in opposite directions.
* **Solving degree.** The paper's Magma runs rise from `D = 5.00` at `n1 ≤ 7`
  to 5.42, 5.42, 5.92, 6.44 at `n1 = 8, 9, 10, 11`. This tower-rewriting F4
  stays at `D = 5` through `t = 9`. That is the only place these runs say
  something the paper's own table does not: on this system presentation the
  degree of regularity does not move over the range the paper measured.
* **Cost.** `s/ρ` is `2·10^4` to `4·10^7` — no speedup, and none is possible
  here. At `|V| = p^(1/2)` and `m = 2` the algorithm needs
  `m!·p/(deg L)^(m-1) = 2√p` targets, so its cost is `≈ 2√p · T` against rho's
  `≈ √p`; it loses by the per-system cost `T` (measured here as `10^4`–`10^6`
  group-operation equivalents). §3.2's own criterion says the same: the win
  needs larger `m` (targets `m!·p^(1/m)`) or a cheaper `T`.

## 4. Independent verification

`verify.py` re-derives the two things a reader should not take on trust.

1. **The construction (§3.4), from its definition.** For the pilot cell
   `p = 100003`, `t = 5`, the emitted `E': y² = x³ + 67190x + 45464`,
   `#E' = 100352`, `V` of 32 values: all 32 are distinct and lift to `E'`; the
   64 signed points are exactly `C ∪ (−C)` for `C = P0 + E'[2^5]` with `P0 =
   (60340, 62458)`, `C ∩ (−C) = ∅`, and `2P0 ∉ H`; so `V` is the `x`-image of a
   coset of the order-`2^5` subgroup and `χ` is not 2-torsion, as Lemma 1
   requires. It then brute-forces the target `x_R = 90553` over `V` and finds
   exactly 2 ordered / 1 unordered decomposition — the `m = 2` sign/swap pair,
   which is also why the "two ordered decompositions per planted target" in the
   earlier NIST pilot is a symmetry count, not a density measurement.
2. **The answer.** The harness plants `k = 1 + seed mod (r − 1)`. `verify.py`
   recomputes that formula outside the harness and compares it with `recovered`
   for all eight runs: `139, 2683, 9331, 9331, 60091, 203179, 2683` all match.
3. Each run's own final check is a group-law test `[d]G == target` with counted
   scalar multiplications, not a comparison of two copies of the same number.

## 5. Where it stops

* **Strict full-rank certificate.** `incremental-gauss-full-rank-checked` wants
  rank `cols + 1 = 135` at 16 bits. Over 1 000 targets it reached 134 with 420
  rows, 286 of them dependent, and stopped. The target itself was pinned much
  earlier — rank 50 at target 128 — and the `incremental-gauss` run verifies
  `k = 60091` there. Report the two separately: the strict certificate is a
  stronger claim than Algorithm 1 step 5, and it is the one that fails.
* **Size.** The factor base materialises all `2^t` abscissae, so `p` is bounded
  by memory along this path; 18 bits (`|V| = 512`) is the largest rung run here.
  Nothing above is a statement about 59-bit primes, P-192, or any deployed
  curve.
* **One map per rung.** `seed=0` fixes one auxiliary curve per rung; the search
  cost is counted but its variance is not measured.
* **No large-prime variation, no descent, no shared precomputation.** These are
  single-target solves per rung.

## 6. Files

* `run.sh` — the exact commands, in order.
* `results/*.json` — the raw `ic bench` output, one file per row above
  (`r16t8_fullrank.json` is the strict-certificate run of §5).
* `verify.py` — §4, all three checks, plus the density table.
* `pilot_iso.json` — the `pkm_tower_pilot` row that emitted `E'`, `#E'` and
  `V` for the §4 construction check.
