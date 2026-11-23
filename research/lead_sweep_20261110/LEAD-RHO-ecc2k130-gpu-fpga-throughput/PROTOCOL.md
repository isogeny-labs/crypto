# LEAD-RHO-ecc2k130-gpu-fpga-throughput

## 1. Lead as recorded
- disposition: open_next_step (last reviewed 2026-10-29; source atlas_survey_20261107 --pursuit)
- family: generic
- title: ECC2K-130 kernel throughput on GPU and FPGA (iterations per second per device and per watt)
- next-step excerpt (truncated in leads.json): "Build and pair TABLE_PHASE_POPC (phase via popc planes, CHEAPER-SELECTION.md) against the 15.88 ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; hit list from rg: research/ecdlp_subexp_desk_20261101/data/atlas_tally.json and research/prime_field_lead_assessment_20261107/atlas_survey.md (line 249, one-line table row only, no further text). CHEAPER-SELECTION.md was not found; cryptanalysis was not searched. Mechanism below is from the excerpt plus general knowledge.

## 2. Hypothesis
Replacing a multiplicative-hash selection of the additive-walk jump index with a bit-sliced selection (index bits = parities of popcount(x & mask_i), i = 0..3, 16 classes) leaves the rho iteration function's statistical quality unchanged: mean steps-to-first-repeat of the walk is within 10% of the hash-selected walk (the throughput gain of the real GPU kernel then is not paid for in extra iterations). Toy model: walk x -> x + a[sel(x)] mod N in Z/N.

## 3. Preregistered prediction and falsification criterion
- Statistic: rho = mean rho length (popc arm) / mean rho length (hash arm), plus the difference d and its standard error.
- Prediction: 0.90 <= rho <= 1.10 and |d| <= 3 SE.
- Falsified: either condition fails while both controls pass (popc selection degrades or improves iteration count beyond noise, so the kernel pairing must charge the iteration-count change).
- Inconclusive: A/A or positive control fails.
- Null / negative control: the hash arm itself is the reference; an ideal random mapping gives sqrt(pi N/2) steps (reported as ratio_to_sqrt_piN_2, expect ~1.0-1.3 for r=16 additive walks).
- A/A arm: hash arm re-run with a different seed (seed 2 vs 1); must satisfy |diff| <= 3 SE.
- Positive control: a 2-class popcount selection (popc2) and low-4-bits selection (low) must be clearly worse; popc2 mean must exceed 1.15 x hash mean, proving the detector sees degraded selection.

## 4. Method
- Pure stdlib, python3 -I run.py. Group Z/N, N = 2^20 - 3 (smoke: 2^12 - 3). 300 independent trials per arm (smoke: 20); each trial draws fresh selection masks/hash multiplier, 16 random jump constants in [1,N), and a random start; run until a point repeats (dict), record steps.
- Seeds: hash 1, hash_AA 2, popc 3, popc2 4, low 5 (random.Random). Arms run in fixed order; no shared state.
- Cost: about 5 x 300 x ~1.5k steps, a few seconds, <50 MB.
- Output: one JSON object with arms, ratio, d, SE, control flags, verdict.

## 5. Classification: runnable_local
The GPU/FPGA throughput measurement itself is blocked here (no GPU/FPGA), but the selection-quality sub-question that the popc-plane pairing depends on is decidable by a stdlib toy.

## 6. What a run cannot establish
- Nothing about iterations per second or per watt, GPU/FPGA hardware, or the 15.88 baseline figure (its unit is unknown to me).
- The toy walks Z/N, not the ECC2K-130 Koblitz group with Frobenius-orbit canonicalisation; additive-walk statistics there may differ.
- Parity planes of random masks are a stand-in for the unseen TABLE_PHASE_POPC definition.
- Results are labelled [pcc:unverified].
