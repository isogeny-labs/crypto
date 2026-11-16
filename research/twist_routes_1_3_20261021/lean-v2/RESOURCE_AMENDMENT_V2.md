# Lean resource amendment, version 2

The initial prerequisite builds were retained: under a 4-GiB virtual-address
limit, the Lean runtime could not create a thread, and with a smaller
mimalloc reserve it could not map an imported object. These are environment
failures, not proof successes or mathematical counterexamples.

Lean maps thousands of cached proof objects. For subsequent correctness-only
shards replace the virtual-address cap with Lean's 3,072-MiB managed-memory
limit and an explicitly sampled 4-GiB resident-memory guard. Use one worker
and one CPU affinity, and retain peak sampled RSS and a 600-second kill
deadline. The guard samples /proc every 25 ms; it is not a kernel-enforced
instantaneous RSS bound. No wall-time ranking is admitted. An RSS/deadline
kill is an incomplete proof shard. Freeze the wrapper before use and record
its source hash, theorem source, toolchain, inputs and exit status. Keep the
original address-limited failures. All proof/audit acceptance rules and all
stated open bridges from PROTOCOL_V1.md remain unchanged.
