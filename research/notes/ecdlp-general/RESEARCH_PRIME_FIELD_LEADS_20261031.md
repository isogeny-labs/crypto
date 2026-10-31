# Prime-field ECDLP after the representation question: what is closed, what the arithmetic says, and the eight leads worth one more round

**Date.** 2026-10-31. **Status.** Position note plus a pre-registered plan.
No measurement in this note; every number below is quoted from the cited
record. The plan's results land in
[`research/prime_field_leads_20261031/`](../../prime_field_leads_20261031/PLAN.md)
(one subdirectory per lead, each with its own report, graph and PDF per the
`research-visuals` skill).
**Provenance.** Owner questions of 2026-10-31: (1) can a different model of
the standardised prime-field curves (NIST P-curves, Curve25519) give an
end-to-end index-calculus speedup through factor-base symmetry or relation
yield, with a high-yield base that minimises the degree of regularity; (2)
given that the answer is no, what is the next best lead, and are there any
unconventional ideas left.
**Coordination.** Conductor returned 404 on `conductor_check_conflicts` from
this session; no task card could be claimed. Scopes touched: this file,
`research/prime_field_leads_20261031/`, and a section appended to
[`RESEARCH_ECDLP_STATE_OF_THE_ART.md`](RESEARCH_ECDLP_STATE_OF_THE_ART.md)
by lead L3.
**Companions.**
[`../index-calculus/RESEARCH_EXOTIC_COORDINATES.md`](../index-calculus/RESEARCH_EXOTIC_COORDINATES.md)
(coordinates, symmetries, isogeny argument),
[`../index-calculus/RESEARCH_RHO_PARITY_PROGRAMME.md`](../index-calculus/RESEARCH_RHO_PARITY_PROGRAMME.md)
(4-torsion lever retracted),
[`RESEARCH_REPRESENTATION_STRUCTURE.md`](RESEARCH_REPRESENTATION_STRUCTURE.md)
(Shoup framing), [`RESEARCH_LATTICE_ECDLP_ROUTES.md`](RESEARCH_LATTICE_ECDLP_ROUTES.md),
[`../cm-isogeny/RESEARCH_EDS_RESIDUE.md`](../cm-isogeny/RESEARCH_EDS_RESIDUE.md),
[`../cm-isogeny/RESEARCH_P256.md`](../cm-isogeny/RESEARCH_P256.md) (canonical-lift
falsification), `/Volumes/SSD990-2/research/ECDLP_MECHANISM_CATALOG.md`
(111 mechanisms, outside the repository),
`/Volumes/SSD990-2/prime-sat-tensor-review-20261031/REVIEW.md` (today's
review of the Boolean-digit prime-field thread, outside the repository).

---

## 1. The representation question, answered from the record

A curve model is a birational change of frame. Three facts, all already in
`RESEARCH_EXOTIC_COORDINATES.md` and attributed there to
Faugère–Huot–Joux–Renault–Vitse 2014, bound what any model can buy:

1. **Every degree-2 coordinate is a Möbius frame on the x-line** (§1.1, §15).
   Edwards, Montgomery, Jacobi and Hessian coordinates change how the
   summation polynomial is written, not which tuples sum to zero.
2. **A coordinate invariant under translation by a torsion subgroup H
   factors through the isogeny E → E/H** (§1.2). A torsion-symmetric factor
   base on E is a factor base on E/H, which carries the same ℓ-subgroup at the
   same cost. Curve25519's rational Z/8 therefore buys nothing that attacking
   the prime-order quotient curve does not already give.
3. **Only rational 2-torsion translations descend to the x-line as pairwise
   symmetries** (§1.3). Those halve the per-point degree of the symmetrised
   summation polynomial (the FGHR effect). Automorphisms and Frobenius act on
   all points at once: they fold factor-base columns and do not lower degree.
   The rational 4-torsion point of Ed25519 swaps the Edwards coordinates and
   halves no degree; `RESEARCH_RHO_PARITY_PROGRAMME.md` §8 registered that
   lever and retracted it after measurement.

| Curve | Rational torsion | Aut(E) | Pairwise symmetry | Column fold |
|---|---|---|---|---|
| P-256, P-384, P-521 | none (prime order) | ±1 | none | 2 |
| secp256k1 | none | order 6 (j = 0) | none | 6, in use (`ec_index_calculus_j0`) |
| Curve25519 / Ed25519 | Z/8 | ±1 | one involution (T₂) | 4 (sign frame; S_{m+1} degree 2^{m−2}) |

**Degree of regularity over F_p.** The decomposition system is S_{m+1} plus a
membership constraint L(x_i) = 0. Over F_p there is no subfield, so a set of
p^{1/m} abscissae has vanishing polynomial of degree p^{1/m} (Bézout). The
only escape is Petit–Kosters–Messeng's composition of low-degree maps (the
PKM tower); their own beat-rho condition is a tower solve below
p^{1/2−1/m}, and the repository's runs give linear degree growth
(m = 4 breaks the plateau at about one degree per eight unknowns; m = 3,
N = 21 died at 99.6 GiB inside degree 8,
`research/pkm_m3_n21_msolve_20261018/RESULTS.md`). Symmetrisation does not
change the slope (`research/ic_symmetry_lever_slope_20260929/RESULTS.md`:
refutation degree rises 1.03 per unit ℓ against a 0.35 pass threshold). The
leading-form d_reg is presentation-dependent in any case: a one-hot
re-encoding drives it to G + 1 at constant information (today's review,
§2.1), which is exactly what a model change does to the printed number.

**Yield.** For a base of size |F| with no additive structure in the group, the
decomposition probability is about |F|^m / (m! · N); R8 measured a 1.04x
spread across coset, interval and random bases. Interval bases in any model
are group-unstructured, so they have the same yield; symmetry does not raise
it, it divides the unknowns by |G|, which is the column fold above.

**Verdict.** No end-to-end gain from representation; at most a constant
factor of 2 to 4 on the linear algebra of a method whose relation phase is
already exponentially behind rho. Recorded, not recommended (same wording as
`RESEARCH_EXOTIC_COORDINATES.md` §4.1).

## 2. The arithmetic that frames every remaining lead

Interval base of size p^{1/m}, arity m, s = log₂(p)/m bits per window.

- Relations needed about p^{1/m}; success per random target about 1/m!.
- Meet-in-the-middle relation collection: p^{1/2 + 1/m} total, memory p^{1/2}.
- If one decomposition attempt costs p^{e/m}, the relation phase costs
  m! · p^{(1+e)/m}; it beats rho iff **e < m/2 − 1**. Enumeration gives
  e = m − 1; MITM gives e = m/2. So m = 3 needs e < 0.5, m = 4 needs e < 1.
- An algebraic decomposition matters only if it beats MITM on an interval
  constraint. For integer knapsacks the only sub-MITM tools (Wagner's k-list,
  Howgrave-Graham–Joux and Becker–Coron–Joux representations) need a
  homomorphic filter modulo M. A prime-order group has no quotients. That is
  Shoup's bound restated, and it is why the x-coordinate-bits filter
  (R9-v3's telescoping gap) could not close.

Nothing in the catalogue's C1 to C8 changes this arithmetic; those cells
confirm the wall at the true solve object. They are worth finishing with
stop rules, and their expected outcome is negative.

## 3. Leads, pre-registered

Each lead names its deliverable, its falsifier and its stop rule. Metrics are
counted operations, degrees and ranks, never wall time: the host carried a
load average near 90 on 14 cores at planning time. Result generation obeys
AGENTS.md "Implementation language: no Python" (Rust harnesses, msolve and
`sage-run` as tools; Python only plots recorded values).

### L1. Conditional lower bound for algebraic index calculus over F_p (the positive deliverable)

- **Model.** A factor base cut out by a polynomial in one coordinate (or a
  composition of maps, PKM style); a decomposition step that is any
  polynomial-system solver charged at least the Macaulay rank at its solving
  degree; linear algebra charged at least the number of columns.
- **Claim to prove.** Within the model, total cost ≥ p^{1/2 − o(1)} for every
  m. Ingredients: Bézout (membership degree ≥ |F|/2 for a one-coordinate
  base), Yokoyama–Yasuda's subresultant bound on the PKM tower, the MITM
  floor of §2 as the comparison, and Jao–Miller–Venkatesan isogeny-class
  uniformity to fold the CM-panel ideas into the generic statement.
- **Deliverable.** `RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md` with each lemma
  either proved or marked as a gap with the exact missing statement.
- **Falsifier.** A decomposition algorithm inside the model with cost below
  p^{1/2}, or a lemma whose hypothesis excludes a measured repository route.
- **Stop rule.** Theorem text with every gap named; no experiment.

### L2a. Slope e of counted decomposition work at m = 3

- **Design.** Interval base, arity 3, s = 6..12 (p from 2^18 to 2^36), fresh
  primes per s, 3 curves per prime, 32 targets per curve, satisfiable and
  unsatisfiable cells both counted. Arms: the repository's prime-field
  SAT/SMT decomposition (`src/cryptanalysis/semaev_sat.rs`,
  `prime_field_smt.rs`, `semaev_decomp.rs`, `prime_small_x.rs`, whichever
  builds and runs) against the enumeration / MITM reference.
- **Metric.** Counted field multiplications plus solver decisions per
  attempt; fitted slope e of log₂(work) against s with a bootstrap interval.
- **Falsifier / decision.** Interval entirely below e = 0.5 reopens the cost
  table; interval containing e = 2 (enumeration at m = 3) closes the lane for
  this encoding. Null prediction: e ≥ 1.
- **Stop rule.** No cell beyond 20 minutes; report every cell that ran,
  including the ones that did not finish.

### L2b. Catalogue M008: d_reg trajectory of the constrained prime-field Semaev system

- **Design.** S_4(x₁, x₂, x₃, x(R)) plus the membership polynomials of an
  explicit interval base V, |V| = p^{1/3}, p from 2^10 to 2^20 (|V| from 10
  to 100), msolve as the solver, generators in Rust
  (`examples/dreg_ladder.rs`, `examples/ic_next10_03_pkm_degree.rs`,
  `examples/pkm_tower_pilot.rs` as starting points). Control: a random
  system of identical variable count, degrees and equation count with
  artificial membership equations of the same degree.
- **Metric.** Maximal step degree reached (d_reg proxy), first fall degree,
  Macaulay matrix size at the final step, against log₂ p at fixed m = 3.
- **Falsifier.** A d_reg trajectory that stays flat as |V| grows while the
  random control grows. Null prediction: both grow alike.
- **Stop rule.** No cell beyond 30 minutes or 16 GiB; every attempted cell
  reported.

### L3. Precomputation and multi-target attacks on fixed curves (write-up)

- **Content.** Bernstein–Lange 2012 (non-uniform discrete logs: online cost
  about ℓ^{1/3} after a per-curve table of about ℓ^{2/3}), Kuhn–Struik 2001
  (k logarithms for about √(kℓ)), and the Corrigan-Gibbs–Kogan 2018 generic
  preprocessing lower bound (S · T² ≳ ℓ). Numbers for P-256, Curve25519 and
  ECCp-131 as a table. This is the only known sub-rho online cost on a
  standardised curve; it is not a break and is absent from the
  state-of-the-art note (only Kuhn–Struik appears, in the ECC2K-130 panel).
- **Deliverable.** A section appended to `RESEARCH_ECDLP_STATE_OF_THE_ART.md`
  and a table in the evidence directory.
- **Stop rule.** Write-up only.

### C1. Why lifting fails: the torsion copy of E(F_p) in every lift

- **Claim to verify, then write.** For any lift Ẽ/Z_p of an ordinary E/F_p
  and N = #E(F_p) prime to p, the sequence
  0 → Ẽ₁(Z_p) → Ẽ(Z_p) → E(F_p) → 0 splits because Ẽ₁(Z_p) ≅ Z_p is
  torsion-free and N is a unit in Z_p; so Ẽ(Z_p)_tors is a canonical
  homomorphic copy of E(F_p), on every lift, and the Serre–Tate canonical
  lift adds only CM and algebraicity. Every p-adic analytic homomorphism to
  (Q_p, +) (formal logarithm, Coleman integral, p-adic height) vanishes on
  torsion, and the complex route needs the N-torsion field of the CM lift,
  of degree about N. Smart's attack is the one case (N = p) where the
  sequence does not split.
- **Deliverable.** A short section for the L1 note, cross-checked against the
  `cm_canonical_lift` falsification in `RESEARCH_P256.md`.
- **Falsifier.** An error in the splitting argument, or a p-adic invariant
  that is non-trivial on torsion.

### C2. One volcano experiment to retire the CM panel

- **Design.** One isogeny class at toy size (p around 2^16 to 2^20) with a
  volcano of depth ≥ 2 and a usable prime-order subgroup shared by crater and
  floor; the constrained S_4 decomposition at m = 3 with matched interval
  bases on the crater curve and on a floor curve
  (`src/cryptanalysis/isogeny_class_search.rs` for the class, the L2b
  generator for the system).
- **Prediction.** Endomorphisms act globally on relations
  (`RESEARCH_EXOTIC_COORDINATES.md` §1.3), so counted work and degrees are
  identical within the curve-to-curve spread.
- **Falsifier.** A systematic gap above 2x in counted work or one full degree
  in the solving degree between crater and floor at equal ℓ.
- **Consequence if the prediction holds.** Catalogue mechanisms M010, M017,
  M027, M039, M083 and M089 close together.

### C3. The self-reducibility ceiling on every heavy-tail mechanism

- **Argument.** Randomising the target (R ↦ R + rP) is one addition, so an
  easy-target fraction f buys at most 1/f on the decomposition phase and a
  sieve already prices it. Score every heavy-tail mechanism in the catalogue
  (M012, M014, M023, M044, M054, M067, M077 and any other that conditions on
  the target) against this ceiling: what f would be needed to matter at
  m = 3 and m = 4 given §2.
- **Deliverable.** A table and recommended closures. Write-up only.

### C4. The interval predicate is pseudo-generic (conjecture, plus a toy measurement)

- **Statement to formalise.** The additive energy of
  F_B = {P ∈ E(F_p) : x(P) < B} equals that of a uniformly random subset of
  the same size up to a factor 1 + o(1), uniformly in E; R8's 1.04x spread
  is the empirical form. Nearest literature: Shparlinski and coauthors on
  points of elliptic curves in boxes and on sums of x-coordinates.
- **Toy measurement.** In Rust, for p from 2^12 to 2^18 and |F_B| = p^{1/2}
  and p^{1/3}, count the number of solutions of P₁ + P₂ = P₃ + P₄ in F_B
  against the same count for random subsets of equal size, 8 curves per p,
  reported with the binomial spread.
- **Falsifier.** An energy ratio that departs from 1 by more than the
  sampling spread and grows with p.
- **Consequence if it holds.** Every sieve-style relation shortcut on the
  interval base is closed at the level of the predicate, and L1 gains its
  missing yield lemma.

### C5. Special prime shapes (closure note)

- **Content.** Why 2^255 − 19 and the Solinas primes touch field arithmetic
  only: doubling does not preserve "x has small high bits", multiplication
  by 2 on x is not a curve map, and no base defined by digit patterns is
  closed under any group operation. Listed so it is not reopened.
- **Deliverable.** A short section in the evidence directory. Write-up only.

## 4. Execution

The leads run as one workflow of model-tiered subagents (owner request,
2026-10-31: Sonnet for L1, L2a, L2b, C2, C4 and the synthesis; Haiku for L3,
C1, C3 and C5). Each writes only inside its own subdirectory of
`research/prime_field_leads_20261031/` (L1 and L3 also write their named
note sections). Each leaves a report, a graph and a PDF per the
`research-visuals` skill; write-ups with fewer than two comparable values
leave a diagram of what was derived instead, as the skill allows. The
synthesis agent folds the outcomes into §5 of this note and refuses any
sentence that claims a speedup not admitted by a counted measurement.

## 5. Outcomes

Pending; filled by the synthesis step of the workflow.

## 6. What this note does not claim

No new algorithm, no scoreboard change, no exponent change. The leads with
experiments (L2a, L2b, C2, C4) carry null predictions that confirm the wall;
their value is a measured exponent or a closed mechanism, in one unit, at a
named object. L1 is the deliverable that turns the record into a statement.
