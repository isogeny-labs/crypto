# Skeptic review: xor-ribbon-build [pcc:unverified]

Reproduced: release example rerun at 1M queries / 8 seeds (not the full 8M/100). It gave the same distinct-key counts (16232, 257607) and hit rates (1.937e-3, 3.068e-2). xor8 bits/key 9.856, ribbon 8.8/9.2, Bloom 11.54, FPR about 3.9e-3 at f=8 all match raw_output.txt. memory_model.py output is byte-identical to model_output.txt.

Findings (none refute the headline, which is already negative/modest):
- Memory-bound gains (1.06-1.55x) are an analytic model with assumed sizes, not a run, and the p83 rows use a prime-field model. They are memory-lever results under a hypothetical budget, not counted-GAE gains vs the incumbent (which is not memory-bound). Lever label memory_only is right.
- Candidate vs baseline: the filter arm pays verification (1 GAE/candidate); the exact-packed baseline pays none, which is consistent. Native probes are zero at w=0 for both arms, which favours the filter's cost structure only through the budget cap.
- The baseline is the author's own packed-table assumption, not a measured incumbent; the 0.80-0.82x membership-only and 0.41x ribbon rows show the sign flips easily with assumptions.
- Per-trial cost 1.0479 GAE is calibrated to the 50,875/48,550 record, not the brief's 40.1k; hit-rate/factor-base mismatch (1.945e-3 vs 1.79e-3) unresolved.
- Queries are random on-curve x, not real walks; FPR on real walk endpoints is untested.
