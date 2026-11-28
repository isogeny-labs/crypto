# Prime31 run61 — fixed-generator mixed Jacobian public-shift ablation

**Status:** original-Rust source checked in with a pinned CI gate; full native timings must be read from that workflow. Do not infer an ECDLP or Semaev extraction improvement from an EC operation kernel.

## Scope and decision

Following [run60](RUN60_EXACT_SUPPORT_SHIFT_GATE.md), the original Rust affine scalar-step operation cost **176.702 ns median** in [Actions #45017313908](https://github.com/isogeny-labs/crypto/actions/runs/45017313908). A hypothetical 1:1 affine-per-public-query implementation of run59's **4,699,949** conditional 1024-target probes would pay **830.492 ms** merely for group shifts (different-host sensitivity: historical 170.113-ms BSGS). That does not rule out a faster coordinate system. The next falsifiable component is the fixed-generator mixed Jacobian addition, evaluated against *the same original curve and original Rust affine backend in the same process*.

## Standard projective formula; no mathematical novelty claimed

For prime field p=2147483647, Jacobian `J=(X,Y,Z)` represents affine `(X/Z²,Y/Z³)`. For the **unchanged** `G=(625037715,2100627251)`, when `J+G` is nonexceptional:

```
ZZ=Z²; U=Gx*ZZ; S=Gy*Z*ZZ; H=U-X; r=S-Y
HH=H²; HHH=H*HH; V=X*HH
X'=r²-HHH-2V; Y'=r*(V-X')-Y*HHH; Z'=Z*H  (mod p)
```

This is a standard mixed projective addition, not an index-calculus invention. The test **fails** on `H=0` rather than incorrectly treating a doubling or infinity as a normal step. It starts from `2G`, never reaches an exceptional branch during its registered 1.25-million additions (far less than n), and cross-checks all 1024 initial steps plus each 250000-step endpoint against original Rust affine addition and scalar multiplication. A separate independently implemented Python affine/Jacobian mathematical replay checked **10,000** steps and 1,310 intermediate affine endpoints.

`research/prime31_ic_20261115/native/examples/prime31_run61_mixed_shift.rs` uses `PRIME31`, `check_curve`, `Arithmetic::add`, `Arithmetic::scalar` from the original slim Rust crate, with no new factor-base, F4 or EC solver library. `.github/workflows/prime31-run61-mixed-shift.yml` pins Rust 1.100.0 and source-specific output.

## Measurement and admission conditions

Registered five 250000-step repetitions for **both** the original affine and specialized Jacobian kernel within the same source-specific Actions run, recording each repetition, checked endpoint and median. Report `affine_median/Jacobian_median` but **do not** report any end-to-end attack speedup. The cross-run **36.195 ns/probe** sensitivity still applies *only* to an algorithm that truly needs one such step per uniform query, and excludes all algebraic extraction. Promotion requires an independently verified public-only unknown-index relation extractor, complete signed ECDLP logs and fully charged paired same-host comparisons against optimized existing Rust index calculus, negated BSGS and rho.

**Leading hypotheses:** (a) mixed projective avoids an inversion per shifted query and may substantially improve the affine kernel; (b) if the projective kernel exceeds 36.195 ns, *even shift-only* violates the historical sensitivity for the exact run59 query model; (c) if it is below, the dominant unresolved expense becomes blind root extraction and negative-query handling. None of these is a complete-solver result.

Literature/previous work checked: run55–60, #4692/#4812/#4822/#4809/#4846, Amadori–Pintore–Sala (2018) prime-field one-relation strategy and Yokoyama et al. (2020) naive Semaev conditional complexity bound. Standard projective group formulas are classical; no priority/novelty claim.

## Frozen source-specific Rust result (2026-11-22, 11:31 UTC)

**PASS:** [Actions run #45017580013](https://github.com/isogeny-labs/crypto/actions/runs/45017580013), job 137193135104, Rust 1.100.0 source-linked release build (12.89 s). On this runner, all **1,024 prefix** Jacobian/affine equality checks and every subsequent **250000-step endpoint** against the original affine arithmetic and scalar multiplication passed. No exceptional cases arose in this registered < n range.

| Repetition | Mixed Jacobian ns/shift | Original affine ns/shift |
|---:|---:|---:|
| 0 | 27.077 | 282.851 |
| 1 | 26.850 | 283.770 |
| 2 | 26.943 | 282.852 |
| 3 | 27.116 | 284.974 |
| 4 | 26.846 | 282.940 |
| **Median** | **26.943** | **282.940** |

**Measured same-process component speed ratio:** **10.501×** for fixed-G mixed Jacobian versus original `Arithmetic::add` *in this test harness*, not a complete ECDLP algorithm. The cross-run sensitivity threshold of **36.195 ns per total public query** is no longer violated by the shift alone: 4,699,949 × 26.943 ns = **126.632 ms** shift-only. Against the unrelated historical 170.113-ms shared BSGS, the *hypothetical* remaining allowance would be only **43.481 ms total**, or **9.252 ns per query** for **everything else** (S3/F4 extraction, unsuccessful probes, hash lookup, factor-base setup and rank). That 9.252-ns requirement is a sensitivity, **not** evidence that blind algebraic root extraction is achievable in that budget.

A different source-specific run60 workflow showed original affine 176.702 ns median; run61 shows 282.940 ns because the benchmark loops and runner conditions differ. Use **run61's within-process paired ratio**, not a cross-run 176.702/26.943 quotient, for the component comparison. The benchmark is on a shared GitHub runner and lacks a matched complete-solver control, CPU isolation and uncertainty bounds across independent hosts.

### Decision for subsequent iterations

Promote mixed Jacobian fixed-generator addition as an **eligible public-shift primitive candidate** (correct and fast under this limited test); do not promote any end-to-end solver claim. Next integrate it behind an opt-in switch in the **existing Rust S3/RR9 index-calculus pipeline**, not a new solver, then measure **negative root extraction cost** and exact all-in public-query budget for 1,32,128,1024 targets. Preserve original affine backend and verify every reconstructed log via `[k]G=Q`, then compare to strongest same-runner original Rust IC/BSGS/rho and monitor memory. The missing non-generic extraction remains decisive.
