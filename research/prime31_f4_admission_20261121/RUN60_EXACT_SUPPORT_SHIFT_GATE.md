# Prime31 run60 — exact target-visibility census and native affine-shift admission

**Date:** 2026-11-22. **Curve:** unchanged ordinary cofactor-one E/Fp, p=2147483647, a=847514846, b=2087444724, #E=n=2147446831, G=(625037715,2100627251).

**Scientific status:** independent local arithmetic census and separately measured local C++ arithmetic-width control; native original-Rust shift benchmark is committed and pending source-specific CI. There is **no public-only unknown-index S3/F4 extraction**, no newly solved blind ECDLP instance, and **no demonstrated cheaper complete index-calculus solver**.

## Prior-lead / literature gate

- In-repo: run55 exact Dickson support (509 points, 518085 distinct sums), run56 exact hybrid support (677 points, 916469 distinct sums, 88 independent public zero rows), run57 complete-rank saturation, run58 target-specific graph stop and run59 exact free-row quotient early target stop; all already in PR #4901.
- Existing algebraic extraction projects **must not be duplicated**: #4692 (selector/RR9 registered public-only protocol), #4812 (14-variable Horner/F3) and #4822 (four-variable RR9/Krylov); rootfinding #4809/#4846. #4822's accessible PR text still describes 2 positive arithmetic/model tests; it is not evidence of blind complete extraction.
- Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, *Finite Fields and Their Applications* **51** (2018), 168–182, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009): one-relation prime-field approach is prior art, not novel here.
- Yokoyama et al., *Complexity bounds on Semaev's naive index calculus method for ECDLP*, *Journal of Mathematical Cryptology* **14** (2020), DOI [10.1515/jmc-2019-0029](https://doi.org/10.1515/jmc-2019-0029): explicit cost obstruction **under stated statistical assumptions** for the naive method, not a universal impossibility result for specialized extractors.
- Shared research-KB MCP is not available in this tool runtime; used the in-repo research lineage and accessible primary publication records. Do not claim an exhaustive literature or KB search.

## Exact new real-curve target coverage, not merely random pair-template probability

The run59 estimate multiplies uniform **pair-template visibility** by all two-point support `916468/n`, which is exact only when visible templates distribute over group sums like the full support and there are no relevant collisions.

To check the approximation, independently replayed the frozen run59 F_n quotient of the 88 real homogeneous rows with **synthetic** uniformly sampled known-scalar pair constraints at m=100/400/900, trial seed `0x58ac16e3+104729`. Reduced e_i coordinates into quotient signatures modulo sign, then enumerated **all 917335 unordered signed-point pair records** on the frozen, original-curve 677-point public hybrid base. Used an independently written local C++ affine EC implementation and Python group-law spot checks; neither routine is a production attack. The exact 916469 distinct-sum total reproduces run56.

| Synthetic successful pair rows m | Favorable nonidentity signed-pair records | Distinct nonidentity recoverable group elements | Exact uniform shifted-target probability | Exact mean uniform shifts |
|---:|---:|---:|---:|---:|
| 100 | 266 | 266 | 1.23868026048e-7 | 8073108.39 |
| 400 | 240008 | 239920 | 1.11723371464e-4 | 8950.68 |
| 900 | 809006 | 808824 | 3.76644482333e-4 | 2655.02 |

At m=400 the run59 product estimator gives 1.11741184440e-4 instead of the exact 1.11723371464e-4 (**0.01594% relative overprediction** for this one synthetic cohort). Thus the model agrees very closely on this *single* cohort, with nonzero collision bias. One cannot interpret this as blind F4 success or as an unbiased relation sampler. The 4096 independently chosen signed point pairs were replayed via separate Python affine arithmetic. No arbitrary public Q logarithm was recovered.

Frozen hybrid base checksum: `sha256:522747b6cf805c025be4f59d194ec3335a57aff97528fb841cf7c2b4ec2f0162`. Snapshot quotient labels SHA256 at m=100/400/900 respectively `71079fd09e4628bfac5c8743c9d77b33a8ea078b1cc503c551e978ccbaf15b55`, `034c22b3020dbdc8a69fb51069918cd2fe3f85c736704510e16c3f009d0afc95`, `0137c1c4cef08b60fe9b1d577ae1c0a3902a3b0d344039232a5c2ee4ab79cb92`. Exact replay code/input files are attached to the research conversation; this PR records the source-specific measured checks without introducing a second production EC library.

## Native original-Rust shift-only cost falsifier

Run59's conditional 1024-target hybrid model requires **4,699,949** uniform public-shift queries. Against the historical cross-run 170.113-ms shared-BSGS control, the required *all-in* query budget would be **36.195 ns**. **This is a cross-run sensitivity threshold, not a qualified same-host benchmark** and not a lower bound over all conceivable implementations.

The independent local C++ EC-affine reference ran five 250000-addition repetitions at **203.306, 209.586, 236.810, 216.413, 208.195 ns per affine addition** (median **209.586 ns**), excluding membership predicates, root extraction, rank updates, target decoding and precomputation. This is ~5.79 times that historical per-query sensitivity, but the local C++ host is **not** the old BSGS host; never classify these values as an apples-to-apples ECDLP comparison.

**New production-side code:** `research/prime31_ic_20261115/native/examples/prime31_run60_shift_floor.rs` reuses the **existing** original Rust `Arithmetic::add`, `scalar` and `PRIME31`. It runs five 250000-shift repetitions, checks group operation counts, cross-checks each endpoint against scalar multiplication and emits the per-shift median, full 1024-target shift-only sensitivity and digest. It invokes no polynomial solver or generic MITM algorithm. The focused `.github/workflows/prime31-run60-shift-floor.yml` builds this exact example with Rust 1.100.0.

**Preregistered decision:** if the same-runner original Rust affine shift time exceeds 36.195ns by a comfortable margin, **retire the current affine-per-probe two-summand batch-speedup hypothesis under this run59 work model**, unless a new explicit batch-shift/equivalence strategy changes the counted number of operations. This does not rule out Jacobian/projective primitives, amortization, or the eight-summand Semaev approaches.

## Ranked next experiments and acceptance gates

1. **P0 — blind algebraic extraction:** in existing #4692/#4812/#4822 lanes, use unchanged original curve, held-out nonplanted public Q, matched timeout and memory. Return signed roots plus proof `[k]G=Q` where k is derived from recovered rank equations, not planted. Record positive, negative, inconclusive and OOM separately. Avoid replacing the index-calculus oracle with generic BSGS/meet-in-middle enumeration.
2. **P1 — exact S3/shift microcost:** integrate the same-runner native Rust affine shift-only price with original #4809/#4846 S3 rootfinding. Measure *negative* requests, not just successful ones. Retry with projective or batch-inversion only if it strictly reduces fully charged `T=1/32/128/1024` total cost and does not hide setup.
3. **P2 — complete solver:** feed genuinely blind roots into run59 quotient-rank+affine right-side recovery. Track useful rank per call, exceptions, factor-base setup and all target verifications. Compare against same-host best **existing index calculus**, optimized negated BSGS and rho; include CI artifact, seed, original Rust build SHA and raw timing vectors. No modeled query count can be promoted as a complete solver benchmark.

**Outcome:** actual-group enumeration checks the run59 uniform-pair support approximation to within 0.016% in one frozen cohort and exposes the tight per-shift admission budget. Only original-Rust CI can establish production primitive cost; the intended **cheaper complete ordinary prime-field ECDLP solver is still unestablished**.

## Source-specific native Rust result (2026-11-22 11:27 UTC)

**PASS:** [Actions #45017313908](https://github.com/isogeny-labs/crypto/actions/runs/45017313908), job 137192304311, original Rust 1.100.0 focused release run. The job compiled and ran `prime31_run60_shift_floor.rs`, using the unchanged original `Arithmetic::add`; all five 250000-addition endpoints matched independently recomputed `Arithmetic::scalar` and counted exactly 250000 timed group additions per repetition.

| Repetition | Original Rust affine shift ns |
|---:|---:|
| 0 | 176.460 |
| 1 | 177.130 |
| 2 | 176.702 |
| 3 | 179.990 |
| 4 | 173.960 |

Median **176.702 ns per affine shift**, 4.882× the **cross-run sensitivity** of 36.195 ns. At 4,699,949 queries this shift-only term would be **830.492 ms**, before extraction, failed probes, factor-base cost or linear algebra; historical shared BSGS was 170.113 ms on a **different host/run**. This **rules out a competitive implementation that uses one of these original-Rust affine additions per modeled query at this measured speed**, not all possible projective/batched implementations. The runner is shared and was not CPU pinned; treat this as source-linked diagnostic, not a fully isolated paired solver benchmark.
