# L3 desk part: translated involutions and the models closed on E (2026-11-22)

Status: derivations, each with an exact numerical check at k = 16 in `smoke/desk_k16.json`
(`scripts/desk_checks.py`, 30/30 checks pass, smoke, not data) [pcc:unverified]. The same script runs at
k = 16, 18, 20 in the full run.

## 0. Setting

- E: y^2 = x^3 + A x^2 + x over F_p, #E = 8r with r prime, E(F_p) ≅ Z/8r (cyclic), T0 = (0, 0).
  A^2 − 4 is a non-square, so T0 is the only rational 2-torsion point.
- E' = E/<T0>, built with Vélu's formulas (kernel point (0,0), v = 1):
  y^2 = x^3 + A x^2 − 4x − 4A = (x + A)(x − 2)(x + 2), with φ(x, y) = (x + 1/x, y − y/x^2).
  We use X = x + A, so that Y^2 = X(X − α)(X − β), with α = A + 2 and β = A − 2.
  - #E' = #E, by a character sum.
  - E'(F_p) ≅ Z/2 × Z/4r, and all of E'[2] is rational.
  - φ is a homomorphism and maps E[r] injectively into E'[r]. This was checked on 2000 samples
    (checks `phi_*`).
- G2 is the 2-part of the group: Z/8 on E, and Z/4 × Z/2 on E'. The coset of P is its image in G2.
  The order-r subgroup is coset 0.
- An x-frame base is B_x(V) = {P : x(P) ∈ V}, where V = ν([0, 2^t)) for a Möbius map ν. This is
  `frame_search`'s base. Its coverage is the number of distinct R ≠ O in E[r] with R = ±P_i ± P_j.

## 1. Every degree-2 function is an ι_T function, and T is rational

Let f: E → P^1 have degree 2 and be defined over F_p.

- All fibres of f are linearly equivalent degree-2 divisors, so the group sum of a fibre is a
  constant T. The fibre over ∞ is Galois-stable, so T ∈ E(F_p).
- f is therefore invariant under ι_T: P ↦ T − P, and F_p(E)^{ι_T} = F_p(f).
- By Riemann–Hurwitz f has 4 ramification points. They are the points with 2P = T, the four halves
  of T.

T = O gives the x-frames, where ι_O = negation.

Explicit generator. For T ∉ E[2], h_T(P) is the slope of the line through −T and P. For T ∈ E[2],
h_T(P) = y/(x − x_T). Since P, T − P and −T are collinear, h_T is ι_T-invariant. It has degree 2,
with poles at O and T. Every ι_T function equals μ∘h_T with μ ∈ PGL2(F_p). The census builds bases
of μ∘h_T by solving the line–cubic intersection (`curves.slope_base`).

## 2. Theorem 1: a rational half makes it a translated x-frame

Suppose S ∈ E(F_p) with 2S = T. Then x∘τ_{−S}, i.e. P ↦ x(P − S), is ι_T-invariant, because
(T − P) − S = −(P − S). It is defined over F_p and has degree 2. So
**f = μ∘x∘τ_{−S}, with μ ∈ PGL2(F_p).**

**Base.** B_f(V) = S + B_x(μ^{-1}V): the base of f is the x-frame base for frame μ^{-1}∘ν, translated
by S.

**Relations.** Write P_i = S + Q_i, where B_x is closed under negation.

| relation on the f-base | equals | as an x-frame relation |
|:--|:--|:--|
| R = P_i − P_j | Q_i − Q_j | target R itself |
| R = P_i + P_j | T + (Q_i + Q_j) | target R − T |
| R = −P_i − P_j | −T − (Q_i + Q_j) | target R + T |

So the mixed-sign relations of a translated frame are exactly the x-frame relations, with no shift.
The same-sign relations are x-frame relations for the shifted targets R ∓ T. As sets:

- D_f := B_f − B_f = Σ_x := B_x ± B_x.
- All signed pair sums of B_f = Σ_x ∪ (Σ_x + T) ∪ (Σ_x − T).

An x-frame user covers that union by decomposing R, R − T and R + T, which is three decompositions
of the same degree. Counting it as one relation search of a "new coordinate" is relabelling.

Check `E_translated_frame_identity` / `Ep_translated_frame_identity`: for 5 random T = 2S per curve,
B_f − S is closed under negation, and h_T = M∘x∘τ_{−S} holds on every base point for a fitted
Möbius M. In the census, `control_check` repeats this for every frame of the three control arms.
There the counts must equal the x-frame census on frame M^{-1}(V) exactly.

## 3. When there is no rational half

**Lemma 2 (holds for every T).** B_f = T − B_f, so B_f + B_f = T + D_f. All signed pair sums are
therefore D_f ∪ (D_f + T) ∪ (D_f − T).

D_f is unchanged under f ↦ f∘τ_R for R ∈ E(F_p): the base becomes B_f − R, which has the same
differences, and T moves to T − 2R. So **the mixed-sign coverage of an ι_T function depends only on
the class of T in E(F_p)/2E(F_p) and on the Möbius frame**, up to translation.

**Lemma 3: which T have a rational half.**

- **E.** E(F_p)/2E(F_p) ≅ Z/2.
  - T ∉ 2E(F_p) ⇔ log T is odd ⇔ x_T is a non-square. This is half of all points. The reason is that
    2E(F_p) = φ̂(E'(F_p)), which is the kernel of P ↦ x_P mod squares.
  - T0 = 4r·g lies in 2E(F_p): its halves are the order-4 points with x = ±1. So the Montgomery
    coordinate y/x = h_{T0} is a translated frame.
  - For odd T, no half lies in E(F_{p^2}). With E[2] ⊂ E(F_{p^2}), the halving criterion asks
    whether x_T − e_i is a square in F_{p^2} for each i. For e_1 ∉ F_p, N(x_T − e_1) = x_T^2 + A x_T + 1
    = y_T^2/x_T, a non-square.
  - Galois argument: e(S) = π(S) − S takes the values {e, e, e + T0, e + T0} on the four halves. With
    no rational half, e is an irrational 2-torsion point, and π^2(S) = S + e + π(e) = S + T0 ≠ S. So
    the halves are defined over F_{p^4}.
  - **Correction to the brief:** on E the translation is defined over F_{p^4}, not F_{p^2}.
  - Checks `E_half_iff_log_even_iff_xT_square`, `E_odd_T_no_half_over_Fp2`.
- **E'.** E'(F_p)/2E'(F_p) ≅ (Z/2)^2. Encode T = k·g + ε·h; its class is (k mod 2, ε).
  - T ∈ 2E'(F_p) ⇔ class (0,0) ⇔ X_T − e_i is a square for all three e_i. That covers 1/4 of the
    points.
  - Every T has a half S ∈ E'(F_{p^2}), because every element of F_p is a square in F_{p^2}.
  - The Kummer map class ↦ π(S) − S ∈ E'[2] is a bijection.
  - At k = 16, in the X coordinate: (0,1) ↦ (A+2, 0), (1,0) ↦ (A−2, 0), (1,1) ↦ (0,0). This is
    computed with explicit F_{p^2} halving (`half_over_fp2`).
  - Checks `Ep_half_iff_class00_*`, `Ep_every_T_has_half_over_Fp2_*`, `Ep_class_to_e_injective`.

**What base and relations look like then.**

- f is defined over F_p. Its base is still a translate: B_f = S + B̃ with
  B̃ = {Q : x(Q) ∈ μ^{-1}V}. But S, B̃ and μ live over F_{p^4} on E (F_{p^2} on E').
- The frame is twisted by the 2-torsion: Frobenius moves Q to Q − e, so μ^σ = m_e∘μ, where m_e is
  the Möbius action of τ_e on x.
- No F_p x-frame has the same difference set in general.
- The relation table of §2 still holds with S over the extension: D_f = B̃ − B̃ ⊂ E(F_p).

**Theorem 4: the within-fibre term vanishes.**

- A fibre {a, T − a} gives the difference a − ι_T a = 2a − T. It lies in E[r] iff 2·coset(a) = coset(T).
  That has a solution only when coset(T) ∈ 2G2, i.e. exactly when T has a rational half.
- So **an ι_T function with no rational half has no within-fibre relations.** An x-frame has them
  whenever 2·coset(a) = 0, i.e. P − (−P) = 2P ∈ E[r]. On E that happens for 2 of the 8 cosets. On E'
  it happens for 4 of the 8, because of the extra rational 2-torsion.
- Cross-fibre relations have the same expected count in both cases (§4).
- Prediction: a no-half ι_T base has strictly lower mean coverage. The deficit is about N/2 points on
  E and about N points on E', out of N^2/4, where N ≈ 2^{t−1} fibres.
- E' x-frames have about N/4 more than E x-frames.

**Degree of the pair polynomial (sketch, not engine-tested).**

- For a no-half ι_T base with the target read in x, the mixed-sign set gives 2 values of x(R) per pair
  (x(a − b) and x(a + b − T)). Given x(R) and f_2, it gives 2 values of f_1. So the relation polynomial
  Φ(f_1, f_2, x_3) has multidegree (2, 2, 2), the same as S3.
- The full signed set gives 12 points per pair: 6 values of x(R), i.e. 3× the target degree for about
  3× the coverage (model ratio full/D = 3.0–3.5 below saturation). It is reported, not counted as a
  gain.

## 4. The named E' models

**Huff.**

- A Huff model exists over F_p: a x(y^2 − 1) = b y(x^2 − 1). It comes from W: v^2 = u(u + a^2)(u + b^2),
  with u = X − e0, where e0 is the unique 2-torsion abscissa in 2E'.
- The map is x = b(u + a^2)/v, y = a(u + b^2)/v (derived here; check `huff_equation_all_points`).
- x has simple poles at W0 and Wb, so it is an ι_T function with T = Wa. Likewise y has T = Wb.
- Both T are 2-torsion points outside 2E'(F_p), and both lie in class (0,1). They have halves only
  over F_{p^2}. Checks `huff_degree_2`, `huff_*_iota_T_with_T_2torsion_not_in_2E`.
- By Lemma 2, Huff x, Huff y and any class-(0,1) ι_T function with a random Möbius frame have the
  same D-coverage distribution. The brief's premise holds for Huff.

**Jacobi intersection: the premise fails.**

- αs^2 + c^2 = 1, βs^2 + d^2 = 1, with s = 2Y/(X^2 − αβ), c = (X^2 − 2αX + αβ)/(X^2 − αβ),
  d = (X^2 − 2βX + αβ)/(X^2 − αβ).
- These were derived from the JI doubling formulas (check `ji_equations_all_points`).
- s, c and d all have **degree 4** (fibre sizes 4 or 0; check `ji_degree_4`), so they are not
  ι_T functions. Each is invariant under a Klein four group, read off the fibres:
  - c: {id, −1, τ_{eβ}, ι_{eβ}}. So c = Möbius∘x_{E'/<eβ>}∘φ_{eβ}, an x-frame pulled back from
    another curve of the 2-isogeny graph.
  - d: the same with eα.
  - s: {id, τ_{(0,0)}, ι_{eα}, ι_{eβ}}. (0,0) = ker φ̂, so s = g∘φ̂ for a degree-2 function g on E that
    is ι_{T0}-invariant, since φ̂(eα) = T0.
- **JI s is an E translated x-frame (by an order-4 point S ∈ 2E) pulled back to E' by the dual
  isogeny.** Its mixed-sign coverage of E'[r] is the E x-frame coverage restricted to base points in
  2E(F_p). This last identity is derived, not separately checked pointwise.
- Degree-4 consequence:
  - Each value carries an orbit of 4 points with probability 1/4.
  - Each difference value is repeated 4 times rather than 2.
  - Cross-orbit relations number about N4^2/2 with N4 ≈ 2^{t−2}, against N^2/4 with N ≈ 2^{t−1}.
  - Prediction: **about half the coverage at equal t**, on top of a higher-degree summation
    polynomial.

## 5. Pair-count model (`scripts/model.py`)

- **Degree 2.** N ~ Bin(2^t, n/2p) fibres, with cosets uniform and independent.
  - The distinct differences in coset z number E[N(N−1)]/4 + E[N]·w2(z), with
    w2(z) = (#{2c = t_T + z} + #{2c = t_T − z})/8.
  - The full set adds the cosets z = ∓t_T (shifted by ±T).
- **Degree 4.** E[N4(N4−1)]/2 + E[N4]·w4, with N4 ~ Bin(2^t, n/4p).
  - w4 = 1 for s. For c and d, w4 = 1 + [e ∈ 2E'].
- Saturation: (r−1)(1 − e^{−M/(r−1)}).
- The model reproduces `frame_search`'s Möbius-frame means to about 1%: 19.1 / 70.2 / 265.5 / 990.4 /
  3271.6 predicted at k = 16, against 19.2 / 70.0 / 266.3 / 988.7 / 3266.5 measured
  (`frame_search` `analysis/summary.json`, `mobius_mean`) [pcc:unverified].

## 6. What the desk part decides before any census

- **ι_T with a rational half (T ∈ 2E(F_p)).** This includes the E' translates and Montgomery y/x.
  They are translated x-frames over F_p, so they are controls. Their mixed-sign census equals the
  x-frame census frame by frame, and the extra signed relations are x-frame relations for R ∓ T.
- **ι_T with no rational half.** This is half of the T on E and three quarters on E', and includes
  the Huff coordinates. They lie outside the F_p frame theorem: the translation exists only over
  F_{p^4} (E) or F_{p^2} (E'). They are a genuinely different F_p factor base. The model predicts
  they are **worse** at equal F4 degree, since they lose the within-fibre relations.
- **JI coordinates.** They are degree-4 pullbacks of x-frames on 2-isogenous curves (for s, the curve
  E itself), predicted at about 0.5× coverage.
- **The only predicted positive difference is E' x-frames over E x-frames.** E' has 4 self-paired
  cosets instead of 2. The predicted mean gain is 1.21 / 1.11 / 1.06 / 1.03 / 1.01 at t = 4..8. It is
  a small component effect on the algebraic track and fades as 1/N. The isogeny transport of targets
  costs a few field operations per point.
