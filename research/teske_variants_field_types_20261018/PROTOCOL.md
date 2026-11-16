# Protocol: Teske-type hidden Weil descents by field type

Frozen 2026-10-18, before the first run of
[`examples/teske_variants_field_types.rs`](../../examples/teske_variants_field_types.rs).
Conductor is not installed on this host (the release installer answers HTTP
403 through the session proxy), so there is no task card; the previous
odd-characteristic study (`research/odd_char_descent_genus3_20261014/`)
ran under the same condition. This study is literature reading plus exact
enumeration over toy fields. It solves no discrete logarithm, times
nothing, and claims no speedup or security property of any deployed curve.
Every number it produces is a count or a formula evaluated on exact
invariants, in the sense of the repository's "formula-level" rows
(`docs/curves/JACOBIAN_TRANSFERS.md`).

## Question

Teske's trapdoor system (J. Cryptology 19 (2006); ePrint 2003/058) is a
pair of isogenous curves over `F_{2^161}`: the private one admits a GHS Weil
descent to a genus-7 or genus-8 hyperelliptic curve over `F_{2^23}`, the
public one does not, and the isogeny is the secret. The pattern needs
three things: a weakness that is **not an isogeny-class invariant**
(the GHS genus depends on the curve, not on its trace), a **transfer**
that follows the isogeny, and a weak set that is **sparse inside the
isogeny class**, so that a public auditor cannot find the weak member by
walking the class faster than Pollard rho solves the problem.

The question of this round: over which field types, and for which
extension degrees `n`, do the three ingredients exist at all; where is the
weak set sparse enough inside the class to hide (the auditor's search
costs at least rho) and where is it dense enough that the field is simply
weak for everyone (Menezes–Teske–Weng's "weak field"); and are there
other ways than the GHS compositum to attach a Weil-descent target to a
curve. The literature answers parts of this for binary fields; the
odd-characteristic side and the cross-field comparison are not written down
in one place, and the counts below have not been checked exactly.

## Definitions used throughout

For `E/F_{q^n}` with a descent target of genus `g` over `F_q`, write (all
exponents in units of `log q`, constants dropped):

- `n/2`: Pollard rho on the prime-order subgroup (`#E ≈ q^n`).
- `2 − 2/g`: index calculus on a genus-`g` Jacobian over `F_q` with the
  double-large-prime variation (Gaudry–Thomé–Thériault–Diem); for a genus-3
  plane quartic Diem–Thomé give `1`. This is the **owner's** exponent, a
  heuristic with constants that grow with `g`; it is quoted, not measured.
- `δ`: the weak set has about `q^{n−δ}` curves among the `≈ q^n`
  isomorphism classes, i.e. a random curve is weak with probability
  `≈ q^{−δ}`.
- The isogeny class of a random curve has `≈ q^{n/2}` members (Hasse–Weil;
  GHS 2002 §5; Yee 2016 §5.2). Under the uniform-distribution heuristic
  of GHS 2002, Menezes–Teske 2006 and Yee 2016 (an unproved assumption,
  tested by them on small binary fields), a class contains
  `≈ q^{n/2−δ}` weak curves, so an auditor who walks the class finds a
  weak member after `≈ q^{min(δ, n/2)}` steps when one exists, and most
  classes have none when `δ > n/2`.
- **Margin** `a = δ − n/2`. `a > 0`: the weak set is sparse in the class
  (a hidden member cannot be found by walking faster than rho; this is
  the regime Teske's construction needs). `a < 0`: the walk is cheaper
  than rho and, when `2 − 2/g < n/2`, the whole field is weak for every
  class that contains a weak curve (the MTW regime).

These exponent formulas are tagged **[H]** in the report. Exact counts
produced by the tool are tagged **[E]**; theorems with a source or a proof
**[T]**; everything not established **[O]**.

## Part A: binary fields, exact combinatorics of the magic number

Input: odd `n` with `3 ≤ n ≤ 99`. For each `n` the tool computes the
cyclotomic cosets of `2` modulo `n`, i.e. the irreducible factors of
`x^n − 1` over `F_2` with their degrees and orders (Lidl–Niederreiter,
Thm 3.62 as used by Karabina–Menezes–Pomerance–Shparlinski 2010 §4).

- **A1 (GHS).** For every product `F` of distinct irreducible factors with
  `lcm(orders) = n` (so that `Ord_b | F` does not force `b` into a proper
  subfield), record `deg F`, the type (`x+1 | F`: type II, else type I),
  `m = deg F + [x+1 ∤ F]`, `g = 2^{m−1} − [x+1 ∤ F]` (GHS Theorem 1;
  Maurer–Menezes–Teske 2002, Thm 5; `ec_trapdoor.rs`), the count of `b` with
  `Ord_b | F`, which is exactly `q^{deg F}`, and `δ = n − deg F`.
- **A2 (gGHS, Hess 2003/2004).** For pairs `(F_α, F_β)`, each a single
  irreducible factor or `(x+1)` times one, with `lcm` of the orders of the
  factors of `F_α F_β` equal to `n` (condition (5) of KMPS 2010), record
  `r = deg F_α`, `s = deg F_β`, `t = deg lcm(F_α, F_β)`,
  `g = 2^t − 2^{t−r} − 2^{t−s} + 1` (KMPS 2010 eq. (4), from Hess 2004 Thms
  11–12) and `δ = n − r − s + 1` (the pairs `(α, β)` with `Ord_α | F_α`,
  `Ord_β | F_β` number `q^{r+s}` and `b = (αβ)^2` is invariant under
  `(α, β) ↦ (λα, β/λ)`, `λ ∈ F_q^*`; Hess 2003 §4 counts `s q^{2d−1}/2`
  for `F_α = F_β = h_i`). Pairs outside this family are not enumerated.
- **A3 (bit table).** For `N ∈ {150, 155, 161, 165, 168, 185, 210, 300,
  600}` and every odd `n | N`, `n ≥ 3`, `l = N/n`, the tool prints, for the
  minimal-genus A1 row of each type and the minimal-genus A2 row: rho
  `N/2`, owner `(2 − 2/g)·l`, auditor walk `min(δ, n/2)·l`, the margin
  `a·l`, and the expected number of weak Frobenius-orbit classes per isogeny
  class `c · q^{deg − n/2} / (n l)` (GHS 2002 §5, Yee 2016 §5.2, with
  `c` the number of factors of that shape), capped at 1.

Checks, as unit tests of the tool:

1. `n = 7`: factors of degrees `1, 3, 3`; type I `m = 4, g = 7, δ = 4`;
   type II `m = 4, g = 8, δ = 3`; gGHS `(r, s) = (3, 3)`: `g = 7, δ = 2`.
   The per-class expectation for type II at `q = 2, 4, 8, 16, 32` must
   reproduce GHS 2002 §5's table `0.404, 0.286, 0.269, 0.286, 0.323`.
2. `n = 31`, `q = 2^5`: basic type II `δ = 25` (GHS 2002: proportion
   `6 q^{−25}`), per-class expectation `≈ 2^{−52}`; gGHS `(5, 5)`:
   `g = 31`, `δ = 22`, per-class `≈ 2^{−38}` (Hess 2003 §4).
3. `N = 161 = 7·23`: `m = 4`, `g = 7`, `log2` of the count of curves
   (both `a`-twists, both factors) `= 94`; `N = 165 = 15·11`: `m = 5`,
   `g = 15`, `log2 count = 57`; `N = 170 = 5·34`: `m = 5`, `g = 15`,
   `log2 count = 171` (MMT 2002, Table 1 and Remark 20).
4. `n = 15`: the minimal non-subfield `m` is 5 (from the order-15
   quartics), not 7 (from `(x^2+x+1)(x^4+x^3+x^2+x+1)`); `n = 21`: the
   minimal `m` is 6 from `(x^2+x+1)(x^3+x+1)` (degree 5), below the 7 of
   the order-21 sextics. Products of non-primitive factors are allowed.

## Part B: odd characteristic, exact counts of weak curves and their classes

Setting as in the previous study: `E/K`, `K = F_{q^n}`, `q = p` an odd
prime, a degree-2 function `x: E → P^1` with branch set `B` (four points of
`P^1(F̄_q)`, `σ^n B = B`), `S = ∪ σ^i B`, `r = |S|`, `m = dim_{F_2}
span{1_{σ^i B}}`, genus `g = 1 + 2^{m−2}(r − 4)` (Theorem 1 there). A
curve is *weak of type `τ`* when some `x` on it has branch type `τ`.

**Hypothesis H_B (derived, [H] until checked).** A branch type whose `S`
is a union of `σ`-orbits of sizes `d_1, …, d_k` with `Σ d_j = r` is
realised by `≈ q^r` branch sets `B`, these are permuted by `PGL_2(F_q)`
(order `≈ q^3`), which preserves the type, so the number of `j`-invariants
(curves up to `K`-isomorphism and twist) that are weak of that type is
`≈ c_τ · q^{r−3}` and `δ = n + 3 − r`. Checked values in the literature:
`n = 2, r = 5` (Scholten: all curves), `n = 3, r = 5` (Momose–Chao: `Θ(q^2)`),
`n = 5, r = 5` (Chao 2007: `Θ(q^2)`), `n = 6, r = 6` (Joux–Vitse App. C:
`≤ 1/q^3`), `n = 7, r = 7` (Chao 2007: `Θ(q^4)`).

Tool, for each frozen `(q, n)`:

1. Build `A = F_{q^L}` with log, antilog and Zech tables (`L` below),
   `K = F_{q^n} ⊂ A`.
2. Enumerate every `σ`-stable `S ⊂ P^1(F̄_q)` that is a union of `σ`-orbits
   of the allowed sizes with `|S| = r ≤ r_max`, each orbit given by a monic
   irreducible polynomial over `F_q` of that degree (or the point `∞`), and
   every 4-subset `B ⊂ S` with `∪_i σ^i B = S` and period exactly `n`.
   Compute `(m, r, g)` from the indicator vectors and keep `g ≤ g_max`.
3. For each kept `B`: `j(B) ∈ K` from the cross-ratio (checked to lie in
   `K`); the trace `t` of `E_B: y^2 = ∏(x − b)` over `K` by exact point
   counting in `A` (`y^2 = P_B(x)` with the correct count at infinity),
   when the evaluation budget allows (below).
4. Record per type: the number of `B` instances, the number of distinct
   weak `j`, `log_q` of it against the predicted `r − 3`, and, when traces
   were computed, the class statistics: the number of traces `t` whose
   class contains a weak curve, the share of all curves in such classes,
   and the mean and maximum number of weak curves per covered class. The
   trace of every `j ∈ K` (one curve per `j`, both twists by `±t`) is
   computed exactly once by point counting, when `q^{2n} ≤ 2·10^11`.

Frozen ranges (`q ∈ {3, 5, 7, 11, 13}` throughout):

| `n` | allowed orbit sizes | `L` | `r_max` | `g_max` |
|--:|:--|--:|--:|--:|
| 2 | 1, 2, 3 | 6 | 6 | 3 |
| 3 | 1, 2, 3, 6 | 6 | 6 | 5 |
| 4 | 1, 2, 4 | 4 | 6 | 5 |
| 5 | 1, 5 | 5 | 7 | 9 |
| 6 | 1, 2, 3, 6 | 6 | 6 | 9 |
| 7 | 1, 7 | 7 | 7 | 7 |

`q = 11, 13` are dropped for `n = 7` (`q^L` too large for the tables) and
for `n = 6` the all-`j` trace table needs `q ≤ 7`. Orbits of size 2 for
`n = 5` and of size 3 for `n = 4, 7` are outside the ambient field and are
not enumerated; the report says so. Trace computation per `(q, n)` has a
budget of `4·10^11` curve-point evaluations, spent on types in increasing
genus; a type that does not fit reports `j`-counts only.

Checks:

5. Every `j(B)` lies in `K`; every computed trace satisfies
   `|t| ≤ 2 q^{n/2}`; for `n = 3` the `(m, r) = (3, 5)` curves have
   `#E ≡ 0 (mod 4)` (Joux–Vitse §4.1, "all the curves of the form (3)
   have a cardinality divisible by 4").
6. The all-`j` table lists exactly `q^n` values of `j`.
7. The types found per `n` agree with `types.csv` of the previous study
   (same `(m, r, g, period)` triples for the allowed orbit sizes).

Decision: **H_B holds for a type** when the weak-`j` count divided by
`q^{r−3}` is bounded away from `0` and `∞` across the three largest `q`
(within a factor of 3 of each other). The sign of `a = δ − n/2` is then
read off per `n`. If a type's count scales with a different exponent, the
report records the measured exponent and the hypothesis is amended, not
the ranges.

## Part C: Weil-restriction-shaped genus-3 hyperelliptic Jacobians over `F_q`

By Tate's theorem, a genus-3 curve `C/F_q` has `Jac(C)` isogenous to
`Res_{F_{q^3}/F_q} E` for some `E/F_{q^3}` of trace `t` exactly when
`L_C(T) = 1 − tT^3 + q^3T^6`, i.e. `#C(F_q) = q + 1` and
`#C(F_{q^2}) = q^2 + 1` ("Res-shaped"), and then `t = (q^3 + 1 − #C(F_{q^3}))/3`.
Such a `C` covers every curve in the class of `E` over `F_{q^3}` (compose
`C → Jac(C) → Res(E) → E` with isogenies), with a degree the census does
not determine. The GHS compositum of a Momose–Chao curve is one such `C`;
Tian 2020 obtains others, non-hyperelliptic, through `(ℓ,ℓ,ℓ)`-isogenies.
This part asks, exactly and at toy size, how many Res-shaped hyperelliptic
genus-3 curves exist, which classes they land in, and how many of them are
GHS compositums.

**Signature (derived, [T] in the report with proof).** For the
`(m, r) = (3, 5)` compositum, the hyperelliptic involution is `τ_1τ_2τ_3`
and the three sign changes induce a Klein four-group `V ⊂ PGL_2(F_{q^3})`
on the hyperelliptic line, permuted cyclically by the `q`-Frobenius, with
the eight Weierstrass points forming two free `V`-orbits. Consequently the
Frobenius cycle type on the Weierstrass points is `(1,1,3,3)` or `(2,6)`
(the previous study recorded exactly these on every Joux–Vitse instance).

Tool, for each `q ∈ {3, 5, 7, 11, 13}`:

1. Enumerate `y^2 = c·f(x)`, `c ∈ {1, ν}` (`ν` the least non-residue),
   `f` monic of degree 8 with zero `x^7` coefficient, and `f` monic of
   degree 7 with zero `x^6` coefficient (all coefficients for `p = 7`),
   `f` squarefree. These models cover every hyperelliptic genus-3 curve
   over `F_q` up to isomorphism.
2. Keep the Res-shaped ones (`#C(F_q) = q + 1`, then `#C(F_{q^2}) = q^2 + 1`),
   compute `#C(F_{q^3})` and `t`, check `3 | q^3 + 1 − #C(F_{q^3})` and
   `|t| ≤ 2q^{3/2}`, canonicalise under `PGL_2(F_q)` and `y`-scaling, and
   record each `F_q`-isomorphism class once with: `t`, `#E = q^3 + 1 − t`,
   whether `#E` is prime, `#E mod 4`, the Frobenius cycle type of the
   Weierstrass points (distinct-degree factorisation of `f`), the order of
   the `F_{q^3}`-rational reduced automorphism group, and the GHS
   signature: a Frobenius-cyclic Klein four-group of `F_{q^3}`-rational
   involutions of `P^1` with the Weierstrass set as two free orbits.
3. Positive controls: the Joux–Vitse `H` instances of the previous study
   (`jv_family.csv`, `p ∈ {5, 7, 11, 13}`, arms `linear`, `split`,
   `conjugate`) must be found Res-shaped with the recorded `t`, with cycle
   type `(1,1,3,3)` or `(2,6)`, and with the GHS signature.

Checks:

8. On every Res-shaped class `#C(F_{q^3}) = q^3 + 1 − 3t` with `t`
   integral in the Hasse range (consistency of `L`).
9. Every class with the GHS signature has `#E ≡ 0 (mod 4)` and cycle
   type `(1,1,3,3)` or `(2,6)`.
10. Each positive control passes.

Decision: **H_C** ("every Res-shaped hyperelliptic genus-3 Jacobian over
`F_q` is a GHS compositum") is refuted if a class without the signature
appears at two or more `q`; it is supported at toy size otherwise. The
report also states, for the non-signature classes, which traces `t` they
reach, including whether any has `#E` prime (Tian 2020 notes that a cover
of a prime-order curve must be non-hyperelliptic when it comes from a
covering map; a Res-shaped hyperelliptic Jacobian of prime order would be
isogenous to `Res(E)` without such a map, and the degree of the induced
cover is left open).

## What does not count

Changing a range, budget or threshold after seeing results; dropping a
`(q, n)` or a class that fails a check; reporting a formula-level exponent
as a measured cost; reading a toy-field count as a statement about a
cryptographic-size field without saying so. A failed check stops the run,
the tool is fixed, and the failure is reported.

## Outputs

`results/run-<UTC stamp>/` with `env.txt`, `binary_types.csv`,
`binary_ggHS.csv`, `binary_bits.csv`, `oddchar_types.csv`,
`oddchar_classes.csv`, `census.csv`, `census_classes.csv`, `checks.txt`.
Reports per `.claude/skills/research-visuals/SKILL.md`: `REPORT.md`,
`REPORT.typ`, `REPORT.pdf`, `figures/` (plot source beside SVG and PNG),
`reports/run-<stamp>/run-report.{typ,pdf}`.
