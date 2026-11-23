# Finish k = 4 characteristic two at m = 18, and replay the ladder

Priority: high (queue item Q5 of `research/ic_track_ruling_20261110/DECISIONS.md`).
Funding: EKS Spot.

**Why.** `research/gaudry_k4_char2_crossover_20261015/` holds the only measured
index-calculus crossover candidate on record. It is algebraic (Gaudry k = 4
with D₄ symmetry, Macaulay solve) and in a weak family. Against rho's
theoretical 0.886·√r, the per-size geometric means of IC/rho are 3.64, 1.68,
0.95 and 0.45 at m = 15–18 (README lines 89–98).

**What blocks a claim** (the study's own list):

- m = 18 is one target, and its rho run was still running;
- two targets per size at m = 15–17;
- measured-against-measured IC/rho scatters 0.62–19 at one size;
- no session has been replayed;
- the completed runs used a checkpointing binary whose source is not on `main`
  (patch `binary/checkpoint-binary-vs-468f7513c.patch`, branch
  `claude/k4-char2-crossover-dev`);
- the red-team review projects m* ≈ 17.6 (16.3–19.2) and calls the crossover
  not established.

**Next.**

1. Land the checkpoint/resume source on `main` (its own PR, with tests).
2. Finish the m = 18 rho run.
3. Replay every session of the ladder on a second host class.
4. Add targets at m = 16–18 under the study's frozen rule.
5. Price the unit as the red team asks: the §5a band on both arms.
6. If the crossover survives, it goes through the validation gate before any
   leaderboard row.
