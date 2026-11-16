# Legendre-feature residue inside P2: result
Prereg: prereg.md (sha256 ec6234d2...0a03, written before any computation). Code: legendre_feats.py, analyze.py. Raw: result_{17,19,23}.json, analysis_raw.txt.

## Result: NULL (feature rejected under the pre-declared rule)
66 P2 arm-A configs (n=17: 27, n=19: 31, n=23: 8; the 8 n=23 are the only ones P2 completed), models LR, LR2, MLP, identical splits and seeds,
paired bootstrap on test rows. Feature = additive-character analogue Tr(z^i x^e), e in {-1,3}, i<n (2n columns).

| model | pooled AUC lift (feat/base) | 95% CI (over configs) | mean AUC base -> feat | configs lift>=1.3 | feat > own q99 |
|---|---|---|---|---|---|
| LR  | 0.9986 | [0.9957, 1.0015] | 0.5009 -> 0.5002 | 0/66 | 0/66 |
| LR2 | 0.9997 | [0.9984, 1.0011] | 0.5004 -> 0.5003 | 0/66 | 3/66 |
| MLP | 0.9987 | [0.9954, 1.0019] | 0.5017 -> 0.5010 | 0/66 | 2/66 |

Floor was 1.30 with CI lower bound >= 1.30; observed CI upper bounds are ~1.002. Largest single-config lift 1.036 (LR, CI 1.006..1.070): not near the floor.
Secondary (non-binding): 5 exceedances of the per-model q99 in 198 tests vs ~2 expected by chance; not replicated across models/configs (V_b replication impossible, see below); does not change the verdict.
Control (planted Tr(z^3 x^-1)=label on 10% of rows, one config per n): detected by LR and LR2 at all n, by MLP at n=17 and n=23 but missed at n=19
(AUC 0.503 vs q99 0.517). The null is therefore powered for linear/quadratic models; the MLP arm at n=19 has power not demonstrated.

## What this does and does not show
- It shows: on P2's closed sumset-membership task, these character features add nothing detectable (AUC stays at chance), consistent with P2's closure.
- It does not test the literal residue. A Legendre symbol (v/p) does not exist in GF(2^n) (every element is a square; 2^n-1 is prime for n=17,19, so no nontrivial
  multiplicative character either). No prime-field analogue of P2 exists, and holdout C / ic-dataset rows were not touched. The literal feature needs a prime-field
  dataset and a label for which P2-like structure is conjectured; none is in the current tracks.
- Limits: V_b (fresh V) test sets were not saved by P2 stage 2, so the replication clause was not available (it could only have strengthened a KEEP). n=23 covers K0 only.
- Process: `nice` was refused by the sandbox (setpriority not permitted), so 3 single-thread processes ran un-niced for about 25 minutes (roughly 1 CPU-h total); host load was above P2's own gate of 40
  when started (55), but caller budget (<=4 procs, <=2 CPU-h) was respected. Nothing in the P2 worktree was written or committed.
