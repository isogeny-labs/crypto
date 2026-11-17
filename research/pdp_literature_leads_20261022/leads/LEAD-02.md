# LEAD-02 — Parametric eliminant: generic-fibre Gröbner basis in the target's coordinates, specialised per target

**Verdict.** closed-by-measurement (provisional, desk level). The repository's measured `C₃`, `C₄` set the
baseline. Exact degree bounds, three of them checked by desk computation, show that evaluating the eliminant costs
more per target than the solve it would replace. One quantity is unmeasured: the fill of the `k = 3` eliminant.
§6 is the single test that settles it.
**Confidence.** medium. The degree counts are verified at `k = 3`. The verdict needs the eliminant to be dense in its
Newton box, which holds at `k = 2` (54 %) but is not measured at `k = 3`. Break-even needs fill ≤ 1.6–2.5 %.
**Expected gain if it works.** None as a plan; it is a regression. Per-target evaluation is 4–60× above the measured
`C₃ = 0.88–0.90·10⁶` at `k = 3` (fill 10 % to 100 %) and about 9·10⁴× above `C₄(D₄) = 2.54·10¹⁰` at `k = 4` at full
fill. The ceiling, if evaluating the
eliminant were free, is `C₃` → about `1–1.7·10⁵` (5–9×) and `C₄(D₄)` → about `10⁷` (about 10³×). That ceiling is
arithmetic, not a result.
**Cost of the discriminating test.** About 2–3 person-days (one Rust example on the existing solver), under 1 CPU-hour,
under 1 GB.

## 1. Mechanism (precise)

**Setting, `k = 3`.** `E/F_{q³}`, factor base `F = {P : x(P) ∈ F_q}`, relation `R = ±P₁ ± P₂ ± P₃`. The symmetrised
fourth summation polynomial is `f̃₄(e₁,e₂,e₃; X) = Σ_{j≤4} c_j(e) X^j`, `e_i = e_i(x₁,x₂,x₃)`. Unverified desk check
(`scratchpad/lead02/k3_structure.py`, numeric `a, b`): 118 terms, total degree 4 in `e`, degree 4 in `X`.
Write `x_R = r₀ + r₁t + r₂t²`. Multiplication in `F_{q³}` is bilinear, so the `X^j` term is **homogeneous of degree
`j` in `r`**. Each descended equation `F_i(e; r)` therefore has bidegree `(≤4, ≤4)` in `(e, r)`.

**Decoupling (my derivation; I found it in no source).** `F_{q³} ⊗ F̄_q ≅ F̄_q³` through the three embeddings `σ_j`.
Over `F̄_q` the three Weil components are equivalent, by an invertible linear map in `r`, to

    g_j(e; ξ_j) = f̃₄^{σ_j}(e; ξ_j) = 0,   j = 1,2,3,   ξ_j = σ_j(x_R),

where `f̃₄^{σ_j}` is the summation polynomial of the conjugate curve `E^{σ_j}`. Equation `j` involves the target only
through `ξ_j`. The decomposition variety `V ⊂ A³_e × A³_ξ` is the fibre product over `e`-space of three quartic
correspondences. Both projections have degree 64, because `f̃₄` has degree 4 in `e` and in `X`.

**Counts (all at `k = 3`).**
- **Unknowns, equations.** 3 unknowns `(e₁,e₂,e₃)`, 3 equations of degree 4, Macaulay degree `3·3 + 1 = 10`
  (252 × 286 matrix, rank 222 in the repository's solver).
- **Solutions over `F̄_q`, after `S₃`-symmetrisation.** `4³ = 64` (Bézout; Diem's generic `2^{n(n−1)}`).
  Desk check on three independent conjugate curves with random `ξ`: quotient dimension 64. The repository measures
  64 at every residual (RESEARCH_RESIDUAL_WALKS §11.4).
- **Solutions before symmetrisation.** In `x₁,x₂,x₃` the count is 384 = `3!·4³`, in free `S₃`-orbits. The card's
  "`D ≤ 64` before symmetry" is the symmetrised count. The useful eliminant is the symmetrised one, in `e₁`, of
  degree 64. The `x₁`-eliminant has degree 384 and is strictly larger.
- **Degree of `h(e₁; ξ)` in `e₁`.** Exactly `D = 64`, the number of `e₁`-values over a target.
- **Degree of `h` in each `ξ_i`.** At most 64. Fix `(e₁, ξ_j, ξ_k)`; the other two equations are two quartics in
  `(e₂,e₃)` with 16 solutions (desk check: quotient dimension 16), and `g_i = 0` gives 4 values of `ξ_i` at each.
- **Total degree of `h`.** In `ξ`, at most 192 (multihomogeneous Bézout on a `ξ`-line: `3·4²·4`; BKK agrees). In
  `(e₁, ξ)`, at most 256 (BKK mixed volume `3!·128/3`). In each `r_i` separately, at most the `ξ`-total, so ≤ 192.
  These are upper bounds and not sharp. The `k = 2` analogue (`h = Res_{e₂}(g₁,g₂)`, `D = 4`) has per-variable
  degrees 4,4,4 but total degree 8 against a bound of 12 (`k2_eliminant.py`).
- **Term count.** Newton box `[0,64]⁴`: `65⁴ = 1.79·10⁷` terms over `F_{q³}` (Galois equivariance leaves `65⁴/3 ≈ 6·10⁶`
  independent). In `r`-coordinates over `F_q`: at most `65·C(195,3) = 7.9·10⁷`. The `k = 2` analogue fills 67 of
  125 box terms (54 %). The `k = 3` fill is **not measured**.
- **Specialisations needed to interpolate.** At least `65³ = 274,625` `F_q`-points, the `F_q`-dimension of the
  Galois-equivariant box. Each solve returns all 65 `e₁`-coefficients at once, because `h(·;ξ)` is the characteristic
  polynomial of `M_{e₁}`.
- **Per-target cost.** Evaluation: `fill · 65⁴/3 · (6–9)` `F_q` multiplications, i.e. `3.6–5.4·10⁷` at full fill.
  Root finding, `gcd(h, e₁^q − e₁)` at degree 64 and `q = 2²⁰`: about `20·2·64² ≈ 1.6·10⁵`. Recovering `e₂,e₃` from
  the one rational root costs `10³–10⁴`.

**`k = 4`.** `S₅`, degree 8, `D = 8⁴ = 4096` (`S₄`-form, measured 4096). By the same count `deg_{e₁} h = 4096`,
`deg_{ξ_i} h ≤ 8³·8 = 4096`, box `4097⁵ = 1.15·10¹⁸`, at least `4097⁴ = 2.8·10¹⁴` specialisations. For the `D₄`-form
(`D = 512`, five unknowns, four degree-4 equations plus `π² − σ₄`): `deg_{σ₁} h = 512`, `deg_{ξ_i} h ≤ 4·4·4·2·8 = 1024`
(my bound; the target enters at degree 8 in `Y_R`), box `513·1025⁴ = 5.7·10¹⁴`, at least `1025⁴ = 1.1·10¹²` specialisations.

**Degree-of-regularity bound.** Setup solves obey the Macaulay bound `Σ(deg f_i − 1) + 1` (Lazard; JV Assumption 2).
The eliminant has no solving degree after setup, only a support. The comprehensive object is worse. The `ξ_i`-degree
of the coefficients of a parametric grevlex basis is bounded only by the row-count minor bound `≈ 4·84 = 336` per
variable at Macaulay degree 10 (desk bound, not sharp). That exceeds the eliminant's 64, so the eliminant is the most
compact parametric object.

## 2. Prior art: literature

- Joux, Vitse, *A variant of the F4 algorithm*, CT-RSA 2011, LNCS 6558, pp. 356–375. URL
  `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CT-RSA2011.pdf`. **read-at-source.**
  Claim: the comprehensive Gröbner basis route is dismissed ("the computational cost of a comprehensive Gröbner basis
  is prohibitive", §1). What is amortised is the F4 *pair list* (the trace), not the basis. The Gaudry-type systems
  "fit into the framework of parametric families" (§4.1). One precomputation, over a small field, serves 8-, 16-, 25-
  and 32-bit `p`. Gains are 2.1–3.4× on 5 equations of degree 8 in 4 variables (overdetermined, 29 steps).
- Joux, Vitse, *Elliptic curve discrete logarithm problem over small degree extension fields. Application to the
  static Diffie–Hellman problem on E(F_{q⁵})*, J. Cryptology 26 (2013) (ePrint 2010/157). URL
  `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf`. **read-at-source.** Claims, from
  §2.1, §2.4, §3.3: the Gaudry–Diem system has generic degree `2^{n(n−1)}`, and FGLM (`O(nD³)`) dominates the lex
  basis, giving `Õ(n!·2^{3n(n−1)}q^{2−2/n})`. The summation polynomial is "computed once and evaluated in `x_R` for
  each relation search". For `n = 6` with `x₆` fixed, building the symmetrised polynomial took 10 min, so they never
  carry the target symbolically. A decomposition test costs about `3·10⁹` `F_p` multiplications at `n = 5`.
- Vitse, *F4 traces and index calculus on elliptic curves over extension fields*, ECC 2010 slides. URL
  `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/talks/ECC2010.pdf`. **read-at-source** (slides 24–26 only;
  the rest grepped, not read). Claim: "generic behaviour" is defined by *computing* the Gröbner basis in `K(Y)[X]`.
  It is a conceptual device, never executed. F4Remake follows the trace instead.
- Weispfenning, *Comprehensive Gröbner bases*, J. Symb. Comput. 14 (1992) 1–29. **recalled** (the reference entry
  `[30]` was read in JV).
- Montes, Wibmer, *Gröbner bases for polynomial systems with parameters*, J. Symb. Comput. 45(12) (2010) 1391–1425
  (Gröbner cover). **abstract-only** (search-result summary). Claim: the output is a partition of parameter space into
  locally closed pieces with polynomial data for the basis. It is a tool for discussing parameter strata, not a
  target-evaluation scheme.
- Faugère, Mou, *Sparse FGLM algorithms*, J. Symb. Comput. 80 (2017), arXiv:1304.1238,
  `https://arxiv.org/abs/1304.1238`. **abstract-only.** Claim: for shape position and sparse multiplication matrices
  (`N₁` nonzeros) the change of order costs `O(D(N₁ + n log D))`, and for generic systems about
  `D^{2+(n−1)/n}`. The abstract formula was garbled on the page; I take only the exponent.
- Traverso, *Gröbner trace algorithms* (ISSAC 1988), Gaudry (J. Symb. Comput. 44, 2008), Diem (Compositio 147,
  2011), Faugère–Gaudry–Huot–Renault (J. Cryptology 27, 2014). **recalled**, through JV's reference lists. Claims
  used: trace idea; `2^{n(n−1)}` generic degree; the 2-torsion `D_n` reduction.

## 3. Prior art: these repositories

- `research/gaudry_quartic_replay_20261013/README.md`: record-and-replay saves 7.0 % of `C₄` (14.5 % of
  elimination; 4,956 of 41,780 rows). `C ∝ D^{3.1}`. **Supports** the point that trace replay is small here.
  It does not bear on the eliminant.
- `research/pdp_root_finder_requirements_20261012/README.md`, S4 row, §6.4, C7/C8: S4 already demands that
  everything independent of `x_R` sits in `prepare`. The eliminant is the extreme end of S4. Ceilings: 4.3 % of rows
  (binary), 58 % of pair rows (`F_p`). **Supports** (frames what a gain must beat).
- `research/gaudry_quartic_d4_20261013/README.md`: `C₄(D₄) = 2.535–2.552·10¹⁰` (elimination `1.9·10¹⁰`, normal forms
  `6.2·10⁹`, charpoly `1.4·10⁸`), quotient 512, 19 of 20 targets close, one degenerate (517). **Supports** (baseline).
- `research/ic_next10_20261014/01-fp4-ladder/README.md`: `D₄` trials 24,830 at `p = 1033`; `D₄` S/rho-neg `1.36·10⁷`;
  trial exponent 0.217. Baseline for the amortisation count.
- `src/cryptanalysis/gaudry_cubic.rs` (header, lines 1–140): the file implements the meet-in-the-middle `S₃` oracle.
  Its header says the full three-unknown `S₄` solve is *not* in this file. The `S₄` solve and `C₃` live in the
  `RESEARCH_RESIDUAL_WALKS.md` ledger. **Unrelated to the eliminant.**
- `src/cryptanalysis/gaudry_quartic_d4.rs` (header, lines 1–70): the `D₄` invariants, 105 monomials, Bézout `4⁴·2 = 512`,
  target `Y_R` at degree ≤ 8. **Supports** (my `k = 4` degree counts).
- `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` (§11.4–11.8, §11.17): **`C₃ = 0.879·10⁶`**, split
  Macaulay `507k` (forward `153k`, normal forms `354k`), charpoly `273k`, eigenvectors `58k`, roots `33k`; `0.904·10⁶`
  at `p = 1039`; quotient 64 on every residual; about 5.1 residuals per base element; `C₄ = 1.213·10¹²`;
  `C ≈ D^{3.1}`; the `S₅` coefficients are already interpolated once per curve (495 monomials × 9 powers of `x_R`).
  It names "`F₄` with sparse FGLM" and the FGHR symmetries as the levers for `C_k`. **Supports** (baseline), and the
  prior mention of sparse FGLM is a lever on `C_k`, not a parametric eliminant.
- Grep hits only: `leads/LEAD-12.md` row 02 and `leads/LEAD-14.md` open question 2 refer to this lead.
  `LEAD-06.md` quotes the same `C₃`. No file in the repositories computes an eliminant in the target coordinates, so
  the idea is not previously closed. The negative below is new.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not lower the degree of regularity. It would move the elimination and the normal forms, 58 % of `C₃` at
`k = 3` and 99 % of `C₄(D₄)` at `k = 4`, into a one-off setup. The characteristic polynomial is `h` itself (the
repository's own `273k` step at `k = 3`), so it moves too.
- **`k = 3`.** Remaining per target would be root finding plus shape recovery: about `1.0·10⁵` at `p = 271`
  (roots `33k`, eigenvectors `58k`), about `1.7·10⁵` at `q = 2²⁰` (`20·2·64²` plus shape recovery). That is 5–9× below
  `C₃`.
- **`k = 4`, `D₄`.** Remaining: roots of a degree-512 polynomial, about `8·10⁶` at `p ≈ 2¹⁵`, rising with `log q`
  (`2·512²·log₂ p`), against `2.54·10¹⁰`, about 10³×.
- **Assumption this rests on:** evaluating `h(·; ξ)` per target costs about as little as a root-finding. Section 5
  shows it does not.
- **Setup amortisation at `k = 3`.** Setup is about `2.7·10⁵` solves at the charpoly-only cost `7.8·10⁵`
  (Macaulay plus charpoly, no eigenvectors), i.e. `2.1·10¹¹` `F_q` mults. The plain relation phase at `q = 2²⁰` needs
  about `5.3|F| = 2.8·10⁶` targets at `C₃`, i.e. `2.4·10¹²`. Setup is 9 % of that, so the setup does amortise over
  the relations one curve needs (`|F| ≈ q/2`). That is not the obstruction.

## 5. Obstructions

1. **Evaluation costs more than the solve it replaces.** `h` has up to `D^{k+1}` terms (box `(D+1)^{k+1}`) because it
   has `k` target coordinates, each at degree `D`. The Macaulay solve costs `≈ D^{3.1}`. The ratio of dense
   `D^{k+1}` to `D^{3.1}` is 0.9 at `k = 2` (`D = 4`), **42 at `k = 3`**, `7·10⁶` at `k = 4` (`D = 4096`).
   This is the number of target parameters equalling the number of unknowns, and it only breaks even at `k = 2`,
   where there is no index-calculus advantage. At `k = 3`, evaluation at fill `f` is `f·(3.6–5.4)·10⁷` `F_q` mults,
   i.e. `(41–61)·f` times `C₃`. Break-even fill is `1.6–2.5 %`; the kill criterion (≥ 10× below `C₃`) needs fill
   `≤ 0.16–0.25 %`. At `k = 2` the measured fill is 54 %. At `k = 4`, `D₄`-form (box `5.7·10¹⁴`, Galois/4, 16 `F_p`
   mults per `F_{p⁴}` mult) evaluation is `f·2.3·10¹⁵`, i.e. `f·9·10⁴` times `C₄(D₄)`: break-even fill `1.1·10⁻⁵`,
   10× kill `1.1·10⁻⁶`. At `k = 4`, `S₄`-form it is `f·4.6·10¹⁸`, `f·3.8·10⁶` times `C₄`. All fill-dependent numbers
   are unverified desk checks.
2. **Setup scales as `D^k` solves.** `k = 3`: `2.7·10⁵` solves, amortisable. `k = 4`, `S₄`-form: `2.8·10¹⁴` solves
   `× 1.2·10¹² = 3.4·10²⁶` `F_p` mults, amortisable only above `2.8·10¹⁴` targets. `k = 4`, `D₄`-form:
   `1.1·10¹²` solves `× 2.5·10¹⁰ = 2.8·10²²`; extrapolating the ladder's trial exponent 0.217, `1.1·10¹²` trials need
   `p ≈ 2³⁹` (`#E ≈ 2¹⁵⁶`), past the extrapolated `D₄` handover `2^{126–137}`. Extrapolation, not measurement.
3. **A cheaper evaluation is a determinantal form, i.e. the baseline.** `h = det(e₁I − M(ξ))` with `M` the 64 × 64
   multiplication matrix. Its entries are rational functions of `ξ` of the same degree as the Macaulay minors. A
   shorter straight-line program for `h` would have to beat the Macaulay-matrix elimination, which is the thing the
   repository optimised (0.18× of its first version).
4. **Exponent unchanged.** Per-trial cost drops at most by a constant. Trials are `~k!·|F|` (or `|F|^{4/3}` with
   double large primes); linear algebra is unaffected. Tier: constant. The oracle is algebraic evaluation plus
   `F_q` root finding on a structured set. It is not a group-operation/membership oracle, so Corrigan-Gibbs–Kogan does
   not apply directly. The structured-GGM constraint does not bind a constant either. Nothing here can exceed
   Gaudry's exponent.
5. **Trace replay, the thing JV amortise, measured small.** 7 % at `k = 4` (repository). JV's 2.1–3.4× is on
   overdetermined systems (5 equations, 4 unknowns) with many zero reductions. Neither is the eliminant.
6. **Sparse FGLM (Faugère–Mou) is the wrong step.** It improves the characteristic-polynomial step. At `D = 64`,
   `n = 3` the abstract exponent gives about `5·10⁴` against the measured `2.7·10⁵` (31 % of `C₃`), so at most
   about 1.3× on `C₃`. At `D = 512` the charpoly is 0.6 % of `C₄(D₄)`. Not this lead.

## 6. Minimal discriminating test (frozen protocol)

**Question.** What is the real fill of the `k = 3` eliminant on a `ξ`-line, and what does that imply for per-target
evaluation against `C₃`?

**Instances.** Gaudry `k = 3`, `p ∈ {1039, 2083}` (`p ≡ 1 mod 3`; `C₃ = 0.904·10⁶` at 1039), curve seeds 1–2 each,
random `F_p`-lines `r = β + s·v` (two lines per curve, seeds fixed in the runbook). `D = 64`, `m = 3`.

**Instrument.** A new Rust example beside `gaudry_cubic.rs` that calls the existing `S₄` Macaulay solve and its
characteristic polynomial of `M_{e₁}`. At each `s` it records the monic characteristic polynomial. A Rust
univariate rational reconstruction (Thiele or Padé) then recovers each coefficient as `h_k(s)/h₆₄(s)`. No Python.

**Procedure.**
1. 600 values of `s` per line; discard residuals whose quotient dimension is not 64 (rate about `1/p`) and resample.
2. Reconstruct from the first 400 valid points; check on 100 held-out points; a mismatch voids the line.
3. Clear denominators to the polynomial `h(e₁, s) = Σ h_k(s) e₁^k`.

**Recorded.** `deg_{e₁}`, `deg_s`, total degree, nonzero terms, fill against `65·(deg_s + 1)`; counted `F_p` mults of
the solves; the held-out check; per-line extrapolated full-box term count `fill · 65⁴` and implied per-target
evaluation `· (6–9)/3` `F_q` mults.

**Prediction.** `deg_{e₁} = 64`; `deg_s` between 100 and 192; slice fill ≥ 20 %; extrapolated evaluation ≥ `4·10⁶`
`F_q` mults, i.e. ≥ 4× `C₃`.

**Kill criterion (the card's, applied).** Killed if the extrapolated per-target evaluation plus `1.7·10⁵` root
finding is not ≤ `C₃/10 ≈ 9·10⁴`. That needs full-box support under about `4.5·10⁴` terms (fill ≤ 0.25 %). I expect it
to be killed. Survives (then go to the full 3-variable eliminant) only if slice fill is under 0.25 % on both curves
at both `p`.

**Cost.** 2–3 person-days of Rust, about 1,200 solves × `10⁶` mults per line set, under 1 CPU-hour, under 1 GB.
The mechanical tier can run it from a runbook (fixed seeds, fixed `s`-grids, JSONL records, pass/fail by the rule above).

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|:--|:--|:--|:--|:--|
| none (regression 4–60× at `k = 3`; ceiling 5–9× if evaluation were free) | medium | 2–3 person-days, under 1 CPU-hour | constant (exponent untouched) | the `k = 3` fill measurement; repository `C₃` (0.88–0.90·10⁶) and `C₄(D₄)` (2.54·10¹⁰) baselines |

Distance to rho is unchanged by this lead. The `k = 4` `D₄` ladder row sits at `1.4·10⁷×` rho-negation at `p = 1033`,
and `k = 3` plain at 528–4,000× rho. A free-evaluation ceiling would move the `k = 4` relation-dominated row by at most
about 10³× on paper. It is not reachable by this route.

## 8. Open questions

- Whether the `k = 3` fill is as dense as `k = 2` (54 %) or sparser. Measurable only by the §6 test; I did not run
  the repository solver (run no experiment). A slice computation by norm over the 16-point fibre needs rational
  reconstruction in `e₁` and was out of this budget.
- The exact `ξ`-degrees and total degree at `k = 3` (BKK gave 192 and 256, not sharp at `k = 2`).
- The `D₄` eliminant degrees (`≤ 1024` per `ξ_i` is my Bézout bound, with the target at degree 8; not derived
  exactly).
- Whether any structured target family (for instance targets on an `F_q`-line, reachable by search) makes
  evaluation cheap. Finding such a target costs `q²` trials per relation, so I rate it closed for the exponent; not
  measured.
- Weispfenning/Montes–Wibmer full texts were not read. JV's one-sentence dismissal is the evidence for the
  comprehensive route.

## 9. Sources

URLs fetched:
- `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CT-RSA2011.pdf`: WebFetch returned HTTP 200 with
  the raw 254 KB PDF (the summariser could not decode it). Text extracted with `pdftotext` from the saved copy.
- `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/DLFq5.pdf`: the same, 456 KB; read in full.
- `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/talks/ECC2010.pdf`: `curl` 200, 437,789 bytes, text extracted;
  slides 24–26 read.
- `https://www-fourier.univ-grenoble-alpes.fr/~viva/research/talks/CRYP-402-Vitse.pdf`: `curl` 200, 295,198 bytes,
  downloaded, **unread**.
- `https://arxiv.org/abs/1304.1238`: WebFetch OK (abstract page).
- Two WebSearch queries (Montes–Wibmer; parametric Gröbner and index calculus): snippets only; no relevant prior art
  on a parametric eliminant for Gaudry–Diem found. This is a search-scope statement, not proof that none exists.

Repository paths read: `research/pdp_literature_leads_20261022/BRIEF_SONNET.md`, `LEADS.md` (standing constraints and
LEAD-02); `research/gaudry_quartic_replay_20261013/README.md`;
`research/pdp_root_finder_requirements_20261012/README.md`; `research/gaudry_quartic_d4_20261013/README.md`;
`research/ic_next10_20261014/01-fp4-ladder/README.md`; `src/cryptanalysis/gaudry_cubic.rs` (lines 1–140);
`src/cryptanalysis/gaudry_quartic_d4.rs` (lines 1–70);
`research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` (lines 1230–1600, 2365–2475). Grep lines only:
`leads/LEAD-12.md`, `leads/LEAD-14.md`, `leads/LEAD-06.md`.

Desk checks (unverified, not measurements, not committed, in the scratchpad):
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead02/k2_eliminant.py` (`k = 2` eliminant:
degrees 4,4,4, total 8, 67 of 125 terms) and
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead02/k3_structure.py` (symmetrised `S₄`:
118 terms, degree 4/4; quotient dimension 64; fixed-`e₁` fibre 16). Cost arithmetic in §1, §4, §5 is hand/`python3`
arithmetic on those inputs.
