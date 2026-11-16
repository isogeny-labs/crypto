# Smaller bases on RTX 4090

V19 verified eight 52-bit-field executions and retained all four 63-bit limits.
Three configurations exceeded memory admission; the fourth had 143,721,864,000
right combinations, beyond the unchanged 2^34 work cap. V20 keeps twelve slots
and lowers the explicit threshold. For R=ceil(r^(1/12)), arms b/u/v use
2R+2, ceil(3R/2)+1 and ceil(7R/5)+1. Arm x repeats v at 52 bits.

The new `solve_with_oracle_profile_budget` entry point accepts a multiplier
1..=256 and caps each trial at multiplier*(columns+1) target queries.
Existing entry points retain 20; V20 uses 128 for every arm. Config layout,
relation checks, target sampling and certificate verification are unchanged.
A control-flow mock checks exact exhaustion and rejects invalid multipliers;
real GPU self-tests require identical certificates at multipliers20 and128.
These tests are pending the V20 GPU preflight at source publication time.

Smaller bases reduce table sizes but may require more target queries. The
paper's sufficient threshold L>=10m is not assumed for these explicit choices.
No coverage or speed guarantee follows from reducing the threshold.

V20 also combines the existing full and partial right-sum caches. All three
search paths already choose full cache first, partial cache second, direct
sums last. The former compile-time prohibition is removed. Both partial tables
remain in conservative admission and construction costs even when a full
cache fits. Reserve25%, max rows2^32 and work2^34 are unchanged. The new
forced-fallback fixture has three right slots and17^3 tuples, with explicit
partial-storage assertions, CPU witness equality, reprepare and reset checks.
Nine query variants run memcheck/racecheck/synccheck before any panel cell.

The earlier partial-cache experiment lost on its matched workload. Reusing
that code here makes no speed claim: this panel measures complete recovery
under smaller bases and a new explicit fallback. Its 52-bit controls use the
same combined service throughout; no causal cache comparison is prescribed.

Prior sources checked: `gpu/explicit-relations/FULL-RIGHT-CACHE.md`, actual
V19 certificates/logs and `results-v19/rows.csv`; NVIDIA CUDA Best Practices
Guide memory optimizations (linked in the cache note); user-supplied paper
sections1.2/3 and the threshold limitation retained in protocol-v20.json.
Frozen native instruments: ladder-memory.rs, analyze-memory.rs, run-memory.sh.
No source or result from V19 is overwritten. Intermediate sizes53–62 are not
measured by this two-rung panel, so its result is not a hardware maximum.
