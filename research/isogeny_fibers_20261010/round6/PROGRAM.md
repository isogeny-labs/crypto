# Rounds 5–6: the exhaustive experimental program for the isogeny-fibers leads

Preregistration of every experiment that the §16 leads, their round-4
measurements (§17 of `RESEARCH_ISOGENY_FIBERS_ECDLP.md`) and the round-5 leads
still leave open. Each design states its question, hypothesis, falsifier, arms,
unit of cost, instance sizes and budget, dependencies and deliverables, before
the run. Results land in `round5/<lead>/` and `round6/<lead>/` with the usual
`RESULTS.md`, raw records, graph and PDF, and are summarised in the note.

Scope, as in every round: toy and synthetic instances; native Rust for every
result (Python only plots recorded numbers); ledger slugs or standard names;
no sub-rho claim and no claim about a deployed curve. A result "changes the
map" only if it moves a row of §15.4 (an exponent, or a constant that the row
bounds); everything else moves a constant inside a row and is reported as such.

## 0. Where every lead stands

| Lead | Round 4 verdict (§17) | Still open |
|:--|:--|:--|
| 16.1 crossover in m | law confirmed; kernel saturation buys feasibility only against the direct x-frame S4 | direct S5 (both frames); `ic` plug-in for the chained x-frame descent; n = 23, m = 4 exceeds the 64-unknown cap; panel vs `ic bench` hit rate 0.04 vs 0.22 at n = 23, ℓ = 12 unexplained |
| 16.2 fast 4E check | identity confirmed, 0 errors; prior art; now `BinaryCurve::is_in_4e` (#1697) | the trace pair also separated cosets 1 and 3, with the labelling swapped between n = 131 and n = 233 (unproved); no identity certificate; not constant-time |
| 16.3 vertical margin | margin only for generic kernels; Prop. 3.3 corrected | √élu exponent 0.82 from Karatsuba (R5.3); ψ_ℓ factoring and Φ_ℓ(j, Y) evaluation untimed; ascending edges unmeasured; generic kernel only to ℓ = 433 |
| 16.4 ζ-fold over F_{p²} | fold exact; the \|M\|^{m−2} penalty is oracle-dependent | characteristic 2 (R5.1); a repository m = 3 oracle instead of the sweep; inversions unpriced; full-rank streams on 3-torsion curves |
| 16.5 class-wide rho | confirmed; transported batch = single-curve L-target rho | ecbench method with replay certificate; L > 64; ℓ ≥ 5 edges; path-finding cost; negation map |
| 16.6 genus 2 | law confirmed; saturated bases need degree-2 divisors | Gaudry/Nagao oracle on `genus2_fp` (#1696); explicit Richelot image arm; the 1/\|F\| A-pair correction applied back to the §15.2 census |
| 16.7 AGM + oracles | "adds nothing" to CJ-DLP; "constants only" false over composite binary fields | Conjecture 10 (R5.4); Proposition 5, whether a transport oracle on factor-base points helps beyond CJ-DLP; literature check |

## 1. Round 5 (in flight)

* **R5.1 Quotient-coordinate oracle in characteristic 2.** Does an oracle on
  E/⟨T⟩ remove the 2^{m−2} saturation penalty on Koblitz curves, as it removed
  the 3^{m−2} penalty on j = 0 curves over F_{p²}? Exact census plus the mitm
  pair table at n ∈ {11, 13, 15, 17}, m ∈ {2, 3}; arms plain / saturated-on-F /
  saturated-on-F/M / image of index 2; re-run the §7.A1-R 2.95× point.
  Falsifier: the quotient arm still pays ≈ 2^{m−2}.
* **R5.2 GHS across an isogeny class.** Over F_{2^N}, N composite, the
  distribution of the GHS magic number within each isogeny class, the weak
  fraction under a stated cost model, and the isogeny distance from a random
  curve to the nearest weak one. Falsifier: weak curves absent from mixed
  classes or unreachable at toy sizes.
* **R5.3 NTT √élu.** Multi-modular NTT in `isogeny_eval::poly`; re-fit the √élu
  exponent (0.82 with Karatsuba) and recompute the §17.3 crossover table.
  Falsifier: the exponent does not drop toward ½ + o(1), or the rational-kernel
  conclusion changes.
* **R5.4 Theory.** Conjecture 10 over prime fields attack family by attack
  family; a refined Prop. 15.2′ whose cost factor depends on the oracle's
  enumeration domain. Predictions for R5.1.

## 2. Round 6 designs

Every design below is independent of the others unless its *Depends on* line
says otherwise. Costs are counted in the unit each row names; wall time is
recorded only as "contended" on the shared 4-core host.

### R6.1 Direct S5 systems and the `ic` plug-in for the chained x-frame descent

*Question.* 16.1 compared chained systems at m = 4 because the library has no
direct S5 in characteristic 2, and compared the symmetrised arm against the
weakest plain arm because the chained x-frame descent (`groebner_decompose`)
has no `ic` oracle plug-in. Does a direct S5 change the m = 4 picture, and does
the plug-in change the crossover verdict?
*Hypothesis.* The direct symmetrised S5 (A(w) + s·B(w), 126 + 56 orbit
coefficients, s² = Σw + s) is feasible wherever the chained symmetrised system
is and cheaper per relation; it still never beats a feasible m = 2 arm at the
same n. The panel vs `ic bench` hit-rate gap is a budget-accounting difference.
*Falsifier.* A direct-S5 arm with total cost S below the best m = 2 arm at the
same n.
*Arms.* (a) chained x-frame m = 3, 4 via the new plug-in; (b) direct x-frame
S5 by interpolation; (c) direct symmetrised S5; (d) the round-4 arms as
controls. n ∈ {13, 15, 17, 19, 23}, m ∈ {3, 4}; a = 0 and a = 1.
*Metrics.* S = GAE/√r from `ic bench`; relations per solve; node budget hit
rate; unknown count against the 64 cap; interpolation cost of each S5 (one-time).
Every panel verdict gated against exact enumeration (0 gate failures required).
*Instances and budget.* S5 interpolation at n ≤ 23 (orbit-coefficient count
is field-size independent); per cell ≤ 15 min; ≤ 24 cells.
*Discrepancy protocol.* Instrument both drivers to log, per target, the node
budget consumed and the reason a trial ended; the hit rates must agree once
budgets are matched, or the difference is a bug to fix before any other number
is reported.
*Deliverables.* `src/cryptanalysis/ic_framework/plugins.rs` gains the chained
x-frame oracle (library change, own PR); `examples/r6_1_*.rs`; the 16.1 grid
re-run with (a)–(c) added.

### R6.2 Pooled multi-target rho in ecbench and the class transport cost

*Question.* 16.5 measured the class-wide discount with the example's own rho.
Does it hold in the harness that the repository's claims are charged in, with
a replay certificate, larger L, odd-degree edges, the negation map and the
cost of finding the path?
*Hypothesis.* The transported batch costs B/C ∈ [0.97, 1.03] of single-curve
L-target rho for L ≤ 256; path finding between two curves of a toy class costs
O(√(class size)) Φ_ℓ-root evaluations and is instance-independent.
*Falsifier.* B/C outside [0.95, 1.05] with the 95% interval excluding 1 at any
L; or a path-finding cost that scales with r.
*Arms.* ecbench methods `rho` (single target, with negation map) and a new
`rho_multitarget_pooled` (shared distinguished-point table); class built by
BFS over rational ℓ ∈ {2, 3, 5, 7} kernels with certified duals (now in
`isogeny_eval::small_degree` for 2, 3; extend to 5, 7 with Vélu); transport via
`isogeny_eval`. L ∈ {1, 4, 16, 64, 256}; r ∈ {2^28, 2^32, 2^36}; ≥ 20 seeds.
*Metrics.* ecbench's counted unit (group operations, with the field-op split
where the curve counts it); transport cost in field ops; path length and
Φ_ℓ-root evaluations for path finding; every recovered log verified on its
original curve; replay certificate per session.
*Deliverables.* ecbench method + spec + session (own library PR, per
`ecbench-extend`); `round6/r6_2_*/RESULTS.md`.

### R6.3 The trace pair as a coset label, and a certificate for the 4E identity

*Question.* On y² + xy = x³ + 1 over F_{2^n}, n odd, round 4 observed that the
pair (Tr(x), Tr(y + x·H(x))) is constant on each of the four cosets of 4E, with
the labels of the two odd cosets swapped between n = 131 and n = 233. Is the
pair a well-defined labelling E/4E → F_2², and what decides the swap?
*Hypothesis.* The first bit is the homomorphism E → E/2E. The second bit is
constant on each coset of 4E, is the homomorphism 2E → 2E/4E on 2E, and on the
odd cosets its value is a fixed function of n mod 8 (so the labelling is a
set map, not a homomorphism, since Z/4 has no injective map to F_2²
respecting addition).
*Falsifier.* Any odd n ≤ 25 with a coset on which the pair is not constant;
or an odd-coset labelling that is not periodic in n mod 8 (or mod 4, or
constant) across the exhaustive range.
*Arms.* Exhaustive enumeration of E(F_{2^n}) for every odd n from 5 to 25,
cosets read from [#E/4]P and the order-4 point (1, 0); tabulate the label per
coset; then the same on sampled points at n ∈ {131, 233, 239, 283, 409, 571}.
*Then prove it* from the halving formulas (λ² + λ = x + a for the half, the
trace of 1 for odd n, and the action of adding the order-4 point on the two
traces), and write the result up as a lemma that extends Prop. 14.2 to a
full E/4E label.
*Also.* Emit an `identity.certificate/v1` record for the 4E identity (the
repository's certificate schema, see `src/cryptanalysis/identity_certificate.rs`)
covering the exhaustive range, and a constant-time variant of `is_in_4e`
(branch-free trace and half-trace) benchmarked against the table version.
*Cost.* Exhaustive to n = 25 is ≈ 3·10⁷ points; minutes.
*Deliverables.* `examples/r6_3_*.rs`; the certificate JSON; a lemma in
`RESULTS.md`; constant-time variant as a library PR if it costs < 2× the table
version.

### R6.4 Vertical margin, phase two

*Depends on* R5.3 (NTT multiplication).
*Question.* 16.3 timed only rational-kernel evaluation and generic kernel
finding by an F_{p^k} ladder, on descending edges, with ℓ ≤ 433 for the
generic case. What do ψ_ℓ factoring, Sutherland-style evaluation of
Φ_ℓ(j, Y), and ascending edges cost, and does NTT move the generic exponent?
*Hypothesis.* Generic kernel finding stays ≥ ℓ^2 (the field-degree term
dominates) so the generic crossover table of §17.3 moves by less than one bit
of ℓ; ascending edges cost the same as descending ones at equal ℓ.
*Falsifier.* A generic-kernel exponent below 2 with NTT, or an ascending edge
measurably cheaper or dearer than a descending one at the same ℓ (95% intervals
disjoint).
*Arms.* (a) F_{p^k} ladder with NTT products; (b) ψ_ℓ via division polynomial
recurrences and factoring (Cantor–Zassenhaus) for ℓ ≤ 200; (c) Φ_ℓ(j, Y) root
finding with the library's modular polynomials for ℓ ≤ 79 and a Sutherland-style
CRT evaluation if feasible for ℓ ≤ 200; descending and ascending edges on the
j = −3375 (D = −7) instances of 16.3 at 61-bit p.
*Metrics.* F_p multiplications (NTT work converted at the ratio R5.3 states),
log-log exponent fits with intervals; the §17.3 crossover table recomputed.
*Deliverables.* `examples/r6_4_*.rs`, `round6/r6_4_*/RESULTS.md`, the §17.3
table superseded in the note.

### R6.5 The finite-size A-pair correction applied back to the §15.2 census

*Question.* 16.6 derived a 1/|F| correction 1 + (h−1)(|A|−2)/(2|F|) to the
A-closed class factor when A fixes a cofactor class, and noted it applies to
the j = 0 elliptic census (+5.6% predicted). Does it explain the measured
ζ-closed m = 2 class factors in `round3/census_j0_*.json` (1.02 measured
against 1 predicted)?
*Hypothesis.* The recorded ζ-closed c(m = 2) values lie within exact-count
error of 1 + (h−1)(|A|−2)/(2|F|) with |A| = 3 (or 6 with negation), h = 3,
|F| = 72, and the residual after correction is within ±1%.
*Falsifier.* Residual after correction larger than before, or sign-wrong.
*Arms.* Re-analysis in Rust of the recorded census JSON (no rerun); then one
confirmatory exact census at p ≈ 2^18 with |F| ∈ {36, 72, 144} on the same
curve to show the 1/|F| scaling directly.
*Cost.* Minutes.
*Deliverables.* `examples/r6_5_*.rs`, corrected table for §15.2 in
`RESULTS.md`, erratum note in the research note.

### R6.6 Genus 2: an algebraic oracle and an explicit Richelot image arm

*Depends on* `genus2_fp` (#1696, merged).
*Question.* 16.6 proved a degree-1 divisor base can never be translation
closed, so an algebraic (Gaudry/Nagao) oracle cannot use a saturated base; the
image arm was the i = 16 subgroup, not an isogeny image. Does the law hold for
an algebraic oracle in genus 2, and for a genuine (2,2)-isogeny image?
*Hypothesis.* With a Nagao-style decomposition oracle (degree-1 base,
x-coordinates in a subset, decomposition by solving the degree-g system),
A-closed bases gain |A| = 10 in unknowns at unchanged relation yield; the
Richelot image base of index 4 gains ×4 in class factor; saturation is
unavailable to the oracle (not merely penalised).
*Falsifier.* A measured A-gain outside [8, 12] or an image gain outside
[3, 5] (bootstrap 95%), or a usable saturated degree-1 base.
*Arms.* Plain / A-closed / Richelot-image degree-1 bases on y² = x⁵ + b over
F_p, p ∈ {1051, 2131, 4111}; the oracle = exact enumeration of m = 2, 3
decompositions restricted to degree-1 divisors (the algebraic oracle's reach)
plus the Nagao system solved by resultants for m = 2 as a timing control;
Richelot built from the three quadratic factors of f when they are rational.
*Metrics.* Unknowns; relations per target; class factor; derived cost; exact
counts with intervals.
*Deliverables.* `examples/r6_6_*.rs` (Richelot in the example first, library
if it fits), `round6/r6_6_*/RESULTS.md`.

### R6.7 ζ-fold over F_{p²}, phase two

*Depends on* R5.1 (quotient-oracle results in characteristic 2, for comparison).
*Question.* 16.4 measured the T-saturated m = 3 arms with its own sweep oracle,
left inversions unpriced and saw the 3-torsion curves' full-rank streams stall.
Does a repository oracle (`LineOracle` family) reproduce the oracle-dependent
penalty, and what are the arms' costs with inversions priced?
*Hypothesis.* Priced inversions change every arm's cost by the same factor
(the law's ratios are unchanged); the repository line oracle on F/⟨T⟩ pays
≈ 1 and on F pays ≈ 3 at m = 3; full-rank streams reach rank with a 4× budget
on the 3-torsion curves.
*Falsifier.* Ratios that move with inversion pricing by more than the
bootstrap interval; the repository oracle on F/⟨T⟩ paying > 1.5.
*Arms and metrics.* As in 16.4 with inversions counted at the measured I/M
ratio of the field; 12 curves per family; 1000 targets.
*Deliverables.* `examples/r6_7_*.rs`, `round6/r6_7_*/RESULTS.md`.

### R6.8 Transport oracles on factor-base points (Proposition 5 of 16.7)

*Question.* Theorem 1 of 16.7 simulated isogeny oracles whose inputs carry
algebraic representations. Proposition 5 left open whether an oracle that
evaluates φ on *arbitrary* points, factor-base points in particular, helps
index calculus beyond the CJ-DLP reduction.
*Hypothesis (theory).* A transport oracle lets the attacker use the union of
pulled-back factor bases ⋃ φ̂_j(F_j); by Prop. 3.2 and Thm. 4.2 pullbacks only
grow, so the union base is the image/pullback law of §14 with constant ≤ the
number of oracles; no exponent moves. Precisely: cost ratio ≥ 1/|Ψ| with the
affine law's factors, and a transport oracle on F is equivalent to enlarging
F by the pulled-back bases.
*Falsifier (measurement).* On a toy class with 2- and 3-isogeny oracles, a
union-of-pullbacks base whose class factor per unknown exceeds that of the
best single base by more than |Ψ| (exact census, m = 2, 3).
*Arms.* Single base on E; pulled-back bases φ̂_j(F_j) for each neighbour; the
union; on j = 0 toy curves of 15.2 and a prime-order toy curve with a 2- and a
3-isogenous neighbour (via `isogeny_eval::small_degree`).
*Metrics.* Unknowns, relations per target, class factor, derived cost.
*Deliverables.* a proof in `RESULTS.md` (Proposition 5′) and
`examples/r6_8_*.rs`.

### R6.9 GHS phase two

*Depends on* R5.2.
*Question.* Whichever of R5.2's outcomes holds, the next step is the same:
for the smallest composite N where a mixed class exists, walk from a random
strong curve to the nearest weak one with the library's binary isogenies and
time the walk; and compare the heuristic genus-g cost model against a measured
hyperelliptic index-calculus relation rate on the descended Jacobian at the
smallest weak instance (the `prime_hyperelliptic` / `jv_cover` code, over
F_{2^l} if the library has it, else state that it does not).
*Hypothesis.* The walk costs ≤ 2·(class size)^{1/2} isogeny evaluations;
the measured relation rate at the smallest weak instance is within 4× of the
model.
*Falsifier.* Walk cost growing with N beyond the class size, or a relation rate
off the model by > 10×.
*Deliverables.* `examples/r6_9_*.rs`, `round6/r6_9_*/RESULTS.md`; a stated
limit if binary-field hyperelliptic arithmetic is absent.

### R6.10 Literature closure

*Question.* Rounds 4–5 cited several results as "recalled". Which of the
programme's claims are already in the literature, and where do they differ?
*Method.* Web search (the only lead that uses it), one query list per claim:
the 4E trace test (King 2004; Eagle–Galbraith–Ong 2009; US 7,257,709);
AGM with oracles (Fuchsbauer–Kiltz–Loss 2018 and follow-ups; any AGM-with-
isogeny-oracle work); isogeny-class invariance of ECDLP exponents (Jao–
Miller–Venkatesan 2005; Galbraith–Hess–Smart 2002 on GHS via isogenies;
Menezes–Teske–Weng); the image/pullback law for factor bases (Faugère–Huot–
Joux–Renault–Vitse 2014; Galbraith–Gebregiyorgis 2014); multi-target rho
across a class (Kuhn–Struik 2001; Fouque–Joux–Mavromati 2014); √élu
(Bernstein–De Feo–Leroux–Smith 2020); genus-2 decomposition (Gaudry 2000,
Nagao 2010).
*Deliverables.* `round6/r6_10_literature/RESULTS.md` with every "recalled"
citation confirmed, corrected or withdrawn, and the note's §18 references
updated; no code.

### R6.11 Conductor-dependence of relation yield (the direct test of Conjecture 10′)

*Depends on* R5.4's Theorem 4: Conjecture 10 can fail only across a prime
conductor gap ℓ ≥ p^ε, through an algorithm whose cost depends on End(E)
beyond O^× and E(F_p)[h].
*Question.* Is the index-calculus relation yield, at matched base size and
oracle, the same on the floor and the crater of an ℓ-volcano with ℓ a large
prime, i.e. is the class factor a function of (p, t) only?
*Hypothesis.* Yes: measured class factors on the two levels agree within exact-
count error at m = 2 and m = 3, for ℓ ∈ {5, 7, 11, 13} volcanoes over toy p.
*Falsifier.* Class factors differing between levels by more than the exact
interval on two independent curves of the same class, at either m.
*Arms.* For each toy prime p ≈ 2^20–2^22 and ℓ ∈ {5, 7, 11, 13}: a crater
curve and a floor curve of the same class (constructed by CM at discriminant
−D and −Dℓ², or found by Φ_ℓ walks); plain x-frame bases of equal size; exact
census of m = 2, 3 decompositions over all targets in G.
*Metrics.* Class factor c and relations per unknown, per level; the ratio
crater/floor with its interval; j-invariants and conductors recorded.
*Deliverables.* `examples/r6_11_*.rs`, `round6/r6_11_*/RESULTS.md`; the first
measured row for "vertical moves across a large-prime gap" in §15.4.

## 3. Waves and resources

| Wave | Leads | Why together |
|:--|:--|:--|
| 1 (now) | R6.3, R6.5, R6.8, R6.10 | minutes of compute each; no dependencies |
| 2 (as R5 finishes) | R6.1, R6.2, R6.6, R6.11 | heavier; each needs ≥ 1 core for an hour |
| 3 (after R5.2, R5.3) | R6.4, R6.7, R6.9 | depend on round-5 results |

Shared target directory for the wave, `CARGO_BUILD_JOBS=1` per agent, runs
under 15 minutes, counted operations over wall time.

## 4. Decision rules

* A §15.4 row moves only on a measured exponent change, or on a constant
  outside the bound the row states, confirmed on two independent instances
  with disjoint 95% intervals. Everything else is reported as a constant inside
  the row.
* A falsifier that fires is reported as such in the lead's `RESULTS.md` and in
  the note before any explanation is offered.
* A run that cannot complete at the planned size reports the largest size that
  did complete and the reason, never a smaller claim dressed as the planned one.
