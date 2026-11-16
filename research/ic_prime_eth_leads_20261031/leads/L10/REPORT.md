# L10 report: Lean-certified exponent rules as knowledge rows (drafts only; no repo modified)

## Draft files (leads/L10/)
- rows/KR-IC-a10001.yaml  (a) black-box D.1 cannot cross rho below 2^256-bit groups; m-balance optimal
- rows/KR-IC-a10002.yaml  (b) no linear-vertex discount; no q-ary form
- rows/KR-IC-a10003.yaml  (c) polynomial method loses to guessing m-1 windows for m <= 2d
- rows/KR-IC-a10004.yaml  (d) target-family amortisation dominated (derived arithmetic, no Lean)
- rows/KR-IC-a10005.yaml  (e1) d_reg is a regrading
- rows/KR-IC-a10006.yaml  (e2) Fermat-indicator counting has degree ~p
- rows/KR-IC-a10007.yaml  (f) ladder rule + solver-saving floor + sweep rho_floor, both conventions
- PR_DESCRIPTION.md; IcTensorRoute_axioms.lean (copy with 4 extra #print axioms); build_IcTensorRoute.txt

## Format source matched
knowledge/frontiers/ecdlp/README.md (row schema) and tools/build_frontier_map.py (REQUIRED, KINDS, STATUSES, KN_REF, verification states); existing example KR-IC-081a58.yaml; curate-knowledge SKILL.md. My schema check (required fields, kind/status enums, source refs resolve) passes for all 7 rows except as listed under "Not yet fileable".

## Build (2026-10-31)
- lake env lean IcTensorRoute.lean: exit 0, all 12 checked theorems depend on [propext, Classical.choice, Quot.sound]; arity_two_never only [propext, Quot.sound]. No sorry/native_decide (no warnings).
- lean Char2Fourier.lean: x_is_principal_root_N3, _N9, length_odd depend on [propext] only. (A network call to release.lean-lang.org was denied; harmless.)
- Not rebuilt: PrimePdpEncoding.lean (sweep, core Lean); rho_floor read from source only.

## Weaker in the Lean than LEADS.md claims (exact)
1. The cost model is an unproved hypothesis. blackbox_balance/no_crossover take E(m) = b/m + c m b/g as given (g real, arbitrary). Nothing in Lean ties it to D.1, Bochum Thm 4, or to g = sqrt(log2 b).
2. "Whatever D.1's constant is" (LEADS s2) is wrong as stated. Lean proves g > 16c; the 2^256 figure needs c >= 1, an outside assumption on an unspecified O(). General threshold log2 b > 256 c^2.
3. crossover_bits is pure arithmetic (16 < g, g*g <= lam => 2^256 < 2^lam); lam = log2 b is a naming convention, not connected to no_crossover. Chain is stitched by hand. (Also g integer > 16 gives lam >= 289, so 256 is not tight.) The docstring "exceeds 2^(2^256)" refers to the group size 2^b; LEADS "2^256 bits" is the same fact.
4. Directional notes: dropping m!, polylogs, linear algebra (3b/m in LEADS s2's own accounting vs coefficient 1 in Lean) makes the Lean hypothesis easier for IC, so the conclusion is conservative. This is fine, but "iff below rho" in LEADS s4 holds only for the relation phase (necessary, not sufficient).
5. interface_lower_bound is an injectivity cardinality (k <= s). It does not state c k/sqrt(log k), the b-bit interface, or "each addition needs >= b edges"; LEADS s4/s5 use those as if formalised. solver_saving_floor takes n >= b as a hypothesis.
6. polymethod_vs_enumeration: non-strict; only the 1/(2d) saving is in Lean; coverage of 1/(5d), 1/(2.7d) is by monotonicity (true, unstated). "d >= 2 at m = 3" and the LPTWY/BKW/Dinur attributions are not in Lean; KN-LIT-3290 has no constants. The Boolean-variable model ignores that PDP over F_p is not natively a GF(2) system.
7. ladder_rule (Nat) is cancellation of s; the p^{(1+e)/m} cost derivation is not in Lean. rho_floor (sweep) and ladder_rule agree up to the factor m and the dropped (m-1)!; they differ in strictness and units (row a10007 states both).
8. Items (b) and (d) have no Lean receipt at all: (b) is a reading of D.1's text, (d) is edge-count arithmetic. Labelled so in the rows. The eth.txt line citations could not be re-checked: scratchpad/dl/eth.txt and bochum.txt are no longer on disk.
9. Sibling REVIEW.md: its draft KR-2 (Fermat indicator) was WITHDRAWN there ("must extend" LEAD-ICB-decomposition-encoding-and-sat / KN-LIT-caa51d); LEADS.md L10 still lists it as ready. Row a10006 follows the withdrawal note. Its draft KR-1 claims equality G+1 for any Boolean system; the sources accept only <= G+1, so a10005 states the accepted bound and flags equality as unreviewed.

## Not yet fileable (needs curation first)
- No KN-LIT for Akshima et al. (Bochum) or Caminata-Gorla; no KN-FIND for the Lean receipt, so rows cite KN-LIT-caa51d / KN-LIT-3290 / KN-LIT-025 as placeholders and carry TODOs. Validator requires resolving KN refs, and a10005's KN-LIT-025 is context only, not the proof source.
- Ids are provisional (allocate_id with --seed rejected non-int; not run per row). Verification_state values copy the prior session's reading and were not re-verified.
- Statuses chosen conservatively: heuristic for model-bound rows; "proven" only for a10005/a10007 (pure arithmetic / reviewed derivation).
