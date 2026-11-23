# F4/F5 construction, recursion and full-pipeline continuation

Baseline: a78cc8906814af71c128aa76dba40970442e3728, including the previously
qualified single-write/vector/explicit-one-worker clear switches. Frozen
source copies and hashes are adjacent. The original n17/n23 planted and
generator systems, seeds230/231/232, remain the solver fixtures. n19's
36-variable admission refusal remains a separate implementation gap.

Questions fixed before candidate timings:

1. Does using the existing active-variable layout, top-degree-only schedule
   and exact linear-tail intersection improve the 16/22-variable recursive
   matrix solvers? Retain the previous schedule as a baseline and this
   prepared-only variant as an ablation.
2. Does completing a quadratic branch with at most16 free variables using
   the existing exact scalar Gray kernel avoid repeated descendant matrices?
   Enumerate the entire bounded cube, restore original coordinates, preserve
   lowest-variable DFS root order and caps, and verify original equations.
   Other split rules, nonquadratic branches and custom node oracles retain
   recursion. Preserve budget exhaustion as exhaustion.
   Require at least one algebraic root reduction before considering a tail.
   Before candidate execution, inspection of the baseline tree changed the
   initial proposed cutoff12 to16: the n23 tree already refutes branches with
   about14 free variables, so a12-variable tail would not address that tree.
   The bounded tail is at most65536 assignments, independent of output caps.
3. Rerun all eighteen solver arms with the final candidate, including all
   qualified GF2 switches and batched Semaev setup. Complete-root and
   first-witness decomposition remain distinct boundaries.
4. Measure cold complete DLP recovery through ecbench: factor-base creation,
   oracle setup, all relation trials, linear algebra, target descent and
   verification. Include strong signed-Frobenius rho, original F4/F5,
   optimized F4/F5, F6, and available root-solving backends. Every parameter
   changing implementation must occur in the method identity; no unrecorded
   environment override is allowed. Smaller plumbing runs are not final
   runtime comparisons. Freeze the full ecbench specs before timed sessions.

The local baseline and first existing-path diagnostic were run without
isolation solely to inspect work/profile counts. Their wall times are not
performance evidence. Native claims require dedicated ephemeral x86-64 VMs,
whole physical-core reservation, one worker, five A/A and five paired rounds,
first eligible of at most three attempts, exact original-root/group audits,
and retained failures. Freeze source/binary hashes before measurement.

Solver paired admission uses the previous exact3125-resample bootstrap and
effect threshold max(2%, twice median absolute A/A noise). Four-worker
unchanged-path controls remain mandatory for kernel dispatch. Do not pool
hosts, multiply ratios across experiments, substitute instruction profiles
for native time, or infer complete-solver gains from matrix calls.

Ecbench uses its native sealed-session admission, paired comparisons and
replay audit. At least8 independent targets per measured curve,5 rounds,
one warmup and baseline/control copies; retain every incomplete/timeout row.
Report wall runtime only where admitted, and cold counted S with all
unpriced counters and its reference/floor. No scaling claim from two sizes,
no external-fastest or whole-IC/rho win without its complete qualification.

All outcomes, analysis in Rust, per-run diagrams/plots/PDFs, raw archive,
benchmark copies and generated leaderboard/browser updates land together.
Do not alter the already frozen 20261018/20261019 measurements.
