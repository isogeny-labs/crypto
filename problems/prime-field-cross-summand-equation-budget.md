# Prime-field algebraic index calculus: one cross-summand equation per relation

- **Status:** `open`
- **Kind:** proof gap (the budget is a counting argument, not a theorem) and
  empirical bottleneck (the fall degree grows on every completed PKM cell)
- **Affected work:** the algebraic track's exponent lane
  (`research/prime_fall_degree_20261027/`, `research/pkm_falldegree_unblock_20261107/`,
  atlas `LEAD-ICP-pkm-tower-algebraic-oracle`); every prime-field lead proposal that
  rests on a bounded solving degree; the note
  [`research/prime_ic_leads_20261110/README.md`](../research/prime_ic_leads_20261110/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-11-10

## Problem

We cannot establish a bounded solving degree for any prime-field factor base presented
with auxiliary variables (PKM Kummer, Dickson, 2-isogeny towers; translate or
endomorphism-image pairs; digit presentations with bounded fibre), because one relation
`R = Σ P_i` supplies one cross-summand equation over `F_p` after the per-summand
membership equations are removed, and every further bounded-degree cross-summand equation
costs about `log₂ p` unknown bits (an extra tower on a fixed rational image of the summand,
an extension `F_{p^k}`, a `p`-adic digit, or a jet component), whereas Weil descent over
`F_{2^n}` supplies `n` such equations for about `n` unknown bits. The one free family of
extra equations, the `[k]`-multiples `[k]R = Σ [k]P_i`, enters the tower quotient at degree
about `2 log₂ k + 2` (6 for `k = 2`), not at a bounded degree. Without equations in balance
with the unknown bits, the system is a one-generator system in a `2^N`-point quotient ring,
and its last fall degree grows (measured: one degree per four tower levels at `m = 2`, half
a degree per unknown bit in `m` at fixed `t`). Parity with rho at arity `m` needs the
fall-degree slope per unknown bit below `H⁻¹((1 − 3/m)/ω')` under a degree-`d` dense
elimination costing `M(N, d)^{ω'}`: `0.017` at `m = 4`, `0.042` at `m = 6`, `0.066` at
`m = 10` for `ω' = 2` (`research/prime_ic_leads_20261110/README.md` §3, derived).

## Evidence and uncertainty

- **Observed:** `research/pkm_falldegree_unblock_20261107/RESULTS.md` Table 1 (Kummer
  `m = 2`, `P1`, `d_F` 3, 4, 4, 4, 4, 5, 5, 5, 5, 6 at `t = 1..10`; 6 at `t = 11, 12`;
  `≥ 6` at `t = 13, 14`), `RESULTS_STAGE_C.md` Tables 1–2 (`d_F` 4, 5, 6, 6, ≥ 7 at
  `t = 2`, `m = 2..6`; 4, 6, 7, ≥ 8, ≥ 8 at `t = 3`), both draws equal in every cell;
  `research/curve25519_exponent_study_20261108/README.md` (`s = 2.77 [2.31, 3.23]` at
  `m = 2`, `k = 20`, symmetrised subgroup base). Every number `[pcc:unverified]`.
- **Derived or proved:** the gate `s < m − 3` (program note §2.4; audit
  `research/semaev_alternatives_audit_20261104`); the slope thresholds and the equation
  count above (`research/prime_ic_leads_20261110/README.md` §3–4), with their assumptions:
  bounded-degree membership on coordinates of the summands and of finitely many fixed
  rational images, auxiliary variables of bounded fibre, dense degree-`d` elimination.
  The one-generator XL lower bound `d + δ` is in `RESEARCH_PKM_TOWER_ORACLE.md` §3.3.
  Not established: a theorem that the HKY last fall degree of a one-generator system in
  the carry-graded cyclic group algebra `F_p[Z/2^t]^{⊗m}` grows linearly in `N`.
- **Suspected explanation:** the growth is a property of the monomial support and the
  carry grading alone, not of the curve (testable: the pilot's `null` control and
  supports of width `w = 1..4`; lead 3 of the note).
- **Missing capability:** none for the toy-scale tests; the decisive `m = 4`, `N = 20`
  closure needs a host that survives about 49 h at 112 GiB (Amendment 8).
- **What this evidence does not establish:** nothing about an engine that never forms the
  degree-`d` matrix; nothing beyond `N ≤ 28`; a flat `d_F` at larger `N` would be the H1
  outcome the preregistration is designed to detect.

## What would unblock it

- [ ] A prime-field construction that gives `Θ(N)` independent cross-summand equations of
  bounded degree in `Θ(N)` bounded-fibre unknowns without new `F_p`-valued unknowns
  (filter of the note's §4), checked by counting before any run.
- [ ] Or: a measured `d_F` that stays flat under one of the free presentations (the
  `[k]`-multiples control, `--controls multiples`) at cells where the tower reads `≥ 8`,
  with both draws, the same seeds and the planted-solution assertions passing.
- [ ] Or: a proof or exact computation that the one-generator closure degree in
  `F_p[Z/2^t]^{⊗m}` is linear in `N` for every support of bounded width, which would
  resolve this record as a theorem-backed obstruction for the tower family.

Meeting the first makes an algebraic lane exist; the second reopens item 3 of the note's
§4; the third closes the family with a mechanism. None of them changes a generic-track
row.

## Next discriminating step

Run the multiples and null-support controls at `t = 2, 3`, `m = 2..5` (minutes on a
4-core host), then the `m = 4`, `N = 20` closure under Amendment 8. Outcomes: flat under
multiples → reopen §4 item 3; grows in every presentation and in the null → file the
mechanism and close the Kummer lane per Amendment 8's rules.

## Update 2026-11-10 (development check, this host; every value `[pcc:unverified]`)

The multiples control ran on Kummer `P1` cells (`research/prime_ic_leads_20261110/devcheck/`):
identical `d_F` at `m = 2`, `t = 2..9` (P1); at `m = 3, 4, 5` a drop of exactly one degree on six
of ten cells and zero on four, with both slopes unchanged (P2). The falsifier (a drop of two,
or a flat ladder) was not reached on any completed cell; the `m = 5`, `t = 3` cell is censored on
both arms (tower `> 7`, multiples `> 6`, the latter by a 2.5·10⁸-entry cap on a 15 GB host) and
is inconclusive. The second "what would unblock it" item is therefore not
met at toy scale; the record stays open on the first and third.

## Update 2026-11-11 (null-support development check; every value `[pcc:unverified]`)

The suspected explanation is now measured at toy scale: a random polynomial on `S₃`'s monomial
support, or on the full width-2 exponent box, has the tower's last fall degree on all 40 completed
cells (`m = 2`, `t = 2..9`; `m = 3, 4`, `t = 2, 3`), with the same fall sequences, and the degree is
monotone in the box width (`research/prime_ic_leads_20261110/README.md` §6b). The growth is a
property of the carry-graded one-generator closure, not of the curve. The third "what would unblock
it" item (a derivation or exact computation of the one-generator closure degree as a function of
support and `N`) is the remaining step; the first two are unchanged.

## Update 2026-11-11, later (the bigger-ring exits; every value `[pcc:unverified]`)

Derived and measured in `research/prime_ic_leads_20261110/jet_exit/README.md`: over `F_p[ε]/(ε²)`
the extra equation comes only with free `F_p` unknowns (no homomorphism `E(F_p) → F_p` when
`p ∤ r`); over `Z/p^k` the torsion section gives `k − 1` extra cross-summand equations in the digit
functions `x_j(P)`, whose popcount degree on Kummer bases equals that of a random function (3, 4, 5
at `t = 6, 8, 10`, and 6 at `t = 12`; 40 cells over 16 curves, two primes and five integer lifts, three
digits), so they enter at a degree that grows with the unknown bits (`t ≤ 12` measured; the asymptotic
is the filter's requirement, not a measurement). All three bigger-ring exits are closed at toy scale; the first unblock item has no known
candidate.
