# RESULT: LEAD-ICE-generic-oracle-arity-cost-floor

## Outcome
- status: ran to completion, single run, no retries.
- exit code 0, wall 72 s (run.status: exit=0 wall=72). run.stderr empty.
- RESULT.json: one JSON object, 132 cells.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "R > 1 (no sub-rho cell). Falsified if any decision cell has ratio <= 1 (with all controls passing). Consistent if R > 1 and controls pass. Inconclusive if any control fails."

Measured:
- R = min over decision cells of cost_IC / cost_rho = 7.09 (vs ideal birthday rho). n_sub_rho_cells = 0.
- Null / A/A (rho vs rho, 300 trials each): ratios 1.12, 0.95, 0.95, 0.91, 1.02, 1.03 for n = 12..22; all in [0.8, 1.25]. aa_ok = true.
- Positive control (FREE m=4, LA uncharged, n=22): ratio 0.101 < 1. positive_control_ok = true.
- Model validation (real counted simulation vs analytic, LA excluded): MITM m=2 0.87, MITM m=3 0.84, ENUM m=3 0.84; all within [0.5, 2]. model_validation_ok = true.
- Verdict: consistent_with_prediction. All controls passed.

## Reading
- The numbers support run.py's verdict field: no decision cell is sub-rho and R = 7.09.
- Slopes (log2 cost per bit, n=24 up to max): ENUM_m4 1.006, ENUM_m5 1.000, MITM_m4 0.971, MITM_m5 0.894, all >= 0.5 as hypothesised. ENUM_m2, ENUM_m3, MITM_m2, MITM_m3 and FREE_m3 report inf: no finite cost at the analytic sizes (presumably p too small or B hits the grid cap), so the slope claim is untested for those cells. They are not sub-rho evidence.
- FREE_m4 slope 0.64 and FREE_m5 slope 0.34 (< 0.5). These are diagnostics, not decision cells, and the lead's claim excludes them, but FREE m=5 below slope 0.5 means the exponent question for free oracles at high arity is not closed by this toy. This matches PROTOCOL.md section 3, which flags the open case.
- Surprise or caveat: the measured Floyd rho over ideal birthday ratio grows from 8.5 at n=12 to 215.5 at n=22 (rho_vs_analytic), much more than the 9-14x quoted in PROTOCOL.md. The walk is poorly mixing, so it is only a sanity check. The decision statistic uses ideal rho, which favours IC, so the 7.09 floor is conservative in the right direction.
- The toy tests a self-defined generic-group cost formula (LA = m*B^2, membership checks free). It does not reproduce EV-SEMBIN or ECC2K-130 numbers, and says nothing about algebraic oracles (Groebner, Semaev). Toy sizes 2^12..2^22 do not establish asymptotic exponents.
- Unpriced native work (0/0.1/1 GAE band, AGENTS.md 5a) is not priced.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: in this model, ENUM/MITM/FREE(m=3) cells never beat ideal rho at toy sizes, which re-derives the recorded generic-oracle cost floor (scope: non-algebraic oracles only).

## Standing
[pcc:unverified], single local toy run, no independent replay.
