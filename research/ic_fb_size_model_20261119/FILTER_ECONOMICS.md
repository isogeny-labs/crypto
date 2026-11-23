# Part B: the economics of decomposition prefilters on binary curves

2026-11-19. A desk audit, not a new measurement: every prefilter on the
binary-curve decomposition path already has a recorded verdict in this
repository. This note puts them in one frame. Numbers are quoted from the
cited studies and are **[pcc:unverified]** here (they keep whatever standing
their source has).

## The frame

On binary and extension-field curves a decomposition trial builds a polynomial
system (summation polynomial, Weil descent) and hands it to a solver (Gröbner
F4/F6, SAT, FES). Most systems have no solution. A filter run before the solver
costs c_f per system; the solver costs c_s. Let π be the fraction of systems
that are solvable, k the fraction of unsolvable systems the filter lets
through, ℓ the fraction of solvable ones it wrongly rejects. Expected cost per
relation goes from c_s/π to (c_f + (π(1−ℓ) + (1−π)k)·c_s) / (π(1−ℓ)). With an
exact filter (ℓ = 0) it pays when

    c_f / c_s < (1 − π)(1 − k)

and the best possible gain, a free and perfect filter, is about
1 / (π + (1 − π)·c_unsat/c_s), the cost share of unsolvable systems.

## What has been measured

| filter | kind | what was found | source |
|:--|:--|:--|:--|
| Perfect UNSAT oracle (the ceiling) | ideal | raising relations per conflict **5.2–9.0×**; this is the size of the prize | `research/pdp_ml_sat_ceiling_20261027` |
| Learned UNSAT predictor (x-bits, products, telemetry) | lossy | test AUC within noise of 0.5 on every family (0.46–0.56); test gain 0.79–1.11×; a fixed conflict cutoff 0.91–1.05×. Closed | same |
| Kosters–Yeo trace row / trace prefilter Tr(√b/X) = 1 | exact | 0 rejections in 29,624 checks; vacuous for odd-order targets and trace-zero bases | `research/ic_improve_20261014/03-tracezero-freerel`; `research/ecc2k_trace_homomorphism_20261109` |
| F4 early abort at degree 2 (`KIC_F4_EARLY_ABORT` 1/2/3) | exact | mode 1 no-change, mode 2 negative. Refuted nodes are ~42–43 % of reductions but already cheap; no root system refuted at degree 2 or 3; the cost sits in nodes that propagate and split, decided only at degree 3 | `research/lit2026_w4_early_abort_20261011` |
| Fall-row gate (`KIC_FALLROW_GATE`) | exact | never fires on a pipeline system | `research/ic_improve_20261015/03-fallrow-gate` |
| x-bitmap in front of the F6 pair index (n83) | exact | full-base query 4.517 s vs 4.407 s, 2.5 % slower; reverted | `research/f6_n83_xfilter_20261005` |
| Pair-table Bloom presence filter (n83) | exact | 1 GiB / 3 hashes 5.9 % faster per target than 0.5 GiB; a memory/latency tuning, not a reject filter | `research/n83_bloom_profile_results_20261015` |
| XOR decided-row cache (SAT) | solver speed-up | 1.415× decisions per second, short of its 1.5× bar | `research/n83_xor_decided_cache_20261019` |

## Reading

1. **The prize is real but it is in the solver, not in a filter.** A perfect
   UNSAT oracle would pay 5–9×, so unsolvable systems are most of the cost. But
   nothing cheap distinguishes them: target features carry no signal (AUC ≈ 0.5),
   and the algebraic certificates that do (a constant in the ideal) appear only
   at the solver's full degree, inside the same work a filter would try to avoid.
   This is what one would expect: if a cheap function of the target predicted
   decomposability, it would be leaking structure about which points split over
   the factor base.
2. **Exact cheap refutations do not fire.** Trace conditions are vacuous for the
   bases and orders in use; degree-2 Macaulay probes never refute a root system;
   the fall-row gate never triggers. Each costs its probe and saves nothing.
3. **What is left is making UNSAT cheaper to prove**, not avoiding it: tree shape
   and splitting order in F4/F6 (where refuted nodes are already cheap and the
   splitting nodes dominate), SAT-side propagation (the XOR cache's 1.4×), and
   choosing factor bases whose systems are smaller. None of these is a filter.

## Not done here

No new run. The only unmeasured exact filter found by the code survey, the F4
early abort, turned out to be measured in `research/lit2026_w4_early_abort_20261011`.
The prime-field analogue (`mitm-prime-blocked:filter=1`) filters a lookup whose cost
is native only, so it cannot move S at w = 0 (part A).
