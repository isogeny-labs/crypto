# Amendment R2-M16 to the mllc protocol: PC-D sample-size cap (filed 2026-11-08, after a harness crash, before any rerun)

Protocol: `protocol.json` freeze.protocol_sha256 `674d224c94b5653265534f19bdebbdced1249b2a6ef8a7afa091496cb438e6e2`
(unchanged; this file does not edit `protocol.json`, whose `amendments` list stays `[]` until the main session records this
amendment there). All numbers below are `[pcc:unverified]`.

## Disclosure
Before this amendment was written, the main session had seen partial m=16 log lines of the crashed job
`mllc-all-20261108` (curve 0, arms up to PC-B rand1, and the traceback). The fix below is a function of the crash
traceback and of the code only; it does not depend on those accuracies, and no accuracy was used to choose it. Sizes
12, 20 and 24 finished (exit 0) in the same job; their outputs are analysed as run, whatever they show.

## Defect (measured: traceback; derived: cause)
`run_size` (mllc_harness.py) sets `ntot = cfg['n_total'][m] or (N-1)`; at m=16 `n_total` is `None`, so `ntot = N_curve-1`
(the full curve group). It then calls `fpstar_arm(m, seed, min(ntot, N-1), rng)`, which draws
`rng.choice(N_f, size=n, replace=False)` with `N_f = p-1`, the order of F_p^* for the safe prime p of m bits. The
traceback `ValueError: Cannot take a larger sample than population when replace is False` is raised whenever
`n > N_f`. Here p is `safe_prime_group(16, seed)`, so N_f need not be at least N_curve-1.

Derived for the three m=16 curves (development check, local, not evidence; seed = 20261107 + 16000 + ci):

| curve | N_curve | N_f = p-1 | old n | old call | new n |
|:--|--:|--:|--:|:--|--:|
| 0 | 63493 | 58402 | 63492 | ValueError (matches the crash) | 58401 |
| 1 | 58109 | 63598 | 58108 | ok | 58108 (unchanged) |
| 2 | 61339 | 63586 | 61338 | ok | 61338 (unchanged) |

At m=12 `n_total` is also `None` and the job exited 0, so no cap was needed there; at m=20 and m=24 `n = 700000`
is far below N_f.

## Why no other arm depends on PC-D (code read, derived)
In `run_size` the per-curve `rng = np.random.default_rng(seed)` is created inside the curve loop with
`seed = master_seed + 1000*m + ci`. Within one curve the rng draws are, in order: `rng.permutation(N-1)` (the k-sample),
`build_arms` (`rng.random` for PC-A, `rng.integers` for PC-B rand0/rand1), the relabelling-null permutations in
`run_cell` for the `curve|*` arms, and only then `fpstar_arm` (`rng.choice`). The PC-D `run_cell` is called with
`null_R = 0` and an arm name not starting with `curve`, so it draws nothing; nothing after `fpstar_arm` in that curve
uses `rng`, and the next curve reseeds. Model fits use `seed`, not `rng`. Hence the fix cannot change any curve, PC-A,
PC-B or null result, nor any PC-D of a curve that previously ran (curves 1 and 2 above have `n <= N_f - 1`, so
`min` is the identity and `rng.choice` gets identical arguments and state).

## Minimal fix
Inside `fpstar_arm`, one line inserted after `p, g = G.safe_prime_group(m, ks_seed); N = p-1`:

```
n = min(n, N-1)
```
where `N = p-1 = N_f`. Exact diff (`r2m16_harness.diff`; `mllc_harness.py` is untouched, the fixed copy is
`mllc_harness_r2m16.py`):

```
--- mllc_harness.py
+++ mllc_harness_r2m16.py
@@ -69,6 +69,7 @@
 
 def fpstar_arm(m, ks_seed, n, rng):
     p, g = G.safe_prime_group(m, ks_seed); N = p-1
+    n = min(n, N-1)   # r2m16 amendment (AMENDMENT_R2_M16.md): cap at N_f - 1 = p - 2, the same cap the curve arms use on the curve group
     ks = rng.choice(N, size=n, replace=False) + 0
     h = G.fp_star_points(p, g, ks)
     return N, ks, (G.bits(h, m), 'lsb')   # label k mod 2 == Legendre(h): smooth character r=2
```

The cap is the F_p^* analogue of the curve arms' `ntot <= N_curve - 1`. It is the only change.

## Scope of the change
- It changes only the PC-D sample size, and only where the old `n` exceeds `N_f - 1`: at m=16, curve 0, from 63492 to
  58401 samples (test split `round(0.2*n)` held out as before). Nothing else in the harness, `mllc_ecgen.py`,
  PROTOCOL.md, protocol.json, the model settings, seeds or the decision rule changes.
- Sizes 12, 20, 24 are not rerun and their outputs from `mllc-all-20261108` are analysed as run.
- m=16 is rerun once, whole (the harness writes one JSON per size), with the same seeds (master 20261107 + 1000*16 + ci),
  the same base image layers and entry-script logic as the crashed job. It runs the fixed harness
  `mllc_harness_r2m16.py` via `eks/r2m16_entry.sh 16` in image tag `mllc-m16fix-5f72e0806c` (digest in
  `eks/image_m16.json`): the a43ceee7 image plus one small layer; the original five layers are unchanged.
- The crashed m=16 job produced no `ladder_m16.json` and its partial log is not used as a result. If the rerun differs
  from the crashed run's partial log on any arm before PC-D (curve, PC-A, PC-B), that is a determinism deviation to be
  recorded (PROTOCOL.md section 6), not a reason to choose between runs.

## How PC-D at m=16 enters the decision rule
As written in PROTOCOL.md: PC-D is a planted smooth character (r = 2), predicted BLIND (section 8, prediction 3: chance for all
models), "reported as a labelled boundary, not a gate". It does not enter `decide` (the SIGNAL / NULL / UNDERPOWERED
call per size, model, label uses only the `curve` arm and the PC-A gate) and does not alter the closing condition
(NULL for all models and labels, and PC-B c = 1 detected). It appears in the `ladder` output (pooled accuracy and a
pooled one-sided binomial p < alpha_cell flag) and is reported cell by cell. The reduced curve-0 sample size at m=16 (58401 instead of 63492 samples, before the 80/20 split) is stated beside that row.

## Files (sha256, measured)
- `mllc_harness_r2m16.py` `1b1dd70268763953c9ec0ecf503f038ede85280b4aae9d017c0a7661223135c1`
- `mllc_harness.py` (committed, unchanged) `b01eeb6285af002d4808ed4b0476bdeb57f5831e41b72920092c2bfc2affbd4e`
- `mllc_ecgen.py` (committed, unchanged, copied into the new layer) `e130f26b57506fc08236e505de92d291fb701fd5b66c9461ef4882e061da7607`
- `eks/r2m16_entry.sh` `8a911a9a7d7b61cf713cbfc8ae1be7fe285f9d5ecd955a505e1b2a04b35ef2c4`
