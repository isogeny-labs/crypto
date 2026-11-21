# Affine-type (2,2,2) codomain: Jacobian over F_p or quadratic twist? Protocol

Frozen 2026-11-21 before any data at the protocol primes (hashes in `frozen.sha256`).
Standing of every number produced under it: `[pcc:unverified]` (one session, shared
host, no independent replay, no pcc claim filed). Follows
`research/g3_transfer_gates_20261120` (lead #3) and
`research/hec_breakthrough_search_20261120/REPORT.md` (lead #3).

## Question

`H: y^2 = f(x)`, `f` monic irreducible septic over `F_p`, `#J(F_p)` prime. Exactly two
affine-type Lagrangians `L_1, L_2` of `J[2]` are Frobenius-stable, so `B_i = J/L_i` is a
ppav over `F_p`, geometrically (generically) a plane-quartic Jacobian. Is `B_i` the
Jacobian of a curve over `F_p` or the quadratic twist of one (Serre's obstruction)?

## Decision criterion (literature)

Lachaud-Ritzenthaler-Zykin (MRL 2010, arXiv 0802.4017) and Meagher (thesis; Meagher-Top),
as reported by search summaries (full texts unreachable from this session): a ppav `A`
of dimension 3 over `k`, geometrically the Jacobian of a smooth plane quartic, is a
Jacobian over `k` iff `chi_18(A, E)` is a square in `k^*`, for a `k`-rational basis `E`
of differentials. Ritzenthaler (LMS JCM 2010, arXiv 0901.2920) computes it over finite
fields by lifting to characteristic 0, using the analytic theta-null expression and
reducing. The square class does not depend on `E` (a change of basis multiplies by
`det^18`).

## Instrument (built and debugged before this freeze; development data at p = 13 only)

1. `derive.py` (seed 1 real configuration, `theta_arm.period_matrix`, frozen in the
   lead-#3 study). For `B = J/L_S` with `tau_B = (tau + S)/2` it writes, for 18 pairs
   `(c1, c2)` of even characteristics of `B` with equal top half, the duplication identity
   `theta_B[c1] theta_B[c2] = sum_beta theta[a+beta; b1+b2](tau+S) theta[beta; b1-b2](tau+S)`
   with each `theta_J(tau+S)` replaced by Thomae's formula
   `kappa^(1/4) det(A)^(1/2) eps_c prod rho_ij` (`rho_ij = (e_j - e_i)^(1/4)`, partition
   matched numerically, `eps_c` an 8th root of unity identified numerically).
   Then `S_L(rho) = prod_pairs (...) = chi_18(J/L, omega) / D` with one universal constant
   `D` (same configuration, same branches, for every `L`). Since `chi_18(J/L, omega)` is in
   `Q(e_1..e_8)` for labelled roots, `S_L` must be independent of the choice of fourth
   roots; this was checked numerically (rel. dev. < 1e-14, 5 random choices, all 64 `L_S`).
2. `fpeval.py`: evaluates `S_L` over `F_p` with the roots in `F_{p^7}` (or `F_p`) and fourth
   roots in a degree-4 tower; `L` relabelled to the target subgroup by a permutation of
   the 8 Weierstrass labels. Even model from the odd model by `u = 1/(x - t)`:
   `Y^2 = f(t) prod (u - u_i)`, `u_8 = 0`; an `F_p`-rational change of differentials.
3. Statistic per codomain: `sign = kappa_p * chi(f(t)) * chi(S_L)`; `+1` = Jacobian over
   `F_p`, `-1` = twist, `S_L = 0` = degenerate (`chi_18 = 0`: not a smooth plane quartic
   Jacobian; reported separately, excluded from the fraction). `kappa_p = chi_p(D')`, the
   unknown rational square class of the universal constant, is fixed by the positive
   control at the same prime (below).

## Gates (every one is reported; a failure is reported, not hidden)

- G1 value in `F_p` for every Frobenius-stable `L` (main) and every control kernel.
- G2 (first 5 curves per prime, both `L`, two values of `t`): independent re-choice of all
  fourth roots, `zeta8 -> zeta8^3`, another permutation in the same coset, and a second
  affine formula (`S = 110111` instead of `001000`) give the same value; and the sign
  `chi(f(t)) chi(S_L)` is the same for both `t` (model independence).
- G3 control: two permutations per control kernel give the same value.

## Positive control (known answer: Smith 2009, Prop. 4; lab smith_prop24)

Split even models `y^2 = c prod_{i=1..8} (x - u_i)`, `u_i` distinct in `F_p`, `c` random
nonzero, 6 curves per prime, all 105 pair-type kernels. For each kernel, the rational
trigonal maps `g` with `g(a_k) = g(b_k)` (solved directly; 0, 1 or 2 maps). Ground truth:
`J/S` is a Jacobian over `F_p` iff `prod_{g(x) = t0} F(x)` is a square (constant in `t0`,
Lemma 4; recorded over all usable `t0`, both maps). Pass rule at each prime: over all
kernels with at least one rational map, consistent Smith symbols and `S_L != 0`, the
agreement between `chi(c) chi(S_L)` and the Smith symbol is either 100% (`kappa_p = +1`)
or 0% (`kappa_p = -1`); anything else fails the control at that prime, and the main
arm at that prime is reported as uncalibrated. At least 50 such kernels per prime.

Prediction P0: `kappa_p = +1` at every prime (development at p = 13 gave +1 on all
nondegenerate kernels). Acceptable alternative: `kappa_p` equal to `chi_p(u)` for one
fixed `u in {-1, 2, -2}` across all primes; any other pattern fails the control globally.

## Population (main arm)

Primes `p in {17, 19, 29, 31, 37, 41, 43, 47}` (two of each class mod 8). Per prime, 100
prime-order curves: `f` monic irreducible septic drawn by `random.Random(104729 p + 3)`
(rejection on irreducibility and primality of `#J`, `#J` from `N_1, N_2, N_3` with the
lead-#3 census code). Both stable affine `L` per curve: 1,600 codomains.

## Statistics and predictions

- Primary: `r = #Jacobian / #(Jacobian + twist)` over all nondegenerate codomains, Wilson
  95%; also per prime.
- P1 (prediction): no structural rule. `r` lies in [0.35, 0.65] and both outcomes occur
  at every prime. (Basis: the lab's pair-type sign law `r(N1) ~ N1/(2p+2)` averages to
  about 1/2.) A rule "always Jacobian" or "always twist" (r > 0.95 or < 0.05) would
  refute P1.
- P2 (exploratory, directional): `r` is higher for curves with `N1 > p + 1` than with
  `N1 < p + 1` (two-proportion comparison, Wilson intervals). Also report binned `r(N1)`
  against `N1/(2p+2)`.
- Coverage: fraction of curves with at least one Jacobian codomain among `L_1, L_2`
  (Wilson 95%), and the 2x2 table of the two signs per curve.
- Degenerate rate: `#(S_L = 0) / 1600`.

## Budget and constraints

Single thread, pure Python + mpmath (scratchpad venv), target < 2 h. Order: control at
all primes, then main arm. Nothing is committed or pushed. Write access only under
`research/g3_affine_twist_20261121/`.

## Limits stated in advance

Small primes only (`p <= 47`). The decision rests on the LRZ/Meagher criterion (literature,
read only through summaries) and on the analytic-to-algebraic identity checked by the
invariance gates; the control validates the whole chain on pair-type kernels with the
same universal constant. No isogeny is computed and no quartic model is produced.
