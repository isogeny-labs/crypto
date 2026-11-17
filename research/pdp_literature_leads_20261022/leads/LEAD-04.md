# LEAD-04 — Homomorphic class pre-partition of the factor base (Tate pairing, ℓ-descent, trace parity)

**Verdict.** constant-factor candidate, narrowly scoped: it exists only on curves
with cofactor > 1 (so only the binary rows of the leaderboard), and it is
**closed-by-theorem on every prime-order row**. It is dominated by choosing the
factor base's class (the repository's "trace-zero bit" and cofactor projection),
which is not this lead.
**Confidence.** medium on the bounds below (read from source, desk-checked on
toy curves); low on the end-to-end effect on `S` (nothing here was measured).
**Expected gain if it works.** 1.000× on all eight Table B prime rows and the
fp32 pair-table study (theorem). On the cofactor-2 and cofactor-6 random-binary
rows (`f2m15`, `f2m21`, `f2m27`, Table B) the filter removes at most the pair-table
build's additions: about 1.1–1.2× on `S` (desk, unverified), not `|A|`. Exponent:
none (`S ∝ r^{1/6}` for the pair-table family is unchanged).
**Cost of the discriminating test.** about 1 person-day (a class-restricted
`PairTable` builder plus a census driver) and under 1 CPU-hour, under 1 GiB.

## 1. Mechanism (precise)

**Setting.** `#E = h·r`, `r` prime, `gcd(h, r) = 1` (every ranked row of Tables A–C; the
withheld Table D curves have full 2-torsion and are never prime order).
Then `E(F_q) ≅ Z/r × H` with `|H| = h`, and `P ↦ [r]P` identifies `E/⟨G⟩` with `H`.
Any homomorphism `χ: E(F_q) → A` with `gcd(|A|, r) = 1` (every ℓ-descent, Tate or
Kosters–Yeo class, since `ℓ ≠ r`) kills `⟨G⟩` and factors through `H`: it is a
quotient of the cofactor class `[r]P`. Two consequences, used throughout:

- A target `R ∈ ⟨G⟩` has class 0, so a relation `R = Σ ε_i P_i` forces
  `Σ ε_i χ(P_i) = 0`. This is a property of the base alone: independent of `R`
  and of every discrete logarithm. `χ` carries no information about the DLP.
- If `h = 1` (prime order) the only quotients of `Z/r` are `1` and `Z/r`. There is
  no class partition at all.

**Class counts.** For any finite abelian `G`, `|G/ℓG| = |G[ℓ]|`.
- Odd `p`, `E[2] ⊂ E(F_p)` (`y² = ∏(x − e_i)`): `|E/2E| = 4`;
  `δ(P) = (x − e₁, x − e₂)` mod squares, injective on `E/2E`, kernel `2E`. Desk
  check (unverified; `p = 1009, 2003`, `#E = 4·r`): 4 classes of equal size, kernel
  `= 2E = ⟨G⟩`, additive. Such a curve has `4 | #E`, so it is never prime order.
  One rational root of the cubic: 2 classes. No root: 1 class.
- `F_{2^n}`, ordinary: `E[2](F) = Z/2`, so `|E/2E| = 2` and the class is
  `χ(P) = Tr((x + a₂)/a₁²)` with kernel `2E` (Kosters–Yeo Prop. 4.2, read). It is the
  char-2 (Artin–Schreier) Tate class (my gloss, not from the paper). Desk check
  (`n = 9, 11`): kernel `= 2E`, additive.
- `ℓ = 3`: needs `3 | #E`, class count 3 (9 with full 3-torsion).

**Cost of `t_ℓ(T, ·)`.** `ℓ = 2`, `T = (e, 0)`: `x_P − e` mod squares, no group
operation, one Legendre symbol (`F_{2^n}`: one parity of a masked word).
`ℓ = 3`: `T` is a flex, `f_{3,T}` is the tangent line, `t₃ = ℓ_T(P)^{(p−1)/3}`: no
group operation, one exponentiation of about `log₂ p` field mults, and it needs
`3 | p − 1`. Classes beyond `E/ℓE` (the 3-part of `f2m27`, the `Z/8` of `f2m18`) cost
`[r]P`, about `1.5·log₂ r` additions per point.

**What the class removes** (code read; §3). A class prunes only a sum the oracle
*constructs* and no probe can equal:
- `m = 2` pair table: entries `P_i ± P_j` with `c_i ± c_j ≠ 0`. Kept fraction
  `Σ_c p_c p_{−c}` (`1/|A|` for balanced classes); `m = 2` probes are uncharged lookups.
- `m = 3` pair table plus probes `R ∓ P_k`: see §5, bound `≤ 2×` on the charged probe
  additions, for every `A`.
- Not touched: hash lookups (membership is already exact), `rootfind-pairs`, direct
  subtraction, Semaev-roots, `native_signed_mitm` 3+3 (every left half has same-class
  right halves).

**Folds.** `−P` maps class `c ↦ −c`: trivial for exponent-2 `A` (all `ℓ = 2`
classes), so `ℓ = 2` classes are fold-invariant exactly. For `ℓ = 3` the negation-folded
`m = 2` table still keeps `1/3` (one of the two signed combinations per abscissa pair is
class-valid), but the `m = 3` probe set cannot shrink (§5). Koblitz `τ`: acts on `A` as
an endomorphism; on `E/2E = Z/2` it is trivial (and `Tr(x²+a) = Tr(x+a)` for `a ∈ F₂`).
"Class-sum zero" is invariant under the diagonal `⟨τ, −1⟩` action, so it commutes with
the `FrobeniusPairTable` fold. A cyclic `Z/3` class group is impossible on Koblitz
curves (`x² ∓ x + 2` has discriminant `−7`, a non-residue mod 3; desk reasoning).

**Algebraic arm.** One affine `F₂`-linear equation `Σ_{i≤m} Σ_j X_{ij} Tr(v_j/a₁²) =
Tr(x_R) + (m+1)Tr(a₂)` in the `mℓ` Weil-descent bits (27 at `n = 27, ℓ = 9, m = 3`; 36 at
`m = 4`), degree 1, one extra equation. For `S₃` (two summands) it is already in the ideal
(Kosters–Yeo Prop. 4.7, first-fall degree 2); for `m ≥ 3` it is not, and Remark 4.8 says
to add it. The degree-of-regularity bound the system obeys is unchanged: the semi-regular
reference, growing about linearly in the unknowns (§3). In odd characteristic the class is a
Legendre symbol, degree `(q−1)/2`: not a low-degree equation.

## 2. Prior art: literature

- Kosters, Yeo, *Notes on summation polynomials*, arXiv:1503.08001v3 (2015).
  **read-at-source** (arXiv PDF, extracted locally). Prop. 4.2: surjective morphism
  `E(F_{2^n}) → F₂`, `P ↦ Tr((x+a₂)/a₁²)`, kernel `2E`. Remark 4.8: add the linear
  equation from the start, or "only look for relations in the kernel". The paper gives no
  speedup figure and no relation-generation experiment. Its table shows the degree of
  regularity growing (3, 3, 4, 4, 4, ≥5 for `n = 12…40`) while the first fall degree stays 2.
- Fong, Hankerson, López, Menezes, *Field inversion and point halving revisited*, IEEE
  Trans. Computers 53(8):1047–1059 (2004). **read-at-source** (§3.1, Lemma 3.1.4): for `G`
  of odd order, `(x,y) ∈ ⟨G⟩ ⟹ Tr(x) = Tr(a)`; halving is an automorphism of `⟨G⟩`.
  This is the odd-order-subgroup-lies-in-`2E` fact.
- Semaev, arXiv:1504.01175; Karabina, arXiv:1504.02347: **abstract-only**; neither
  abstract mentions a trace class or `2E`.
- Galbraith, Granger, Merz, Petit, *On index calculus algorithms for subfield curves*,
  ePrint 2020/1315 (SAC 2020): **abstract-only**; Frobenius-stable bases and symmetry
  breaking; no class use stated.
- Silverman, *AEC* 2nd ed., X.1 (2-descent, `x − e_i`, kernel `2E`); Washington, *Elliptic
  Curves* ch. 8; Frey–Rück, Math. Comp. 62 (1994) (Tate pairing non-degenerate over finite
  fields). **recalled**; a fetch of Sutherland 18.783 lecture 23 returned the wrong lecture
  (isogeny volcanoes), and the Wikipedia Tate-pairing page is a stub.
- Corrigan-Gibbs–Kogan, Shoup (generic bounds); CGHW ePrint 2026/384; DKM ePrint 2025/783:
  **recalled**, via `LEADS.md`.

## 3. Prior art: these repositories

- `/home/user/crypto-autoresearcher/knowledge/findings/KN-FIND-14706d.md`: the discriminant
  identity `disc(S₃(x_free,T,t_b), x_free)/f(T) = 16 f(t_b)`; `χ(disc) = χ(f(T))`
  identically. **Does not cover this lead**, exactly: `f(x) = ∏(x − e_i) = y²`, so the
  filter tests the *product character* `χ₁χ₂χ₃`, trivial on `E(F_p)` by the curve equation.
  A 2-descent class is `(χ₁, χ₂)`, which takes all four values on `E(F_p)` (desk: 261 points
  per class) and lives in the *kernel* of the product. A class test therefore rejects
  genuine on-curve candidates (mismatched classes), which 14706d could never do. The identity's moral does
  hold in another form: every genuine relation passes the class test, so the filter only
  ever prunes dead candidates. The roots `x(T ± t_b)` of the Semaev quadratic are always
  rational, so the discriminant carries no class information. (Supports the distinction.)
- `…/ledger/evidence/EV-SEMBIN-0c8bf4.yaml`, `…/ledger/hypotheses/H-SEMBIN-83a856.yaml`:
  trace parity confines a base `V ⊂ ker Tr`; `π(P) = Tr(x)+Tr(a)` is the quotient
  `E → E/2E`; per-target mean `2λ` on `2E`, `0` off it; 79,751 targets, 0 violations (toy,
  preliminary, `model_verified: false`). The hypothesis itself calls the quotient "trivial
  on the odd-order subgroup where the DLP lives, and therefore not an attack ingredient".
  Supports the closure.
- `src/cryptanalysis/ic_boundary.rs`: `PairTable::build_negation_folded` iterates every
  pair, no class test; `exact_decomposition_ceiling` and `summands_admissible` already
  compute `[r]P` classes, for accounting and admissibility only; `binary_subspace_factor_base`
  builds raw window points. `orbit_pair_table.rs`, `ic_framework/rootfind.rs`: additions
  charged, lookups "counted, not charged". `native_signed_mitm.rs`: exact 3+3 enumeration.
- `src/cryptanalysis/koblitz_index_calculus.rs`: `distinct_cofactor_classes` (one `[r]` per
  signed orbit), `build_subgroup_orbit_factor_base` (doc: a base off the subgroup "wastes
  most of its sums on the wrong coset"), `CofactorProjection` / `ProjectedFactorBase`, SAT
  option `trace_constraint` (the Kosters–Yeo equation, already implemented). The fold struct
  confirms the table fold is a build/probe trade. Supports "known, engineering".
- `research/notes/ecc2k130/RESEARCH_ECC2K130_DECOMPOSITION.md` §3.1: a base in `ker Tr`
  yields `1.97–2.04×` (exhaustive, `n = 13, 17, 19`); "Class: engineering … exactly one bit".
  `RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md` row A3 same. `koblitz_factor_base_search.rs`
  header: "free and unmeasured … doubles the yield". Closes the yield side.
- `research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md` §3.5, §10.5;
  `docs/ic/boundary_targets.json` (`ceiling_caveat`, L4712, L9957): the trace-zero caveat is
  an **accounting correction, no `S` moves**; index `1.92–2.21` on `f2m18` (cofactor 8,
  4 classes), `3.04` on `f2m15-tm275`. Unrelated to cost; confirms the class census exists.
- `research/ic_leads_review_20261018/README.md`: "trace-zero prefilter never fires on
  odd-order targets" (closed). `research/fall_degree_bounds_20261008/RESULTS.md` Prop. A: the
  trace relation is "one linear equation, and nothing else"; `RESEARCH_ECC2K130_DESCENT_DEGREE.md`:
  first fall degree 3 "everywhere", refutation degree 5, 6, 6–7, 7 for `ℓ = 2…5` at `m = 3`
  (`n ≤ 15`), tracking the semi-regular reference. Closes the algebraic half by measurement.
- `docs/ic/LEADERBOARD.md`: Table B factor-base shares and gaps used in §7; Table A is
  98.5–99.7 % reusable set-up. Pair-table study is on `fp32`, prime order.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not lower either. Step by step, with the assumption each rests on:
1. The class equation is one linear form in `mℓ` unknowns. At `m = 3` the measured
   refutation degree rises by about 0.7 per `ℓ`, about 0.2 per unknown (derived from the
   `5, 6, 6–7, 7` series; unverified inference), so one removed unknown is below the integer
   resolution of `D`. Assumption: the semi-regular reference governs, as measured for `n ≤ 15`.
2. Kosters–Yeo's low first-fall degree is this same equation; their own table and the
   repository's runs show the degree of regularity still grows with `n`.
3. For pair-table oracles the cost that falls is table-build additions, by
   `A^{1/3}` at the `m = 2` family optimum (`ops = aF² + b/F`, `a → a/A`,
   `F* = (A·#E)^{1/3}`; `family_optimum_s` shape law). `A = 2, 4, 6` give `1.26, 1.59, 1.82`
   re-optimised, `1.20, 1.33, 1.38` at fixed `F` (desk, unverified).
4. Compare the *base-class* choice (confine or project): `b → b/A`, a factor `A^{2/3}`
   (`1.59×` at `A = 2`), or `A×` the yield. That is the real lever, it is not a filter,
   and the repository has it (Koblitz) and names it "one bit".

## 5. Obstructions

- **O1, prime order (theorem).** `Z/r` has no quotient of order `< r`. All eight Table B
  prime rows and fp32 have prime `#E` and no root of the cubic mod `p` (desk check:
  `fp24`: `#E = 10933753` prime). `|A| = 1`, gain exactly 1.000×. Curves with
  `E[2] ⊂ E(F_p)` have `4 | #E`: the card's "up to 4" exists only off prime order.
- **O2, subgroup (hint made explicit).** `r` odd ⟹ `⟨G⟩ ⊂ 2E` and `⊂ ℓE` for `ℓ ∤ r`:
  `χ(⟨G⟩) = 0`. The filter acts only on factor-base points outside `⟨G⟩`. If the base lies
  in `⟨G⟩` (projected, or confined to class 0) the filter is vacuous. Oracles left
  untouched: every oracle on a prime-order curve; every oracle with a subgroup/projected
  base; direct subtraction, Semaev-roots, `rootfind-pairs`, `native_signed_mitm` 3+3, all
  hash lookups; F4/msolve/SAT beyond one linear form; any odd-characteristic algebraic system.
- **O3, `m = 3` cap (proved here, desk-checked).** A probe set `S ⊂ A` must meet every
  zero-sum triple. Let `T = A∖S`; if `|T| > |A|/2` then for `t ∈ T` the sets `T` and `−t−T`
  intersect, giving `a + b + t = 0` inside `T`. So `|S| ≥ |A|/2`: the probe-side saving is
  at most 2× for every class group, and 1× for `Z/3`, `Z/3²` (brute force on 8 groups
  agrees). The card's "`|A|` for `m = 3`" is false. A toy `m = 3` restricted scheme
  (probe class 0, equal-class pairs) found exactly the same decomposable targets
  (72/152/226 of 246), table `0.52–0.71` of the full table, probes `0.44–0.80` of the base
  (class imbalance at small `F`; unverified desk check).
- **O4, the metric.** "Candidate tuples examined" in a hash oracle is an uncharged counter
  (`rootfind.rs`); the hit probability per probe is unchanged (`(F/A)/r` vs `F/(h r)`). A
  fall in that counter is relabelling. Only charged additions count.
- **O5, class cost.** `[r]P` costs about `1.5 log₂ r` additions per point, more than
  `|A|`: the card's kill criterion removes the full-class variant at small `F`. Cheap
  classes (trace, Legendre, cubic character) pass. Native class operations must be priced at
  0, 0.1, 1 GAE per op (AGENTS §5a); at 1 GAE a Legendre symbol (about 30–45 mults) is near
  the saving at `F ≈ 500`.
- **O6, generic line.** The filter uses group operations, membership and `[r]`: on the
  Shoup / Corrigan-Gibbs–Kogan line; it needs no structure on a `q^{-1/2}` fraction.
  Constant factor, by construction.

## 6. Minimal discriminating test (frozen protocol)

Step 0 (read-only, 0.2 person-day): from existing round-2 evidence, tabulate per
instance `Σ_c p_c p_{−c}` and the build/trial addition counts. Stop if every treatment row
has `Σ > 0.8`.

Instances. Treatment (cofactor > 1, base unconfined per ledger `y/c ≈ exact`):
`icv1-f2m21-t2591-de54d72e` (h=2), `icv1-f2m27-t12543-569dca8b` (h=6),
`icv1-f2m15-t131-ed7256f0` (h=2). Negative controls: `icv1-f2m18-t393-d2c04152`,
`icv1-f2m24-tm3047-2d0c0e8c` (trace-confined, predicted ratio 1.00 for the `ℓ = 2` class);
`icv1-fp24-t1577-77336155` (prime order, exactly 1.000). Positive control: a
`random_binary_instance` with `n` odd, `a = 0`, cofactor 4 (n ∈ {21, 23}), full `[r]P` class,
predicted about 4.

Instrument: `ic_boundary` ladder rung `mitm_m2_negfold_walk` (base, seeds, walk as the
ledger). Arm A baseline; arm B identical except `build_negation_folded` iterates only
class-valid pairs (class `Tr` for `ℓ = 2`; `[r]P` as separate sub-arm, charged); arm C
(adjacent, not part of the kill): same abscissa count, base class-0 or `[h]`-projected.
20 seeds, paired.

Record: charged build additions, charged trial additions, uncharged lookups, relations,
verified `[d]G = Q`, per-phase `S` in GAE at `w = 0, 0.1, 1`, class-evaluation native ops.
Exactness gate: A and B return identical decompositions on a shared list of 10,000 targets.

Predictions. P1: build ratio A/B `= 1/Σ p_c p_{−c}` (about 2 on h=2 treatment, about 4
on the positive control, 1.000 on `fp24`, about 1 on `f2m18/f2m24`). P2: trials and hit sets
identical. P3: `S(A)/S(B)` about 1.10–1.20 on `f2m21` and `f2m27` (ledger analogues:
table 3,721 vs trials 10,919 additions; table 69,169 vs 75,384), 1.0 on controls. Gap to
matched rho, now and desk-after: `f2m21` 18× → about 16×; `f2m27` 38.2× → about 32×; `f2m15`
13.1× → about 12× (unverified; still more than 10× away).

Kill: any decomposition mismatch (stop, bug); measured P1 below `0.8×` the census value; or
`S(A)/S(B) < 1.05` on both `f2m21` and `f2m27` at `w = 0` and across the band (then
closed-by-measurement). Cost: 1 person-day code, under 1 CPU-hour. The algebraic arm needs
no test: already implemented and measured (§3).

## 7. Ranking inputs

expected gain: 1.0× prime rows (theorem); about 1.1–1.2× on 3 binary rows (desk) |
confidence: medium (bounds), low (end-to-end) | cost: about 1 person-day, under 1 CPU-hour |
tier: constant (`A^{1/3}`; not `|A|`; no exponent) | depends on: none to test; any
leaderboard row needs the validation gate (replay certificate, validator report, matched rho
with the §5a band). Rank below the base-class lever (arm C: `A^{2/3}`, 1.4–1.6× at fixed `F`
on `f2m21`, desk).

## 8. Open questions

- Which Koblitz Table A/C bases are confined, projected or mixed. I did not open the per-row
  `cofactor_classes` fields; Table A is 98.5–99.7 % set-up, so any effect there is diluted
  by an unknown pair-table share. The Koblitz base search scores exact `⟨G⟩` coverage, so
  it may already be class-aware.
- Whether the random-binary ladder was ever run with a projected base. I found no record;
  absence of a record is not evidence. Arm C answers it.
- Classes beyond `E/2E` in char 2 (`Z/4`, `Z/8`) via Witt-vector Artin–Schreier pairings,
  cheaper than `[r]P`: not read, not computed.
- Odd `p` with `E[2] ⊂ E(F_p)` is not on the leaderboard and was only desk-checked at
  `p ≈ 2·10³`; Silverman X.1 and Frey–Rück are recalled, not re-read.
- Table/trial split of `S` on the exact leaderboard rows (the ledger analogues use a
  different base size than "family optimum").

## 9. Sources

Fetched: `https://arxiv.org/pdf/1503.08001` (200; tool returned binary PDF, text read with
`pdftotext`); `https://arxiv.org/abs/1503.08001` (tool rejected: no scheme, not an HTTP
failure); `https://arxiv.org/abs/1504.01175` (200); `https://arxiv.org/abs/1504.02347`
(200); `https://eprint.iacr.org/2020/1315` (200, abstract page); `https://www.ece.uvic.ca/~fayez/research/gf3m/fong_j04.pdf`
(200, PDF read); `https://research.birmingham.ac.uk/files/104974339/On_Index_Calculus_Algorithms_for_Subfield_Curves.pdf`
(403); `https://math.mit.edu/classes/18.783/2015/LectureNotes23.pdf` (200, wrong lecture, not
used); `https://en.wikipedia.org/wiki/Tate_pairing` (200, stub). Three web searches (2-descent,
GGMP, Fong et al.); results used only for attribution.

Repository paths read: `crypto-autoresearcher/knowledge/findings/{KN-FIND-14706d,KN-FIND-007}.md`;
`crypto-autoresearcher/ledger/evidence/EV-SEMBIN-0c8bf4.yaml`; `…/ledger/hypotheses/H-SEMBIN-83a856.yaml`;
`crypto/src/cryptanalysis/{orbit_pair_table,native_signed_mitm,ic_boundary,koblitz_index_calculus,koblitz_factor_base_search}.rs`
(parts); `…/ic_framework/rootfind.rs`; `src/bin/ic/workflow.rs` (part);
`docs/ic/{LEADERBOARD.md,boundary_targets.json,leaderboard.json}`; `docs/curves/{registry.json,ICV1.md}`;
`research/notes/index-calculus/{RESEARCH_IC_BOUNDARY_LEDGER,RESEARCH_FACTOR_BASE_SHAPE_SEARCH,RESEARCH_FACTOR_BASE_SOLVE_COST}.md`;
`research/notes/ecc2k130/{RESEARCH_ECC2K130_DECOMPOSITION,RESEARCH_ECC2K130_DESCENT_DEGREE}.md`;
`research/{ic_leads_review_20261018,fall_degree_bounds_20261008,ic_tree_split_cutout_20260930,isogeny_fibers_20261010}/…`;
`research/ic_tool_program/rule/v3/README.md`; `LEADS.md`, `BRIEF_SONNET.md`.

Unverified desk checks (scratchpad, not committed, not measurements):
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/l04/{bin_check,odd_check,m3_check}.py`.
