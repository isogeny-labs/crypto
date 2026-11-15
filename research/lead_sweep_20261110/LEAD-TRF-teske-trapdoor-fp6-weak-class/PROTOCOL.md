# LEAD-TRF-teske-trapdoor-fp6-weak-class

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_unexamined (last_reviewed 2026-10-24); family: transfer; source: atlas_survey_20261107 --pursuit.
- Title: Teske-style trapdoor over F_{p^6} built on the Joux-Vitse weak class.
- next_step_excerpt (truncated in leads.json): "For random ordinary classes over F_{p^6}, measure the cost of the public ISO-1 walk to the neare[st weak curve]..." (cost and why_it_might_beat fields absent).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-61, 282; the only substantive hit; lists the lead as an extension-field transfer, "apply to F_{p^k} curves, not E(F_p)", alongside TRF-jv-sieve-fp6-weak-class with S ratio 0.0042 vs strong rho, weak class only, not prime order). research/ecdlp_subexp_desk_20261101/data/atlas_tally.json matched but was not read.

## 2. Hypothesis
Model: a public curve E is isogenous (via a public ISO-1 walk of l-isogenies inside its isogeny class) to a member of a weak set W of fraction w; the trapdoor is the path. Claim H: for a weak set not aligned with the isogeny graph, the cost (steps) of a blind public non-backtracking walk to the nearest weak vertex equals the random-sampling cost 1/w (pooled ratio mean_steps*w in [0.85, 1.15]), so the walk is no cheaper than sampling fresh curves; only a holder of the graph (trapdoor) pays much less (BFS distance, ratio < 0.5). Toy analogue: real 2-isogeny graphs of ordinary curves over F_p (Phi_2 modular polynomial), not F_{p^6}.

## 3. Preregistered prediction and falsification criterion
- Statistic: pooled public_ratio = (mean steps of blind walk from random non-weak start to first weak vertex) * w, weak set uniform random of fraction w = 0.05, over 5 primes x 300 sets x 40 starts.
- Prediction: public_ratio in [0.85, 1.15]. Falsified if outside that range while controls are valid (<0.85: walk exploits graph structure and beats sampling; >1.15: walk is worse than sampling, i.e. isogeny-class locality hurts). If controls fail: inconclusive.
- Negative control / null: independent uniformly random weak sets (this is the primary arm; ratio 1 expected for a well-mixing walk).
- Positive control: trapdoor-informed navigation (BFS distance to W); detector valid only if informed ratio < 0.5.
- A/A arm: the null sets are split into two interleaved halves (even/odd replicate) with independent draws; valid only if relative difference of means <= 0.08.
- Descriptive extra arm: graph-aligned weak set (BFS ball of the same size); no verdict role.

## 4. Method
For p in {1009, 1511, 2003, 2503, 3001}: for each j (not 0, 1728) take the curve y^2=x^3+3kx+2k, k=j/(1728-j), count points by Legendre sums, |trace| t; keep the |t| class (ordinary, t != 0 mod p) of largest size; edges where Phi_2(j,j')=0 inside the class; use the largest connected component. Walks: non-backtracking random walk (backtrack only at leaves), cap 1000 steps. Seeds: random.Random(20261110 + p), one stream per prime; arm assignment by replicate parity. Cost: about 40 s total, memory < 50 MB, stdlib only. run.py --smoke uses p=211, tiny reps, and always reports inconclusive.

## 5. Classification: runnable_local
A stdlib toy decides whether blind isogeny walking beats sampling for unaligned weak sets; it is a prime-field proxy and cheap.

## 6. What a run cannot establish
Nothing about F_{p^6}, the Joux-Vitse weak class, ISO-1 walk costs there, or real trapdoor security: the weak set is random, not the JV class, and l=2 on prime-field toy graphs of at most a few hundred vertices (mixing is trivial; cycle-like surface). Nothing about GAE pricing or the matched-rho band. Results are [pcc:unverified].
