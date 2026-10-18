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
