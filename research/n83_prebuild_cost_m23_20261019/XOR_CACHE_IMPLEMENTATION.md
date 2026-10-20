# Opt-in cache implementation

The [frozen protocol](XOR_CACHE_PROTOCOL.md) precedes this change. The
existing solver remains cache-off by default. `set_xor_decided_cache`
enables the candidate and invalidates prior cache state when toggled.
`propagate_xors` marks a row only after it has no unassigned variable and
its residual is false. Cache entries clear on backjump, matrix clear or
rebuild, variable growth, row addition/rebuild, and destination row XOR.
The solver tracks final `xor_decided_skips` separately from repivot visits.

Why the cached check can be reused: a satisfied row containing only
assigned variables cannot change its residual when further variables are
assigned. A backjump can unassign a variable, so it invalidates all entries.
A matrix mutation can change the represented row, so it invalidates the
affected entry (or the whole matrix). No row is cached before its parity
is checked, and no conflict is cached as satisfied. These are implementation
invariants, checked by directed and differential tests; no broader solver
completeness theorem is claimed.

Directed tests revisit a cached row, backjump into a contradictory
assignment, rebuild, add variables/rows and toggle off. A 400-case mixed
CNF/XOR differential test enumerates all 256 assignments per case and
compares off/on verdicts, exact SAT models, original constraints, search
counters and repivot-plus-skip accounting. The independent general-field
replayer retains target-0-only acceptance for the original policy and
accepts target 1 only with the new explicit policy, boolean cache setting
and 10,000-conflict limit. Its original point, subgroup, phase, sign, row
and tamper checks remain in force.

`small-m4-cache` is a separate CLI command. Old cards, source snapshots,
binaries and evidence remain frozen. New receipts record the policy,
cache setting, conflict limit and final skip count. A wall cap can leave
skip accounting unavailable; such a pair remains censored.

The shell card uses the already authorized SSH runner and creates a new
output tree. Four constructed controls and their separate replays gate
all sixteen public searches. Commands, binary hashes and a source revision
must be pinned in the host card before execution. No experiment is reported
as executed by this implementation note; validation and launch receipts
are separate additions.

Conductor allowed the source expansion with an advisory overlap on a
read-only `src` reservation (T-861). The standalone checkout isolates this
branch from other editors. A transient local control-plane connection
failure preceded the successful MCP scope check.
