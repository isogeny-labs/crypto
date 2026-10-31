# LEAD-01 — MinRank / Kipnis–Shamir / support-minors modelling of the summation chain

**Verdict.** closed-by-theorem, as stated. The three MinRank modellings of the
Sylvester rank drop are the repository's three existing formulations (S₄ direct,
the chain, and the chain's ideal), so none can read a lower degree than the
chain does. One residual constant-factor probe (R1, §6) survives, with a prior
of about 10 %.
**Confidence.** medium. The identifications are exact algebra plus one literature
theorem I read (Bardet–Bertin, Prop. 1). "Closed" is about the ideal, though, and
a generating set can still move the solving degree by one; that part is unmeasured.
**Expected gain if it works.** Only R1 can gain, and at most one degree (4 → 3).
Stage level: the degree-4 matrix has 7.1× (N = 30), 9.5× (N = 40) and 65× (N = 263)
more columns than the degree-3 one. In the repository's n = 131 cost model, `m = 4`
moves from +7.62 to +5.34 bits over rho at `w = 1` and from +19.80 to +14.41 at
`w = 2` (unverified desk check). Both remain above rho. No leaderboard row moves
through the root-finder arms. The closest F4-based rows are inherited-F4 S/rho =
1.105, 1.582, 2.091 at n = 13, 19, 23 (lower bounds, `docs/ic/LEADERBOARD.md`, "F6-IC E_0 ladder").
**Cost of the discriminating test.** About 1–1.5 person-days of engineering
(a seed exporter) plus about 4 CPU-hours and at most 1.5 GB.

## 1. Mechanism (precise)

**Setting.** `E: y² + xy = x³ + ax² + b` over `F_{2^n}`. `S₃(x₁,x₂,X) =
(x₁+x₂)²X² + x₁x₂X + (x₁x₂)² + b` (5 monomials, `binary_semaev.rs`).
`V ⊂ F_{2^n}` is an F₂-subspace of dimension ℓ, `x_i = Σ c_{ia} v_a`, `c ∈ F₂^ℓ`.
Frobenius is F₂-linear, so `x^a` has bit-degree `wt(a)`, the binary weight.

**Coefficients and the correspondence matrix.** In `Y`: `A = (x_i+x_j)²`,
`B = x_ix_j`, `C = (x_ix_j)² + b`, with bit-degrees 1, 2, 2. Equivalently
`S₃(x,y,z) = v(y)ᵀ M(x) v(z)` with `v(z) = (z²,z,1)` and
`M(x) = [[1,0,x²],[0,x,0],[x²,0,b]]`. The entries of `M(x)` are affine-linear in
the bits of `x ∈ V`. This is the one place a linear-matrix-space structure exists.

**m = 3.** `S₄(x₁,x₂,x₃,x_R) = Res_Y(f,g)` with `f = S₃(x₁,x₂,Y)` and
`g = S₃(x₃,x_R,Y)`.
- *Sylvester, 4×4 over `F_{2^n}`.* Rows `(A₁,B₁,C₁,0)`, `(0,A₁,B₁,C₁)`, `(A₂,B₂,C₂,0)`,
  `(0,A₂,B₂,C₂)`. Applied to `(Y³,Y²,Y,1)` it returns `(Yf, f, Yg, g)`. Entry degrees
  in `x` are 2, 2, 4 and bit-degrees 1, 2, 2. `x_R` is a constant, so `(A₂,B₂,C₂)` is
  affine-linear in `c₃`.
- *Bézout, 2×2.* Entries are the 2×2 minors of `[[A₁,B₁,C₁],[A₂,B₂,C₂]]`:
  `h₁₁ = A₁B₂+A₂B₁`, `h₁₂ = A₁C₂+A₂C₁`, `h₂₂ = B₁C₂+B₂C₁`, all of bit-degree 3.
  `det = h₁₂² + h₁₁h₂₂` is exactly the formula in `binary_semaev.rs`. The kernel
  equations `h₁₁y+h₁₂ = 0` and `h₁₂y+h₂₂ = 0` have bit-degree 4.
- *Unverified desk check:* `det(Sylvester) = S₄` as a polynomial identity; `S₄` has
  24 monomials in `x₁..x_R, b`, per-variable degree 4, total degree 12, and its
  descended bit-degree is 6 (the monomial `x₁³x₂³x₃³` occurs).

**Counts after Weil descent (bits; N = unknowns).**

| formulation | N | equations | max bit-degree | (n,ℓ) = (15,5) | (131,44) |
|---|---|---|---|---|---|
| Minors = `S₄` direct | 3ℓ | n | 6 | 15 | 132 |
| KS, free kernel `(k₃,k₂,k₁,1)` | 3ℓ+3n | 4n (2n cubic, 2n quadratic) | 3 | 60 | 525 |
| KS, Veronese kernel = the chain | 3ℓ+n | 2n (n cubic, n quadratic), plus rows `Yf`, `Yg` | 3 | 30 | 263 |
| SM, corank 1 | as KS | as KS | 3 | 60 | 525 |
| Bézout-SM, kernel `(y,1)` | 3ℓ+n | 2n | 4 | 30 | 263 |

SM at corank 1: the four 3×3 Plücker coordinates of the rank-3 support of a 4×4
matrix are its kernel vector up to scale, and the SM equations are `M·κ = 0`. For
`m = 4` the chain has `4ℓ+2n` unknowns and `3n` equations (394 and 393 at ℓ = 33).
A single 6×6 Sylvester of `S₃` and `S₄(·,·,·,X)` needs `S₄`'s coefficients in `X`
(degree 4 per variable). Its free kernel has 5 unknowns of n bits, `4ℓ+5n = 787`
at ℓ = 33. The nested form is the chain.

**What survives descent.**
1. Every chain equation is multi-affine in the blocks `(c₁,c₂,c₃,u)`, with at most
   one variable per block, and affine in `u` (the intermediate point's bits).
   Unverified desk check, toy `n = 11, ℓ = 3`: `f₁` has block profile
   `(c₁,c₂,u) = (1,1,1)` and `f₂` has `(c₃,u) = (1,1)`. So an SM-style bidegree `(D,1)` is meaningful.
2. The matrix entries are not linear in `c` (`B = x₁x₂`). There is no MinRank
   instance over F₂ with `c` as the linear variables unless each product is
   linearised (ℓ² new variables, rank-one constraint). That is the
   `linearization_reach` / `xl_bilinear_core` instrument.
3. The low Frobenius rank that HFE-style arguments use is destroyed by the
   factor-base restriction: the vanishing polynomial of `V` has Frobenius degree ℓ
   (`RESEARCH_FALL_DEGREE_BOUNDS.md` §2.3).
4. At `m = 4` the middle equation `S₃(y₁,x₃,y₂)` is affine in `y₁` and in `y₂`
   separately but jointly cubic (`y₁x₃y₂`). No single kernel block exists.

**Degree bounds each modelling is expected to obey.** (D_reg values are
unverified desk checks of the Bardet–Faugère–Salvy series; the `(n,ℓ)` cells are the repository's.)
- *F4 floor.* Inputs have degree 3, so a chain-shaped model cannot read below 3.
  Measured: msolve F4 reads 4 (some draws 3) at `m = 3`, (n,ℓ) = (15,5), and at
  `m = 4`, (11,4). The card's "(ℓ,m) = (11,4)" is this `m = 4` cell.
- *Semi-regular (BFS)* for the chain: 6, 7, 7, 8, 9 at (9,3), (13,4), (10,5),
  (15,5), (19,7). Measured XL: 6, 6, 7, ≥7. Measured F4 and MutantXL closure: 4
  (closure > 4 at m = 3, ℓ = 7). F4 sits 2–4 below any generic reference.
- *Faugère–Safey El Din–Spaenlehauer (FSS), Thm 6.1.* A generic affine bilinear system
  with `n_x+n_y` equations in `n_x+n_y` unknowns, characteristic 0 or large, has
  `d_reg ≤ min(n_x+1, n_y+1)`. The chain is neither bilinear (cubic term) nor
  square, and the field is F₂. Read as `min(3ℓ,n)+1`: 16 at (15,5), 20 at (19,7),
  132 at (131,44). That is 4× to 33× above measured F4.
- *Bardet et al. support minors (SM).* Unknowns `K + C(n,r)`, equations
  `m·C(n,r+1)`, solved by linearisation at bidegree `(b,1)` when
  `m·C(n,r+1) ≥ K·C(n,r) − 1`. Applied to the chain's u-degree-1 structure
  (generic count, assumes full independence): `D_c` = 8, 11, 53 at (15,5), (19,7), (131,44), with
  3.7×10⁵, 2.8×10⁷ and 2^132.9 columns, against 3.2×10⁴ columns for F4 at (15,5).
  Structured systems can refute earlier, which is what F4 does here.
- *Bardet–Gilard (Gilard) Hilbert series.* Exact for generic homogeneous matrices with
  degree-D entries and `K ≥ m(n−r)` over a large field. The Sylvester instance is
  structured and Boolean, so I derive no prediction from it.

## 2. Prior art: literature

- Faugère, Safey El Din, Spaenlehauer, *Gröbner bases of bihomogeneous ideals generated
  by polynomials of bidegree (1,1)*, arXiv:1001.4004v2 (JSC 2011, venue recalled).
  `https://arxiv.org/abs/1001.4004`. **read-at-source** (Thm 6.1, Lemma 6.1, Prop 6.1, Cor 6.1).
  Claim: generic square affine bilinear systems have `d_reg ≤ min(n_x,n_y)+1`.
- Bardet, Bros, Cabarcas, Gaborit, Perlner, Smith-Tone, Tillich, Verbel, *Improvements
  of algebraic attacks for solving the Rank Decoding and MinRank problems*,
  arXiv:2002.08322v4 (Asiacrypt 2020, venue recalled). `https://arxiv.org/abs/2002.08322`.
  **read-at-source** (introduction, Modeling 1, Remark 1; the complexity sections only skimmed).
  Claim: SM has `K + C(n,r)` unknowns, `m·C(n,r+1)` bilinear equations, and the linear
  variables must enter linearly.
- Bardet, Gilard, *Computation of the Hilbert series for the support-minors modeling of
  the MinRank problem*, arXiv:2502.12721v2. `https://arxiv.org/abs/2502.12721`.
  **read-at-source** (abstract, introduction, main formula (2), genericity statement; sections 3–6 not read).
  Claim: exact SM Hilbert series for generic degree-D matrices, generic for `K ≥ m(n−r)`.
- Bardet, Bertin, *Improvement of algebraic attacks for solving superdetermined MinRank
  instances*, arXiv:2208.01442v1 (PQCrypto 2022). `https://arxiv.org/abs/2208.01442`.
  **read-at-source** (abstract, Modelings 1–4, Prop. 1). Claim: the KS and SM-C ideals are
  equal for any instance, and the Minors equations lie in the KS ideal.
- Guo, Ding, *Algebraic relation of three MinRank algebraic modelings*, WAIFI 2022.
  **abstract-only** (search listing, and Gilard's introduction). Claim: same
  relation, derived independently.
- Faugère, Safey El Din, Spaenlehauer, *On the complexity of the generalized MinRank
  problem*, arXiv:1112.4411. `https://arxiv.org/abs/1112.4411`. **abstract-only.**
  Claim: Minors-modelling complexity under genericity.
- Kipnis, Shamir, *Cryptanalysis of the HFE public key cryptosystem by relinearization*,
  CRYPTO 1999. **recalled.** Unreachable: Springer answered 303 to an IdP, `iacr.org`
  archive PDF 403, DBLP denied. The KS modelling is read at source as restated in Bardet–Bertin Modeling 1.
- Huang, Kosters, Yeo, *Last fall degree, HFE, and Weil descent attacks on ECDLP*, CRYPTO
  2015, ePrint 2015/573. **abstract-only.** Claim: HFE has small last fall degree, and the
  first-fall-degree assumption is doubtful for ECDLP.
- Huang, Kosters, Yang, Yeo, arXiv:1505.02532. `https://arxiv.org/abs/1505.02532`.
  **abstract-only.** Claim: the `n`-independent bound needs zero-dimensional systems; summation
  systems without field equations are not.
- Ning, Ran, Samardjiska, *Multi-homogeneous XL*, ePrint 2025/2060. **abstract-only.**
  Not applied here; relevant to LEAD-14.
- Kosters–Yeo (arXiv:1503.08001), Kousidis–Wiemers (arXiv:1906.05594), Caminata–Gorla
  (arXiv:2112.05579): **recalled**, via the repository ledger `RESEARCH_FALL_DEGREE_BOUNDS.md`.
- *Search scope.* Two web searches for MinRank or KS modelling of summation polynomials
  or ECDLP found nothing. This is not proof that no such work exists.

## 3. Prior art: these repositories

- `research/factor_base_designs_20261015/README.md` row K4 and `PREREGISTRATION.md` §4
  (E-K4): "reopened, no checkable statement". **Supports**: §1's identifications are checkable statements
  (cf. `chain_sound`, `quartic_norm_factor`) and dispose of K4's Sylvester branch.
- `research/xl_bilinear_core_20261006/README.md`: bidegree XL on the bilinear core; best charged gap at
  n = 131 stays 71.82 bits (ω = 2). **Closes** the SM-type `(D,1)` route there.
- `research/linearization_reach_20260930/README.md`: the `b(b+1)/2` symmetry defect; gap ≥ 56 bits at every
  arity. **Closes** linearisation-based resolution past `n/3`.
- `research/notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md`: FFD 3, XL refutation 5–7+ on the chain,
  closely tracking the semi-regular reference. **Supports** the baseline readings above.
- `research/ic_next10_20261014/06-equivariant-f4-m4/README.md`: msolve F4 reads 4 or 3 on 56 draws, with the committed rising
  degrees being XL's. **Supports** (this is the baseline to beat).
- `research/dreg_closure_scale_20261016/RESULTS.md`, `dreg_closure_driver_20261016/RESULTS.md`,
  `dreg_closure_specialise_20261017/RESULTS.md`, `dreg_closure_m4l6_20261018/RESULTS.md`: closure degree 4 for
  `m = 3`, ℓ ≤ 6 and `m = 4` to (15,6); rises past 4 at `m = 3`, ℓ = 7. **Supports.**
- `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md`: no `n`-independent last-fall bound for the subspace base;
  Prop. B is a low-rank statement (the quadratic parts span ≤ n′ dimensions for a subfield base). **Closes** the HFE-rank route.
- `research/notes/index-calculus/RESEARCH_DEGREE_REDUCTION.md`: L4 explicit degree falls lower `D*` (generic 4 → 2) but pay only
  where the base degree is high; L2 symmetrisation, the nearest analogue of a Plücker-style
  variable change, is "blocked on reach". **Supports** R1's framing.
- `research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` §2: cost model and free-oracle floor 2^68.58 (`m = 3`).
  **Closes** the `m = 3` row regardless of degree.
- `research/notes/ecdlp-general/RESEARCH_NZMS_AMS_AUSTMS_2024_IDEAS.md` row 9 (MinRank structure of `B_V`) and
  `docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md` G4 (mixed-field F4, predicted to fail, not run): **supports** closure.
- `src/cryptanalysis/binary_semaev.rs`, `binary_semaev_s4.rs` (`AnfF2m`), `semaev_higher.rs`
  (`sylvester_resultant_in_x`): the objects of §1. `AnfF2m::mul` makes R1's seeds a short exporter.
- Autoresearcher `knowledge/open-problems/KN-OPEN-007.md`: bond ranks of the prime-field resultant tree near full at toy
  scale. **Adjacent**, not binary.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not, through MinRank. KS with the Veronese kernel is the chain, so its degree
is the chain's. KS with a free kernel adds `2n` unknowns and `2n` equations that
`F4` must rediscover; its degree-4 matrix is 14.7×–16.4× wider (desk check), so it
fails the card's second kill clause. SM at corank 1 equals KS (Bardet–Bertin Prop. 1),
and Minors is `S₄` direct, which has bit-degree 6. The bounds from FSS and Gilard
need genericity this system lacks, and read at 3ℓ+1 or `D_c ≈ 0.4·3ℓ` they grow
linearly in ℓ, which at n = 131 means a column count above exhaustive search.

The only residue is a generating-set effect. `y·f₁` is Sylvester row 1, and the same
binary-carry rule gives two more degree-3 elements. Unverified desk check, toy
`n = 11, ℓ = 3`: as F_{2^n}-valued polynomials `x₁·f₁`, `x₂·f₁` and `y·f₁` have bit-degree 3
where the bit-level product has 4; `x₃·f₁` stays at 4. So the closed-form falls are the `3n` bit
components of those three products. They lie in the degree-4 Macaulay span, so the closure at
cap 4 already contains them. At cap 3 they are new, but being degree 3 they cannot be multiplied
further. A degree-3 refutation would have to follow from `3n` extra rows alone. That is the
assumption R1 tests, and I put 10 % on it.

## 5. Obstructions

- **Identification theorem (derived plus Bardet–Bertin Prop. 1).** Minors ⊂ KS = SM-C. For corank 1 these
  are S₄ direct, the chain, and the chain.
- **Degree floor.** The inputs have degree 3. Even a perfect model buys one degree.
- **Growth.** Every bound a MinRank theory supplies (FSS `3ℓ+1`, SM `D_c`, BFS) grows with ℓ.
  At n = 131 the SM-type count is about 2^133 columns, above exhaustive search (2^129).
- **No `n`-independent last fall bound** (HKYY, via `RESEARCH_FALL_DEGREE_BOUNDS.md`). The constant-degree
  rows rest on unproven `d_F ≈ d_ff`.
- **Bilinear core closed** by `xl_bilinear_core` (gap 71.82 bits at n = 131) and `linearization_reach`.
- **Yield conservation (KN-FIND-007).** Yield per trial depends only on `|V|` and `m`; any change here is
  per-target solver cost, so the tier is constant. The oracle is algebraic, not a membership or table
  oracle, so Corrigan-Gibbs–Kogan and Shoup do not bound it directly. The free-oracle floor does:
  `m = 3` is above rho at any solver cost (`2^68.58` vs `2^60.81`), and at `m = 4` the `D = 2` row already
  spends the margin (`+2.07`). Structured-GGM (ePrint 2026/384, from the card, not read) is not engaged.

## 6. Minimal discriminating test (frozen protocol)

The card's own test (KS and SM systems fed to F4) is already decided by §1 for the Veronese
variants (identical system) and the free kernel (wider). The test worth running is R1, the closed-form
carry seeds.

**Stage E (engineering tier, not mechanical).** Add `examples/lead01_carry_seeds.rs`, a variant of
`ic_next10_06_chain_export` that appends, to the exported msolve input, the bit components of
`x₁·f₁`, `x₂·f₁`, `y·f₁` (`AnfF2m::mul`, `m = 3`). Unit tests: each seed vanishes on every
solution of four satisfiable draws (`--sat-ok`); the bit-degrees are 3, 3, 3 and `x₃·f₁` is 4.
Estimate 1–1.5 person-days.

**Stage M (mechanical).**
- *Instances.* `m = 3`: (n,ℓ) ∈ {(9,3), (13,4), (10,5), (15,5)}; the committed unsatisfiable draws 0–3 from
  `dreg_ladder`'s cell seeds, as selected by the exporter. Optional, if M passes: `m = 4`, (11,4), draws 0–3.
- *Instrument.* msolve 0.6.5, `-t 1 -g 2 -v 2`, DRL, characteristic 2, field equations appended, 3 GB address-space cap, as
  `ic_next10_20261014/06-equivariant-f4-m4/run_f4.py`. msolve is not on PATH in the director container and must be provisioned.
- *Arms per draw.* A: committed system. B: A plus the `3n` carry seeds. N (null): A plus `3n` random
  F₂-combinations of existing degree-3 Macaulay rows.
- *Recorded.* Maximum F4 step degree, largest matrix (rows × columns), GB = [1], wall time, peak RSS.
- *Controls.* C1: arm A reproduces the committed degrees (4, with 3 on some draws). C2: seed soundness as
  above. C3: N must equal A.
- *Scoring.* Count only draws where A reads 4; a draw where A reads 3 is at the floor and is excluded.
- *Prediction.* B equals A on at least 90 % of the counted draws.
- *Kill.* B's maximum step degree is below A's on fewer than 25 % of the counted draws, or N
  also lowers it (then the effect is redundancy, not carries). Close R1 and the lead.
- *Success.* B below A on at least 75 % of the counted draws with N equal to A. Then extend to the (11,4) `m = 4` cell and to ℓ = 7.
- *Between.* Inconclusive; double the draws once, then apply the same rule.
- *Cost.* `m = 3` cells at most 80 s and 0.35 GB per run (item 6 figures), three arms, 16 draws:
  under 1 CPU-hour. Optional `m = 4`: at most 805 s and 1.5 GB per run, 12 runs: under 3 CPU-hours.

## 7. Ranking inputs

| expected gain | confidence | cost | tier | depends on |
|---|---|---|---|---|
| ≤ 5.4 bits (model `m = 4`, `w = 2`); ≤ 7–65× columns at stage level; 0 on current root-finder rows | low for R1 (10 %); medium that the main lead is closed | 1–1.5 person-days plus about 4 CPU-hours, ≤ 1.5 GB | constant | independent; shares the degree-instrument reconciliation with LEAD-05; its multi-homogeneous half belongs to LEAD-14; LEAD-12 gate |

Distance to rho is unchanged by anything here. The model's best `m = 4` row after R1 would
still sit about 5.3 bits above rho (`w = 1`, unconverted units).

## 8. Open questions

- msolve F4 at `m = 3`, ℓ ≥ 7 was never read (memory), so whether F4 rises past 4 where the closure does is unknown.
- The exact Hilbert series of the structured, Boolean Sylvester ideal was not computed (no Magma/Macaulay2 here).
- The multi-homogeneous (border-regular) prediction for the four-block Boolean structure was not computed. Field
  equations break the multigrading, so applicability is unclear (see LEAD-14).
- Kipnis–Shamir 1999 itself was unreachable; Guo–Ding and the other abstract-only items were not read in full.
- K4 could be retired in the repository's own idiom by a Lean statement of the §1 identification
  (determinant equals resultant; KS rows ⊂ chain ideal over any commutative ring). I did not write it.

## 9. Sources

Fetched (HTTP outcome):
- `arxiv.org/abs/1001.4004`, `/abs/2002.08322`, `/abs/2502.12721`, `/abs/2208.01442`, `/abs/1112.4411`,
  `/abs/1505.02532`: success.
- `arxiv.org/pdf/1001.4004`, `/pdf/2002.08322`, `/pdf/2502.12721` via WebFetch: PDF bytes returned and
  saved, text extracted with `pdftotext`. `arxiv.org/pdf/2208.01442` via `curl`: 200.
- `eprint.iacr.org/2015/573`, `/2025/2060`: abstract pages returned.
- `link.springer.com/chapter/10.1007/3-540-48405-1_2`: 303 to `idp.springer.com`, not followed.
- `iacr.org/archive/crypto1999/17160019/17160019.pdf`: 403. `dblp.org/rec/conf/crypto/KipnisS99.html`: access denied.
- Two WebSearch queries (MinRank / KS with summation polynomials, ECDLP, Sylvester): no relevant hit.

Repository paths read: `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md}`;
`research/factor_base_designs_20261015/{README.md,PREREGISTRATION.md}`;
`research/xl_bilinear_core_20261006/README.md`; `research/linearization_reach_20260930/README.md`;
`research/notes/index-calculus/{RESEARCH_DREG_MEASUREMENT.md,RESEARCH_FALL_DEGREE_BOUNDS.md,RESEARCH_DEGREE_REDUCTION.md}`;
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md`;
`research/ic_next10_20261014/06-equivariant-f4-m4/README.md`;
`research/dreg_closure_{scale_20261016,driver_20261016,specialise_20261017,m4l6_20261018}/RESULTS.md`;
`research/literature_search_20261011/README.md`; `research/ecdlp_leads_20261018/README.md`;
`research/notes/ecdlp-general/RESEARCH_NZMS_AMS_AUSTMS_2024_IDEAS.md`;
`docs/ic/RESEARCH_IC_NOVEL_DIRECTIONS_20261007.md`; `docs/ic/LEADERBOARD.md`;
`src/cryptanalysis/{binary_semaev.rs,binary_semaev_s4.rs,semaev_higher.rs}`;
`examples/ic_next10_06_chain_export.rs`; `crypto-autoresearcher/knowledge/open-problems/KN-OPEN-007.md`.

Unverified desk checks (scratchpad scripts, not committed, not measurements):
`s4_support.py`, `dreg_counts.py`, `price_rows.py` and `seeds_degree.py` under
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/work/`. `price_rows.py`
reproduces the repository's `m = 4` rows `D = 2` (+2.07, +8.83) and `D = 4` (+7.62) and the `m = 3` `D = 4` row (+21.46) before
computing `D = 3`.
