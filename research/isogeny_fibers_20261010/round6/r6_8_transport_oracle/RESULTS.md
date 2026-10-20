# R6.8 Transport-oracle census and proof correction

## Review status (2026-10-20)

The proposed exact simulation in Proposition 5-prime is **not established**.
Lemma 5(b) and 5(d), as written, are false because the image set removes the
identity while the decomposition oracle requires exactly m summands. The
16-cell census and its raw records remain unchanged; those finite checks do
not prove the general simulation or its claimed overhead and instance independence.
The earlier verdict that Proposition 5(c) was closed for IC attackers is withdrawn.

The original report, source and PDF are retained byte for byte in
[`historical-04c3e817/`](historical-04c3e817/), with a SHA-256 manifest.
They document the superseded claim, not the current conclusion. No benchmark,
ledger promotion, or measured speedup is added by this correction.

## Exact counterexample

Work in a cyclic subgroup of order 5, written additively, with generator P.
Use the identity transport (the root path rho_1 in the proposed lemma),
S = {O, P}, and m = 2. This is an allowed set of formed points under IC2.
The original two-term multisets have sums O, P, 2P. After the proposed
operation S-star = rho_1(S) minus {O}, S-star = {P}, whose only two-term sum
is 2P. The valid decomposition P = O + P disappears. Thus the asserted
sumset inclusion and D_m monotonicity fail, whether or not the count excludes
the zero target. The size lower bound also fails: the identity has kernel
size 1, but |S-star| = 1 < |S| = 2.

`python3 review_counterexample.py` exhaustively enumerates the multisets and
checks this failure. The script also checks that retaining O restores the
sumset in this example. That local repair does not establish the full theorem.

## Further proof obligations

- **Lifting fibers:** choosing an arbitrary preimage of each image element
  does not enumerate all original decompositions when the rational kernel
  is nontrivial. For the projection C5 x C2 -> C5, S = {(1,0),(1,1)} and
  target (2,1), the mixed pair is valid, while consistently lifting image
  1 to (1,0) loses it. A corrected simulator must enumerate or otherwise
  represent all valid lifts and account for that work. This is an explicit
  group-level obstruction to the stated arbitrary-lift argument.
- **Adaptive paths:** IC2 does not forbid the attacker from choosing later
  transport paths using challenge-dependent observations. Hence the claim
  that the realized path set Gamma, and therefore F-star, is independent of
  the challenge needs an additional restriction or a different construction.
- **Cost and scaling:** a cost comparison for one decomposition call does
  not charge all lifting, filtering, table construction and path evaluations.
  A constant-factor asymptotic conclusion also needs a family-wide bound on
  the relevant path, degree, base-size and cofactor parameters.

These gaps concern the proposed proof. They do not prove that a corrected
simulation is impossible or that transport supplies an asymptotic advantage.
A corrected statement must specify fixed versus at-most-m decompositions,
identity/kernel treatment, adaptive choices, lift multiplicity, and complete
costs before the closure claim can be restored.

## Recorded finite measurements

The original executable recorded eight census files covering four curves,
two base sizes and two decomposition arities (16 cells). Its programmed
falsifier did not fire in any cell. The union/best class-factor-per-column
ratios and derived cost tables below are retained as finite measurements.
They do not validate the withdrawn theorem. The previously reported 6r
unrestricted-image count remains explicitly distinct from a subgroup-restricted
identity check.

## Part II. Measurement

### 7. Commands

```sh
export CARGO_TARGET_DIR=<target> CARGO_BUILD_JOBS=1
cargo +1.98 build --release --example r6_8_transport_oracle
sh research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/run.sh
/path/to/venv/bin/python research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/plot.py
```

(`run.sh` honours `EX_BIN=<built binary>`; the recorded run used the built
binary directly, same arguments, seed 7.) Each census run takes about 1 s
(contended wall time). All counts are exact enumerations; only the 2000
targets of the relation-transport simulation are sampled.

### 8. Instances (measured, recorded in `curve_E` / `neighbours`)

| family | slug | p | N | h | r | isogenies |
|---|---|---|---|---|---|---|
| j0 (§15.2) | icv1-fp18-t857-6ed1b1af | 195709 | 194853 | 3 | 64951 | ℓ = 3, kernel x = 0 |
| j0 | icv1-fp19-tm169-e87dee41 | 493159 | 493329 | 3 | 164443 | ℓ = 3 |
| j0 | icv1-fp20-t1847-1a56b6c7 | 903163 | 901317 | 3 | 300439 | ℓ = 3 |
| 6r | icv1-fp21-t1842-89bf7f00 | 1048583 | 1046742 | 6 | 174457 | ℓ = 2 and ℓ = 3 |

Every isogeny and dual from `isogeny_eval::small_degree` passed 500/500
on-curve, homomorphism, ψ̂ψ = [ℓ] and ψψ̂ = [ℓ] checks (`dual_certified`).
The relation-transport simulation (Proposition 5′ proof, B covering A's
decompositions) held on every sampled target: `B_covers_A` =
`A_hits_on_E'` and `pulled_back_relation_holds` equal on all runs, and the
Theorem 1 simulated answers agreed 2000/2000.

### 9. Falsifier table (measured, exact; c/U = class factor per unknown)

| file | m | |Ψ| | best single | best c/U | union c/U | union/best | union cost / plain cost | fires |
|---|---|---|---|---|---|---|---|---|
| census_j0_p18_s72 | 2 | 1 | pullback_l3 | 0.02737 | 0.0057 | 0.208 | 0.436 | false |
| census_j0_p18_s72 | 3 | 1 | pullback_l3 | 0.0181 | 0.0017 | 0.094 | 1.139 | false |
| census_j0_p19_s72 | 2 | 1 | pullback_l3 | 0.02752 | 0.00739 | 0.268 | 0.316 | false |
| census_j0_p19_s72 | 3 | 1 | pullback_l3 | 0.023 | 0.00339 | 0.147 | 0.642 | false |
| census_j0_p20_s72 | 2 | 1 | pullback_l3 | 0.02761 | 0.00726 | 0.263 | 0.314 | false |
| census_j0_p20_s72 | 3 | 1 | pullback_l3 | 0.02489 | 0.00432 | 0.173 | 0.516 | false |
| census_6r_p20_s72 | 2 | 2 | pullback_l2 | 0.00915 | 0.00172 | 0.188 | 0.305 | false |
| census_6r_p20_s72 | 3 | 2 | pullback_l2 | 0.0087 | 0.00078 | 0.09 | 0.636 | false |
| census_j0_p18_s144 | 2 | 1 | pullback_l3 | 0.01295 | 0.0028 | 0.217 | 0.416 | false |
| census_j0_p18_s144 | 3 | 1 | pullback_l3 | 0.00181 | 0.00011 | 0.063 | 3.733 | false |
| census_j0_p19_s144 | 2 | 1 | pullback_l3 | 0.01347 | 0.00327 | 0.243 | 0.346 | false |
| census_j0_p19_s144 | 3 | 1 | pullback_l3 | 0.00436 | 0.00029 | 0.066 | 2.555 | false |
| census_j0_p20_s144 | 2 | 1 | pullback_l3 | 0.01368 | 0.0035 | 0.256 | 0.327 | false |
| census_j0_p20_s144 | 3 | 1 | pullback_l3 | 0.00679 | 0.00052 | 0.077 | 1.7 | false |
| census_6r_p20_s144 | 2 | 2 | pullback_l2 | 0.00459 | 0.0008 | 0.175 | 0.327 | false |
| census_6r_p20_s144 | 3 | 2 | pullback_l2 | 0.00298 | 6e-05 | 0.02 | 3.39 | false |

### 10. Per-arm census, |F| = 144 (measured counts; c, c/U, cost derived from them)

U = unknowns, D = distinct m-sums in G (exact), rel/target = D/r,
cost = U·(r/D)·L_m (§15.2 model, L_2 = 1, L_3 = |F|).

**census_j0_p20_s144**

| arm | m | |F| | U | D | rel/target | c | c/U | cost |
|---|---|---|---|---|---|---|---|---|
| plain | 2 | 144 | 72 | 3416 | 0.0114 | 0.329 | 0.00458 | 6332 |
| plain | 3 | 144 | 72 | 127266 | 0.4236 | 0.256 | 0.00355 | 24476 |
| neighbour_l3 | 2 | 144 | 72 | 3388 | 0.0113 | 0.327 | 0.00454 | 6385 |
| neighbour_l3 | 3 | 144 | 72 | 127224 | 0.4235 | 0.256 | 0.00355 | 24484 |
| pullback_l3 | 2 | 144 | 72 | 10212 | 0.034 | 0.985 | 0.01368 | 2118 |
| pullback_l3 | 3 | 144 | 72 | 243450 | 0.8103 | 0.489 | 0.00679 | 12795 |
| filtered_l3E | 2 | 144 | 72 | 10118 | 0.0337 | 0.976 | 0.01355 | 2138 |
| filtered_l3E | 3 | 144 | 72 | 240464 | 0.8004 | 0.483 | 0.00671 | 12954 |
| union | 2 | 288 | 144 | 20884 | 0.0695 | 0.504 | 0.0035 | 2072 |
| union | 3 | 288 | 144 | 299418 | 0.9966 | 0.075 | 0.00052 | 41613 |

**census_6r_p20_s144**

| arm | m | |F| | U | D | rel/target | c | c/U | cost |
|---|---|---|---|---|---|---|---|---|
| plain | 2 | 144 | 72 | 1760 | 0.0101 | 0.17 | 0.00236 | 7137 |
| plain | 3 | 144 | 72 | 65710 | 0.3767 | 0.132 | 0.00183 | 27527 |
| neighbour_l2 | 2 | 144 | 72 | 1702 | 0.0098 | 0.164 | 0.00228 | 7380 |
| neighbour_l2 | 3 | 144 | 72 | 65630 | 0.3762 | 0.132 | 0.00183 | 27560 |
| pullback_l2 | 2 | 144 | 72 | 3424 | 0.0196 | 0.33 | 0.00459 | 3668 |
| pullback_l2 | 3 | 144 | 72 | 106776 | 0.612 | 0.215 | 0.00298 | 16940 |
| filtered_l2E | 2 | 144 | 72 | 3478 | 0.0199 | 0.335 | 0.00466 | 3612 |
| filtered_l2E | 3 | 144 | 72 | 106720 | 0.6117 | 0.214 | 0.00298 | 16949 |
| neighbour_l3 | 2 | 144 | 72 | 1688 | 0.0097 | 0.163 | 0.00226 | 7441 |
| neighbour_l3 | 3 | 144 | 72 | 65880 | 0.3776 | 0.132 | 0.00184 | 27456 |
| pullback_l3 | 2 | 144 | 72 | 1688 | 0.0097 | 0.163 | 0.00226 | 7441 |
| pullback_l3 | 3 | 144 | 72 | 65880 | 0.3776 | 0.132 | 0.00184 | 27456 |
| filtered_l3E | 2 | 144 | 72 | 5162 | 0.0296 | 0.498 | 0.00691 | 2433 |
| filtered_l3E | 3 | 144 | 72 | 132134 | 0.7574 | 0.266 | 0.00369 | 13689 |
| union | 2 | 432 | 216 | 16162 | 0.0926 | 0.173 | 0.0008 | 2332 |
| union | 3 | 432 | 216 | 174456 | 1 | 0.013 | 6e-05 | 93313 |
| union_pullbacks | 2 | 288 | 144 | 8354 | 0.0479 | 0.201 | 0.0014 | 3007 |
| union_pullbacks | 3 | 288 | 144 | 171970 | 0.9857 | 0.043 | 0.0003 | 42072 |

The other six files have the same arms; see the JSON.

![class factor per unknown](class_factor_per_unknown.svg)

![cost ratio](cost_ratio.svg)


See the tracked [proof-gap record](../../../../problems/transport-oracle-exact-simulation-gap.md).

## Next validation

Repair the theorem with the obligations above, then independently review its proof. Preserve the census as a separate finite result. The general simulation and its cost bound remain open.
