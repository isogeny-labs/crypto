# Confirmation: branch traces and deterministic trace growth on jv_cover (frozen before data, 2026-11-24 16:35 UTC)

**Question.**
- Do branch traces make `jv_cover --exp dlp --trace` faster end to end?
- Is its counted cost now independent of thread scheduling?
- Are relations and the logarithm identical?

**Binary.** One build of this branch (sha256 `95af01e3cced8389…`). It is a same-binary A/B: no second build.

**Arms.** All arms run `jv_cover --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --trace` with RAYON_NUM_THREADS=4; the stop rule is the default Hilbert rule.
- `B1`: branches on (default).
- `B1a`: a second copy of `B1`, the A/A control.
- `B0`: `F4_FP_BRANCH=0`, branches off. Same warm-up and determinism.

**Schedule.** 10 rounds. Each round runs the three arms once, in an order rotated by the round. No build or benchmark runs at the same time.

**Identity and determinism.** Every run must find 31 relations from 19,020 residuals and a correct logarithm. Counted oracle multiplications must be the same in every run of an arm:
- B1 and B1a: 7,003,932,907;
- B0: 7,302,797,284.

These values come from pilot runs on this binary: three identical B1 runs and one B0 run.

**Predictions.**
- Counted S: B0/B1 = 298.081/285.951 = 1.042, exact in every round.
- Wall B0/B1: median in [0.97, 1.05]. A paired pilot showed no measurable wall effect: branches move about 4 % of counted F4, and replay was already cheap. A null is the expected outcome and will be reported as one.
- A/A B1/B1a: within ±3 % median.

**Conditions.** One shared 4-vCPU VM, isolation L0, no PMU. The VM is about 30 % slower today than on 2026-11-21 (the same old binary: 8.1 → 10.8 s), so wall times are compared only within this run. All results are `[pcc:unverified]`.
