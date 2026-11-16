# Retry and scheduling clarification

This additive proposal resolves the retry ambiguity in the source-bound parent packet. It does not freeze a protocol or authorize execution. The prior packet remains unchanged.

**Two attempts means the initial attempt plus at most one retry.** The count follows the same exact prepared execution across owner/controller restarts; a new process or request does not reset it. A retry is eligible only for an authenticated infrastructure failure for which owner/controller launch receipts positively establish that the scientific payload never started. The exact launch/start boundary and receipt verification must be source-reviewed and prospectively bound. Missing stdout, rows, receipts, an exec error, a Pod/Job status, or an unauthenticated JSON assertion cannot establish nonexecution.

A started payload, uncertain spawn, post-spawn failure, preemption, restore, timeout of unknown classification, scientific timeout, cap or wrong answer fails closed: no automatic scientific retry. Preserve every partial row, failure and charge. An eligible no-start retry still requires reconciliation of live handles, authenticated evidence, exact unchanged inputs/instrument/seed, remaining budget and deadline, and the frozen protocol's admission conditions. A second failed infrastructure attempt ends that path.

The seven-hour (25,200 active worker-second) ceiling and 2026-10-31 06:00 UTC absolute deadline remain unchanged. Initial attempts, retries, failures, controls, warmups, validation/replay, diagnostic/owner work, collection, recovery and active reporting all count. No restart resets the ledger. Unknown accounting prevents admission. The proposed 05:30 payload cutoff and 1,800-second collection reserve only tighten scheduling; neither is a measured runtime or approved guarantee.

This is bounded, potentially censored scheduling: neither the per-execution cap nor campaign ceiling assures completion. Keep the full 1,260-target scope and separate 18-slot upper gate, recording all uncompleted slots and null unqualified ratios rather than reducing the denominator.

The distinct proposal for **one changed-instrument scientific gate remains unapproved**, pending source/counter-equivalence review and a prospective scientific decision. A diagnostic change and this infrastructure retry clarification do not reopen the known-C1 stop. The seven observed executions, four measured rows, eleven missing rows, two pencil watchdog timeouts and zero pencil solves remain recorded. Complete gate results and independent review remain prerequisites to remainder.

`clarification.json` supplies exact parent hashes, machine-readable rules and proposed final-document replacements. No source, runtime, owner or execution identity has been invented. No job, build or scientific run was launched by this addendum.
