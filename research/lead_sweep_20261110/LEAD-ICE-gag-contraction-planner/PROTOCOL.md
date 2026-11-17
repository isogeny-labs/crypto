# LEAD-ICE-gag-contraction-planner
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step. Family: ic-engine. Source: atlas_survey_20261107 --pursuit. Title: empty in leads.json.
- Excerpt (truncated): "Under a frozen experiment.protocol/v1, measure matched presentations of the same instances with ..."
- Atlas survey line (2026-11-01): n=m=3 over F_{p^3}, p in {1009,4099,32749}, msolve F4, 30 planted instances: monolithic S_4 used 0.110x the chained system's F4 matrix cells; tree 1.095x (inconclusive); descriptive counted metric, "not an ECDLP cost claim". Next: prime-field m=3 and m=4 (chain vs tree) below the 2^16 msolve bound.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 41-55, 220-222); /home/user/crypto/docs/session-logs/2026-11-01.md (grep context only).
## 2. Hypothesis
A planner that ranks presentations of the Weil-descended Semaev relation system by a pure counted-Macaulay-matrix proxy (affine monomial counts, first degree D with rows >= cols, cells = rows*cols) ranks monolithic S_{m+1} as strictly MORE expensive than the chained S_3 system for every (m,n) tested, and cannot distinguish chain from tree (ratio exactly 1). Both rankings contradict/underdetermine the recorded msolve result (mono 0.110x chain; tree 1.095x), so the proxy is not a usable contraction planner.
## 3. Preregistered prediction and falsification criterion
- Statistic: mono_over_chain = cells(mono)/cells(chain) per (m,n); tree_over_chain.
- Prediction: mono_over_chain > 1 for all configs and tree_over_chain == 1.0 exactly. Falsified if any mono_over_chain < 1 or tree_over_chain != 1 (the proxy then has planning content; follow with real msolve at the recorded sizes). Inconclusive if a control fails or a cell has no D below cap 400.
- Positive control: "dup" presentation (doubled equations); must give a ratio != 1 (detector sees a change).
- A/A arm: chain vs chain, must equal exactly 1.0.
- Negative control / null: tree vs chain is a topology-blind null for this proxy; ratio 1 is expected, not evidence of equivalence for real F4.
## 4. Method
Pure integer arithmetic, no randomness (seed 20261110 recorded only). Total degree of S_k = (k-1)*2^(k-2). Presentations: mono = n eqs, n*m vars, deg S_{m+1}; chain/tree = (m-1)*n eqs, n*(2m-2) vars, deg 4. Configs (m,n): (3,3),(3,2),(4,3),(3,4),(4,2). Cost: well under 1 s; math.comb only. Field equations and p are not modelled.
## 5. Classification: runnable_local
A stdlib counting toy decides the proxy question; real msolve F4 is not required for this narrow hypothesis.
## 6. What a run cannot establish
Nothing about real F4 cost, the prime-field m=3/4 msolve runs, or ECDLP cost; the proxy ignores sparsity, field equations, Weil-descent structure and degree fall. A falsified/consistent verdict concerns only the proxy. Result labelled [pcc:unverified]. Not an improvement over the IC baseline.
