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
