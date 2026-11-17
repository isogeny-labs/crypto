# LEAD-RHO-r-adding-walk-mixing
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (last reviewed 2026-10-24). Family: generic. Source: atlas_survey_20261107 --pursuit.
- Title: Iteration-function choice: Teske r-adding walks, mixing, hashed partitions.
- Next step (truncated excerpt): "Revisit when: A walk family with provable mixing strictly better than a random mapping (in-degre..."
- cost / why_it_might_beat_sqrt_p: see leads.json entry (no extra fields were present beyond those above).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 321 context); /home/user/crypto/research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md (lines ~1873-1879: a walk whose steps are not effectively random elements pays extra steps).
## 2. Hypothesis
The recorded obstruction: no r-adding walk beats a random mapping. Falsifiable claim on Z_N (N=1048573): the expected steps to first repeat, normalised by sqrt(pi N/2), for an r-adding walk with hashed partition is >= 1.10 at r=4, and within [0.95, 1.07] at r=64; no tested walk has ratio < 0.95 (i.e. none beats the random mapping).
## 3. Preregistered prediction and falsification criterion
- Statistic: mean steps to first repeated element over 1500 walks, divided by sqrt(pi N/2); SE from sample sd.
- Prediction: hash r=4 ratio >= 1.10; hash r=64 ratio in [0.95, 1.07]; every arm ratio >= 0.95.
- Falsified if any arm ratio < 0.95, or r=64 ratio > 1.10, or r=4 ratio < 1.02. Consistent if all prediction bounds hold. Otherwise inconclusive.
- Null/negative control: a lazily sampled truly random mapping on Z_N; ratio must be within 3 SE (+0.03) of 1.
- Positive control: r=2 hashed walk (known poor mixing); ratio must exceed 1.25 or run is inconclusive.
- A/A arm: r=64 hashed walk run twice with different seeds (4001, 4002); |z| < 3.5.
- Any failed control forces "inconclusive".
## 4. Method
Cyclic group Z_N additive, N=1048573 (abstract, so step cost is uniform; no curve arithmetic). Walk x -> x + m_{h(x)}, m_j uniform random in [1,N), fresh per trial, random start. Partition h: "hash" = splitmix-style 64-bit mix of x, reduced mod r, "mod" = x mod r (weak, linear partition, comparison arm). r in {2,4,8,16,32,64}. Detection by a visited set (exact first repeat, not Floyd). Seeds: null 1001; arms 2000+r (hash), 3000+r (mod); A/A 4001/4002; Python random.Random. Cost (mod arms with small r may run several times longer, still bounded): about 1300 steps x 1500 trials x ~14 arms, about 30M dict ops, expected 1-4 minutes, memory < 100 MB. Smoke: N=4099, 20 trials, r in {2,4}.
## 5. Classification: runnable_local
Stdlib toy decides the question directly (re-derivation of the known Teske mixing penalty).
## 6. What a run cannot establish
- Nothing about elliptic-curve groups, negation maps, distinguished points, or parallel/multi-walk cost; Z_N additive is an idealisation (steps are random group elements).
- Not a proof of mixing bounds; finite N and 1500 trials give SE of roughly 2.5%, so differences under ~5% are unresolved.
- Does not price native work (AGENTS.md 5a) and cannot show an improvement over index calculus. Results are [pcc:unverified].
