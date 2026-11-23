# Native continuation: frozen before timing

User request: finish native runtime comparisons of F4/F5, F6, the generic
Boolean/generalized root solvers and the Semaev rootfinding backend, then
continue optimizing measured bottlenecks. Baseline is PR #2231 commit
`1340a6176701b3f541a36112a02a3d4a90d1075a`. The existing instruction study
and all its failed attempts remain immutable.

Run on one ephemeral c7i.2xlarge, on-demand, TTL 90 minutes. Build before
timing, keep the release binary and digest, use one Rayon worker, reserve a
whole physical core including its SMT sibling, and retain every isolation
receipt, output, exit status and command. Admission requires exit zero,
`run.contended == false`, no residual user thread on reserved CPUs, exact
input/output checks and matching counted work for an implementation change.
Three bounded attempts per sample; select the first admissible attempt.
No changes to the isolation thresholds. Local shared-host results are not
substitutes for this run. VM neighbours/frequency remain subject to A/A noise.

Primary matrix: original K1 n23, seed230, planted target, degree4 and22
variables. Five A/A pairs with the old kernel, then five alternating A/B
pairs, old/new single-write switch in the same binary. Scaling: five A/B
pairs at n17 seed230, and fresh n23 seeds231/232. Retain one attempt for each
n19 arm/configuration even if existing matrix limits refuse it.

Complete-root comparison: `solve-f4`, `solve-f5`, `solve-inherited`,
`generic-gb`, `roots-gray`, `roots-moebius`, `roots-monica`, and
`generalized-native`. Five interleaved rounds on the same planted/random
targets at n17/n23, seed230. Every complete sorted set must equal independent
Gray enumeration. A/A controls repeat Gray twice per round before comparison.
The generic basis retains its existing50-second internal budget. A timeout
or capped answer is missing performance evidence, never a negative verdict.

Decomposition: `decomp-f4`, `decomp-f5`, `decomp-f6`, `decomp-f6-pairs`,
`decomp-generalized`, `rootfind-mitm` (actual `ic_framework/rootfind`) and
`rootfind-table`. Same four fixtures and five interleaved rounds, with F6
A/A controls. All completed verdicts agree and witnesses are re-added to
the original target. Charge per-process table/oracle preparation once in the
cold result; retain query-only values separately. Common public curve/base
construction is outside this solver boundary for every arm. Generalized
decomposition includes system encoding, complete solving and lifting.
All solver comparisons use the candidate table switch, with matrix A/B
separately qualifying its effect. No comparison conflates all roots with a
first witness, or this solver boundary with an entire IC/rho pipeline.

Report per-arm median/min/max and five paired ratios. Use all3125 paired
bootstrap resamplings for percentile95% intervals. An implementation gain
requires a ratio above1 beyond max(2%, twice the measured A/A absolute noise)
and an interval excluding1. Preserve losses, neutral and inconclusive results.
Native runtime is a separate metric from the existing Callgrind Ir results.
Hardware PMU support is probed; unavailable cycles/IPC remain missing.

Optimization follows baseline observation: inspect retained instruction
profiles and native costs, freeze each new hypothesis before its own timing,
retain the baseline build and source, check complete outputs and account for
setup/verification, and use the same controls for any claimed improvement.
No default switch changes without qualifying native evidence and holdouts.
