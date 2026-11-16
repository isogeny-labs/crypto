# LEAD-RHO-prime-gpu-rho

## 1. Lead as recorded
- Disposition: open_blocked (also open_blocked on 2026-11-01); family: generic; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit.
- Title: GPU rho on prime and generic binary curves with an admitted CPU/GPU comparison.
- Next-step excerpt (truncated in leads.json): "Run the gpu-match workload on a CUDA host with qualified traffic telemetry: interleaved CPU/GPU".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry only). The rg search over RESEARCH_*.md, docs, research/notes in /home/user/crypto found no other hit; /home/user/cryptanalysis was not searched. Work is from the entry plus the standard mechanism (distinguished-point parallel rho).

## 2. Hypothesis
In W-walker round-robin distinguished-point (DP) rho on prime-order prime-field curves, total group operations to solve equal the single-walker cost plus a DP-lag term: extra steps ≈ c*(W-1)/theta with c in [0.75, 2.25] (theory ~1.5: after the collision the later of two trails needs ~1/theta rounds, each round costing W steps). Hence any GPU gain in an admitted CPU/GPU comparison must come from throughput per op, not from fewer counted ops; W/theta must be charged against sqrt(pi n/2).

## 3. Preregistered prediction and falsification criterion
- Statistic: c_vs_W1 = (mean steps W=16 at theta - mean steps W=1 at same theta) / ((W-1)/theta), and c_dose_slope = (mean steps at theta_B - at theta_A, W=16) / ((W-1)*(1/theta_B - 1/theta_A)). Computed at n ~ 2^16 and ~ 2^20.
- Threshold: all four values in [0.75, 2.25] => consistent_with_prediction. Any value outside, with controls valid => falsified (the lag model is wrong; counted overhead is not W/theta). Controls invalid => inconclusive.
- Null / negative control: walkers with DPs disabled must solve 0 of 20 instances within 30*sqrt(pi n/2) steps (the detector cannot report solutions without DP collisions).
- A/A arm: W=1 at identical theta run twice with disjoint derived seeds; |diff| must be <= 3 combined SE.
- Positive control: dose-response, W=16 mean steps must increase from theta_A to theta_B (larger 1/theta), and the effect (W-1)/theta_A*0.5 must exceed 3 SE of the A/A noise (power check). Every recovered key is verified as kP == Q.
- Decides: only the op-count part of the lead. It does not decide the CPU/GPU admitted comparison.

## 4. Method
- Curve y^2 = x^3 + x + b over p ~ 2^16 and ~ 2^20 (p = 3 mod 4), smallest b giving prime order n; point counting with a squares table; P any point.
- Walk: 16-partition r-adding walk (partition x mod 16, random multipliers M_j = u_j P + v_j Q), DP when (x>>4) mod 2^k == 0; trail cap 20*2^k then restart; round-robin W walkers; DP table collision with differing (a,b), b differing, gives k. No negation map.
- Arms interleaved per trial: W1_A, W1_Aprime (k=2 / 3), WW_kA (W=16, same k), WW_kB (W=16, k=4 / 5). Trials 1000 (2^16) and 600 (2^20), paired keys per trial. Per-trial cap 50*sqrt(pi n/2); more than 1% unsolved => inconclusive.
- Seeds: key seed 20261110*1000+bits*100000+trial; walk seed key-seed-base+trial*10+arm+1; negative control seeds 7+t, 99+t.
- Cost: about 5M group steps, ~1 min estimated, one core, <100 MB, stdlib only.

## 5. Classification
runnable_local. The real next step needs a CUDA host and is blocked, but a stdlib toy can sharpen the op-count accounting (DP-lag) that any admitted comparison must charge.

## 6. What a run cannot establish
- Nothing about GPU or CPU wall-clock, memory traffic, telemetry, or the admission gates; no hardware comparison.
- Toy sizes (n ~ 2^16..2^20) with W=16, not GPU-scale W (1e4+) where W/theta vs sqrt(n) behaves differently; extrapolation is not supported.
- Not a matched-rho/5a priced comparison and not a leaderboard row. Results are labelled [pcc:unverified].
