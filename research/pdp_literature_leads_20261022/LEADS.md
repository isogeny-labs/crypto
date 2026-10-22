# Lead cards: point decomposition without (or beyond) summation polynomials

**Date.** 2026-10-22. **Status of every card.** *Proposed.* Nothing here is
derived, verified or measured until the dossier in `leads/LEAD-NN.md` says
otherwise, and even then a dossier is a desk assessment, not a run record.
**Question.** Which reformulations of the point decomposition problem (PDP)
could lower the Gröbner-basis cost or the degree of regularity of the relation
search in elliptic-curve index calculus, measured against the current baseline
(`docs/ic/LEADERBOARD.md`, every row above matched rho)?

Each card names: the mechanism; why it could move the Gröbner bottleneck or
the degree of regularity; the prior art (literature and this repository);
what nearby ground is already closed; the minimal discriminating test with a
kill criterion; and the repository hooks. The cards are briefs for the
director-tier (Sonnet 5.5) dossiers and the mechanical-tier (Haiku 5.5)
verifications; they are not conclusions.

## Standing constraints every card must respect

- **Generic bounds.** Corrigan-Gibbs–Kogan and Shoup bound every oracle built
  from group operations and membership tests (pair tables, enumeration, root
  finding over the factor base). The structured generic-group model
  (Corrigan-Gibbs–Henzinger–Wu, ePrint 2026/384) says an index calculus must
  exploit structure on at least a `q^{-1/2}` fraction of points to beat rho;
  Dinur–Keller–Marmor (ePrint 2025/783) closes non-adaptive table-lookup
  hybrids below `√N` advice. A lead that is "enumeration in disguise" is a
  constant-factor lead and must say so.
- **Closed by theorem or exact computation** (do not re-open): BKK/mixed
  volume (`MV = Bézout`), Semaev-cover monodromy `(Z/2)^{m-2}`, yield
  conservation under factor-base geometry (KN-FIND-007), isogeny transport
  (constant at most), xedni/lifting, GGMP Frobenius-stable bases at `n = 131`,
  GHS/JV covers for ECC2K-130, single-polynomial Coppersmith for `S₅`
  (reach `1/80`), shared-target Coppersmith (1.12×, below trigger, 2026-10-14),
  the Kummer-exponent Fourier spectrum (flat, 2026-10-14), quasi-subfield
  bases at `n = 31` (no yield gain), solution counts of algebraic factor bases
  (0.93–1.03× random), record-and-replay F4 (2–7 %).
- **Measured in the repository and relevant to several cards**: the PKM tower
  plateau at degree 7 broke (D ≥ 8 at N = 20, msolve, 2026-10-14); msolve F4
  reads solving degree 4 (some draws 3) through `(ℓ, m) = (11, 4)` and at
  `(15, 5)` for `m = 3`, where XL/Macaulay refutation reads 5–7; the genus-3
  transfer after cubic GHS runs at `q^{1.30}` with double large primes against
  rho's `q^{1.5}` (the one live exponent regime); root finding beats F4 by
  `10²`–`10⁵` at stage level but moves end-to-end `S` by ≤ 1.26×; Nagao's
  Riemann–Roch encoding at prime `n` is "`S′₄` with the linear part hidden".
- **Success criterion.** A verified, measured improvement over the current
  index-calculus baseline is a success; the gap to rho is reported as distance,
  never as a gate (CLAUDE.md, AGENTS.md).

## The cards

### LEAD-01 — MinRank / Kipnis–Shamir / support-minors modelling of the summation chain

**Class.** Formulation (both fields). **Tier.** Exponent-relevant through the
solving degree; falls back to constant.

**Mechanism.** `S_{m+1}` is a resultant: `S_{m+1}(x₁..x_m, x_R) =
Res_X(S_m(x₁..x_{m-1}, X), S₃(X, x_m, x_R))`. Vanishing of a resultant is a
rank drop of a Sylvester (or Bézout) matrix whose entries are low-degree
polynomials in the summand coordinates. Model the rank drop with a kernel
vector (Kipnis–Shamir) or with the maximal minors of the kernel's Plücker
coordinates (support minors, Bardet et al., Asiacrypt 2020): the system
becomes bilinear or multi-homogeneous in (coordinate block, kernel block).
For bilinear affine systems the grevlex degree of regularity is bounded by
`min(n_x, n_y) + 1` (Faugère–Safey El Din–Spaenlehauer 2011) and the
support-minors Hilbert series is known in closed form (Gilard 2025). Compare
this with the measured chain degrees (4–7).

**Prior art.** Repository: `research/factor_base_designs_20261015/README.md`
lists "K4 MinRank / big-field low-rank modelling: reopened, no checkable
statement disposes of it" (E-K4); `research/xl_bilinear_core_20261006/` ran
bidegree XL on the bilinear `S₃` core and found no gain at `n = 131`;
`research/linearization_reach_20260930/` proved the `b(b+1)/2` symmetry
defect of that core. Literature: Kipnis–Shamir 1999; Bardet–Bros–Cabarcas–
Gaborit–Perlner–Smith-Tone–Tillich–Verbel 2020 (support minors);
Bardet–Bertin 2022 and Guo–Ding 2022 (KS ≡ SM ideals); Gilard, arXiv
2502.12721 (SM Hilbert series); Faugère–Safey El Din–Spaenlehauer, arXiv
1001.4004.

**Discriminating test.** For `m = 3`, `ℓ ∈ {4, 5}`, `n ∈ {17, 19, 23}` over
`F_{2^n}` (planted instances as in `linearization_reach`), write the KS and SM
modellings of the `S₃ ∘ S₃` chain with `x_i ∈ V`, Weil-descend, and record
the maximal F4 step degree (msolve or the repository's `inherited_f4`)
against the direct chain and against `S₄`. **Kill** if neither modelling reads
a lower maximal degree than the chain at `ℓ = 5`, or if the variable count
makes the Macaulay matrix larger at equal degree.

**Hooks.** `src/cryptanalysis/binary_semaev.rs`, `semaev_higher.rs`,
`inherited_f4.rs`, `groebner_compare/` (msolve bridge),
`research/notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md`.

### LEAD-02 — Parametric eliminant: generic-fibre Gröbner basis in the target's coordinates, specialised per target

**Class.** Solver / amortisation (extension fields `k = 3, 4`, then binary).
**Tier.** Constant, but potentially large (per-target cost becomes a
univariate root-finding).

**Mechanism.** Every PDP instance for a fixed `(E, V, m)` is the same ideal
with the target coordinates `r = (r₀, …, r_{k-1})` of `x_R` as parameters.
Compute once, over `F_q(r)` or by evaluation–interpolation in `r`, the lex
Gröbner basis in shape position (shape lemma: `h(x₁; r) = 0`,
`x_i = g_i(x₁; r)`), i.e. the eliminant of the decomposition variety. Per
target: evaluate the coefficient polynomials at `r`, then find the `F_q`-roots
of a univariate of degree `D = #V(I)` over `F̄_q` (gcd with `x^q − x`). The
Gröbner computation moves entirely into setup. Degree bookkeeping for Gaudry
`k = 3`: `S₄` descends to 3 equations of degree 4 in 3 unknowns, `D ≤ 64`
before symmetry; `k = 4`: `S₅`, `D ≤ 4096`, `/48` under `D₄`.

**Prior art.** Joux–Vitse F4Remake/F4 traces (CT-RSA 2011) replays the row
operations of one instance on the others; the repository measured
record-and-replay at 2–7 % (`research/gaudry_quartic_replay_20261013`) and
charges `x_R`-independent work once in `prepare`
(`research/pdp_root_finder_requirements_20261012/README.md`, row S4). The
parametric eliminant goes further: no per-target elimination at all.
Comprehensive Gröbner systems (Weispfenning; Montes), sparse FGLM
(Faugère–Mou), Gröbner bases over function fields by
evaluation–interpolation. The repository's rf-mitm and pairs oracles are
root-finding over the factor base, not elimination in `r`.

**Discriminating test.** Gaudry `k = 3` at `q ≈ 2^{20}`: compute the
eliminant by interpolation in `(r₀, r₁, r₂)` from Gröbner bases at enough
specialisations (count them), record its degree in `x₁` and in each `r_i`,
its term count, and the per-target cost (evaluation + root finding) in field
multiplications. Compare with `C₃` of the existing `gaudry_cubic.rs` solver
and with `prepare` + per-target cost of the root finder. **Kill** if the
eliminant's size in `r` exceeds what one curve's relation budget amortises
(state the budget: number of relations ≈ `|F|`), or if per-target cost is not
below the measured `C₃` by ≥ 10×. Then state the `k = 4` prediction.

**Hooks.** `src/cryptanalysis/gaudry_cubic.rs`, `gaudry_quartic_d4.rs`,
`f4_fp.rs`, `ic_framework/rootfind.rs`, `research/gaudry_quartic_d4_20261013/`,
`research/ic_next10_20261014/01-fp4-ladder/`.

### LEAD-03 — ℓ-torsion symmetrisation beyond 2-torsion, and Kummer-model coordinates

**Class.** Formulation (extension fields, prime fields with rational
torsion). **Tier.** Constant, measured 30–416× for 2-torsion elsewhere in the
repository; unknown for `ℓ = 3, 4`.

**Mechanism.** Faugère–Huot–Joux–Renault–Vitse (Eurocrypt 2014) use a
rational 2-torsion point `T` so that the summation polynomial is invariant
under `(Z/2)^{m-1} ⋊ S_m` and its expression in invariants is sparser and of
lower degree. With rational 3-torsion (Hessian form) or 4-torsion (Jacobi
quartic / Edwards with a 4-torsion point) the group is `(Z/ℓ)^{m-1} ⋊ S_m`, of
order `ℓ^{m-1} m!`, so the degree of the invariant form drops by a larger
factor if the invariant ring is manageable. Kummer models (Montgomery, theta;
Barbulescu–Robert–Sarkis, "Kummer models") give `x`-only biquadratic addition
laws with symmetric/anti-symmetric forms when a rational 2-torsion point
exists; the summation relation in theta coordinates may have smaller support.

**Prior art.** FHJRV 2014 (2-torsion only, up to `S′₈`);
`research/notes/index-calculus/RESEARCH_SYMMETRIZED_SEMAEV.md`,
`RESEARCH_EXOTIC_COORDINATES.md` (1,999 lines; read its conclusions before
proposing), `RESEARCH_K5_TORSION_JOUX_VITSE.md` (Edwards 2-torsion cut 416× on
the four-point `C″`), `src/cryptanalysis/symmetrized_semaev.rs`, `fghr_line.rs`,
`fghr_full.rs`. The autoresearcher's EcGFp5 arm failed to realise the
quotient (`EXP-GFPN-05ff43`).

**Discriminating test.** Compute `S′₄` and `S′₅` symmetrised under
`(Z/3)^{m-1} ⋊ S_m` on a Hessian curve with rational 3-torsion over `F_{p}`,
`p ≈ 2^{20}`, and under `(Z/4)^{m-1} ⋊ S_m` on a Jacobi quartic; record
degree in each invariant, monomial count and F4 time per decomposition at
`k = 3` against the FGHR 2-torsion arm on the same field size. **Kill** if the
invariant-form degree or the F4 time is not below FGHR's by the group-order
ratio to within 2×, or if the invariant ring needs secondary invariants that
add more variables than the quotient removes.

**Hooks.** `symmetrized_semaev.rs`, `fghr_full.rs`, `gaudry_cubic.rs`,
`research/gaudry_quartic_d4_20261013/README.md` (D₄ measured 48× against a
`D^{3.1}` rule that overstated by 13×).

### LEAD-04 — Homomorphic class pre-partition of the factor base (Tate pairing, ℓ-descent, trace parity)

**Class.** Filter (every oracle, pair tables included). **Tier.** Constant,
up to `ℓ` per rational `ℓ`-torsion point, up to 4 with full 2-torsion.

**Mechanism.** Any group homomorphism `χ: E(K) → A` with `A` small and `χ`
cheap partitions `F` into classes and forces `χ(Q) = Σ χ(P_i)` on every
relation. Candidates: the Tate pairing `t_ℓ(T, ·): E(K)/ℓE(K) → μ_ℓ` with
`T ∈ E[ℓ](K)` (one Miller loop of length `log ℓ`, so `O(1)` group operations
for small `ℓ`); 2-descent `δ: E(K)/2E(K) → (K^*/K^{*2})^{2}` via `x − e_i`
when `E[2] ⊂ E(K)`; in characteristic 2 the Kosters–Yeo morphism
`Tr(x) ∈ F₂` (the "trace parity" the repository already found confines
yields). For a pair-table oracle, sort the table by class and look up only
the class `χ(Q) − χ(pair)`: candidate tuples drop by `|A|`. For an algebraic
oracle, the class is one extra low-degree equation or a restriction of `V`
by class; measure whether it lowers the F4 degree or only the solution count.

**Prior art.** KN-FIND-14706d (autoresearcher): the *branch-discriminant
quadratic-character filter* never rejects, because it collapses to on-curve
membership — a Tate-pairing or descent class is a genuine homomorphism to a
finite group, not a membership test, so this finding does not cover it; the
dossier must say precisely why. EV-SEMBIN-0c8bf4 / H-SEMBIN-83a856: trace
parity confinement explains the low-degree yield deficit (binary). Galbraith–
Granger–Merz–Petit 2020 use Frobenius for symmetry breaking, not classes.
`docs/ic/boundary_targets.json` notes bases with trace zero lie in `2E`.

**Discriminating test.** On the leaderboard's own rows (Table B prime
`fp24`, random binary `f2m27`, Koblitz `f2m47` from `docs/ic/LEADERBOARD.md`),
pick a curve with rational `ℓ`-torsion (`ℓ ∈ {2, 3}`), sort the existing pair
table by Tate-pairing class, and count candidate tuples examined per relation
against the unsorted table at equal relation count. Predicted ratio `|A|`.
**Kill** if the measured ratio is below `0.8 |A|`, or if the class
computation per factor-base point costs more than `|A|` group operations
amortised.

**Hooks.** `src/cryptanalysis/orbit_pair_table.rs`, `native_signed_mitm.rs`,
`ic_framework/rootfind.rs`, `koblitz_index_calculus.rs` (folded pair table),
`research/ic_tool_program/rule/v3/README.md`.

### LEAD-05 — Degree-of-regularity growth law for descended S₃ and S₄: Solovyev 2026, Kosters–Yeo, Huang–Kosters–Yeo

**Class.** Measurement / theory import (binary, prime degree). **Tier.**
Exponent-deciding for the `m = 3, 4` routes (the "106-bit spread").

**Mechanism.** A 2026 preprint (A. Solovyev, GitHub release
`alekssolov/weil-descent-s3-dreg` v1.0.0, "The first fall degree assumption and
the degree of regularity of Weil-descent systems for summation polynomials
over `F_{2^n}`") reports exact degree-capped closures: for `S₃`, degree of
regularity 3 for `n ≤ 16`, 4 for `17 ≤ n ≤ 36`, 5 for `37 ≤ n ≤ 52`, then
linear growth with slope about `1/20` in range and `0.036` asymptotically;
closed-form closure dimensions (`dim V₃ = (3N − 2)n − C(N, 2)`, formulas for
`dim V₄` and the degree-5 part); exactly `3n` quadrics in the ideal; for `S₄`
the degree-7 closure contains the whole truncated ideal (solving degree =
first fall degree = 7, no generic regime); and an M4GB Gebauer–Möller fix
(upstream PR #14). Kosters–Yeo (arXiv 1503.08001) measured `D_reg` 3, 4, 4,
≥ 5 at `n = 16, 20, 30, 40`; Huang–Kosters–Yeo (CRYPTO 2015) doubt the
first-fall-degree assumption; Kousidis–Wiemers (JMC 2019) give first fall
`m² − m + 1`. The closure dimensions are Hilbert-function values of the
truncated ideal, so they are a zero-compute predictor the repository can
check against its own `dreg_closure_*` data.

**Prior art (repository).** `research/dreg_closure_scale_20261016`,
`dreg_closure_driver_20261016`, `dreg_closure_mutantxl_20261016`,
`dreg_ell_grid_20260925`, `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`,
`RESEARCH_DREG_MEASUREMENT.md`; autoresearcher `research/dreg-linear-law/`,
RQ-DREG-*, EV-SEMBIN-07960c (first-fall median 2). The 2026-10-14 campaign
read msolve solving degree 4 at `(11, 4)` and `(15, 5)` where XL read ≥ 7.
Reconcile the three instruments (XL refutation, MutantXL closure, F4).

**Discriminating test.** Replicate the `S₃` boundaries `n = 16/17` and
`36/37` (random `V` of the preprint's dimension) with the repository's
closure driver and with msolve; check the closure-dimension formula at three
`(n, N)` points; then price Semaev-2015 chains and the `m = 3, 4` routes at
`n = 131` under the fitted slope. **Kill** (for the import) if two of three
closure dimensions disagree with the formula, or the boundaries move by more
than one in `n`. Record the preprint's provenance status honestly (GitHub
release, no venue).

**Hooks.** `research/dreg_closure_driver_20261016/`, `examples/dreg_*`,
`src/cryptanalysis/ffd_harness.rs`, `pc_degree_harness.rs`.

### LEAD-06 — Trace-zero variety representation (Gorla–Massierer) as the coordinate system for E(F_{q^n}), n = 3, 5

**Class.** Representation / factor base (extension fields). **Tier.**
Constant (same `q^{2−2/n}` family as Gaudry) unless the equation degree drops.

**Mechanism.** For `E/F_q` and `n = 3, 5`, the trace-zero subgroup `T_n` has
dimension `n − 1` and an optimal representation of `(n − 1) log q` bits
(Gorla–Massierer, DCC 2015) derived from Semaev's polynomials; "Index
calculus in the trace zero variety" (AMC 2015, ePrint 2014/318) applies
Gaudry's abelian-variety index calculus to `T_n`. When `gcd(#E(F_q), #T_n) = 1`
the prime-order subgroup of `E(F_{q^n})` lies in `T_n`, so this is an
alternative coordinate system for the same ECDLP. The question is whether the
decomposition system in `T_n` coordinates has fewer variables or lower degree
than Gaudry's `S_{n+1}` with `x ∈ F_q`, and what the relation yield is.

**Prior art.** Gorla–Massierer 2014/2015; Massierer's thesis (Basel 2014);
repository mentions only in `RESEARCH_ISOGENY_FIBERS_ECDLP.md:1819` and
`RESEARCH_IC_BOUNDARY_LEDGER.md:442`; Gaudry `k = 3` pipeline
`src/cryptanalysis/gaudry_cubic.rs`, `glv_gaudry.rs`.

**Discriminating test.** At `q ≈ 2^{20}`, `n = 3`: write the `T_3` relation
system in the optimal representation, count variables, equation degrees and
`F̄_q` solutions, and time one decomposition in msolve against the `S₄`,
`x ∈ F_q` system; record relation yield per target for both. **Kill** if
neither the degree nor the per-target time is below Gaudry's by ≥ 2× at equal
`|F|`, or if the `T_n` factor base is not a subgroup-compatible object.

**Hooks.** `gaudry_cubic.rs`, `ic_framework/plugins.rs` (factor-base
plug-ins), `research/ic_next10_20261014/01-fp4-ladder/`.

### LEAD-07 — Crossbred (Vidal–Ionica–Delaplace 2024; 2026 preprocessing) on the Weil-descent degree cells

**Class.** Solver (binary, `m = 3, 4`). **Tier.** Memory-wall lever for the
exponent-deciding cells; constant otherwise.

**Mechanism.** Crossbred (Joux–Vitse 2017; theory in ePrint 2024/992;
improved preprocessing ePrint 2026/360) fixes `k` variables after a
degree-`D` Macaulay preprocessing restricted to monomials of degree ≤ `d` in
the remaining variables, then enumerates the fixed variables with a tiny
linear-algebra step each. For over-determined Boolean systems it is the state
of the art. The deciding cell `(ℓ, m) = (4, 6)` needs about 70 GB dense at
degree 5 (`dreg_closure_driver_20261016/RESULTS.md`); Crossbred trades that
memory for enumeration over `k` bits of the subspace coordinates.

**Prior art.** Repository tried Crossbred only on the `m = 2` toy oracle at
`n = 7, 9` (`research/crossbred_20260920/e2e_panel.txt`, `src/cryptanalysis/crossbred.rs`);
`research/ic_leads_review_20261018/README.md` §2.5 names the gap. SAT/WDSat
is measured as an enumerator (`RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md`).

**Discriminating test.** Cells `(ℓ, m) = (4, 4)`, `(5, 3)` and `(4, 6)` at
`n ∈ {11, 13, 17}` and the largest `n` the host allows: record Crossbred's
`(D, d, k)` that solves, peak memory and operation count, against msolve F4
and the MutantXL closure. **Kill** if at `(4, 6)` Crossbred needs more than
`2^{k}` with `k > ℓ` (then it is enumeration over the whole summand) or more
memory than msolve at the same `n`.

**Hooks.** `src/cryptanalysis/crossbred.rs`, `pq_xl.rs`, `sparse_macaulay.rs`
(zero tests), `research/dreg_closure_driver_20261016/`.

### LEAD-08 — Learned orderings and branching heuristics for F4 and WDSat on Semaev and PKM systems

**Class.** Solver (ML; home repository `ml-cryptanalysis`). **Tier.**
Constant; null hypothesis stated.

**Mechanism.** Kera–Ishihara–Kambe–Vaccon–Yokoyama (NeurIPS 2024, arXiv
2311.12904) train Transformers to output Gröbner bases of 0-dimensional
radical ideals; HATSolver (Malhou–Perret–Lauter, arXiv 2512.14722 / ePrint
2025/2219) learns orderings with hierarchical attention. The cheap, testable
version: learn a variable order (for F4's Macaulay column order and for
WDSat's branching) from instance features, and measure solver work. The
degree of regularity in a graded order is order-independent generically, but
the Weil-descended Semaev systems are structured (trace parity, the
`b(b+1)/2` defect), so an order that exposes the structure earlier can change
the maximal step degree reached in practice.

**Prior art.** `reports/Index calculus leads next steps.md` lists learned
orderings as an open literature gap, constant-factor; ml-cryptanalysis
`csd/` (hardness prediction nulls, `experiments/closure-degree-predictor/`
null); `RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` (WDSat visits `2^{3ℓ}/3!` nodes
to within 0.2 %). The null: WDSat is an enumerator, so ordering cannot change
its node count by more than a constant.

**Discriminating test.** `m = 3`, `ℓ = 4`, `n ∈ {13, 17, 19}`, 200 planted
instances: train on 150, test on 50; compare F4 maximal step degree and
Macaulay rows, and WDSat conflicts, under default, random and learned orders;
permutation null over labels. **Kill** if the learned order is within the
random-order band on both solvers.

**Hooks.** ml-cryptanalysis `csd/semaev.py`, `csd/fpn_degrees.py`; crypto
`wdsat_oracle.rs`, `inherited_f4.rs`, `semaev_corpus.rs` (60-instance corpus).

### LEAD-09 — Ore / linearized-polynomial quotient-algebra elimination for the subspace constraint

**Class.** Solver (binary, `m = 2` core and the pairs-and-solve residual).
**Tier.** Constant (sits on the enumeration line by the generic bounds) but
removes Weil descent and the degree of regularity entirely for the residual.

**Mechanism.** `x ∈ V` is exactly `L_V(x) = 0` with `L_V` the subspace
polynomial, a `2`-linearized (Ore/skew) polynomial of degree `2^ℓ`. Instead
of descending `S₃(x₁, x₂, x_R)` to `n` Boolean equations, eliminate in
`F_{2^n}[x₁, x₂]/(L_V(x₁), L_V(x₂))`: `Res_{x₂}(S₃, L_V(x₂))` then
`gcd(·, L_V(x₁))`. Linearized polynomials admit fast composition, factoring
and gcd (Caruso–Le Borgne 2017; Puchinger–Wachter-Zeh 2017; the linearized
Euclidean algorithm), so the cost is `Õ(2^ℓ)` per target with a small
constant and no degree growth. Huang–Kosters–Petit–Yeo–Yun's quasi-subfield
substitution `x^{2^{n′}} → λ(x)` is the sparse special case of this
substitution; the generic `L_V` is dense but has the Ore structure.

**Prior art.** HKPYY (JMC 2020; `KN-LIT-0a321c`, read at source); repository
`semaev_decomp.rs` (linearised root finding), `ic_framework/rootfind.rs`
(rf-mitm, rf-pairs), `research/pdp_root_finder_20261012/` (rf-mitm
3,007–115,160× the baseline oracle at stage level), KN-OPEN-007 (low-rank
Semaev resultant tree). The dossier must position this against rf-mitm, the
best measured oracle, and against the CGK line.

**Discriminating test.** `n ∈ {31, 41, 47}`, `ℓ = n/3` rounded, `m = 2`
residual of the pairs oracle: count field operations per target for the
Ore elimination against rf-mitm on the same instances. **Kill** if not below
rf-mitm by ≥ 3× at `n = 47`.

**Hooks.** `semaev_decomp.rs`, `ic_framework/rootfind.rs`, `quasi_subfield.rs`,
`binary_semaev.rs`.

### LEAD-10 — Chord-parametrised (slope-variable) small-root formulation for prime-field PDP

**Class.** Formulation (prime fields, `m = 2, 3`). **Tier.** Exponent-relevant
only if the lattice reach beats the W10 family; otherwise closed.

**Mechanism.** Over `F_p` with the small-`x` base `x(P_i) < B`, parametrise
the relation by chords instead of summand abscissae: the line through `Q`
with slope `λ` meets `E` again in `P₁, P₂` with `x₁ + x₂ = λ² − x_Q`,
`x₁x₂ + x_Q(x₁ + x₂) = a − 2λν`, `ν = y_Q − λx_Q`. "Both small" is a
small-root condition on polynomials in `λ` (and in `(λ₁, λ₂)` for `m = 3`
through the intermediate point). This is a different polynomial family from
`S_{m+1}` in `(x₁, …, x_m)`: fewer unknowns, higher degree per unknown,
different Newton polytope, so the Jochemsz–May/LP reach is not the W10 one.

**Prior art.** `research/lit2026_w10_s5_coppersmith_20261011/` (exact LP:
`δ* = 1/6, 1/24, 1/80` for `S₃, S₄, S₅`; the `m = 4` oracle needs about
`1/5`); `research/ic_next10_20261014/05-multipoly-lattice/` (shared-target
systems 1.12×, below trigger); Petit–Kosters–Messeng (PKC 2016, prime-field
digit systems); Semaev 2004 (bounded solutions of summation polynomials).
Finding points with Coppersmith (Hirai–Takagi? verify) and "Solving modular
cubic equations with Coppersmith's method" (Inf. Comput. 2024) are the
nearest lattice results on curve equations.

**Discriminating test.** Zero-compute first: write the `m = 2` and `m = 3`
chord systems, their Newton polytopes, and run the W10 LP optimiser on the
new family to get `δ*`. **Kill** if `δ*` for `m = 3` is below `1/24` (no gain
over the `S₄` family) and for `m = 4` below `1/32` (the known family
ceiling). Only a `δ* ≥ 1/10` for `m = 4` justifies a lattice run.

**Hooks.** `research/lit2026_w10_s5_coppersmith_20261011/` (LP code),
`semaev_higher.rs`, `prime_small_x.rs`.

### LEAD-11 — McGuire–Mueller Gröbner-free algorithms (ePrint 2017/1262): full-text verification and classification

**Class.** Literature verification. **Tier.** Unknown until read; the
knowledge note `KN-LIT-6966ea` is abstract-only.

**Mechanism.** The paper claims (abstract) an index calculus that evaluates
summation polynomials fast and needs no Gröbner basis, plus a second
algorithm that needs neither Gröbner bases nor summation polynomials, with a
complexity estimate and extensive data (prime fields, per the 2015 UCD
seminar abstract). The task is to obtain the full text (IACR ePrint PDFs are
Cloudflare-blocked from this container; try arXiv/HAL/author pages/Google
Scholar caches; record every failed route), extract Algorithms 1 and 2
precisely, and classify them against the repository's rf-mitm, rf-pairs and
line oracles and against the CGK/structured-GGM bounds.

**Prior art.** `KN-LIT-6966ea` (abstract only; "reported still worse than
rho"); Caen 2018 talk "A few more index calculus algorithms for the ECDLP"
(Mueller; server timed out); `research/elliptic_snfs_survey_20261016/README.md:363`
(Pintore, Momonari–Kudo, Hu 2024/1923 in the same paragraph);
`ic_framework/rootfind.rs`, `line_oracle.rs`.

**Discriminating test.** Produce a one-page mapping: each McGuire–Mueller
algorithm → the repository oracle it coincides with (or "new"), its cost per
relation in field operations as the paper states it, and whether the generic
bounds cover it. **Kill** if both algorithms are table/enumeration oracles the
repository already has.

### LEAD-12 — Theory gate: structured-GGM and non-adaptive time–space audit of every lead

**Class.** Theory (all fields). **Tier.** Gate; it ranks the other cards.

**Mechanism.** For each of LEAD-01 to LEAD-14, state the oracle it would
build, whether it is built from group operations and membership tests
(Corrigan-Gibbs–Kogan `S·T² ≳ n`, Shoup), what fraction `δ` of points it
exploits structure on (Corrigan-Gibbs–Henzinger–Wu, ePrint 2026/384:
`Ω(min(√q, 1/δ))`), and whether it is non-adaptive with advice below `√N`
(Dinur–Keller–Marmor, ePrint 2025/783). Classify each as "can move the
exponent in principle", "constant only", or "already excluded". Record where
the mapping is interpretation rather than a theorem of the paper (as
`research/lit2026_w3_ggm_audit_20261011/` does for ePrint 2026/384).

**Prior art.** `research/lit2026_w3_ggm_audit_20261011/README.md`,
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1,
`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §0.1 and lines 79–89,
`research/literature_search_20261011/README.md` §1.

**Deliverable.** A table with one row per lead; no kill criterion (it is the
gate others use).

### LEAD-13 — Summation ideals (Faugère–Wallet 2018) and Nagao on Joux–Vitse covers: the even-characteristic solution-count reduction

**Class.** Formulation on the cover (the one live sub-rho regime). **Tier.**
Constant on the cover's decomposition step; the regime already has an
exponent advantage.

**Mechanism.** Faugère–Wallet (DCC 86, 2018) model PDP on hyperelliptic
Jacobians with "summation ideals" and show that in even characteristic the
number of `F̄`-solutions of the decomposition system drops by up to
`2^{(n−1)(g+1)}` depending on the curve equation (a type-II genus-2 curve over
`F_{2^{93}}`: 4096 → 64 solutions), against Nagao's general Riemann–Roch
system of `(n² − n)g` quadrics. The repository's Joux–Vitse path
(`jv_cover.rs`, Nagao decompositions on a genus-3 cover) and the cubic-GHS
genus-3 route (`q^{1.30}` with double large primes, 2026-10-14) both pay the
Gröbner cost of Nagao's system per decomposition. The question: does the
repository's cover path realise the solution-count reduction, and what is
the odd-characteristic analogue (rational 2-torsion on the cover)?

**Prior art.** Faugère–Wallet 2018 (HAL hal-01658573); Nagao 2007/2010
(ePrint 2007/112; ANTS 2010); Joux–Vitse 2012 (Eurocrypt; ePrint 2011/020);
repository `RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` §4.2, §5 (Nagao at prime
`n` is `S′₄` with the linear part hidden), `research/ic_next10_20261014/07-cubic-ghs-genus3/`,
`research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md`,
`jv_cover.rs`, `hyperelliptic_index_calculus.rs`.

**Discriminating test.** On the genus-3 cover at `q ≈ 2^{11}` (the 10-14
sizes), count `F̄`-solutions of the decomposition ideal with and without the
summation-ideal modelling (msolve degree of the ideal), and time one
decomposition. **Kill** if the solution count does not fall by ≥ 8× or the
per-decomposition time by ≥ 2×.

### LEAD-14 — Prime-field digit and Kummer-tower systems re-modelled as multi-homogeneous blocks

**Class.** Formulation (prime fields). **Tier.** Exponent-relevant only if a
block structure gives a provable degree bound below the measured D ≥ 8.

**Mechanism.** Petit–Kosters–Messeng write `x_i` in digits with Boolean
constraints (or the repository's Kummer tower, `RESEARCH_PKM_TOWER_ORACLE.md`)
and solve `S_{m+1}` with F4; the measured solving degree plateau at 7 broke
(D ≥ 8 at `N = 20`, 2026-10-14). The digit expansion is a *product* structure:
`x = Σ b_j 2^j` makes every monomial of `S_{m+1}` a sum of products of digits
from different summands, so the system is multi-homogeneous in the `m` digit
blocks before field equations are added. The multilinear bound
`Σ n_i − max n_i + 1` (Faugère–Safey El Din–Spaenlehauer, "unmixed case") and
sparse Gröbner bases over the product polytope may give a degree bound that
is linear in the digit count with a small constant, or may show the bound is
no better than measured. Also assess the Kummer tower (`x` as `t`-fold square
roots) as a tower of bilinear equations `x_{j+1}² = x_j`.

**Prior art.** PKM 2016; Amadori–Pintore–Sala 2018 (one Gröbner basis for
all relations); Momonari–Kudo 2018 (improvement; locate the primary source);
Hu, ePrint 2024/1923 (critical review); repository
`RESEARCH_PKM_TOWER_ORACLE.md`, `f4_fp_tower.rs`,
`research/ic_next10_20261014/03-pkm-degree/`, `research/pkm_m3_n21_msolve_20261018/`,
`RESEARCH_HAMMING_IDEAL_PDP.md`.

**Discriminating test.** Zero-compute: for `m = 3`, `t ∈ {3, 4, 5}` digits
(or tower depth), compute the multi-homogeneous Hilbert series of the
generic system with the block structure and read the predicted degree;
compare with the measured `D = 6, 6, 7, 7, ≥ 8` at `N = 9–20`. **Kill** if the
predicted bound is ≥ the measured degree at every `N` (then the block
structure explains nothing and offers nothing).
