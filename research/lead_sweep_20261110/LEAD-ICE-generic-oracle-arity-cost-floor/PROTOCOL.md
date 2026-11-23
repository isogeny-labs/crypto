# LEAD-ICE-generic-oracle-arity-cost-floor

## 1. Lead as recorded
- disposition: closed_obstructed (cost_model), last reviewed 2026-10-24 (survey: 2026-11-04); family: ic-engine.
- title: "Generic (enumerative / MITM / free) decomposition oracles cannot reach rho at any arity". next_step_excerpt: empty in leads.json.
- Recorded obstruction (atlas_survey.md line 40): zero in-domain sub-rho cells for ENUM/MITM/MITM_CAPPED oracles (EV-SEMBIN-b6d042, binary model); product law relations x targets x oracle = m 2^n; free oracle at m=3 on ECC2K-130 costs 2^68.58 vs rho 2^60.81 (KN-FIND-aa2efc). Scope: oracles not reading coordinates algebraically. Prime analogue: LEAD-ICP-generic-table-decomposition.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 38-48, 136-146); /home/user/crypto/research/prime_field_leads_20261031/L1_lower_bound/atlas_update.md (lines 9-16); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (line 56 context). KB not queried (no MCP in this subagent).

## 2. Hypothesis
In a generic cyclic group of order N (2^12..2^22), an index-calculus solve whose decomposition oracle is ENUM (enumerate (m-1)-subsets, hash the residual) or MITM (table of floor(m/2)-subsets) at arity m in {2,3,4,5}, or a FREE oracle (zero cost) at m=3, with factor-base size B optimised per cell and linear algebra charged m*B^2 group-op equivalents, costs strictly more counted group operations than ideal birthday rho (sqrt(pi N/2)): every cell has ratio > 1, and ENUM/MITM log2-cost slope vs n is >= 0.5 (fitted at n=24..128 analytically).

## 3. Preregistered prediction and falsification criterion
- Statistic: R = min over decision cells (kind x m x n, toy n in {12,14,16,18,20,22}) of cost_IC / cost_rho.
- Prediction: R > 1 (no sub-rho cell). Falsified if any decision cell has ratio <= 1 (with all controls passing). Consistent if R > 1 and controls pass. Inconclusive if any control fails.
- Negative control / null: rho against itself is the A/A arm: two independent measured Floyd-rho means (different seeds, 300 trials each) per n; their ratio must be in [0.8,1.25].
- Positive control: FREE oracle at m=4 with linear algebra NOT charged must be sub-rho (ratio < 1) at n=22, showing the detector can see a sub-rho cell.
- Model validation: real counted simulation (MITM m=2,3; ENUM m=3; N~2^12) must match the analytic cost (LA excluded) within factor [0.5,2].
- Reference rho is the ideal birthday count, deliberately favourable to IC; the measured Floyd rho (poorly mixing 16-way additive walk) is 9-14x above ideal and is reported only as sanity. FREE m=4,5 (not decision cells) are reported as diagnostics; at n=22 they approach rho (ratio 2.0, 1.35), so the exponent question is open there.

## 4. Method
Cost model (counted group adds): total = B + build + (B+1)/p * (1 + lookup) + m*B^2, p = 1 - exp(-C(B,m)/N); ENUM: lookup=C(B,m-1), build=0; MITM: k=floor(m/2), build=C(B,k) (0 if k=1), lookup=C(B,m-k); FREE: lookup=build=0. B minimised over a geometric grid (x1.12, up to min(N,2^22)). N = largest prime below 2^n for toy n, exact 2^n for analytic extension n in {24,32,40,56,80,128} (slopes only). Seeds: 20261110 (+n, +1000+n for A/A; +rep for simulation). Pure stdlib; cost: well under a minute expected (rho 12 trials sets up to 2^22: ~300 trials x ~1e4 steps x 12 runs, a few seconds to tens of seconds). Output: one JSON object with statistic, cells, controls, seeds, verdict. `--smoke` uses n in {12,14}, m in {2,3}, 10 rho trials.

## 5. Classification
runnable_local. A stdlib closed-form plus toy simulation re-derives the recorded cost floor; no compute beyond one core is needed.

## 6. What a run cannot establish
- It is a generic-group counted-op model with an own cost formula (LA = m*B^2, set-membership checks free), not the EV-SEMBIN binary Semaev replication; it does not reproduce ECC2K-130 numbers.
- It says nothing about oracles that read coordinates algebraically (Groebner, rootfind, Semaev), which is the lead's stated scope exclusion.
- Toy sizes (2^12..2^22) and the analytic extension are not evidence of asymptotic exponents; FREE m>=4 is genuinely near rho and decides nothing here.
- Ideal-birthday rho ignores negation maps and distinguished-point overhead; unpriced native work (0/0.1/1 GAE band of AGENTS.md section 5a) is not priced. Results are [pcc:unverified].
