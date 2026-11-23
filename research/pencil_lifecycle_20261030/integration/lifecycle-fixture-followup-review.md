# Lifecycle fixture and remaining integration review

Read-only 2026-10-30 after parent reported release compilation and six child fixture failures from missing required method parameters. No builds, tests, solver calls or source changes by this reviewer. Parent is correcting rho trial_cap and pencil required parameters.

No further deterministic fixture failure identified statically after parameter correction. Two concrete fixture coverage corrections remain useful:

1. fixture currently puts the selected method's full configuration hash into all three launch arms. Compute each arm's ResolvedMethod and full JSON Value hash separately using its own required parameters. Current child validation checks only selected arm, so incorrect unrelated hashes can silently pass these fixtures.
2. fake_callback_all_arms loops over false/true, meaning rho and pencil only. Explicitly select pencil, rho, rho_AA to substantiate all-three-arm coverage; both controls should lack prefix IO. This remains fake callbacks, not workload construction.

The panic fixture uses separate roots and a subsequent fresh scope, so prior exclusive files do not invalidate cleanup verification. Normal errors retain lifecycle Error; panic does not publish finished. Current child_main remains gated off.

## Minimal remaining integration before activation

- Keep the actual rebuilt-workload ID plus full Value hash check before solver hooks. Scope the common lifecycle around the actual method call for every arm and install the pencil prefix only for candidate. Preserve returned mathematical report independently of diagnostic summaries, with explicit diagnostic failures.
- Runner currently ingests only input.diagnostics (v1). Add a distinct v2 custody branch after child quiescence, read lifecycle using exact context identity, ingest pencil prefix only when applicable, and bind the exact sealed record bytes. Never present a partial prefix as accepted result or cost. Missing lifecycle or interrupted writer remains absent/error, not zero or successful completion.
- Resource API currently has no runner callsites. Begin outer envelope before diagnostic context/directory and ChildInput preparation; capture actual child start at successful spawn (not before run_child), then reap, record durability, diagnostic durability and session durability in order. Failure branches preserve only reached stages. Add an actual spawn observation hook or equivalent precise event rather than inventing child-start time from caller timing.
- Publish prefix and then its observed publication tail, retain the journal separately, and leave final tail pending until exact external closure. closed_payload_wall_ns is wall only; total_active_worker_ns stays null here. Do not relabel timeout, controller lifetime or sum of nested spans as active work.
- Existing timeout parameter remains unchanged. Resource and custody instrumentation must not extend it or silently retry a failed scientific execution.

These are integration gaps in a deliberately disabled slice, not evidence that an enabled run emitted incorrect results. No end-to-end, admission or all-cost qualification follows from source fixtures.
