# Red-team review of the prime-field IC lead chase (2026-11-14) [pcc:unverified]

Scope: 40 review-set + 12 desk-refuted records; per-record output in `checks/redteam.json`. One desk check (`rt/exponent_check.py`,
stdlib, deterministic, 0.4 s). research-kb and ecdb unavailable (grep only). No repository files were edited.

## Proposed changes (25; 1 of them is a column fix only)
Flagship set reduced from 9 to 2. **CH-L1** (PKM fall-degree growth at m>=8, sparse panel A) and **CH-L4** (growing-rank lifting)
stay flagged. They are the two questions the closure ledger and CONTINUATION_PROTOCOL name.
- Duplicates unflagged: LEAD-ICP-pkm-tower-algebraic-oracle, ENC-0.1 (= CH-L1); CL-A8, LEAD-TRF-global-lift-small-height,
  LEAD-EXO-global-lift-height (= CH-L4).
- ENC-0.2 unflagged. The Velu tower is "falsified as a gain" (1.00-1.33x a plain base, same F4 D; non_semaev_decomposition_20261113),
  and its first fall degree is 5 against 4 for Kummer.
- IDEA-20261109-2debfa: open/flagship -> refuted/desk. Desk check: the record's own route (enumerate m-2 variables, then the joint k=2 solve
  at B^e2) has best exponent 2/(3-e2), which is at least 2/3 even at e2=0. Its m=8 per-call gamma=6+e2 is above the 3.5 gate.
- CH-D83: blocked -> refuted (Lean model proof). The Lean cost floor (p < W^2, c >= B^4) removes the m=8 gain even if D(8,3)=9. The cell is now
  a diagnostic only. Check: min_b max(4b,1-3b)=4/7.
- Closed at exponent level by Lemma T (relations among base points only, total >= sqrt(2cn)): LEAD-ICP-lattice-ic-relation-construction
  and LEAD-ICP-prym-witness-relation-diversity blocked -> refuted; IDEA-f7c381 open -> refuted (column BARRIER).
- Closed by existing theorems: LEAD-ICP-isogeny-transported-relation-yield blocked -> refuted (yield conservation + isogeny-class
  invariance); LEAD-STR-class-group-ecdlp-bridge open -> refuted (End(G)=Z/N GGM-simulable; cost is a class invariant).
- Records whose positive outcome is a constant only (open -> engineering): LEAD-ICP-split-graph-secant-presentation (fixed-d degree)
  and LEAD-ICE-gag-contraction-planner (contraction order).
- SAT oracles: MOON-1 open -> refuted/toy. alpha_FRESH=1.877; clause reuse is multi-target amortisation, and the first REUSE solve costs
  1.0-2.5x a FRESH one. IDEA-a59470 blocked -> refuted/toy: its experiment is essentially F4-RUN-02, which the record missed.
  IDEA-d0ce11 blocked -> refuted/desk: it predicts a negative, and a positive would be a constant on a route measured far above rho.
- IDEA-3a4152 open -> refuted. Its sub-B^2 3SUM lever is dominated by the trivial O(B) hash for arity-2 decomposition.
- PIL-6b open -> refuted/toy. Every measured width grows (W=3/4 step at t=9; W=1 d_F>=6 at t=12 per PR #4151, as reported). The missing
  growth proof can only close PKM, so it belongs under CH-L1.
- Out of scope: LEAD-EXO-hnp-advice-leakage (needs auxiliary input) and IDEA-16e2d3 (pure formula verification, self-declared).
- LEAD-EXO-torsor-deck-period open/toy -> unexamined/none: the record itself says the toy does not touch the object and that no test exists.
- Column fixes: LEAD-EXO-lange-winterhof-interpolation ORACLE -> BARRIER. The Lange-Winterhof results are lower bounds, recalled
  from memory; the paper was not re-read. LEAD-ICP-full-width-arity-grid FB -> BARRIER (a measurement and control, not an exponent lever).

## Cheapest falsification for each remaining flagship
- CH-L1: run CONTINUATION_PROTOCOL panel A at the smallest replayable cells, with natural targets and log_B p in {7,8,9}.
  - KILL if the lower confidence bound on per-call gamma is >= 3.5 (all-output), or >= 3 (one-output).
  - First step, and the real blocker: recover or replace the missing L1 closure source.
  - Do not substitute D(8,3) for this test; it no longer decides anything.
- CH-L4: write an executable public small-height lift selector. Then, at 8-12 bits where honest relations occur, compare rho_G of
  successful lifts with unconditioned lifts (L4-check F1). Without a selector, no census is informative. The r=7..9 height census that
  LEAD-TRF/EXO-global-lift propose is barred by CONTINUATION_PROTOCOL.

## Missing controls and cost arms
1. Whole-solve (single-target) floor: omitted wherever D(8,3) is still called decisive. Affected: pkm-tower next test, CH-D83, and the
   chase README "Next action". The README's own postscript supersedes them.
2. Target attachment: relation-source leads (lattice, Prym, f7c381, Heegner) price relations but not attaching R. Lemma T charges
   n/(2B), and generic B^2 linear algebra then forces >= n^(2/3).
3. Linear algebra B^2 and the trivial O(B) baseline: omitted in 2debfa (inner-solve exponent quoted as if it were the attack exponent)
   and in 3a4152 (3SUM compared with B^2, not with the O(B) hash).
4. First-solve vs amortised cost: MOON-1's surviving rule-2 clause reads multi-target reuse. Its warm runs at n<=7 cover a large share
   of an r~2^7 group, so a memorisation control (targets as a fraction of r) is missing.
5. Fixed-target-curve arm: isogeny-transported yield reads a cross-curve variance excess. On a fixed target curve that excess cannot
   lower cost, and isogenous curves give rho the same benefit (closure-ledger B4).
6. Provenance gap: PR #4151's W=1, t=12 value is not in this checkout (growth_law/falsify/nullw1_m2_t12_k0.jsonl and its log are
   0 bytes). PIL-6b and ENC-0.1 rely on the reported value.
7. Instrument power: every toy null is bounded by LEAD-EXO-weak-class-instrument-calibration. The instruments miss anomalous and
   small-k curves at 36 per class, p<=2^14.
8. Producer-only closures: MOON-6 rests on the generator's own kill line. Its exact identity check was never recorded; an independent
   reason (a decision oracle does not find decompositions, and B12 already makes the per-target cost O(1)) is given in the JSON.
9. Desk-refuted sample: none of the 12 rests on an artefact; level-structure-torsor keeps its "only Hasse-Weil" caveat.
