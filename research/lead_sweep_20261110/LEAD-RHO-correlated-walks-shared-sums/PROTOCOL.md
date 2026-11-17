# LEAD-RHO-correlated-walks-shared-sums

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (leads.json; atlas_survey_20261107 --pursuit); the 20261101 tally records closed_obstructed. Family: generic. Last reviewed 2026-10-24.
- Title: "Correlated walks: one shared index sequence or shared precomputed sums across walks".
- Next-step excerpt (truncated): "A sharing scheme that keeps walks a deterministic function of the point while saving additions (..."
- Recorded numbers: "proof: shared index sequence 0.073x; pair table 1.027x" (atlas_survey.md line 67).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 246-248); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (lines 242-247). Full next-step text not available (truncated).

## 2. Hypothesis
In a toy additive rho (group Z_n, W=8 walks, 16 jumps), (H-B) a shared index sequence (step t uses the same jump for every walk, independent of the point) makes first collisions non-merging: at most 15% of first collisions continue to coincide for 16 further steps, versus >=95% for a point-function walk, so no distinguished-point detection is possible. (H-C) a point-function walk whose jumps are the 256 pair sums b_i+b_j of 16 base jumps keeps the steps-to-first-collision within [0.95, 1.10] of an independent 256-jump walk, i.e. the structured sum table costs no collision-rate loss (recorded 1.027x).

## 3. Preregistered prediction and falsification criterion
- Statistics: merge fraction (arm B) and ratio of mean total points visited to first collision, C over A256, per size n.
- Prediction: B merge <= 0.15 at every n; C/A256 in [0.95,1.10] at every n. Both hold: consistent_with_prediction. Either fails at any n (given controls pass): falsified. Controls fail: inconclusive.
- Null/negative control: arm B is the null for DP-compatible merging (A16 is the contrast); A256 independent jumps is the null for C.
- Positive control: A16 merge fraction >= 0.95 and A16 steps/sqrt(n) within [1.0,2.0] (birthday scale; detector sees merging when it exists).
- A/A: A16 re-run with disjoint seeds; mean-steps ratio in [0.9,1.1].
- Not decided: any saving in additions (a sums table of 256 entries is just a larger jump set; saving is accounting, not an improvement).

## 4. Method
- Walk: x -> x + jump (mod n), point-function index H(x) = ((x*0x9E3779B1) mod 2^32) >> 11, taken mod r. Arms: A16 (16 random jumps), A256 (256 random jumps), B (16 jumps, index sequence from RNG shared by all walks, step t same for all), C (index h mod 256 splits to (i,j), jump b_i+b_j mod n), A16b (A/A replicate).
- W=8 walks start at random points, advance round-robin; dictionary of visited values; stop at first duplicate (other than same walk/time). Merge test: advance both clones 16 steps from their own (walk,time) states; equal means merged.
- Sizes n = 2^14, 2^18, 2^20; 300 trials per arm per size; seeds base = 1000003*n + 7919*k (arms), +555555 (A/A). Step cap 30*sqrt(n) rounds.
- Cost: well under 1 minute total, <50 MB, one core, stdlib only.

## 5. Classification: runnable_local
A stdlib toy decides both sub-claims cheaply. See run.py (`--smoke` passes).

## 6. What a run cannot establish
- Toy Z_n, not an elliptic curve group; no true cost of group additions, no inversion batching, no GPU memory effects.
- Birthday on values only, no (a,b) relation bookkeeping or useless-collision (b equal) accounting, no DP overhead.
- Does not show the lead is closed or open in general; the recorded obstruction proof is not re-derived. Results are [pcc:unverified].
