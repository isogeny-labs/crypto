# Result: K=16,627 m=4 root-phase control

The exact stored primary N83 signed-Frobenius base passed group preflight,
full orbit-model construction and seven-block pinning in the frozen one-CPU,
4 GiB container. Initial root propagation returned `Fixpoint` in
**185.023 ms** after three XOR passes, with zero decisions and zero conflicts.
Ordinary SAT began at 35.437 s and did not return before the 1,800-second
outer wall cap. The outcome is `UNKNOWN_wall_cap`: exit 124, no OOM,
**2,661,863,424-byte** cgroup peak, no worker JSON and no `sat_returned`
phase. Post-root decisions, conflicts, models, group lift and rank are
unknown. The dependent unpinned arm was not attempted.

The [run report](RUN_REPORT.md), [phase records](positive/worker.json.phases.jsonl),
[outer outcome](positive/outcome.json), [diagram](root-phase.svg) and
[rendered PDF](RUN_REPORT.pdf) contain the source-linked evidence. Root
propagation was fast in this instrumented run; the earlier arm did not
have a root marker. A new frozen control must sample completed search
progress before a solver policy or factor-base quality comparison can be
selected. This stage result does not update an index-calculus cost row.
