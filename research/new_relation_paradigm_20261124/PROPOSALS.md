# New relation paradigm for ECDLP index calculus: three proposals (Idea Generator, 2026-11-24)

Status: **proposals only**. Nothing here is a measurement, a result, a status change or an
assignment. Every statement is `[pcc:unverified]`. Labels: *quoted* (path given, copied from
a committed file), *derived* (argument in this file, not run, not machine-checked),
*proposed* (an idea or prediction to be frozen by the Coordinator before any run).

## 0. Honest framing

- `tools/lessons.py brief` could not be run (this session had read, search and web tools only,
  no shell). Substitute: I read the three adopted `idea-generator` lessons directly:
  `docs/lessons/entries/L-dcb31297f97e.json` (PKM/tower is OPEN-leaning-KILL, not
  "measured-killed"; the open item is a degree-slope theorem, not a measurement),
  `L-f2ff07653111.json` (grep research/ and docs/ before saying "untried"; a miss is not
  proof of absence), `L-4c9e8e66ea68.json` (KN-FIND-007: mean yield is a base-geometry
  invariant; do not re-propose coverage levers). All three are applied below.
- research-kb and the lead atlas were unavailable. Every novelty statement is a grep miss or
  hit, listed in section 5, and is not proof of absence.
- **The repository is saturated on this question.** The prior desk notes already close, on
  paper or by measurement, almost every paradigm I could construct (section 4). The three
  proposals below are the doors those notes themselves leave open or unmeasured, each with a
  new mechanism statement. I put each at 1 to 3 percent. I would rather say that than pad.
- Exponent frame (quoted from `research/ecdlp_subexp_ideas_20261110/README.md` section 3):
  with a free `S_{m+1}` oracle, total cost is `p^{2/3}` at `m=2`, and at `m=3` it is `p^{1/2}`
  (tie) with `B^2` linear algebra but `p^{1/3}` with quasi-linear linear algebra. So an
  exponent change needs one of: (T1) a decomposition oracle at `m=3` with per-call cost
  `p^{delta}`, `delta <= 1/3`; (T3) a relation matrix that admits a sub-quadratic solve;
  or a non-generic information source (Question C, section 4 of that note).
  Meta-finding quoted from `research/hybrid_ic_directions_20261120/SYNTHESIS.md`: relation
  collection is 84 to 89 percent of an `ic logs` run, linear algebra 0.11 to 1.9 percent.
  That caps what (T3) can buy in constants, and is why (T3) is only an exponent lever.
- Track (SUCCESS_CRITERIA section 1): all three are generic-decomposition or
  non-generic-structure tracks. A T2 or T3 claim in the generic-decomposition track
  contradicts the `Omega(sqrt r)` bound and would be treated as an accounting error.

## 1. Proposal NRP-1: Frobenius-equivariant Fourier test of subspace-membership along `<P>` (binary Question C)

```yaml
id: NRP-1-binary-congruence-sieve-fourier
title: "Is there an F_2-affine-in-the-scalar structure on V-decomposability for Koblitz curves? (the binary form of Question C)"
obstruction_attacked: >
  Why IC on E has no sieve: factor-base membership is not a congruence condition on the
  scalar a in R = Q + aG (ecdlp_subexp_ideas_20261110 section 4, last row; Lemma A explains
  it for F_p, where cheap maps have degree below p/4). In F_{2^m} Lemma A's degree
  hypothesis fails for Frobenius-sparse (linearized) maps, so the prime-field closure does
  not transfer, and the binary analogue of the test has not been measured.
claim: >
  Let G generate the prime-order subgroup of a Koblitz curve E over F_{2^m}, V an F_2-subspace
  of F_{2^m} of dimension l, and I_V(k) = 1[x(kG) in V] for k in Z/r. Claim: for some
  (m, l) in the toy range, max over nonzero xi in Z/r of |Fhat_V(xi)| exceeds the size-matched
  random-set null (mean of the maximum over 400 random subsets of Z/r of size |V|/2 ... same
  cardinality as the observed support) by a factor that grows with m, and the maximising xi
  is reproducible across independent V of the same dimension through the Frobenius
  relation xi -> lambda * xi (lambda = eigenvalue of tau on <G>, order m).
mechanism: >
  Frobenius is F_2-linear on F_{2^m} and acts on <G> as multiplication by lambda, so
  I_{sigma V}(k) = I_V(lambda^{-1} k). The family of indicators is therefore one function
  seen through a cyclic group of m scalings, and membership in V is an F_2-LINEAR condition
  on the bits of x. A bias would have to be the binary analogue of "x(kG) is close to a
  character in k". Weil-type bounds predict none; the test measures it exactly at toy size
  instead of assuming it.
exponent_argument: >
  If |Fhat(xi)| >= c * |V| / sqrt(r) * (growth in m) at a computable xi, then the set
  {k : Q + kG in decomposition locus} has a nontrivial additive-character correlation, which
  gives a sieve over k with O(1) amortised work per candidate. Per the m=3 row of the
  ecdlp_subexp_ideas table, a sieve that makes the oracle cost p^{delta} with delta <= 1/3 and
  keeps B^2 linear algebra gives p^{1/2}; with structure in the matrix it gives p^{1/3}.
  A null leaves the exponent at 1/2 and closes Question C for binary fields.
prediction:
  expected_outcome: "null: ratio to the random-set null within [0.9, 1.15] at every (m, l)"
  probability_of_signal: 0.01
  toy_sizes: "m in {13, 17, 19, 23, 29}, all points enumerated; l in {ceil(m/3), ceil(m/2)}"
discriminating_test: >
  Native Rust (no Python), O(r log r) per cell: enumerate k in Z/r, record x(kG), build I_V,
  take the exact discrete Fourier transform over Z/r (r is prime, so use Rader or a direct
  O(r^2) pass at r < 2^14 and a size-r FFT via the ecbench/ic_framework transform if present),
  report max |Fhat| against the null. Positive control: the scalar base {kG : k <= B},
  whose indicator is an interval and whose coefficient is about B/xi (this must be detected).
  Negative control: the same V with the x-coordinates permuted by a random F_2-linear
  bijection of F_{2^m} that does not commute with Frobenius, plus a random non-Koblitz b.
falsification: >
  Killed if the observed maximum is within the null band at every cell and the positive
  control is detected at power >= 0.9. A signal that does not reproduce under the
  random-linear-bijection control is an artefact of presentation, not structure.
cost_band: >
  Setup: one enumeration of E(F_{2^m}) per m (about r point additions, priced at 1 GAE each
  with the section 5a band for the unpriced table writes). Transform: O(r log r) word ops,
  priced at 0, 0.1 and 1 GAE per unpriced op. Reference arm: matched signed-Frobenius rho at
  the same m, cost sqrt(pi r / (4m)) additions. Both arms listed; no cross-method ranking is
  claimed because this is a structure test, not a solver.
controls: [positive scalar-base control, random-bijection negative control, A/A with two independent V, non-Koblitz b]
risks:
  - "Toy m has ord_m(2) smaller than m-1 for some m (for example m = 13, 19, 29 differ), so Frobenius-invariant subspaces exist at toy size and are absent at m = 83 and 131; report each m separately and do not pool."
  - "A null at m <= 29 is a bounded search failure, not proof of nonexistence (AGENTS.md, Name the obstruction)."
closest_baseline: >
  Prime-field probe P2 of ecdlp_subexp_ideas_20261110 section 6 (coordinate-box base, proposed,
  not run) and research/coord_spectrum_20261110 (Fourier spectrum and additive energy of
  coordinate functions on prime-order F_p curves, measured null). Different here: binary
  field, subspace membership (F_2-linear) instead of a coordinate box, and the Frobenius
  scaling relation as a built-in reproducibility check.
lean_obligation: >
  None for a null measurement. If the Weil-bound argument for "no bias" is to be relied on,
  the obligation is a statement of the exponential-sum bound for x(kG) restricted to an
  F_2-subspace indicator (Kloosterman-type, over Z/r), conditional on the standard bounds.
probability_survives: 0.01
```

## 2. Proposal NRP-2: displacement-rank screen of the orbit-reduced relation matrix (the (T3) door)

```yaml
id: NRP-2-orbit-matrix-displacement-rank
title: "Does the orbit-reduced Koblitz relation matrix have low displacement rank, so that linear algebra is quasi-linear?"
obstruction_attacked: >
  The B^2 linear-algebra term. ecdlp_subexp_ideas_20261110 section 3 splits the arity-3 goal into
  an S_4 oracle at delta = 1/3 (T1) and a relation-generation scheme whose matrix admits a
  sub-quadratic solve (T3), and records P3 (spectral structure of a toy m=2 relation graph) as
  proposed, not run. For Koblitz curves the orbit fold already divides the column count by
  2m, which is a constant. The open question is whether the *entries* carry more structure
  than the constant.
claim: >
  Take the relation matrix of a Koblitz index-calculus run after orbit reduction. Its
  entries are +-lambda^{a} (a in Z/m, lambda of order m in F_r^*), so the matrix is a
  module over F_r[X]/(X^m - 1) restricted by the row supports. Claim: for an unstructured
  random sparse matrix of the same weight, the displacement rank (with respect to the cyclic
  shift operators of the orbit ordering) grows linearly in the dimension; for the measured
  matrix it is smaller by a factor that grows with n.
mechanism: >
  Relations come in Frobenius orbits: one relation and its m conjugates differ by the
  scalar lambda^j on the target, and the orbit-reduced matrix therefore has entries in the
  small cyclic subgroup <lambda>. Whether a *choice of relation generator* (for example
  targets of the form R + jT with T fixed, which the same note names) makes the supports
  correlated enough for a displacement or Toeplitz-like solver is exactly what has never
  been measured on a Koblitz run.
exponent_argument: >
  Quasi-linear linear algebra changes the balance point at arity 3 from B = p^{1/4} (total
  p^{1/2}) to B = p^{1/3} (total p^{1/3}), conditional on an S_4 oracle at delta = 1/3,
  which nobody has. So this proposal alone does not change the exponent; it changes a
  goalpost and falsifies or confirms the "random sparse" assumption. State it as such.
prediction:
  expected_outcome: "no statistic separates from the Erdos-Renyi null of equal row weight and density at n in {31, 41, 53}"
  probability_of_signal: 0.02
discriminating_test: >
  Reuse the orbit-reduced relation sets already produced by `ic logs` / ic_framework at
  n = 31, 41 (no new relation collection). Native Rust: (a) numerical displacement rank
  over F_r for the cyclic-shift operator pair, against 400 random matrices of equal
  support statistics; (b) singular-value profile of random blocks over the reals as a
  proxy; (c) wall time of structured elimination against block Wiedemann, both counted in
  field operations. Positive control: a planted Toeplitz-structured sparse matrix of the
  same shape must be detected.
falsification: >
  Killed if displacement rank is within the null band at every n and the planted control is
  detected. Also killed if any apparent gain disappears when the row-generation rule is
  randomised (random targets instead of the structured rule).
cost_band: >
  Linear algebra is 0.11 to 1.9 percent of S in the quoted ledger, so even a perfect
  result moves S by at most that fraction at toy size. Every arm priced in the same unit
  (S = total operations / sqrt(r)): baseline block Wiedemann, candidate structured solve,
  and matched signed-Frobenius rho as the reference row. The ratio against rho is reported
  as the distance remaining, not as a gate.
controls: [Erdos-Renyi null, planted structured positive control, randomised row-generation control]
risks:
  - "The constant-factor ceiling is 1.9 percent at toy size, so a positive result is only interesting if it grows with n."
  - "Relation rows are produced by an oracle whose output is not under the attacker's control (the note's own caveat); a structured generator may not preserve the oracle's yield."
closest_baseline: >
  P3 of ecdlp_subexp_ideas_20261110 (proposed for prime m=2, not run); orbit factor base
  (compact-orbit-scan, research/ecc2k130_open7_20261114 L1, CLOSED-BY-PRIOR as a constant).
  Different here: the Koblitz entry group <lambda> and the displacement-rank statistic,
  on binary relation sets.
lean_obligation: >
  None for the measurement. A claim that "orbit reduction gives exactly a factor m^2 and no
  more" would need a Lean statement about the rank of a G-equivariant matrix over F_r[G];
  not required unless the result is cited.
probability_survives: 0.02
```

## 3. Proposal NRP-3: Lemma A for characteristic 2 (a theorem proposal that opens or closes the cheap-nonalgebraic-map door)

```yaml
id: NRP-3-lemma-A-char2-linearized
title: "Extend the algebraic-transfer rigidity lemma to Frobenius-sparse (linearized) maps over F_{2^m}"
obstruction_attacked: >
  The residual door in ecdlp_subexp_ideas_20261110 section 5 item 3: "a map violating
  Conjecture A': a cheap non-algebraic additive map out of E(F_p)[N]". Lemma A requires
  degree below p/4, and the note admits cheap high-degree maps exist (compositions of x -> x^p).
  Over F_{2^m} the cheap high-degree maps are different and richer: every linearized
  polynomial sum a_i x^{2^i} has degree up to 2^{m-1} but only m terms.
claim: >
  (a) Every map f from E(F_q)[r] to the F_q-points of a connected commutative algebraic
  group G (q = 2^m, r an odd prime) that is additive on E(F_q)[r] and is given by a
  straight-line program of length poly(m) over F_q using field operations and Frobenius
  powers agrees on E(F_q)[r] with the restriction of an algebraic homomorphism. (b) By
  Chevalley, such a homomorphism is trivial unless G has an abelian-variety quotient with E
  as an isogeny factor, or N divides the order of a linear part, i.e. the embedding degree is
  small. So the door "cheap additive map into an algebraic group" is closed for binary fields
  as it is stated for F_p.
mechanism: >
  The Lemma A proof uses Lang-Weil to bound the zeros of f(X+Y) - f(X) - f(Y) on E x E.
  With Frobenius powers allowed, the difference map is a morphism in the Frobenius-twisted
  coordinates of degree 2^{O(m)}, which defeats the bound p^2 > O(d p). The repair would work
  on the Weil restriction (an abelian variety of dimension m over F_2, where Frobenius
  becomes a rational endomorphism of low degree) and apply Lang-Weil there. I do not know
  whether that repair goes through; that is what the proposal asks to be settled.
exponent_argument: >
  This is not an algorithm. It decides whether any exponent-changing route can hide in
  "cheap maps that are not low-degree", which is the only non-generic information source in
  the note's inventory that is not yet a theorem for F_{2^m}. If (a) is proved, the
  remaining sources are the coordinate-level operations of section 4 of that note; if (a)
  has a counterexample, it is a new primitive and would contradict the generic-group
  lower bound used by the program, so it would be treated first as an accounting error.
prediction:
  expected_outcome: "(a) provable by working on the Weil restriction A = Res E, with the cost of the Frobenius-twisted degree bookkeeping the only real work"
  probability_counterexample: 0.005
discriminating_test: >
  Desk or Lean, no compute: write the statement for the Weil restriction and try the
  Lang-Weil step there. As a toy sanity check, at m in {11, 13, 17} enumerate all
  F_2-linearized maps of the form sum a_i x(P)^{2^i} + b_i y(P)^{2^i} into F_q as a
  vector space (target F_2-vector space of exponent 2, so no nontrivial homomorphism from an
  odd-order group exists; this confirms the lemma's easy case and rules out the cheapest
  counterexample class). The informative case, targets of order divisible by r, is exactly
  the algebraic-group case covered by (b).
falsification: >
  Killed (as an open door) by a proof of (a). Falsified (as a lemma) by an explicit
  straight-line map of length poly(m) that is additive on E(F_q)[r], not algebraic, and
  verified at toy size by exhaustive check of f(X+Y) = f(X) + f(Y).
cost_band: >
  Desk. No group operations are priced. If the toy check is run, its cost is the exhaustive
  enumeration of r^2 pairs at r < 2^12, and the reference row is the matched rho at that size.
controls: [exhaustive additivity check on a planted true homomorphism (the identity on a subgroup), random linearized map as negative control]
risks:
  - "A proof of (a) closes a door; it does not produce an algorithm. Under the success criterion it is not an improvement, and I make no claim that it moves any tier."
  - "The Weil-restriction repair may require bounds on the number of F_2-points of subvarieties of A that are weaker than needed (the degree of the difference map in Frobenius-twisted coordinates)."
closest_baseline: >
  Lemma A and Corollary B of ecdlp_subexp_ideas_20261110 section 1 (prime fields; derived,
  the proof sketch has not been reviewed). Probe P1 there proposes a Lean statement for the
  prime-field case. Different here: the binary-field case, where the degree hypothesis fails
  for a concrete, cheap class of maps.
lean_obligation: >
  Required if relied on (AGENTS.md "Mathematical results should include Lean proofs").
  Obligation: a morphism from E x E to a connected commutative algebraic group over F_q that
  vanishes on all F_q-points and has degree below a stated bound is zero (the Lang-Weil
  count), then rigidity, then Chevalley's structure theorem. The first count is the part
  worth checking, as P1 of the prime-field note says. Check with search_theorems in
  crypto-lean before writing.
probability_survives: 0.5   # probability the statement is provable as stated; it is a theorem proposal, so "survives" means "gets closed in the expected direction"
```

## 4. Paradigms I examined and killed on paper (do not re-propose; each has a one-line obstruction)

All `[derived]`, not machine-checked, and each overlaps a recorded lead.

| idea | obstruction | where it is recorded |
|:--|:--|:--|
| Generalized Jacobian / modulus on E; relations carrying an F_q^*-valued invariant | the extension of E by a linear group splits on the r-part because gcd(r, q-1) = 1 (and r is not the characteristic); the F_q^*-valued invariants reduce to a trivial pairing (MOV with embedding degree 1) | Lemma A / Corollary B and "Kummer map is MOV", ecdlp_subexp_ideas_20261110 sections 1.1 to 1.3; Miller-cocycle leads in ecdlp_subexp_desk_20261101 section 4 rank 4 |
| Canonical lift of the Koblitz curve (CM, D = -7) and torsion in the ray class field | r-torsion lifts to torsion; the Galois action is scalar multiplication on `<P>` | ecdlp_subexp_desk_20261101 section 1 rows "p-adic lifts"; LEAD-TRF-padic-canonical-section (closed, proof) |
| Class-field theory, elliptic units, Hecke character structure | all structure on `<P>` is multiplication by known scalars of `Z[tau]/(Phi_m(tau))`; elliptic units reduce into F_q^*, where r-torsion is trivial | ecdlp_subexp_ideas_20261110 section 1.2 row "class-group action"; ecdlp_leads_20261018 (49a1 Eichler-Shimura, L6) |
| Hecke correspondences on the modular-curve model of the Koblitz curve (X_0(49)) as a relation generator | the horizontal neighbours close only with the descending neighbours, which are irrational and arbitrary, so the relation set does not close on a small base; at the prime 2 the Hecke operator is Frobenius plus Verschiebung, i.e. the existing tau structure | ecdlp_leads_20261018 (Eichler-Shimura, modular twists); automorphism fold |
| Trace of degree-2 places as a one-term decomposition; embedding into E(F_{q^2}) | buys a subfield factor base for a larger group, not for the original | ecdlp_subexp_desk_20261101 row 7; row 4 |
| Isogeny-class lottery for a decomposition-friendly representative | solving degree, rank and field multiplications agree with E0 within 3.85e-5 across 27 Elkies neighbours and 27 random curves | research/isogeny_class_ic_yield_20261102/report/REPORT.md |
| Cheap action of both tau and its conjugate (rank-2 S-unit group) | constant factor in rho step cost; the orbit under a rank-2 group is too large to canonicalise | research/big_prime_halving_20261115, auxiliary_halving_setup_20261115, ecc2k130_open7_20261114 L1 |
| Minimal-genus F_2-cover carrying the Weil restriction (genus about 100 to 300 would beat rho) | GHS genus floor 2^129 at m = 131; cyclic and Prym constructions leave only genus-n covers with an order-n automorphism, excluded by Riemann-Hurwitz over F_2 except n = 3 | research/cover_route_prime_binary_20261008, cover_descent_genus_floor_20261015, cover_decomposition_gap_search_20261018 section 4, ecdlp_leads_20261018 closed table rows C7, C8 |
| Bilinear / MinRank reading of the m=2 decomposition (S_3 is a Frobenius-linear map of x1*x2 plus quadratic terms) | dim(V.V) >= 2l - 1 for prime m puts any pair test at about 2^{n-l} tests; best case is rho parity | ic_m2_cycle_leads_20261030 lead 5; factor_base_designs_20261015 (E-K4, reopened) |

## 5. Prior-art check: what I grepped (a miss is not proof of absence)

Directly read: AGENTS.md sections "Preserve scope and report evidence", "Name the obstruction",
"Implementation language: no Python", "The rule: boundary, table, ratio"; docs/ic/SUCCESS_CRITERIA.md
(full); `docs/lessons/entries/L-dcb31297f97e`, `L-f2ff07653111`, `L-4c9e8e66ea68`;
research/ecdlp_subexp_ideas_20261110, ecdlp_subexp_desk_20261101, elliptic_snfs_survey_20261016,
cover_route_prime_binary_20261008, cover_descent_genus_floor_20261015, ecc2k130_open7_20261114,
ecc2k130_followups_20261116, hybrid_ic_directions_20261120/SYNTHESIS.md,
isogeny_class_ic_yield_20261102 (grep), b_weak_class_battery_20261022 (head),
g2plus_leads_20261116, ic_m2_cycle_leads_20261030, ecdlp_leads_20261018 (closed table),
docs/leads/2026-11-17/subexponential-ecdlp-goal/REPORT.md,
docs/leads/2026-11-18/novel-decomposition-oracle/REPORT.md,
research/curve25519_autolab_20261104/ideas/NONCOLLISION-20261108.md (record format).

Greps over research/, docs/, problems/, papers/ (patterns): `congruence.sieve|Question C|affine-in-the-scalar|Fourier.*indicator`
(hits: ecdlp_subexp_ideas_20261110 only as a proposal for primes; coord_spectrum_20261110 for F_p coordinate functions;
no binary-field subspace-indicator transform found); `MinRank|support.minors` (hits: factor_base_designs_20261015 E-K4,
ic_m2_cycle_leads lead 5, lfd_lower_bound minrank field; no relation-matrix displacement-rank measurement found);
`Stickelberger|elliptic unit|Eichler.Shimura|Gauss sum` (hits: ecdlp_leads_20261018 for the 49a1 twist; none for
elliptic units as relation sources); `Hamming|tau-adic|isogeny.*lottery` over ecc2k130*/README.md (hit: onb-weight
base, CLOSED-BY-PRIOR in ecc2k130_open7_20261114 L1). `Honda|Prym|cyclic cover|genus floor` (hits: the cover lanes
listed in section 4). Not searched: crypto-autoresearcher's `knowledge/leads/ecdlp/` atlas and the
cryptanalysis repository (unavailable here); a novelty claim for NRP-1 to NRP-3 therefore rests on the greps above only.

Literature named, not re-read this session: Semaev summation polynomials (ePrint 2004/031); Diem
(Compositio 2011; ANT 2013); Petit and Quisquater (Asiacrypt 2012); Galbraith and Gebregiyorgis
(summation polynomials in characteristic two); Faugere, Gaudry, Huot, Renault (symmetries); Silverman and Jacobson et
al. (xedni); Boneh and Lipton, Maurer and Wolf (black-box fields, DH/DL); Lange and Winterhof (interpolation degree).
Recalled, so each statement is `[literature, recalled]` and needs the research-kb check before quotation.

## 6. Ranking by expected value

Expected value here means probability of a surviving signal times the size of the exponent
consequence, plus the information value of closing a door cheaply.

1. **NRP-3 (Lemma A, characteristic 2).** Near-certain to resolve, desk cost, creates a
   Lean obligation that the program itself already flagged (probe P1). Exponent consequence
   is a closure, not a gain. Highest information value per unit cost.
2. **NRP-1 (binary Fourier / congruence-sieve test).** Cheapest measurement (minutes per
   cell), tests the single operation the prior note leaves without a recorded obstruction.
   Signal probability about 1 percent; if it ever fired it would be the only proposal here
   with a direct exponent consequence.
3. **NRP-2 (displacement rank of the orbit matrix).** Reuses existing relation sets. Even a
   positive result is capped at the 0.11 to 1.9 percent linear-algebra share unless it
   grows with n, and it needs an S_4 oracle at delta = 1/3 before it moves the exponent.

No proposal claims a cost gain, a tier, or a status change. The honest summary is that I did
not find a new relation paradigm that survives the repository's own closures; I found three
unmeasured doors and nine paper-killed families.
