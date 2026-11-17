# LEAD-ICE-semaev-chain-cost-model

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id LEAD-ICE-semaev-chain-cost-model; disposition `open_next_step` (leads.json) but `closed_obstructed` in the 2026-11-01 disposition; family ic-engine; last_reviewed 2026-10-24.
- Title: "Chain length t, merge-tree topology and memory charging in Semaev's charged cost".
- Next-step excerpt (truncated): "A solver whose degree (not variable count) depends on topology, e.g. the transfer-matrix dynamic..."
- Atlas survey records: closed by cost model, t* = m* at every n in [250,600], charged cost topology-blind (EV-SEMBIN-1d36a3); binary-engine lead, prime-field relevance only as precedent.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 47, 150, 226); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only). EV-SEMBIN-1d36a3 itself was not available.

## 2. Hypothesis
H: the recorded "topology-blind" result is a property of the cost model, not of the problem. (a) The relation yield (number of ordered signed factor-base decompositions of R) is identical for every merge tree of m leaves (a set identity), and Semaev's fixed-degree model (S_{m+1} has degree 2^(m-1) per variable) assigns every tree the same degree. (b) Under the topology-aware model "each S_3 elimination doubles degree, solver degree = 2^(max leaf depth)", degree varies with topology for every m >= 4 and the balanced tree is strictly below the caterpillar. So the obstruction closes only the fixed-degree model, and the lead's next step (a topology-dependent solver degree) is not excluded by it.

## 3. Preregistered prediction and falsification criterion
- Statistics: S1 = number of distinct Semaev-model degrees across all Catalan trees at each m in 3..8; S2 = number of distinct relation counts across all trees at m=4; S3 = number of distinct topology-aware degrees per m >= 4.
- Prediction: S1 = 1 for all m, S2 = 1, S3 >= 2 for all m >= 4 (min = 2^ceil(log2 m)).
- Falsified if S1 > 1 or S2 > 1 (the invariance premise is broken, a bug or a wrong model). Inconclusive if S1 = S2 = 1 but a control fails or S3 = 1. Consistent if all hold.
- Negative control / null: the Semaev-model degree itself (must be topology-blind). Positive control: the naive proxy sum of 2^(leaves of node) must be topology-dependent for m >= 4 (detector can see topology). A/A arm: the same trees evaluated in a seeded shuffled order must give the identical multiset of counts.
- Decides: consistent means the obstruction is model-bound and the open next step is only reopenable with a solver whose measured degree depends on topology; it does NOT show a speedup.

## 4. Method
Pure stdlib. Enumerate all binary merge trees over m ordered leaves (Catalan, m up to 8, 429 trees). Evaluate the three degree/cost models above. Then build a toy curve y^2=x^3+3x+7 over F_1019 (b incremented until over 100 points), factor base = 12 points with smallest x, closed under negation, random target R drawn with fixed seed 20261110, and count ordered signed decompositions of R as a sum of 4 factor-base points through every one of the 5 merge trees by explicit point-addition tables. Seeds: 20261110 only. Cost: well under 1 minute, under 50 MB.

## 5. Classification: runnable_local
A stdlib toy decides the invariance and model-dependence claims in seconds.

## 6. What a run cannot establish
- It cannot reproduce EV-SEMBIN-1d36a3 or t* = m* over n in [250,600] (that cost model was not available).
- The topology-aware degree model (2^depth) is an assumption of this toy, not a measured Groebner or transfer-matrix solver degree; no solver is run.
- The toy relation counts say nothing about the cost of finding relations or about memory charging.
- No speedup, no IC improvement, no rho comparison. Results are [pcc:unverified].
