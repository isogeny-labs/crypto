# LEAD-ICE-boolean-f4-f5-f6-engines

## 1. Lead as recorded
- disposition: closed_dominated (also disposition_20261101 closed_dominated); last_reviewed 2026-10-24
- family: ic-engine; source atlas_survey_20261107 --pursuit
- title: Faster Boolean F4 / F5 / F6-IC / inherited-F4 / matrix-F5 engines for the descended Semaev system
- next-step excerpt (truncated in leads.json): "Revisit when: A descended cell where the pair table is infeasible (m >= 4 at large n) and the al..."
- Recorded obstruction (secondary): inherited F4 priced ~22x-33x strong rho at n13; learn/apply F4 trace reuse gave 1.4-2.3x engine gain on the binary Boolean S3 line.
- Files read (all lines containing the id, with context):
  - /home/user/crypto/research/lead_sweep_20261110/leads.json
  - /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 47-48, 309)
  - /home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md (lines ~4056-4062, ~5170-5176)
  - /home/user/crypto/docs/session-logs/2026-11-02.md (lines ~897-901)
- Not read: the atlas YAML (encrypted), the original Boolean-F4 records (EV ids not available here), cryptanalysis repo.

## 2. Hypothesis
For Macaulay-type GF(2) matrices of fixed support and re-drawn coefficients (a stand-in for repeated Semaev descents of the same shape), replaying the pivot trace of one system on another reduces the counted elimination cost (row XORs plus pivot-candidate tests) by a median factor in [1.0, 2.3], i.e. no more than the recorded 1.4-2.3x trace-reuse gain, and an unrelated-shape trace gives no larger gain. This re-derives the recorded bound that trace reuse cannot rescue the Boolean engine line.

## 3. Preregistered prediction and falsification criterion
- Statistic: median over cells of (fresh cost / replay cost) on same-support, different-coefficient pairs (per-cell median over trials).
- Threshold: 2.3 (upper end of the recorded band).
- consistent_with_prediction: statistic in [1.0, 2.3], null (unrelated trace) ratio <= statistic, controls valid.
- falsified: statistic > 2.3 (trace reuse beats the recorded bound in this toy, which would reopen the lead for the stdlib model only).
- inconclusive: controls invalid or statistic < 1.0 or null exceeds the statistic.
- Negative control / null: trace from an unrelated random support set, replayed on the same target (expect ratio near 1).
- Positive control: replaying the target's own trace (must give ratio > 1, the maximum possible saving, else the cost model cannot see the effect).
- A/A arm: two independent fresh eliminations of the same matrix (deterministic; ratio must equal 1 to 1e-9).
- Rank equality across all arms is asserted (replay must not change the result).

## 4. Method
- Matrix: degree-3 Macaulay matrix over GF(2) (Boolean, x^2=x) of m quadratic equations in n variables, each equation multiplied by 1 and by each variable; columns = monomials of degree 3..0, high degree first. Rows are Python ints.
- Elimination: forward GF(2) elimination, column order fixed. Cost = row XORs + one unit per pivot-candidate or reduction bit test. Fresh: scan unused rows for first pivot. Replay: try the traced pivot row; on miss, fall back to a scan (counted).
- Cells (n, m, support density, trials): (9,10,0.3,12), (10,12,0.3,12), (11,13,0.25,12). Seed 20261110 (single random.Random stream, fixed order). Smoke: (5,5,0.5,3).
- Cost: well under 10 min, one core, a few MB RAM, stdlib only.

## 5. Classification
runnable_local. A stdlib toy decides whether the cost model the recorded gain rests on can exceed 2.3x.

## 6. What a run cannot establish
- It is not F4/F5 (no symbolic preprocessing, no pair selection, no signatures) and not the Semaev system; it is a linear-algebra proxy for trace reuse only.
- It says nothing about the 22x-33x gap to rho at n13, the m>=4 infeasible-pair-table regime, or GF(p) fields.
- Cost weights (XOR = test = 1) are arbitrary; no GAE pricing (AGENTS 5a) is applied.
- Toy sizes only; results are [pcc:unverified] and cannot reopen or close the lead on their own.
