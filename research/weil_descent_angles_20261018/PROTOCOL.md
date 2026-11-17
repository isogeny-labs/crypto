# Protocol: four further Weil-descent angles (round 3), frozen before the first run

Study directory: `research/weil_descent_angles_20261018/`. Third round of the
Teske-variant search on branch `claude/edlyn-teske-trapdoor-variants-eghwfy`
(PR #2170); rounds one and two are
`research/teske_variants_field_types_20261018/` and
`research/weil_restriction_isogeny_census_20261018/`.

The owner asked (2026-10-18) to pursue four angles exhaustively:

1. summation polynomials plus Weil descent (point decomposition over
   `F_{q^n}`);
2. prime fields through descent over a subspace (Petit–Kosters–Messeng);
3. constant factors at `n = 2` (genus-2 index calculus over `F_p` against rho
   on `E(F_{p^2})`);
4. `n = 4`: whether a genus-4 Jacobian over `F_q` can be isogenous to
   `Res_{F_{q^4}/F_q} E`.

This file freezes **item 4** (Parts T, H4, K) now. Items 1–3 are frozen in
addenda appended to this file, each committed before that item's first run,
after the repository survey of existing work on them is complete (the
repository already holds a decomposition programme and a PKM series, and the
addenda must target their gaps, not repeat them).

Conductor: the installer is unreachable from this host (HTTP 403 through the
session proxy, as in rounds one and two); no task card; `conductor check` not
run.

Tool for item 4: `examples/genus4_res_census.rs` (std-only Rust,
`rustc --edition 2021 -O`).

## Item 4. Genus-4 Jacobians in the isogeny class of `Res_{F_{q^4}/F_q} E`

### Why

Round two found `F_q`-rational kernels in `Res E [ℓ]` for `n = 4` but could
not say whether any quotient is a Jacobian. The necessary condition is weaker
and decidable by point counting (Tate): a genus-4 curve `C/F_q` has
`Jac(C) ~ Res_{F_{q^4}/F_q} E` iff `L_C(T) = 1 − t T^4 + q^4 T^8`, i.e. iff
`#C(F_{q^k}) = q^k + 1` for `k = 1, 2, 3` (then `#C(F_{q^4}) = q^4 + 1 − 4t`
and `|t| ≤ 2q^2` holds automatically). Costs, as exponents [T]: rho on
`E(F_{q^4})` `q^2`; Gaudry's decomposition `Õ(q^{3/2})`; hyperelliptic genus-4
index calculus `Õ(q^{2−2/4}) = Õ(q^{3/2})`; non-hyperelliptic genus 4 (plane
model of degree 5, Diem) `Õ(q^{2−2/3}) = Õ(q^{4/3})`. So only a
**non-hyperelliptic** target beats every generic method.

### Part T: Theorem 5 (proved in the report) and its finite check

Statement to be proved: for `q` odd and `n ≥ 3`, if a hyperelliptic curve
`C/F_q` of genus `n` has `Jac(C)` isogenous over `F_q` to
`Res_{F_{q^n}/F_q} E`, then `t = tr(π_{q^n} | E)` is even, so
`#E(F_{q^n}) = q^n + 1 − t` is even; and the Frobenius cycle type
`(c_i)` on the `2n + 2` Weierstrass points satisfies
`∏ (T^{c_i} + 1) = (T + 1)^2 (T^n + 1)^2` in `F_2[T]`. (Mechanism: the
characteristic polynomial of Frobenius on `A[2]` is the mod-2 reduction of the
Frobenius polynomial, an isogeny invariant; for hyperelliptic `C` it is
`∏(T^{c_i} + 1)/(T + 1)^2`; for `Res E` it is `χ̄_E(T^n)`.)

Finite check (Part T of the tool): for every `n = 1, …, 16`, every partition
`(c_i)` of `2n + 2`, and `t` odd / even, test
`∏(T^{c_i}+1) = (T+1)^2 χ̄(T^n)` in `F_2[T]` with `χ̄ = T^2 + T + 1` (t odd) or
`T^2 + 1` (t even). Prediction **P5**: a solution with `t` odd exists only for
`n ≤ 2`; at `n = 2` it is `(3, 3)` (Scholten's odd-order genus-2 case); at
`n = 3` the even-`t` cycle types are exactly round one's four; at `n = 4` the
even-`t` cycle types are the partitions of 10 into powers of 2.

### Part H4: every hyperelliptic genus-4 curve over `F_3` (control for Theorem 5)

Every squarefree binary form `f(x, z)` of degree 10 over `F_3` (the models
`y^2 = f`), weighted by `1/|GL_2(F_3)|`. Record the mass (must equal
`#H_4(F_3) = 3^7 = 2187`, the known count `q^{2g−1}` of hyperelliptic curves
weighted by `1/|Aut|`; **check K1**), the Res-shaped models
(`N_1, N_2, N_3 = 4, 10, 28`), their `t`, and the Frobenius cycle type on the
roots of `f` (including `∞`). Prediction: every Res-shaped model has `t` even
and cycle type in powers of 2 (Theorem 5; **check K2**).

### Part K: every non-hyperelliptic genus-4 curve over `F_3`

A non-hyperelliptic genus-4 curve is canonically `Q ∩ K` in `P^3` with `Q`
the unique quadric through it; `F_3`-isomorphisms are induced by
automorphisms of `Q`. Three cases, all enumerated exhaustively:

- **split** `Q ≅ P^1 × P^1`: bidegree-(3,3) forms with `F_3` coefficients
  (`3^16`), modulo `F_3^×`; group `G = (PGL_2(F_3)^2) ⋊ C_2`, `|G| = 1152`.
- **nonsplit** `Q`: over `F_9` the same forms, with the `F_3`-structure
  `c_{ji} = c_{ij}^3` (Frobenius acts by `(x, y) ↦ (y^3, x^3)`), `3^16`
  forms modulo `F_3^×`; group `{(g, g^{(3)}) : g ∈ PGL_2(F_9)} ⋊ C_2`,
  `|G| = 1440`. `#C(F_{3^k})`: `k` odd counts `x ∈ P^1(F_{3^{2k}})` with
  `F(x, x^{3^k}) = 0`; `k` even counts the grid `P^1(F_{3^k})^2`.
- **cone** `Q` of rank 3: `y^3 + a_2(s,t) y^2 + a_4(s,t) y + a_6(s,t) = 0` in
  `P(1,1,2)` (`3^15` forms with leading coefficient 1); group
  `(GL_2(F_3) × F_3^× × {quadratic b(s,t)}) / μ`, `|G| = 1296`
  (`y ↦ λy + b(s,t)`, `(s,t) ↦ A(s,t)`, weighted scalars acting trivially).

Smoothness is decided exactly: points with `x` or `y` at infinity by
univariate gcds over the coefficient field; affine singular points by
`R(x) = Res_y(F, F_y)`, every irreducible factor `h` of `R` over the
coefficient field, and `gcd(F(α,y), F_y(α,y), F_x(α,y))` in `L[y]`,
`L = K[x]/(h)`; `R ≡ 0` is singular (a repeated component, a component at
infinity, or a purely inseparable curve of arithmetic genus 4 > 0).

Record per case: smooth forms; the mass `Σ 1/|Aut| = (#smooth forms)/
(|F_3^×|·|G|)` (cone: `#smooth/|G|`); the Res-shaped forms and their
`G`-orbits (isomorphism classes, by canonical form = least image under `G`);
per class `t`, `#E(F_81) = 82 − t`, whether it is prime, whether an elliptic
curve with trace `t` exists over `F_81` (ordinary: `3 ∤ t`; supersingular:
`t ∈ {0, ±9, ±18}`), and `|Aut|`.

Checks:

- **K1** hyperelliptic mass `= 2187` (Part H4).
- **K2** every Res-shaped hyperelliptic model has `t` even and cycle type in
  powers of 2 (Theorem 5 at `n = 4`).
- **K3** non-hyperelliptic mass (split + nonsplit + cone) compared with the
  literature value `#M_4(F_q) − #H_4(F_q) = q^9 + q^8 − q^6 = 25515` (from
  `#M_4(F_q) = q^9 + q^8 + q^7 − q^6`, Tommasi; Bergström). A mismatch is
  investigated and reported as found; it is not silently corrected.
- **K4** for every Res-shaped class, `N_4 = 82 − 4t` with `t` an integer,
  `|t| ≤ 18`.
- **K5** every class's `|Aut|` divides `|G|`; classes counted by orbit agree
  with `Σ_orbits |orbit| = #Res-shaped forms`.

Predictions frozen now:

- **P6**: Res-shaped non-hyperelliptic genus-4 curves over `F_3` exist (the
  three conditions cut about `q^{−3}` of the `≈ q^9` curves, so of order
  `q^6 ≈ 700` classes), and **some have `82 − t` prime** — i.e. for some
  prime-order `E/F_81` a genus-4 non-hyperelliptic Jacobian lies in the
  isogeny class of `Res E`. A null here (no prime-order class at `q = 3`) is
  reported as found and is evidence against the `n = 4` route, not proof.
- No prediction on the split/nonsplit/cone shares.

Budget: Part T and H4 one minute; Part K 90 minutes wall on 4 threads. A case
that does not finish is reported as not run.

### What item 4 does not do

- Existence of `C` with `Jac(C) ~ Res E` is not an explicit isogeny; no
  transfer is computed and no DLP solved. Diem's `Õ(q^{4/3})` is quoted [T].
- Exhaustive only at `q = 3` (`E` over `F_81`, a toy size).

## Addendum A (items 1–3), frozen before any run of items 1–3

Written after three read-only surveys of the repository (decomposition
programme; PKM series; genus-2 and cover tooling). What exists, quoted from
the surveys, and the gaps this round fills:

- **Decomposition over `F_{q^n}`**: measured end-to-end at `n = 3, 4, 5` (odd)
  and `n = 4` (binary), all above rho; the 1/n! rate was sampled at `k = 3`
  (`0.152–0.180` vs `1/6`, `p = 271–2083`), `k = 4` and the JV variants.
  **Gaps**: no Weil-descended summation oracle at `n = 2` (odd); no rate
  sweep at `n = 2`; nothing for binary `F_{(2^m)^n}` with `n = 2, 3, 5`; no
  null control (random factor base of the same size) anywhere.
- **PKM over prime fields**: solving degrees measured (`m = 2, 3, 4`), ecbench
  end-to-end `17–141×` rho with table oracles. **Gap**: "nobody has measured"
  the exact decomposition probability of the PKM factor base against its
  heuristic `|V|^m/(m!·r)`, nor against a random factor base of the same size.
- **`n = 2` Jacobian route**: genus-2 arithmetic and single-large-prime
  genus-2 index calculus exist (`genus2_fp.rs`, `hyperelliptic_index_calculus.rs`);
  IC vs rho on the *same* Jacobian measured (`0.42–1.78`). **Gaps**: no
  explicit map `E(F_{p^2}) → Jac(C)(F_p)` (no Scholten construction); no
  comparison against rho on `E(F_{p^2})`; no double-large-prime genus-2 IC
  over odd `F_p`.

### Item 1, Part D: exact decomposition coverage over `F_{q^n}`

Tool: `examples/decomposition_coverage.rs` (std-only).

For `E/F_{q^n}` (random curves, coefficients generic in `F_{q^n}`), factor base
`F = {P : x(P) ∈ F_q}`, and `m = n` summands, compute **exactly** over the
whole group: the coverage `π = #{R : R = P_1 + … + P_m, P_i ∈ F}/#E`, the
mean number of multiset decompositions per point `μ`, and compare with the
Gaudry heuristic `1/n!` and with the Poisson model `1 − e^{−λ}`,
`λ = (#size-m multisets of F)/#E`. Cells (each: every prime power in range,
3 curves per field):

| char | n | q range |
|:--|:--|:--|
| odd | 2 | primes 53 ≤ p ≤ 1500 |
| odd | 3 | primes 13 ≤ p ≤ 101 |
| odd | 4 | primes 5 ≤ p ≤ 23 |
| odd | 5 | primes 5 ≤ p ≤ 11 |
| 2 | 2 | 2^m, 4 ≤ m ≤ 10 |
| 2 | 3 | 2^m, 3 ≤ m ≤ 7 |
| 2 | 4 | 2^m, 3 ≤ m ≤ 5 |
| 2 | 5 | 2^m, 2 ≤ m ≤ 4 |

Controls on every curve: **null** — a random subset of `E` closed under
negation with `|F|` elements; **positive** — `F = {±jG : 1 ≤ j ≤ |F|/2}` (an
interval in discrete-log space), whose sums form an interval, so coverage must
fall far below Poisson. Prediction **P7**: the subfield factor base matches
the null control within sampling spread at every `n` and both
characteristics, and `π` is the Poisson value, not `1/n!`
(`1 − e^{−1/2} = 0.393` at `n = 2`, `0.154` at `n = 3`, `0.041` at `n = 4`).
A systematic subfield-vs-null difference would be a structural effect of the
Weil-descent factor base and is reported as found.

### Item 1, Part O and item 3, Part J: the `n = 2` routes end to end

Tool: `examples/n2_routes.rs` (std-only).

Instances: `E/F_{p^2}` with `#E = h·ℓ`, `ℓ` prime, `h ≤ 8`, at
`log2 #E ≈ 20, 24, 28, 32` (p ≈ 2^10 … 2^16), from the Scholten-type family
below so that both routes apply to the same curve; 3 instances per size, 3
targets per instance; every answer verified (`k·P = Q`).

- **Arm R (reference)**: rho on `E(F_{p^2})`, negation map, r-adding walk
  (r = 32) with distinguished points, affine coordinates with simultaneous
  inversion off; cost counted in `F_p` multiplications (`F_{p^2}` multiply =
  3, square = 2, inverse = 1 `F_p` inverse + 4; an `F_p` inverse is
  counted separately and priced at its measured multiplication equivalent).
- **Arm J (Jacobian route, item 3)**: `C : y^2 = f(x)` over `F_p` with
  `x^6 f(κ/x) = κ^3 f(x)`, `κ` a non-square, so the involution
  `(x, y) ↦ (κ/x, κ^{3/2} y/x^3)` is defined over `F_{p^2}` and swapped with
  its conjugate by Frobenius; `E = C/⟨ι⟩` over `F_{p^2}` and
  `Jac(C) ~ Res_{F_{p^2}/F_p} E`. Transfer `τ(P) = φ^*P + (φ^*P)^σ` (pull-back
  along `φ : C → E`, then the trace to `F_p`), verified as a homomorphism with
  `#Jac(C)(F_p) = #E(F_{p^2})` and the kernel of `τ` on `E[ℓ]` trivial. DLP
  in `Jac(C)(F_p)` by index calculus with degree-1 factor base of size
  `p^{1/2}·c` and double large primes (Gaudry–Thomé–Thériault–Diem), the
  linear algebra by structured Gaussian elimination then dense elimination
  mod `ℓ`. Cost: `F_p` multiplications of the Jacobian arithmetic and the
  splitting tests, plus the transfer, plus the linear algebra.
- **Arm D (decomposition route, item 1)**: Gaudry's decomposition in
  `E(F_{p^2})` with factor base `x ∈ F_p` (reduced to the same size as Arm J)
  and double large primes; the decomposition oracle is the Weil-descended
  summation polynomial `S_3(x_1, x_2, x_R) = 0` solved over `F_p` (two
  equations in `e_1 = x_1 + x_2`, `e_2 = x_1 x_2`, eliminated to a quartic,
  roots by gcd with `x^p − x`), verified against brute force on every point
  for `p ≤ 101`. Same linear algebra as Arm J.

Native operations not reducible to `F_p` multiplications (linear algebra mod
`ℓ`, hashing, distinguished-point storage) are counted separately and priced
at `0`, `0.1` and `1` `F_p`-multiplication equivalents; two arms are ranked
only where the order holds across the band (AGENTS.md §5a). Prediction
**P8**: both IC arms cost a constant multiple of rho with exponent ≈ 1 in
`p` (no exponent gain at `n = 2`); the constant favours rho at every size;
Arm D's per-trial cost is below Arm J's (an oracle call versus a Jacobian
addition) but its linear algebra is the same. A crossover inside the measured
range would be reported, not extrapolated beyond it.

### Item 2, Part P: exact decomposition coverage of PKM factor bases

Tool: `examples/pkm_coverage.rs` (std-only).

Prime-order `E/F_p`, `r = #E`, `p` over all primes in the ranges below with
`2^t | p − 1` (Kummer, split Dickson) or `2^t | p + 1` (torus Dickson).
`V` = the `2^t` roots of `L_t ∘ … ∘ L_1(x) = c` for the tower family
(Kummer `x ↦ x^2`; Dickson `x ↦ x^2 − 2`), `c` chosen so all roots lie in
`F_p`; `F = {P : x(P) ∈ V}`. For `m = 2, 3` and `t` at the sizing
`|V|^m ≈ m!·r` and one step either side, compute exactly `π`, `μ`, the
heuristic `|V|^m/(m!·r)` and the Poisson value. Same null and positive
controls as Part D. Ranges: `m = 2`: `r ≤ 2^24`; `m = 3`: `r ≤ 2^21`; at
least 20 primes per (family, m). Prediction **P9**: Kummer and Dickson
factor bases match the null control (their algebraic structure does not
change decomposition statistics; the PKM advantage, if any, is in solving,
not in coverage).

### Common

Budgets: Part D 60 min, Part P 60 min, Parts O/J 4 h wall (4 threads). Each
part writes CSVs, `checks.txt`, `env.txt`; each recorded run gets a run
report, graphs and the round report. A cell over budget is reported as not
run.

## Amendment B (before any recorded run of Parts D and P)

A smoke run of Part P (`PKM_SCALE=0.15`, outputs discarded) showed that the
raw coverage difference "family minus null" is dominated by the factor-base
size each arm happens to draw (a random `x`-set gives a different `|F|`, and
at `m = 3` coverage moves as `|F|^3`). The primary statistic for P7 and P9 is
therefore each arm's **residual from its own Poisson value**,
`π − (1 − e^{−λ})` with `λ` the arm's own multiset count over `#E`, paired per
field between the structured arm and the null arm, with a z-score over the
fields. The raw `π` are still reported. Nothing else changes.

## Amendment C (Parts O/J pricing and parameters), before any code of `n2_routes.rs` ran

- **Factor-base size**: `|B| = ⌈c·√p⌉` with `c ∈ {1, 2, 4}`; every `c` is
  run and reported; the IC arm's best `c` per size is reported **and** flagged
  as a post-hoc minimum (it favours IC).
- **Walks**: r-adding walks with `r = 32` (IC arms) and `r = 64` plus the
  negation map (rho), IC walks restarted from a fresh random combination every
  4096 steps (the restart's scalar multiplications are counted).
- **Pricing of an `F_p` inversion**: `I ∈ {3M, 2·log2(p)·M}` (batched
  Montgomery inversion; Fermat-style), both arms.
- **Pricing of a genus-2 Jacobian operation (Arm J)**: (a) the measured
  `F_p` operation count of this tool's Cantor implementation; (b) the
  IC-favourable bound "one Jacobian operation costs the same as one measured
  affine addition in `E(F_{p^2})`". No literature constant is used (a search
  for Lange's explicit-formula table did not return it; recalled values are
  not admissible).
- **Native operations** (linear algebra mod `ℓ`, hashing, table lookups,
  sorting) priced at `0`, `0.1`, `1` `M`.
- **Ranking rule**: an IC arm is ranked against rho at a size only if the
  order holds in every cell of {I pricing} × {Jacobian pricing} × {native
  weight}; otherwise "unresolved".
- **Linear algebra**: large-prime columns are eliminated by sparse merging
  mod `ℓ` (no sign bookkeeping), then dense elimination on the factor-base
  columns with the `(a, b)` combination tracked; the answer `k` is accepted
  only if `k·P = Q` verifies.

## Amendment D (follow-up diagnostic for Part D, binary `n = 2`), before it ran

The recorded Part D run (`results/item1D-run-20261018T2056Z`) rejects P7
in one cell: binary `n = 2`, paired residual difference `−0.0308`
(z −4.72, 21 curves). The subfield residual settles at `−0.0189 ± 0.0002`
for `m = 7 … 10`, where the null arm's residual is about `0`. Per the
protocol, this is reported as a structural effect of the Weil-descent
factor base. The diagnostic below is frozen before it runs. It neither
replaces nor reruns Part D.

Mechanism to test. For `y² + xy = x³ + ax² + b` and `x_1, x_2 ∈ F_q`, the
descended relation equation for a target `x_3 ∈ F_{q²}` is
`S_3 = x_3² e_1² + e_2² + x_3 e_2 + b = 0` with `e_1 = x_1 + x_2` and
`e_2 = x_1 x_2`. It is **F_2-affine** in `(e_1, e_2) ∈ F_q²` because
squaring is additive in characteristic 2. The number of `(e_1, e_2)`
solutions per `x_3` is therefore `0` or `2^{dim ker L_{x_3}}`, which is
not Poisson-distributed. Equivalently, `x(P+Q) + x(P−Q) = x_1 x_2/(x_1+x_2)²`
does not depend on `a` and `b`. In odd characteristic the analogous
quantity involves `a` and `b`.

Measurements (`decomposition_coverage <out> mult`, binary `m = 4 … 10` and
odd `n = 2`, `p ∈ {53, 101, 211, 401, 809}`, 3 fresh curves each, seed
fixed in the tool):

- **M1**: the identity `x(P+Q) + x(P−Q) = x_1x_2/(x_1+x_2)²` holds on every
  pair of subfield points with `x_1 ≠ x_2` (binary only).
- **M2**: for every `x_3`, the F_2 rank of `L_{x_3}`, the solvability of
  `L_{x_3}(e) = b`, and the coset size. The prediction is that every
  solution count is `0` or a power of 2.
- **M3**: the per-point multiplicity histogram (the number of multisets
  summing to `R`) for the subfield and null arms. The index of dispersion
  is var/mean over `R ≠ O`, and Poisson gives 1. The cross-check: from the
  `M2` cosets, after the split condition and the point-existence
  conditions, the subfield arm's distinct count must be reproduced
  exactly.

Prediction **P7b**: binary subfield dispersion > 1 (clustering explains the
deficit); the null arm and odd subfield arms are ≈ 1. Attack relevance:
a coverage deficit means fewer relations per trial, so the effect, if
confirmed, is a constant-factor penalty for the attacker at `n = 2`, offset
in solving cost only by the fact that the oracle is linear algebra over
F_2.

## Amendment E (Parts O/J), after smoke runs and before the recorded run

Smoke runs at `log2 #E = 16 … 28`, with outputs discarded, showed four
things.

1. **Rho restarted after every distinguished point.** Each restart cost two
   fresh scalar multiplications, so rho was overpriced. The reference arm now
   follows van Oorschot–Wiener: a trail continues through a distinguished
   point, and a fresh start is taken only when a trail meets its own stored
   point or a fruitless cycle exceeds the length bound. This makes the
   reference stronger.
2. **Seeds.** Job seeds were successive xorshift outputs, so two jobs ran
   shifted copies of one stream. They found the same curve. Seeds now come
   from splitmix64 of (size, instance).
3. **IC answers mostly come from large-prime cycles.** The answer usually
   arrives as a cycle among large primes, which is a direct relation
   `a + b k = 0` with no factor-base columns, long before `|B| + 10`
   B-relations exist. Every arm now tries `k` as soon as a direct relation
   appears, and still verifies `k·P = Q`. The B-relation path and dense
   elimination are unchanged. The expected cost of the first cycle in a
   random multigraph on `n ≈ p/2` vertices is `n/3` edges, so about `p/3`
   trials. This does not depend on the cofactor `h`. Rho needs
   `≈ 0.886 p/√h`. The factor-base size `c` should therefore barely
   matter; it is still run at `c ∈ {1, 2, 4}`.
4. **Jacobian pricing (a)** is now the measured count of this tool's genus-2
   arithmetic. That arithmetic has a reduced-formula fast path for the
   generic addition (41 M + 1 I) and doubling (48 M + 1 I), with Cantor
   (113 M + 4 I, 116 M + 4 I) for every other case. The fast path agrees
   with Cantor on every tested pair (tests; `check` K5). The Cantor-only
   count is reported in `checks.txt` but is not used for ranking. Pricing
   (b) is unchanged.

Sizes: `log2 #E ∈ {20, 24, 28, 32}` as frozen, plus **36 and 40** as an
extension declared now, so the per-`p` constants can be read where the
fixed setup (r scalar multiplications) is amortised. Every other parameter
of Addendum A and Amendment C stands. The added prediction **P8b** is
`steps/p → 1/3` for Arm J at every `h`, against `0.886/√h` for rho.

## Amendment F (control for Arm J), after the recorded Part O/J run and before this control ran

The recorded run (`results/itemOJ-run-20261018T2124Z`) gives Arm J
`steps/p ≈ 0.24–0.39` at every size. Cofactors there are `h ∈ {4, 8}`,
and the pricing band is unresolved at every size. Arm J's r-adding walk
can collide with itself (a rho collision), and a repeated divisor appears
as a duplicate relation. That is a 2-cycle in the large-prime graph. Part
of Arm J's success could therefore be rho inside the IC walk rather than
the large-prime-graph mechanism (P8b).

Control (`n2_routes <out> control`): the same instances as the recorded
run, three fresh targets each, `c = 2`. Two arms are measured:

- **Jwalk**: the recorded Arm J walk. It records every relation whose
  divisor repeats an earlier one, and whether the solving relation was
  such a repeat.
- **Jind**: every trial is a fresh `a τP + b τQ` from scalar
  multiplications, with no walk. A repeat then has only birthday
  probability on the `ℓ`-subgroup.

Only `steps/p` and the repeat statistics are read; Jind's cost is not
compared with rho. Prediction **P8c**: Jind `steps/p ≈ 1/3`, not larger
than Jwalk's within sampling spread, with few solves by repeats.
Otherwise the measured Arm J advantage in group operations is partly a
rho effect, and the report says so.

## Amendment G (reference arm repaired; Part O/J re-run), before the re-run

The recorded run `results/itemOJ-run-20261018T2124Z` has a weak reference
arm at the two largest sizes. Its mean rho `steps/p` is `0.63` at 36 bits
and `0.81` at 40 bits, against the theoretical `0.886/√h ≈ 0.44` for
`h = 4`. Rho with the negation map also enters fruitless cycles of length
4 and more, about once per `4r²` steps. The only way out was the
trail-length cap (`40·2^d` steps), which wasted up to that many steps per
trap. The run is kept as recorded, and its rho rows are superseded.

Changes, frozen before the re-run:

- **Rho fruitless cycles.** 2-cycles are caught at once, as before.
  Longer cycles use Bos–Kleinjung–Lenstra-style detection: every 32 steps
  the walk records its point. Meeting that point again within 32 steps
  means a cycle of length ≤ 32. The walk goes around it once more, at
  counted cost, and leaves by doubling the cycle's smallest point. That
  choice is canonical, so merged trails stay merged. The reference arm
  gets stronger.
- **Precision.** Targets per instance go from 3 to 10 for every arm, so
  each size has 30 runs per arm. Rho's per-run spread is wide (sd ≈ 0.52
  × mean). With 9 runs, the size means carried about ±17% standard error.
- **Check K6 (rho positive control).** On scratch runs (40 targets on
  three instances), mean rho `steps/p` was `0.450` and `0.490` against
  `0.443` (36 bits, `h = 4`), and `0.381` against `0.313` (32 bits,
  `h = 8`). The re-run reports each size's rho mean against
  `0.886/√h`.

Everything else stays as in Addendum A and Amendments C and E. The re-run
is the recorded Part O/J result.

## Addendum H (round 4: follow-up experiments), frozen before any of them runs

The owner approved all eight leads on 2026-10-18 ("yes to all"). Cloud jobs
go through the `ephemeral` runner (AGENTS.md §9a). Every job records the
commit, the instance type, `host.txt` and `rusage.txt`. Costs are counted
operations, as in rounds 1–3. Each experiment states its decision rule
before it runs.

### H1 (lead 3.1): the cofactor law for Arm J — first

**Hypothesis.** Arm J's trials scale as `p/h^α`. Rho scales as
`0.886·p/√h`. If `α > ½`, Arm J beats rho by count on large-cofactor curves
over `F_{p²}`.

**Instances.** The Scholten family, so `h` is even. The cofactor is exact
(`#E = h·ℓ`, `ℓ` prime), with
`h ∈ {2, 4, 6, 8, 12, 16, 24, 32, 48, 64, 96, 128}` and
`log2 #E ∈ {32, 36, 40}`. Each `(h, size)` cell has 4 instances × 10
targets.

**Arms.** R (rho, as in Amendment G) and J (`c = 2`, walk). At 32 bits
only, also Jind (independent trials, Amendment F).

**Recorded per run.** trials/p, the split fraction, and the cofactor
group's structure (cyclic or not, from `ℓ·P` on random points).

**Model check M-H1.** For each instance at `log2 #E ≤ 36`, compute the
class `ℓ·P` of every degree-1 point. Simulate the uniform admissible-pair
graph: an edge joins two points whose classes cancel up to sign. Read the
first cycle with union-find, then trials = edges / (measured split
fraction). Compare the measured trials with the model, cell by cell.

**Statistic.** Fit `α` by least squares of `log(trials/p)` on `log h`
over the cell means, with a bootstrap 95% interval (resampling instances
within cells).

**Decision.**
- Interval entirely below ½: Arm J never beats rho on count; stop this
  line.
- Interval entirely above ½: report the crossover `h*`, where the count
  ratio times the per-operation cost ratio (measured, and IC-favourable)
  falls below 1 in every pricing cell.
- Otherwise: unresolved.

### H2 (lead 3.2): the general transfer, and `h = 1`

**Hypothesis.** Any `E/F_{p²}` with `j ∉ F_p` can be glued to `E^σ` along
Frobenius on `E[2]` (Kani / Howe–Leprévost–Poonen). This gives a bielliptic
genus-2 `C/F_{p²}`, `y² = s_1 Π(x² − (a_i − s_2)/s_1)`, with quotient maps
`X = s_1x² + s_2` and `X' = t_1/x² + t_2`. Here `(b_i − t_2)(a_i − s_2) = k`
with `b_i = a_i^p`; `s_2, t_2, k` are solved linearly, and
`s_1 = −kΠ(a_i − s_2)`. `C` should descend to `F_p` through the descent
datum `x ↦ λ/x^σ`, with `λ² = k/N(s_1)`.

**Tool.** A new `examples/res_glue.rs`:
- `F_{p⁶}` arithmetic for the 2-torsion of odd-order `E`;
- the gluing;
- the `F_p` model through a Baer-subline (cross-ratio) map;
- balanced real-model (sextic) Jacobian arithmetic, since odd order means
  no rational Weierstrass point;
- the transfer `τ = φ_1^* + σ`.

**Checks.**
- G1: `#C(F_p) = p + 1` and `#C(F_{p²}) = p² + 1 − 2t`;
- G2: the group law is associative, `A + (−A) = 0`, and `N·A = 0`;
- G3: `τ` is a homomorphism, nonzero on the `ℓ`-part, with `ℓ·τ(P) = 0`;
- G4: on the Scholten family, the result agrees with `n2_routes`
  (same `#Jac`).

**Measurement.** Arm J and rho at `h = 1` and `h = 2`, with
`log2 #E ∈ {28, 32, 36, 40}`, 4 instances × 10 targets.

**Decision.** If G1–G3 hold, the route reaches every curve over `F_{p²}`.
The report then states the `h = 1` gap to rho.

### H3 (lead 3.3): a cheaper measured Jacobian operation

**Work.**
- Reduced formulas with values precomputed per walk multiplier (the
  resultant and inverse pieces of `u_2`).
- Mixed addition in the walk.
- An honest split-test price: the Legendre symbol at one `I`, replacing
  the table lookup.

**Checks.** The new formulas must agree with Cantor on 10⁵ random pairs.
They replace the measured price (a) only if they agree everywhere.

**Decision.** Report `c_J/c_E` before and after. Not admissible: any
literature constant.

### H4 (lead 3.4): memory against time

**Arm Jθ.** Keep a relation only if both of its points are
"distinguished", with `θ ∈ {1, ½, ¼, ⅛}`.

**Record.** Trials, and the peak count of stored rows. Rho's stored
distinguished points are the reference.

**Prediction.** Trials ∝ `1/θ` and memory ∝ `θ`. The time × memory product
is then about constant, near `p²/9`, against rho's `≈ p·(p/2^d)`.

**Sizes.** `log2 #E ∈ {32, 36}`.

### H5 (lead 3.5): automorphisms

**Census.** An exact census over `F_p`, `p ∈ {5, 7, 11, 13}`, of genus-2
curves with even L-polynomial (`a_1 = 0`) and a nontrivial reduced
automorphism group over `F_p`. Each class records whether its Jacobian
splits over `F_p`.

**Decision.** If a non-split class exists, measure Arm J with the
automorphism-reduced graph on it, against rho in `Jac(C)` with the same
automorphism.

### H6 (lead 3.6): larger sizes

**Run.** `log2 #E ∈ {44, 48}`, arms R, J and D at `c = 2`, 3 instances ×
10 targets.

**Prediction.** The per-`p` constants of Amendment G hold.

### H7 (leads 1.1 and 1.2): coverage of the target subgroup

**What is measured.** For each curve and arm (subfield, null), exact
coverage of the `ℓ`-subgroup (`h·E`) and of each layer `2^k E`.

**Cells.**
- **1.1:** binary `n = 2` (`m = 4…10`) and `n = 4` (`m = 3…5`), 3 curves
  each.
- **1.2:** odd `n = 2` and 4 with `E ≅ Y² = Π(X − d e_i)`, `e_i ∈ F_q`,
  `d ∈ F_{q^n}` generic. The factor base is `X ∈ d·F_q`.
- **Controls:** odd generic `E` (Part D curves), and the null arm in every
  cell.

**Prediction.** Gain over null = `[E : H]` for the coset subgroup `H`:
2 for binary, 4 for odd full-2-torsion at even `n`. The `ℓ`-subgroup is
covered like `2E`.

**Statistic.** Paired ratio of coverage on the `ℓ`-subgroup (subfield
over null), with a z-score.

### H8 (lead 1.3): decomposition crossover at `n = 3`

**Arm D3.** Gaudry decomposition in `E(F_{p³})` with factor base
`x ∈ F_p`. The oracle is the descended `S_3`. Resultants eliminate the
system to a univariate polynomial; its roots are found by gcd with
`x^p − x`. It is verified against brute force at `p ≤ 31`.

**Comparison.** Against rho on `E(F_{p³})`, at `p ≈ 2^6 … 2^13`, 3
instances × 5 targets, with the pricing band.

**Record.** The exponent and the extrapolated crossover. An extrapolation
is labelled as one.

### H9 (lead 2.1): solving degree of PKM systems

**Run.** The repo's degree-of-regularity tools (`examples/dreg_*`) on the
PKM summation systems at `m = 3, 4` over primes of 12–24 bits, with a
generic control system of the same shape.

**Prediction.** The solving degree grows with `p` (no PKM gain).

### Order and budget

**Order.** H1, H6 and H7 run first on one `c7i.4xlarge` (16 vCPU,
on-demand, TTL 4 h); their tools exist or need small changes. H2, H3, H4,
H5, H8 and H9 follow as their tools land, one ephemeral job per
experiment. Each recorded run follows the research-visuals rule.

### Amendment H-a (before any recorded H run)

**H1.**
- **Cofactor list.** The Scholten family never yields `h ∈ {2, 6}`:
  every one of the 222 instances generated so far has `4 | #E`, and the
  generator looped on `h = 2`. The `h` list becomes
  `{4, 8, 12, 16, 24, 32, 48, 64, 96, 128}`. `h = 2` and `h = 1` move to
  H2, whose general gluing has no such constraint.
- **Model check (scratch smoke, one cell).** At `log2 #E = 24`, `h = 4`,
  model M-H1 predicted trials/p 0.365 against 0.377 measured (4 instances).
- **Where it runs.** The `ephemeral` runner cannot authenticate here: there
  is an AWS access-key ID but no secret key or credential source. H1 and H6
  therefore run on RunPod CPU pods through
  `research/weil_descent_angles_20261018/runpod/podrun.py`. The pod fetches
  the tool at a pinned, pushed commit from the public repository and checks
  each file's SHA-256 before building. Its command is fixed at launch, and
  it serves only read-only status and results. `pod.json`, `host.txt` and
  `rusage.txt` are kept with each run.

**H7.**
- **Split-twist curves.** A split-twist curve with `d` a non-square in
  `F_{q^n}` has no factor-base points except the three 2-torsion points,
  because every `X ∈ d·F_q` has a right-hand side in `d³·F_q`. With `d` a
  square, every `X` carries 2 points, so the factor base is twice the
  null's, and the target layer saturates (coverage 1.0 at `p = 401`).
- **Change.** The recorded H7 keeps the frozen cells. It also adds a
  size-matched split-twist cell: `d` a square, and the factor base the
  first `|F_q|/2` elements of `d·F_q` in a fixed random order. That gives
  as many points as the null arm, so the comparison isolates the coset
  confinement from the density.

### Amendment H-b (H2 and H3 tools), after scratch checks and before the recorded H2 run

**Gluing map.** Gluing along `ψ(a) = a^p` on the 2-torsion roots does not
descend unless `E[2]` is split: in 117 of 117 scratch attempts with odd
`#E`, `t_2 ≠ σ(s_2)`. The descent cocycle needs `ψ^σ ∘ ψ = id` on the
roots. `ψ(a) = a^{p³}` satisfies it whenever the 2-torsion cubic is
irreducible over `F_{p²}` (roots in `F_{p⁶}`, so `a^{p⁶} = a`) or split.
With exactly one rational root, no Galois-equivariant `ψ` satisfies it.
So `h ≡ 2 (mod 4)` is not reached this way, consistent with the Scholten
family's `4 | #E`. H2 therefore uses `h ∈ {1, 4}`, not `{1, 2}`.

**Scratch checks** (`n2_routes <dir> gluecheck`).
- 16 instances, `p` = 131 … 4549.
- Descent holds in every instance: `t_2 = σ(s_2)` and `λ² ∈ F_p`.
  `λ ∈ F_p` holds in about half of the attempts; the others are discarded
  and redrawn.
- G1: `#C(F_p) = p + 1` everywhere, and `#C(F_{p²}) = p² + 1 − 2t` where
  counted (`p < 400`).
- G2 and G3 (associativity, inverse, `N·A = 0`, homomorphism, `τ ≠ 0`,
  `ℓ·τ = 0`) all hold. The only exceptions are three checks reported as
  "degenerate": a point at infinity was met at `p ≤ 277`, and no
  inequality was found.

**H3 (folded in): reduced real-model formulas.** These agree with the
generic real-model law on 2,400 of 2,400 pairs. They cost 41 M + 1 I per
addition and 51 M + 1 I per doubling; the generic law costs 113 M + 4 I and
123 M + 4 I. The recorded H2 run uses them; this is pricing (a).

**Break-even, stated before the run.** With trials `T_J·p` and rho steps
`0.886·p/√h`, the IC needs `(c_J + c_split)·T_J < 0.886·c_E/√h`, where
`c_E = 12 M + I` and `c_split = 2 M + 1 Legendre` (priced as one `I`).
At `h = 1`, with `T_J ≈ 0.44` from the scratch run and `I = 3M`, that
requires `c_J ≤ 22 M + I`.

### Closing note on Addendum H (after the runs)

- **Ran:** H1, H2, H3 (folded into H2), H4, H5, H6 and H7, each under
  the frozen text plus Amendments H-a and H-b. Results are in
  `REPORT_ROUND4.md`.
- **H8 (decomposition at `n = 3`) was not run.** The repo's Semaev
  tools are binary-field only, and a descended-`S_4` oracle over `F_{p³}`
  is a separate build. It is listed as the next item-1 experiment.
- **H9 is answered by existing studies, with no new run:**
  `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` and
  `research/pkm_tower_round*`, `pkm_dickson_torus_20261015`. Re-measuring
  would duplicate them.

### Erratum V (2026-10-24, after every run): native work in GAE, not M

This erratum was written after all the runs, during validation for the
benchmark gate (`docs/VALIDATION_GATE.md`). It changes no recorded result.

- **What was wrong.** Amendment C and the Addendum H pricing price native
  operations at `0`, `0.1` and `1` `F_p` multiplications, yet both cite
  AGENTS.md §5a. §5a prices native work at `0`, `0.1` and `1`
  group-addition equivalents (GAE) per operation. The frozen band is
  therefore narrower than §5a requires.
- **Re-pricing.** One GAE is the row's own measured affine `E(F_{p²})`
  addition at the cell's inversion price, `eadd_M + ipr·eadd_I`: about
  15 M at `I = 3M` and about 50–110 M at `I = 2·log2 p M`.
- **Everything else stays frozen**: the cost model, the cells, the
  Legendre split test at one `I` for H2, and Arm D's 6 cells.
- **Tool and outputs.** The tool is `n2_routes <dir> gaeband <csv>`
  (commit d01c574a2). Its outputs are in `validation/gae-band/`; the
  recorded results are unchanged.
- **Ranking rule.** Unchanged: rank only where the order holds in every
  cell.
