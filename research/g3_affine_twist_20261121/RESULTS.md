# Affine-type (2,2,2) codomains of prime-order genus-3 Jacobians: Jacobian over F_p or twist? Results (2026-11-21)

Standing: every number here is `[pcc:unverified]`: one session, a shared 4-vCPU host, no
independent replay, no pcc claim filed. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen
before any data at the protocol primes (`frozen.sha256`). Data and analysis hashes:
`data.sha256`. Literature: [`LITERATURE.md`](LITERATURE.md). Compute: formula derivation
about 4 min, positive control 3.5 min, main arm 19 min, all single-threaded.

Labels: **[lit]** literature (source given), **[der]** derived (argument given),
**[comp]** computed (data given), **[conj]** conjecture.

## Answer

**Neither outcome holds generally: it is a coin flip per codomain.** Over 1,600
affine-type codomains (800 prime-order curves, both Frobenius-stable affine kernels,
p = 17 to 47), **803 are Jacobians of curves over F_p, 753 are quadratic twists, 44 are
degenerate** (`chi_18 = 0`). Jacobian share r = 803/1556 = **0.516 [Wilson 95: 0.491,
0.541]** [comp]. The two kernels of one curve behave independently, so **606/800 = 0.758
[0.727, 0.786] of prime-order curves have at least one affine kernel whose codomain is a
plane-quartic Jacobian over F_p** [comp]. On the others (about one in four) both codomains
are twists (174/800) or one is twisted and the other degenerate.

## Method

1. **Criterion [lit].** A ppav `A` of dimension 3 over `k` that is geometrically a smooth
   plane-quartic Jacobian is a Jacobian over `k` iff `chi_18(A, E)` is a square in `k^*`
   for a `k`-rational basis `E` of differentials (Lachaud-Ritzenthaler-Zykin, MRL 2010;
   Meagher; seen through search summaries only). Ritzenthaler (LMS JCM 2010) evaluates it
   over finite fields by lifting to characteristic 0 and using theta-nulls.
2. **Exact formula [der + comp].** For `B = J/L_S` (`tau_B = (tau + S)/2`), the duplication
   identity writes each product `theta_B[a;b1] theta_B[a;b2]` (18 pairs covering the 36
   even characteristics) as a sum of eight products of `theta_J(tau + S)`; Thomae's formula
   writes each `theta_J` as `kappa^(1/4) det(A)^(1/2) eps_c prod rho_ij` with
   `rho_ij = (e_j - e_i)^(1/4)`. So `chi_18(B, omega) = D * S_L(rho)` with one universal
   constant `D`, and `S_L` an explicit polynomial in 28 fourth roots (`derive.py`; the
   partition matching, the 8th roots of unity `eps_c` and the identity were fixed and
   checked numerically on one real configuration: duplication and Thomae reproduce the
   direct thetas to 4e-14, and `S_L` changes by under 1e-14 under random re-choices of
   all fourth roots, for all 64 kernels `L_S`).
   Since `chi_18(J/L, omega)` lies in `Q(e_1..e_8)` for labelled roots, `S_L` must take the
   same value for every choice of fourth roots, so it can be evaluated over `F_{p^28}`
   with arbitrary fourth roots and must land in `F_p` for a Frobenius-stable `L`. Both
   properties are gates (below) and held everywhere.
3. **Over F_p (`fpeval.py`, `run.py`).** Odd model `y^2 = f(x)` moved to an even model by
   `u = 1/(x - t)` (`Y^2 = f(t) prod (u - u_i)`, an `F_p`-rational change of differentials),
   roots in `F_{p^7} = F_p[z]/(f)`, fourth roots in a two-step quadratic tower, `L` placed on
   the labelled roots by a permutation of the eight Weierstrass points. Statistic:
   `kappa_p * chi(f(t)) * chi(S_L)`; the factor `chi(f(t))` is the twist (`c^27`).
4. **Calibration by a positive control.** `kappa_p` = square class of the universal
   rational constant (normalisations of `chi_18` differ between sources by signs and
   powers of 2, [lit] L5). Fixed at each prime by Smith's construction on pair-type
   kernels, which has a known answer.

## Positive control (Smith pair type, known answer) [comp]

Split even models (6 per prime), all 105 pair-type kernels, trigonal maps solved over
`F_p`; ground truth = Smith's Prop. 4 fibre-norm symbol (consistent over all `t0` and both
maps on every kernel).

| p | p mod 8 | compared kernels | agree | disagree | `kappa_p` | `S_L = 0` (all on one-map kernels) |
|---|---|--:|--:|--:|--:|--:|
| 17 | 1 | 115 | 115 | 0 | +1 | 25 |
| 19 | 3 | 130 | 0 | 130 | -1 | 19 |
| 29 | 5 | 235 | 235 | 0 | +1 | 12 |
| 31 | 7 | 176 | 0 | 176 | -1 | 16 |
| 37 | 5 | 209 | 209 | 0 | +1 | 54 |
| 41 | 1 | 245 | 245 | 0 | +1 | 11 |
| 43 | 3 | 226 | 0 | 226 | -1 | 9 |
| 47 | 7 | 233 | 0 | 233 | -1 | 5 |

- 1,569/1,569 compared kernels are decided correctly once `kappa_p = chi_p(-1)` is applied:
  exactly the preregistered alternative `u = -1` (P0, `kappa_p = +1` everywhere, was
  refuted; `u = 2, -2` do not fit). Both Smith outcomes occur at every prime (pass
  54-128, fail 61-140 per prime), so this is not a constant being matched.
- Post-hoc reading of `u = -1` [der, not preregistered]: in the derivation
  `kappa < 0`, so `(kappa^(1/4))^2 = i |kappa|^(1/2)`, and the 18 pair factors carry
  `i^18 = -1`.
- `S_L = 0` occurred only on kernels with a single (double-root) trigonal map, Smith's
  degenerate case.
- G1 (value in `F_p`) and G3 (two permutations agree): 0 failures.

## Main arm [comp]

800 prime-order curves (100 per prime), both stable affine kernels each.

| p | Jacobian | twist | degenerate | r [Wilson 95] |
|---|--:|--:|--:|---|
| 17 | 99 | 93 | 8 | 0.516 [0.445, 0.585] |
| 19 | 100 | 89 | 11 | 0.529 [0.458, 0.599] |
| 29 | 104 | 92 | 4 | 0.531 [0.461, 0.599] |
| 31 | 105 | 93 | 2 | 0.530 [0.461, 0.599] |
| 37 | 106 | 86 | 8 | 0.552 [0.481, 0.621] |
| 41 | 89 | 109 | 2 | 0.450 [0.382, 0.519] |
| 43 | 92 | 106 | 2 | 0.465 [0.397, 0.534] |
| 47 | 108 | 85 | 7 | 0.560 [0.489, 0.628] |
| all | 803 | 753 | 44 | **0.516 [0.491, 0.541]** |

Gates (G2, first 5 curves per prime, both kernels, two values of `t`: 160 evaluations):
fresh fourth roots 160/160, `zeta8 -> zeta8^3` 160/160, another permutation 160/160, a
second independently derived affine formula (`S = 110111`) 160/160 equal values; the
sign is the same for both models (`t`) on 80/80. G1: all 1,600 values in `F_p`.

Predictions:

- **P1 held:** r in [0.35, 0.65] overall and both outcomes at every prime.
- **P2 not supported:** `N1 > p+1`: 383/771 = 0.497 [0.462, 0.532]; `N1 < p+1`:
  420/785 = 0.535 [0.500, 0.570]. The direction is opposite to the prediction, and the
  intervals overlap. Binned against `N1/(2p+2)` (0.36, 0.45, 0.55, 0.64 at bin centres
  with n >= 150): r = 0.48, 0.55, 0.48, 0.54. **There is no trace of the pair-type sign
  law** `r ~ N1/(2p+2)` (`fig_twist.svg`, right panel).
- **Two kernels per curve** (756 curves with both nondegenerate): ++ 197, +- 188, -+ 197,
  -- 174. That is consistent with independence (expected -- at r = 0.516: 177).
- **Degenerate:** 44/1600 = 2.75%, at most one per curve.

## What this means for the transfer route

- **[der] Degenerate codomains are hyperelliptic.** `#J` prime makes `J`, hence `B`,
  simple over `F_p`. A simple, hence indecomposable, ppav of dimension 3 is geometrically
  a Jacobian (Oort-Ueno [lit]). `chi_18(B) = 0` then means a hyperelliptic Jacobian.
  Those 2.75% land on genus-3 hyperelliptic again, with no exponent gain. (No twist issue
  there: hyperelliptic Jacobians descend.)
- **[comp] Coverage.** At p <= 47 about 76% of prime-order genus-3 hyperelliptic
  Jacobians (odd model) have an `F_p`-rational affine `(2,2,2)`-isogeny onto the
  Jacobian of a plane quartic over `F_p`, where Diem's index calculus (exponent 1 versus
  4/3 for double large prime and 3/2 for rho [lit, lab]) would apply. About 24% have only
  twisted or degenerate affine codomains. For them, the affine route lands in
  `Jac(X)(F_{p^2})` and is worse than rho.
- **[conj]** r stays near 1/2 with independent kernels as p grows, so coverage tends to
  about 3/4. That rests on the stability of the per-prime rates from 17 to 47 and on
  the absence of any N1 dependence. It is untested at cryptographic sizes.
- **[der] Points cannot decide it (confirmed in the instrument).** `A` and `J` share the
  Frobenius polynomial; the decision needs `chi_18` or an explicit codomain curve.
- **For the other 24% (not tested here):** the pair-type route is closed for prime order
  (no stable pair kernel, lead #3 study). The remaining options are compositions with
  other isogenies (e.g. odd `l`, lead #5) or the twist side, which would need the DLP in
  `J^tw`, not `J`.

## What remains before an end-to-end transfer

1. **The isogeny itself.** This study decides the twist **without constructing the
   codomain curve or the map**. Still missing:
   - an algorithm for the affine-type `(2,2,2)`-isogeny `J -> Jac(X)` on points (kernel
     over `F_{p^7}`; Milio's theta method gives no explicit map to non-hyperelliptic
     quotients [lit]);
   - a model of `X` over `F_p`, for example via the theta-nulls of `B` (computable from this
     instrument's `theta_B` products up to the square-root issue), Weber's / Riemann's
     bitangent model, and Dixmier-Ohno reconstruction and descent;
   - evaluation of divisor images.
2. **A cheap test usable at cryptographic size.** The present evaluation is polynomial
   (it needs `F_{p^7}` and `F_{p^28}` arithmetic and 28 fourth roots), so it already
   scales to large p in principle. It is unoptimised pure Python and was run only at
   p <= 47.
3. The Diem destination measurement (lab rank #1), on which the value of any transfer
   depends.
4. Independent replay of this study and a pcc claim.

## Limits

- The LRZ/Meagher criterion and Ritzenthaler's method were read only through search
  summaries.
- The universal constant was calibrated, not derived. Its consistency with a single
  `u = -1` across 8 primes in all 4 classes mod 8, with 1,569/1,569 agreement, is the
  evidence.
- The control validates the whole chain on pair-type kernels. The affine arm uses the
  same constant and derivation; affine-specific checks are the internal gates only, since
  no affine ground truth exists.
- Small primes only.

## Deviations

- **D1.** The first launch used `python -I`, which removes the script directory from
  `sys.path`. Every job failed on import, and no data was produced
  (`data/run_failed_launch.log`). `run_all.sh` was relaunched without `-I` and re-hashed
  in `frozen.sha256`; no other change.
- **D2.** The development data at p = 13 (`dev/devtest.py`, and a 3-curve main-arm
  smoke run) was used before the freeze to debug the instrument. p = 13 is not a
  protocol prime.

## Files

`PROTOCOL.md`, `frozen.sha256`, `thetalib.py`, `derive.py`, `formulas/f_*.json`
(64 derived formulas with their numeric reports), `fpeval.py`, `smith.py`, `run.py`,
`run_all.sh`, `analyze.py`, `render.py`, `data/control.jsonl`, `data/main.jsonl`,
`data/summary.json`, `data/run.log`, `data/run_failed_launch.log`, `fig_twist.svg`,
`dev/devtest.py`, `LITERATURE.md`, `RESULTS.md`, `data.sha256`.
