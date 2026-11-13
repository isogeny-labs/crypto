# PROPOSAL: Stange's division polynomials for arbitrary isogenies and the index-calculus baseline

Date: 2026-11-13. Author role: Idea Generator (proposals only; no experiment was
run, no status changed, no work assigned). Companion to `LEAD.md` in this
directory (lead recorded earlier the same day; this file is the separate-author
PROPOSAL that `LEAD.md` "Next step" asks for).

Standing: every statement here is **[pcc:unverified]** unless it names a pcc
claim id; none does. Numbers quoted from disk keep the standing their source
files give them (mostly toy scale, single run, [pcc:unverified]). Hand
estimates are labelled as such.

## 0. Headline (read this first)

**The honest prediction is "no index-calculus improvement."** Stange's [pcc:unverified]
construction (arXiv:2503.15428) changes which *normalised function* is attached
to an isogeny whose kernel does not sum to the identity. No stage of the
repository's index-calculus pipeline consumes such an object, and the closest
isogeny-related index-calculus results on disk are already recorded as
invariants or constants-only. The counted cost ratio against the current IC
baseline is predicted to be exactly **1.000** on every cell (or, if a
verification step is added, **>= 1.000**, a cost increase). The distance to the
matched rho is unchanged. This agrees with the owner's view recorded in
`LEAD.md`: it closes a certification gap and does not solve arbitrary-target
lifting or dependency discovery.

Three records follow. They are independent and none is bundled with another.

| id | what it tests | predicted IC effect | needs a run |
|---|---|---|---|
| STANGE-IC-01 | whether any IC stage's cost can depend on Stange's objects (upper-bound audit) | none; ratio 1.000; bound `speedup <= 1/(1-f)` with `f` tiny | no (read existing records) |
| STANGE-RR-02 | whether the arbitrary-index net polynomial lowers the working degree `D` of the already-planned Riemann-Roch tower arm | none; `D` equal | only as an extra arm of a run that is already recommended |
| STANGE-CERT-03 | whether a native verifier closes the even-degree (biased) certification gap | none (this is not an IC improvement; AGENTS.md: relabelling and accounting changes are not improvements) | yes, a local `cargo test`-scale check |

## 1. Obstruction named (AGENTS.md "Name the obstruction")

- **Attack target.** For prime-order `E(F_p)` the cost exponent stays 1/2 [pcc:unverified]
  because (i) relations need *membership* of decomposition parts in a factor
  base (barrier map L1-L3), and (ii) a relation source that supplies relations
  only among base points cannot attach an arbitrary target faster than
  `n/(2B)` (Lemma T, a proof sketch, not formalised).
  Sources: `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_BARRIER_MAP_20261104.md`;
  `research/non_semaev_decomposition_20261113/README.md` section 4.
- **Barrier-map filter (section 5 of that note) applied.** The idea evades none
  of L1, L2/L3, k-SUM, p-adic, height, or (T). It is not a candidate attack on
  the cost exponent and this proposal does not claim to be.
- **Existing problem records.** No `problems/*.md` file covers "relation source
  with no target attachment". The closest recorded obstruction is Lemma T in
  `research/non_semaev_decomposition_20261113/README.md`. I recommend the
  Coordinator link this PROPOSAL from that record rather than create a new
  one, because it names the same blocker (arbitrary-target lifting).
  I did not create a `problems/` file (scope: proposals only).
- **What would unblock arbitrary-target lifting** (acceptance criteria, none
  met by Stange's paper): a map from `Q = [k]P` to a relation that involves `k`
  without being a group-law collision, not simulable in Shoup's generic group
  model. Satisfying it would establish a non-generic target oracle; it would
  not by itself establish a cost advantage, which needs the relation count and
  linear-algebra cost priced in `S`.
- **What would unblock dependency discovery**: a procedure that finds linear
  dependencies among factor-base points at cost below `~B^2` unstructured
  linear algebra (Lemma T note: unstructured linear algebra alone forces
  `n^{2/3}`). Stange's paper has no such procedure.

## 2. Prior-art check

### 2.1 Sources checked (and what was not possible)

- **research-kb: not queried.** This session's toolset has no shell and no MCP
  `research-kb` connection, so neither `kb_status`/`search_knowledge` nor
  `scripts/research-kb` could run (the script also needs the sibling
  `crypto-autoresearcher/research-kb/.venv`, which the 2026-11-13 session log
  reports absent). **No `source_id` is recorded.** Novelty and "not tried"
  statements below are therefore *not established*; a session with the KB should
  repeat the negative-form queries before any claim is cited.
- **Lead atlas: not read** (records are `VLT1`-encrypted per the 2026-11-13
  session log; plaintext copies listed below were used instead).
- **Read on disk (this session):**
  - `AGENTS.md` sections "Preserve scope and report evidence", "Name the
    obstruction", "Implementation language: no Python", "The rule: boundary,
    table, ratio".
  - `research/stange_torsion_division_20261113/LEAD.md`.
  - `research/torsion_division_derivative_20261031/README.md`, `PROTOCOL.md`
    (lines 1-150).
  - `research/lead_sweep_20261110/LEAD-EXO-elliptic-net-eds/PROTOCOL.md`,
    `RESULT.md`.
  - `research/notes/cm-isogeny/RESEARCH_EDS_RESIDUE.md` (all 791 lines).
  - `research/non_semaev_decomposition_20261113/README.md` (all).
  - `research/pair46_factor_bases_20261015/07-torsion-division/RESULTS.md`
    (lines 1-60).
  - `docs/ic/LEADERBOARD.md` (lines 1-90), `docs/isogeny-walk/README.md`
    (lines 1-80), `problems/odd-isogeny-quartic-transfer-execution.md`
    (lines 1-70), `docs/ic/RESEARCH_ISOGENY_CONDUCTOR_GAP_20261007.md`
    (lines 1-120), `RESEARCH_ISOGENY_FIBERS_ECDLP.md` (Thm 15.1, lines
    1472-1500).
  - Source headers: `src/cryptanalysis/ecbench/isogeny.rs`,
    `src/cryptanalysis/f4_fp_tower.rs`, `src/cryptanalysis/binary_isogeny.rs`.
  - Not found: any `src/**/*division*` module; no file cites arXiv:2503.15428
    (consistent with `LEAD.md`).
- **Web (this session):** arXiv abstract page and HTML v3 of arXiv:2503.15428
  (characters 0-200,000 of 248,265 read; the last ~48k, including part of the
  Examples/Section 9 material, were not read); the PDF fetch returned
  compressed binary and was unusable; ePrint 2025/521 and a faculty-page
  listing seen only as search results.
- **Correction to `LEAD.md`** (not edited; frozen lead): `LEAD.md` dates the
  paper "March 2025, revised April 2025". The arXiv page lists **v3 on 17 April
  2026** ("corrections and many new examples"). Any citation should name v3.

### 2.2 What the paper says (from the arXiv HTML v3 text read, via a summarising fetch tool, not a verbatim read)

- Definitions: a nonzero isogeny is *unbiased* if its kernel elements sum to
  the identity with multiplicity; otherwise it is *biased* and "must sum to a
  non-trivial two-torsion point" `P_phi`. For biased `phi`,
  `Psi_phi` has divisor `phi^*(O') - (deg phi)(O) + (P_phi) - (O)` (Eq. 2.1),
  plus an auxiliary `Psi-hat_phi` with divisor `2(P_phi) - 2(O)`.
- Theorem 3.3 (first chain rule): `Psi_{alpha beta} = (Psi_alpha o beta) Psi_beta^{deg alpha}`
  for unbiased `alpha, beta`; a squared version with `Psi-hat` terms in general
  (the paper itself calls the general biased case "somewhat unsatisfying").
- Theorem 5.1 (relation to `x`): for `alpha != +-beta`,
  `(Psi_{alpha+beta} Psi_{alpha-beta} Psi-hat_alpha Psi-hat_beta)/(Psi_alpha^2 Psi_beta^2 Psi-hat_{alpha+beta}) = x' o beta - x' o alpha`.
- Recurrences cost `O(log n)` applications from `Psi_1..Psi_4` ("allows for
  efficient computation"). Theorem 6.3: specialising at a point `P` with
  `phi_i(P)` not in `E'[2]` and `(phi_i +- phi_j)(P) != 0` gives an elliptic net
  for `E'` at the points `phi_i(P)`. Section 8: vector-of-isogenies version,
  Theorem 8.1 (chain rule in higher dimension).
- The text read contains **no cryptographic application, no ECDLP statement and
  no complexity claim beyond the recurrence cost.** (Cross-listed cs.CR only.)
  Because the fetch tool summarises, formulas above should be checked against
  the paper before any Lean statement is written.

### 2.3 Closest prior work (published and in-repo) and the difference here

| closest | what it covers | difference |
|---|---|---|
| Mazur-Tate; Satoh (division polynomials for isogenies; cited in Stange's references) | unbiased case only | Stange adds the biased case; only this is new relative to them |
| Stange 2007/2011 (Tate pairing via elliptic nets; Elliptic Nets and Elliptic Curves); Lauter-Stange 2008 (SAC; arXiv:0803.0728) | EDS/net problems are sub-exponentially equivalent to ECDLP | unchanged by the new paper; `RESEARCH_EDS_RESIDUE.md` section 2.2 already realises this in-repo |
| `RESEARCH_EDS_RESIDUE.md` sections 5.3a, 5.3b | residue bits are information-tight (~`log2 m` bits) but need `O(m)` extraction; rank-2 net is a reparametrisation of the rank-1 EDS when `Q in <P>` | Stange's product-of-curves extension is the same net family; for `Q = [k]P` the same reparametrisation holds (derived from section 5.3b; not re-run) |
| `LEAD-EXO-elliptic-net-eds` | zero-test locator costs `Theta(N)`; slope gap 0.682 (arm A, primary), 0.457 (arm B, A/A) vs BSGS; A/A difference 0.225 against limit 0.25; sieve/BSGS ratio non-monotone in arm B (54.3 then 40.7) | tests only the GGM-simulable zero test; does not test any target-specific nonlinear locator (its own section 6) |
| H109 (`torsion_division_derivative_20261031`) | `D psi_r` at torsion targets: 0/28 exact formulas; best match 0.500-0.611 vs controls 0.500-0.630 | classical `[r]` only; arbitrary-isogeny polynomials are outside its frozen family, so H109 neither covers nor refutes them (`LEAD.md`) |
| pair-46 torsion/division factor bases (`07-torsion-division/RESULTS.md`) | exact obstruction: `E(F_p)[m] = {O}` for prime order; `[m]` automorphism so coverage change is exactly 0; membership degree `m^2 |S|` | covers `[m]`; for rational isogenies of prime-order groups Thm 15.1 below is the analogue |
| `RESEARCH_ISOGENY_FIBERS_ECDLP.md` Thm 15.1 | with oracles for every rational isogeny and known scalars, success `<= (Q+2)^2/r + 1/r` | proof as recorded: `ker psi` meets `G_i` in 0 or `G_i`; any new normalisation of `Psi_phi` is a function on the same kernel, so it cannot add an oracle the theorem withheld |
| `isogeny_class_ic_yield_20261102`, `drastic_prime_conductor_gap_20261012` (as summarised in the 2026-11-13 note) | IC cost is a class invariant (ratios 1 +- 3.85e-5) | the biased-isogeny case is about *which* curves are reachable/certified, not the cost on them |
| `pkm_isogeny_tower_ecbench_20261015` (as summarised) | 2-isogeny tower base 1.00-1.33x a plain base of equal size; 18-136x rho; same F4 degree `D` as Kummer and as shape-matched null | the 2-isogeny step in `f4_fp_tower` (`y_j^2 = y_j y_{j+1} - xi y_{j+1} + xi y_j - tau`) is the ground case of a biased isogeny; this is where STANGE-RR-02 asks one question |
| Velu / Schoof / Stark / Kucuksakalli kernel polynomials; sqrt-Velu (Bernstein-De Feo-Leroux-Smith 2020); JMV 2005 | kernel polynomials, isogeny evaluation, random self-reducibility inside one endomorphism ring | cited from Stange's references and the repository notes, not re-read |

Literature attributions marked "as summarised" come from the 2026-11-13
`non_semaev_decomposition` table, whose paths were checked for existence but
whose numbers were not re-derived there or here.

## 3. Facts used by the records (hand derivations, [pcc:unverified])

1. **Odd degree is always unbiased.** For a separable isogeny of odd degree, the
   kernel is an abelian group of odd order. It has no element of order 2, so the
   map `R -> -R` pairs off the nonzero kernel points and the kernel sum is `O`.
   Hence `P_phi` can exist only for even degree. (Statement for separable
   `phi`; inseparable cases follow the paper's multiplicity convention and are
   out of scope.)
2. **Consequence for the repository's certified walks.** `docs/isogeny-walk/README.md`
   states the walker is "prime fields and odd `l` only" and records "0 failures
   across 325,000 certified edges". By fact 1, none of those edges is biased,
   so the classical (unbiased) kernel-polynomial certification already covers
   them. The new paper closes **zero** of those 325,000 edges.
3. **Where biased edges do occur in the repository.** (a) `transfer.isogeny`
   (`src/cryptanalysis/ecbench/isogeny.rs`) supports `l = 2` with a rational
   kernel `h(x) = x - x0` (a root of the curve cubic): kernel `{O, T}` sums to
   `T != O`, a biased isogeny. (b) The 2-isogeny step of the PKM tower
   (`f4_fp_tower.rs`). (c) 2-isogenies on binary curves
   (`binary_isogeny.rs`; characteristic 2 is **not** established to be covered by
   Stange's paper from the text read here).
4. **Existing certification of those edges.** `transfer.isogeny` already checks
   the final answer by `[k]G = Q` on the source curve, and its header states the
   kernel search and the two evaluations of `phi` are `O(log p)` field
   multiplications against `O(sqrt r)` group operations. So the correctness of
   a transfer answer is not at stake; what is missing is a standalone
   certificate for the *edge itself*.

## 4. Records

### 4.1 STANGE-IC-01 (primary; the question as asked)

```yaml
id: STANGE-IC-01
title: No index-calculus stage can gain from arbitrary-isogeny division polynomials (upper-bound audit)
obstruction: >
  Arbitrary-target lifting and dependency discovery (Lemma T; barrier map
  L1-L3). Stange's objects supply neither.
claim: >
  For every ecbench cell whose baseline arm is recorded on disk, the counted
  cost ratio candidate/baseline of any pipeline that adds Stange's
  arbitrary-isogeny division polynomials is exactly 1.000, or above 1.000 if a
  verifier is charged, because no stage of factor-base construction, relation
  collection, membership/decomposition, linear algebra, or target attachment
  consumes an isogeny division polynomial. Any gain is bounded by
  speedup <= 1/(1 - f), where f is the recorded share of counted operations in
  the only stage that touches isogenies (the `transfer` phase of
  `transfer.isogeny`).
mechanism: >
  (i) Kernel-sum bias is a property of even-degree kernels with a 2-torsion
  point, which prime-order E(F_p) lacks (pair-46 E1: E(F_p)[m] = {O}); the
  isogenies that reach other curves in the class are odd and already unbiased
  (fact 1). (ii) Thm 15.1 shows oracles for every rational isogeny with known
  scalars add nothing in the GGM; a new normalisation of Psi_phi evaluates a
  function with the same kernel, so it is inside the theorem's hypotheses.
  (iii) Stange's Thm 5.1 factors x' o beta - x' o alpha over endomorphism/
  isogeny indices; for the ECDLP, Q = [k]P implies a P + b Q = [a + bk]P, so
  net values are a reparametrisation of the rank-1 EDS
  (RESEARCH_EDS_RESIDUE section 5.3b). (iv) Lemma T: relations among base
  points only cannot beat sqrt(2 c n).
prediction: >
  Point prediction: ratio = 1.000 on all cells; f_transfer <= 1e-3 on every
  recorded transfer.isogeny cell. Declared interval for f_transfer: [1e-6, 1e-3]
  (hand estimate [pcc:unverified]: for p ~ 2^36, kernel search ~ 36 modular
  squarings of a degree-<=3 polynomial, about 2^8 field multiplications,
  against rho at S ~ 1.3 * 2^18 group operations of roughly 2^4-2^6
  multiplications each, giving f of order 2^-14; to be replaced by the
  recorded native counts).
discriminating_test: >
  Zero-compute, read-only. Step 1: write the stage-by-stage consumption table
  (six stages x "does this stage's counted cost depend on a quantity that an
  isogeny division polynomial changes?") from the code paths in
  `src/cryptanalysis/ecbench/` and `f4_fp_tower.rs`, with file:line evidence.
  Step 2: extract f_transfer = native_transfer_ops / total counted operations
  from existing ecbench records of `transfer.isogeny`, priced at 0, 0.1 and 1
  GAE per native op (AGENTS.md section 5a). Step 3: compare f_transfer with the
  A/A noise band recorded for the same cell.
falsification: >
  Killed if any of: (a) a stage is found whose counted cost depends on an
  object that a Stange polynomial replaces, with a written mechanism; (b)
  f_transfer >= 1e-2 on some recorded cell at the 0 GAE price, in which case
  the "bounded by 1/(1-f)" statement no longer makes a gain negligible and a
  paired replay is warranted; (c) a recorded transfer.isogeny cell shows a
  candidate/baseline ratio different from 1.000 under an unchanged solver.
  If f_transfer is below the cell's A/A band, a gain is unmeasurable by
  construction and the record is closed as "no measurable IC effect".
cost_band: >
  Compute: none (record reads). Arms priced if later run: rho reference
  (S ~ 1.3 per AGENTS.md, negation-rho ~0.886 sqrt(n) in the Lemma T note);
  unmodified IC baseline (cell-specific S); candidate = baseline + Stange
  verifier (adds native ops, priced at 0/0.1/1 GAE per op; order of
  O(log l) field multiplications per edge, hand estimate). All three arms in
  the same unit S = total operations / sqrt(n) per AGENTS.md rule.
controls: >
  Positive control: the audit must reproduce f_transfer for a cell where
  `transfer.isogeny` is known to run (it does recover k and verify [k]G = Q).
  Negative control: apply the same audit to a stage known to dominate cost
  (relation collection) and confirm it reports f near 1.
risks: >
  Record numbers may not exist for transfer.isogeny at the cells of interest
  (I did not locate them; one search timed out) -- then Step 2 needs a small
  native run, outside this proposal. The audit could miss an indirect
  dependency (e.g. through trait detection feeding cell selection).
closest_baseline: >
  Thm 15.1 (RESEARCH_ISOGENY_FIBERS_ECDLP.md :1472); pair-46 E4; Lemma T.
lean_obligation: >
  None for the claim (it is an accounting and audit statement). If the report
  adds Lemma T or fact 1 as a premise, they carry their own obligation:
  fact 1 (a finite abelian group of odd order has element-sum zero) is a
  Mathlib-sized lemma; Lemma T is an unformalised Shoup-style collision bound
  and must stay labelled [pcc:unverified] until proved.
```

### 4.2 STANGE-RR-02 (conditional; one extra arm, no standalone run)

```yaml
id: STANGE-RR-02
title: Does an arbitrary-index (biased) net polynomial lower the working degree D of the Riemann-Roch tower encoding?
obstruction: >
  Membership on a tower (barrier map L2/L3): D slope 0.125-0.15 per level,
  k-SUM heuristic D >~ 0.04 N + c. The only live class that passes the
  filter is the Riemann-Roch / net encoding on a tower
  (non_semaev_decomposition section 5).
claim: >
  On the m = 4 cells already specified for the radix-vs-levels discriminator
  (|V| in {2^6, 2^12}, 3 primes x 2 planted targets, shape-matched null per
  arm), the Riemann-Roch encoding built with Stange-normalised net polynomials
  for indices including even entries has the same solving degree D and the same
  matrix width as the existing p1_rr_dickson encoding: Delta D = 0.
mechanism: >
  Stange's polynomials are normalised generators of ideals whose zero sets are
  fixed by divisors (Eq. 2.1, 6.2). A change of normalisation rescales
  functions by constants (leading-coefficient condition) and fixes the
  2-torsion multiplicity; it does not change the variety, so it cannot change
  the ideal, and for a fixed monomial order the Groebner basis. The only way
  Delta D != 0 is through the generating set fed to F4 (selection
  heuristics), which is a constant-level effect, not an exponent change.
prediction: >
  Delta D = 0 (declared interval [-1, 0] for |V| = 2^12, accounting for
  generator-set effects on F4 intermediate degree; no change in the matrix
  width above the shape-matched null's spread).
discriminating_test: >
  Add one arm ("Stange-normalised net generators") to the radix-vs-levels
  run that non_semaev_decomposition section 6 already recommends. No separate
  run is authorised by this record. The run, if approved by the Coordinator,
  goes through isolab-eks with a frozen task card, `.resume/RESUME.md`
  checkpoint and a bench.record, in Rust (`f4_fp_tower` with a `CubeRule`
  extension, `p1_rr_dickson` system builder reuse); no Python.
falsification: >
  Killed if D(Stange arm) <= D(existing RR arm) - 1 at |V| = 2^12 with matrix
  width not larger, and the same relation holds against the shape-matched null
  (that would be the first evidence that the encoding, not just the
  presentation, moves the cost). Not killed (but closed) if Delta D = 0 within
  the null's spread.
cost_band: >
  Marginal: one extra arm on cells already priced by the parent run; native
  F4 operations counted at 0/0.1/1 GAE; reference arms in the same unit:
  flat Semaev S_{m+1}, existing RR arm, shape-matched null, and rho at
  S ~ 1.3. The m = 5 cell is excluded here (Spot interruptions and the
  56 GiB chain-arm cap, per the parent note).
controls: >
  Shape-matched null per arm; positive control = a known-answer planted target
  solved by every arm; A/A = the existing RR arm run twice with different
  seeds.
risks: >
  The parent run needs a `CubeRule` extension that does not yet exist; if the
  parent is not run this record has no test. Even entries (biased indices) may
  not occur in the tower encodings actually used (the (1,1,...,1) index is
  unbiased), in which case the arm is vacuous and the record closes as "not
  applicable" -- that would itself confirm STANGE-IC-01.
closest_baseline: >
  p1_rr_dickson (stage-positive at m = 3, undecided at m = 5, pcc:unverified);
  barrier map section 1 ("group-law d_reg = 2 everywhere; the encoding is not
  the cost").
lean_obligation: >
  None for a measured D comparison. Any statement that two normalisations
  generate the same ideal needs Stange's chain-rule/relation theorems
  (Thm 3.3, 5.1) as Lean premises. Whether the lab's Lean library already
  holds them was not checked: `search_theorems` could not be run (see 2.1).
```

### 4.3 STANGE-CERT-03 (certification only; explicitly not an IC improvement)

```yaml
id: STANGE-CERT-03
title: Native verifier for biased (even-degree) isogeny edges, closing the certification gap
obstruction: >
  The 2-isogeny edges in transfer.isogeny, the PKM tower step and the binary
  isogeny walk have no standalone certificate; only the end-to-end check
  [k]G = Q covers transfer answers (fact 4).
claim: >
  A Rust verifier that evaluates Stange's recurrence for a degree-2 Velu
  isogeny with kernel {O, T} (biased, P_phi = T) accepts every genuine isogeny
  and rejects every corrupted one on a frozen set of curves, and agrees with the
  classical psi on odd-degree (unbiased) controls.
mechanism: >
  Theorem 3.3 / 5.1 / Cor 5.2 give a checkable identity chain with the
  Psi-hat correction for biased indices; the verifier checks these identities
  at sample points over F_p. Recurrence cost O(log n) applications.
prediction: >
  Accept 100% of genuine edges and reject 100% of single-coefficient
  corruptions on at least 1,000 curves. Declared soundness bound: a corrupted
  rational function of degree d agrees with the genuine one at a random point
  of F_p with probability at most d/p, so t independent points give at most
  (d/p)^t; the frozen task card must state d, p and t. Added native cost per
  edge: O(log l) field multiplications (hand estimate: tens to low hundreds),
  which is not a gain; S_candidate/S_baseline >= 1.000.
discriminating_test: >
  `cargo test` level, local, no cluster: positive control = odd-degree edges
  from the 325,000-edge walk evidence (must agree with the classical
  kernel-polynomial certificate); biased case = degree-2 Velu isogenies on
  curves with rational 2-torsion; negative control = flipped Velu coefficient;
  convention control = compare the verifier's multiplicity convention with
  SageMath's `two_torsion_multiplicity` option on 20 curves (Sage is a
  reference oracle for conventions only, not part of the deliverable).
falsification: >
  Killed if the verifier accepts any corrupted isogeny at a check point where
  the corruption changes the function, or rejects a genuine edge, or the
  biased-case identities in Stange's paper fail on a degree-2 Velu isogeny
  (that would be a finding about the paper's normalisation or my reading of
  it, to be reported, not smoothed over).
cost_band: >
  Verifier native ops priced at 0 / 0.1 / 1 GAE per op, reported against the
  edge's certification-free baseline and against the solve it supports
  (rho reference S ~ 1.3; unmodified IC baseline cell-specific). The ratio is
  reported as a component accounting item, not as a speedup.
controls: >
  See discriminating_test; plus A/A: the same verifier run twice on disjoint
  curve sets.
risks: >
  Characteristic 2 coverage by the paper is not established from the text read
  (binary_isogeny.rs would then be out of scope). The reading of Eq. 2.2
  normalisation and of Theorem 3.3's general case is the largest correctness
  risk; the last ~48k characters of the HTML were not read.
closest_baseline: >
  Existing kernel-polynomial certification in `isogeny_walk` (odd l,
  0 failures in 325,000 edges) and the [k]G = Q check in transfer.isogeny.
lean_obligation: >
  If the verifier's acceptance is cited as a proof of isogeny correctness, a
  Lean statement is required: "verifier accepts (E, E', h) implies the Velu map
  has kernel exactly ker(h) and is a homomorphism". A practical scoping that
  avoids the heavy divisor theory: make acceptance a finite polynomial-identity
  check whose soundness is the standard Velu theorem (Washington Thm 12.16),
  and label any Stange-specific step [pcc:unverified] until proved. H109's
  `formal/H109TorsionDerivative.lean` is the format precedent for standalone
  Lean interfaces.
```

## 5. Order of work and decision table (who decides is the Coordinator)

| outcome | next action |
|---|---|
| STANGE-IC-01 audit finds f_transfer below the cell's A/A band and no stage consumes Stange objects | close the IC question as "no measurable IC effect"; keep the certification record only if a user wants the gap closed |
| STANGE-IC-01 finds f_transfer >= 1e-2, or an undocumented dependency | open a paired native replay (candidate vs baseline) under the usual frozen card; this would be a component question, not a pipeline gain |
| STANGE-RR-02 not run | no loss; the parent run stands on its own |
| STANGE-RR-02 shows Delta D <= -1 at |V| = 2^12 with matched null | the first evidence that the encoding moves the cost; escalate to a priced S comparison at growing sizes before any claim |
| STANGE-CERT-03 passes | a scoped certification result; no leaderboard row, no `benchmarks/README.md` row (validation gate; success criterion excludes accounting changes) |

## 6. Scope and honesty notes

- The success criterion (CLAUDE.md) counts only verified, measured [pcc:unverified]
  improvements over the current IC baseline. Nothing here is a predicted
  improvement; the gap to the matched rho is predicted **unchanged**. Recent
  matched rows read from `docs/ic/LEADERBOARD.md` sit at 1.27x-1.31x IC/rho for
  the closest cells (1.0x is parity); the page states no row of tables A-C is
  below its reference. This proposal moves none of them.
- No claim here establishes a lower bound beyond what Lemma T (unformalised) and
  Thm 15.1 (as recorded) already state. Bounded searches and a failed research-kb
  query do not prove "not tried" or "not novel".
- Legacy Python: `run.py` in H109 and in LEAD-EXO-elliptic-net-eds are Python
  from before the Rust rule; they are historical evidence, are not extended
  here, and any new measurement above is specified in Rust (`cargo test`,
  `ecbench`, `tools/task_card.py` for the frozen card).
- Not done: no experiment, no Lean file, no problems/ record, no status change,
  no PR. Unresolved requirements: research-kb queries (`kb_status`, 2-4
  `search_knowledge` including negative form, `search_theorems`) remain
  **not attempted** for lack of tool access; the last ~48k characters of the
  paper's HTML remain unread.

## STANGE-SP-04: summation polynomials over isogenous curves (added 2026-11-13)

All statements in this record are [pcc:unverified] reasoning; nothing was run.

**Mechanism.** Curves E_1..E_m with isogenies phi_i: E_i -> E; seek relations
sum_i phi_i(P_i) = O on E, with P_i in factor bases F_i on E_i. Equivalently,
Stange's product-of-curves net polynomial at coefficient vector v = (1,...,1).

**Expected obstruction.** Each phi_i is a homomorphism and, for deg phi_i != r, [pcc:unverified]
a bijection on E[r]. The relation is Semaev's S_m on E evaluated at
x(phi_i(P_i)), rational functions of degree ~ell_i in x_i: degree per variable
grows by ~ell_i, and the factor base {phi_i(P)} is just another subset of E[r]
with no Weil-descent structure over F_p. The "linear in subgroup size" cost of
the net zero test arises from scanning v, which is not needed at v = (1,...,1);
the remaining wall is the ordinary summation-polynomial one (degree ~2^(m-2)).
The non-split direction (gluing E_1, E_2 into a genus-2 Jacobian, Kani/Richelot)
is prior work under the ecdlp-surfaces tooling, and genus-2 index calculus does
not beat rho.

**Preregistered prediction.** The pulled-back S_m and the (1,1,1) net polynomial [pcc:unverified]
have the same zero set as S_m on E (up to known factors), strictly higher degree,
and never solve faster at matched factor-base size.

**Cheapest discriminating test.** Toy prime, m = 3, 2-isogenous curves: build
both polynomials, check zero-set agreement, compare degrees and Groebner solve
time against plain S_m on E at the same |F| and relation yield.

**Falsification.** The isogenous variant yields relations at lower measured cost
than plain S_m at matched factor-base size and relation yield (A/A-banded).
