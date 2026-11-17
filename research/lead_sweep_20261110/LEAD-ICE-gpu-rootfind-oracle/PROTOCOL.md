# LEAD-ICE-gpu-rootfind-oracle

## 1. Lead as recorded
- Disposition: open_blocked (last reviewed 2026-10-24). Family: ic-engine.
- Title: CUDA root-finding 2-decomposition oracle (embarrassingly parallel target x factor-base cells).
- Next-step excerpt (truncated in leads.json): "Execute `./bench throughput` and the fail-closed validation harness of gpu/rootfind on a real sm..."
- Atlas survey line: "no NVIDIA device has executed the kernel; 300-1000x is an estimate."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 45-50, 287-290); listing of /home/user/crypto/gpu/rootfind (no source read). The tally JSON hit was not read.

## 2. Hypothesis
The GPU throughput step is blocked (no GPU). What a CPU toy can test is the oracle's mathematical contract: for a prime-field curve and factor base FB = {points with x < p/64}, the Semaev S3 root-finding oracle (roots in x2 of S3(x1,x2,xR)=0 for each x1 in FB) returns exactly the x-coordinates of R -/+ P1, and the yield of 2-decompositions R = P1 + P2 (P1,P2 in FB) per random target equals |FB|^2/N within sampling error.

## 3. Preregistered prediction and falsification criterion
- Statistics: (a) per-target mismatches between oracle root set and direct group-arithmetic set, summed over all (target, x1) cells; (b) z = (total ordered-pair hits - T*lambda)/sqrt(2*lambda*T), lambda = |FB|^2/N (factor 2 inflates variance for ordered pairs; conservative).
- Prediction: mismatches = 0 and |z| < 4.
- Falsified if any mismatch, any planted decomposition missed, or |z| >= 4. Inconclusive if only a control or the A/A check fails.
- Positive control: planted R = P1+P2 (P1,P2 in FB); oracle must return x(P2) in 100% of cases.
- Negative control: oracle run with wrong curve constant b+1; must disagree with direct arithmetic (mismatches > 0), proving the comparison can see errors.
- A/A arm: split targets into two halves; z_AA = (s1-s2)/sqrt(s1+s2), |z_AA| < 4.

## 4. Method
Python stdlib only. p = largest prime = 3 mod 4 below 2^16, curve y^2=x^3+ax+b with (a,b) from random.Random(20261110), N by Legendre point count. FB from x in [0, p//64). T = 600 random targets (seed 20261110). Per cell: solve the quadratic in x2 via sqrt (p=3 mod 4), compare with {x(R-P1), x(R+P1)}; yield counted by direct addition. Cost: about 600 targets x ~1000 cells, estimated well under 10 min, <50 MB. `--smoke` uses p~2^10, T=6.

## 5. Classification
runnable_local. The recorded next step (GPU `./bench throughput`, validation harness on an sm_* device) is blocked here, but the oracle algebra and yield model can be checked on CPU.

## 6. What a run cannot establish
Nothing about GPU speed, the 300-1000x estimate, the CUDA kernel's correctness, fp256 arithmetic, m>2 decompositions, cost against matched rho (section 5a pricing), or any index-calculus-vs-rho gap. Toy sizes only. Results are labelled [pcc:unverified].
