# Degree-3 cover push-forward: is every relation on the transferred subgroup a full fibre? Frozen protocol (2026-11-12)

Frozen before any data. Native Rust only: `src/cryptanalysis/chained_transfer/pushforward_deg3.rs`,
binary `cover_pushforward_deg3`. Step 4 of `research/chained_prime_transfer_20261111/NEXT_STEPS_20261112.md`
(owner-approved). Builds on `research/cover_pushforward_20261112/` (degree 2) and
`research/chained_prime_transfer_20261111/tower_ladder/` (the degree-3 cover to genus 4).
Every number in this file is a prediction or a definition, `[pcc:unverified]`.

## Question

The degree-2 experiment found that every genus-2 index-calculus relation on the transferred subgroup is a
fibre `P + iota P = phi1^*(Q)`, so the push-forward is the degenerate `[2]Q = Q + Q` and the index
calculus is a birthday search on `E`. Prediction under test: for a degree-`d` cover `c : C -> E`
the transferred subgroup lies in `c^*(E)`, whose effective divisors of degree `d` are the full
fibres `c^*(Q)`; every relation the index calculus finds on the subgroup is a fibre, so the index
calculus is again a collision search on `E`, at `d = 3`.

## The cover (the tower_ladder rung 4)

`E : Y^2 = c(s)`, `c(s) = s^3 + a2 s^2 + a4 s + a6`, `h(x) = x^3 + h1 x + h0`,
`C : y^2 = c(h(x))` (degree 9, genus 4), `c(x, y) = (h(x), y)`, degree 3, `p < 256` because
`HCurve` stores coefficients in `u8` (a larger `p` or genus needs a new Jacobian implementation, which
is out of scope). `c^*(Q)` for affine `Q = (s, Y)` is the Mumford pair `(h(x) - s, Y)`; `c^*(O)` is the identity.
Since the degree of `c^*(Q)` is 3 <= g = 4, the reduced Mumford representative of a class in
`c^*(E)` is unique, so the reduced class of `[alpha]D + [beta]T`, `D = c^*(G_E)`, `T = c^*(T_E)`, is
`c^*([alpha]G_E + [beta]T_E)`. The factor base is the full set of affine degree-1 places `(x, y)`,
`y <= (p-1)/2`. A trial is accepted when the reduced class (degree 3, or 4 only if it were not a pull-back)
splits over `F_p` into places counted with multiplicity.

## Arms (per cell: one prime, one instance, 15 stream seeds `s = 0..14`)

Cells: `p` in {53, 79, 101, 131, 151, 181, 211, 251}; instance `j` in {0, 1, 2}: `tower::build(p, 11 + i + 37 j)`
(`i` the index of `p`) for the curve, generator `G_E`, target `T_E = [x] G_E` of prime order `r`, and the genus-4
cover found by the `tower_ladder` search (`h` stream seed `1000 + i + 37 j`, require `r || #J(C)`, `r^2` not
dividing it); `j = 0` is the `tower_ladder` instance of that prime. A cell is admissible when the build succeeds,
the pull-back homomorphism checks pass for every `k` in `1..r` and `|B3 cap <G_E>| / ±` is at least 2
(defined below); a failed candidate is reported, never silently dropped, and `j` is then the next
integer. Stream seed for the arms is `300 + s`.

- **A0 rho on E**: `rho_reference` on `EcGroup`, seed `100 + s`, the matched reference; and
  `rho_reference_negation` (the A = 2 reference) as a second reference column. Gap ratios use A0 with
  `rho_reference`; the second is reported next to it.
- **A1 IC on J(C)**: per trial draw `alpha, beta` in `Z/r` (xorshift stream seeded by the stream seed, two
  draws per trial), `[alpha]D`, `[beta]T`, one addition, all by counted Cantor operations (convention of
  `measure::scalar_counted`: the first addition into the identity is free). Identity classes are counted
  and skipped. A nonidentity class is accepted when `u` has all its roots in `F_p` (brute-force root
  search with multiplicities, `deg u` roots in total); the row has entry `m * e` at the column of the place
  `(x, y_0)`, `y_0 = min(y, p-y)`, `e = +1` if `y = y_0` else `-1`. Stop at `nfb + 8` rows
  (`nfb` = number of columns). The library's `solve_linear_free`.
- **A2 classification and push**: for every accepted row and every trial: (a) is the class equal
  to `c^*(Q)` (Mumford equality) for `Q = [alpha]G_E + [beta]T_E` computed on `E` alone; (b) the
  decomposition into `c`-fibres: group the places by image `c(P) = (h(x), y)`; the row is **single full
  fibre** if its place multiset equals the fibre divisor of one image point `Q`, **union of full fibres**
  if it is a disjoint union of two or more full fibres, **other** otherwise; (c) `c_*` of the row by
  independent point arithmetic: `sum_i m_i (h(x_i), y_i)` on `E`, compared with
  `[3 alpha] G_E + [3 beta] T_E` and with `[3]Q`.
- **A3 structure of B**: `B1` = images of rational places, `B3` = images whose fibre is completely
  rational (the acceptance set); `|B1|, |B3|` over `#E`, `|B3 cap <G_E>| / r`; histogram of distinct
  rational places per affine point of `E` (0, 1, 2, 3) with the count of ramified fibres (double root
  of `h - s`). `|B1|` is computed twice (from the places of `C`, from `E` and `h` alone) and must agree.
- **A4 E-fibre arm (native 1-term)**: same stream; `Q = [alpha]G_E + [beta]T_E` with counted elliptic
  scalar multiplications (same convention), accepted iff the fibre `h(x) = s` is completely rational
  (tested on `E` and `h` alone, not on the Jacobian); row = one column, the class of `Q` up to sign; stops
  at A1's row count; same elimination.
- **A5 birthday control**: same stream, every nonzero `Q` kept, keyed by `±Q`, stop at the first
  usable collision (no cover, no filter).
- First-collision cost of A1 (relation list read at the first repeated image `±Q`) and A4 are also
  reported.
- Every recovered log is judged by an independent `[k]G_E = T_E` (double-and-add that shares no code with the
  solvers) and, for A1, `[k]D = T` on `J(C)` by the library's uncounted multiplication.
- The cover construction and `c^*` are a one-time cost reported as wall time per cell and never in a total.

Units: one counted group operation is 1 GAE. Linear-algebra multiplications mod `r` (A1, A4) are unpriced
native work, priced at 0, 0.1 and 1 GAE (AGENTS.md section 5a); lookups and brute-force root searches are
unpriced and listed as a limit. Two arms are ranked only where the order holds at all three prices.

## Measures

M1 per trial: class equals `c^*(Q)`. M2 per accepted row: fibre classification (counts of single/union/other).
M3 push-forward validity on `E` (valid and invalid counts; `c_*` equals `[3]Q`). M4 acceptance rate per
non-identity trial against `|B3| / #E` and against 1/6. M5 per cell and seed: A1 and A4 accepted-trial index
sets, group operations, rows, recovered log. M6 totals with LA at 0/0.1/1 against A0 (distance remaining,
mean over seeds). M7 first-collision cost of A1, A4, A5 against A0. M8 |B1|, |B3|, histogram.

## Frozen predictions (probability that the stated outcome holds in every measured cell, unless "pooled")

- P1 every accepted A1 row is a single full fibre (M2 other = 0, union = 0): 0.97.
- P2 every trial (accepted or not) has class equal to `c^*(Q)` (M1 100%): 0.98.
- P3 A1 and A4 accept identical trial index sets in every cell and seed: 0.97. Identical counted group
  operations (by construction of the shared counting convention): 0.99. Identical row counts and identical recovered log: 0.97.
- P4 pushed relations all valid on `E` (0 invalid), `c_*(row) = [3]Q`: 0.98.
- P5 pooled over cells, the acceptance rate of non-identity trials is within [0.12, 0.21]; pooled
  `|B3| / #E` within [0.14, 0.20]: 0.85. Each cell's `|B3| / #E` within [0.08, 0.27]: 0.85.
- P6 the distinct-rational-places histogram has fibre sizes 0, 1, 3 for unramified points and size 2
  only at ramified points (double root plus simple root): 0.97.
- P7 every recovered log in A1, A4, A5 is certified: 0.97.
- P8 A1 and A4 full-stop totals exceed A0 (`rho_reference` mean over seeds) at every cell and at all
  three prices: 0.90.
- P9 the A1/A4 first-collision median is below A0's median in at least half of the cells: 0.55.
  A5 is at or below A4 at the first collision on the same seed in every cell: 0.90.

## Decision rule (frozen)

The **generalization holds** (verdict "no help from the degree-3 cover: the index calculus on the
transferred subgroup is a collision search on E") iff, at every prime, cell and seed:

1. 100% of the collected A1 relations are single full fibres of `c` (M2: union = 0, other = 0);
2. A1 and A4 accept identical trials and cost identical counted group operations, with identical row counts;
3. M3: 0 pushed relations are invalid on `E`.

If any relation is not a single full fibre, the generalization is **falsified** at that cell and the table
reports the class and counts. If (1) holds and (2) or (3) fails, the verdict is "mechanism differs from
the prediction" and the table says how. Distances to matched rho (M6, M7) are reported as distance
remaining and enter no verdict. A cell where a full-stop arm is below rho is a measured improvement
only after the certificates and the rho band are checked (success criterion); relabelling, accounting or
a change that leaves the search on `E` is not an improvement.
