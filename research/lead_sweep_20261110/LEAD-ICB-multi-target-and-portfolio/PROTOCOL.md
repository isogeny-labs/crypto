# LEAD-ICB-multi-target-and-portfolio

## 1. Lead as recorded
- Disposition: open_next_step (atlas 2026-10-24); a later desk pass (20261101) recorded `closed_obstructed`.
- Family: ic-binary. Title: "Batch IC and IC/rho portfolios: amortising the set-up over many targets, restart portfolios".
- Next-step excerpt (truncated): "A non-generic oracle (then amortisation matters), or IC attempt costs that are heavy-tailed with ..."
- Closure reason (docs/status/IC_STATUS_REPORT_20261031.md line 78): a restart portfolio of IC and rho cannot beat pure rho when the IC cost is at a floor ratio >= 1.
- Files read (all under /home/user/crypto):
  - research/lead_sweep_20261110/leads.json (entry)
  - research/prime_field_lead_assessment_20261107/atlas_survey.md (lines ~205-226)
  - docs/status/IC_STATUS_REPORT_20261031.md (grep context, line 78)
  - research/prime_field_leads_20261031/C3_heavy_tail_ceiling/REPORT.md (first 60 lines: self-reducibility ceiling on heavy-tail mechanisms)
- No hits in /home/user/cryptanalysis were sought beyond the above; the mechanism below is from the recorded text plus standard theory.

## 2. Hypothesis
(a) Amortisation: for a generic group of prime order q, solving K targets with one shared tame table (about sqrt(K q) points) costs total ops ~ sqrt(K q), so per-target cost scales as K^(-1/2) (log-log slope in [-0.65, -0.35]); independent per-target solving has slope 0.
(b) Recorded obstruction: if each IC attempt costs at least r*mu (r >= 1, mu = mean single-target rho cost) with a Pareto(1.5) heavy tail, no time-sharing/restart portfolio (rho share w in 0.5..0.95, restart cutoff tau in 0.5..inf mu) has mean cost below pure rho by more than 2 standard errors. Heavy tails above the floor do not rescue it; only a floor r well below 1 does.

## 3. Preregistered prediction and falsification criterion
Statistics: S1 = log-log slope of median ops/target vs K in {1,4,16,64} (shared); S2 = same slope for the independent null; S3 = same slope for the A/A repeat (different seeds); S4 = best (over w, tau) paired mean ratio-1 of portfolio cost to pure rho cost, with SE, for r = 0.1, 1.0, 1.5.
- Negative control / null: independent K=1 runs per target; require |S2| <= 0.15.
- Positive control: r = 0.1 portfolio must beat pure rho: S4 < -0.2 (shows the detector sees a gain).
- A/A arm: S3 vs S1 must agree within 0.1.
- If any control fails: verdict "inconclusive".
- Consistent_with_prediction: controls pass, S1 in [-0.65, -0.35], and for r = 1.0 and 1.5, S4 > -2 SE - 0.01.
- Falsified: controls pass and either S1 outside the band (amortisation not ~sqrt) or some r >= 1 portfolio beats pure rho by > 2 SE (the closure would be wrong in this toy model).
Decides: consistent means the recorded obstruction is re-derived (lead stays closed except for a non-generic oracle); falsified means the closure needs revisiting for the model tested.

## 4. Method
- Group: order-q subgroup of Z_P^*, P = 2q+1 safe prime, q = first safe-prime q >= 2^20 (smoke: 2^11). Generator g = 4.
- Multi-target kangaroo-style walk: 16-way partition on x & 15, steps g^m_j, m_j random in [1, 2 sqrt(q)]. Shared arm: 4 tame walks totalling ~sqrt(K q) stored points; each wild walk (start h_i g^r) steps until it lands on a stored point (collision gives x_i = t - d, verified) or a cap 4q/T+60, then restarts. Ops = every group multiplication/exponentiation, tame plus wild, divided by K.
- Null arm: K independent K=1 solves (reps/3 repetitions).
- reps = 60 per K for shared and A/A. Seeds: multi 20261110+K, null 20261111+K, aa 20261112+K, portfolio 20261113.
- Portfolio MC: pure-rho samples are the measured K=1 costs (empirical, resampled, n = 3000 paired draws). IC attempt cost = r mu U^(-1/1.5) (floor r mu); attempt abandoned at cutoff tau (cost tau) and restarted, up to 40 attempts. Portfolio cost = min(rho/w, IC/(1-w)). Common random numbers across settings; best of 30 settings reported (selection favours a spurious gain, so the 2 SE margin is conservative against the obstruction).
- Cost: well under 10 minutes, < 50 MB, stdlib only, one core. Full run is expected to take a few minutes (not yet timed; only --smoke was run).

## 5. Classification
runnable_local. A stdlib toy decides both the amortisation scaling and the portfolio obstruction in a modelled setting.

## 6. What a run cannot establish
- The IC attempt-cost law is synthetic (Pareto floor); it says nothing about real IC cost distributions, which the lead actually needs measured on real instances.
- Toy group order about 2^20 in Z_P^*, not an elliptic curve; slopes at larger q or with the non-generic oracle mentioned in the excerpt are not tested.
- Time-sharing model ignores restart state reuse and any shared IC set-up amortisation across targets (the batch-IC half of the lead).
- Slope band is judged on medians of 60 reps with a tame-table-size heuristic; constants are not optimised.
- Nothing here is an improvement over the IC baseline or a rho comparison under the AGENTS.md 5a pricing band. All outputs are [pcc:unverified].
