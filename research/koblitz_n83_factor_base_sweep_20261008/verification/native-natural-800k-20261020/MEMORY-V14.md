# Correct worker memory declaration before further 8 GiB attempts

Frozen before any v14 worker. Keep original PROTOCOL.md, MEMORY-V13.md,
all three actual prior attempts, cards/receipts/relays/assessments unchanged.
K2048-v13 at6131efb96d024d1e6ebe653dace461425650d743 set Docker8GiB but
incorrectly passed4096MiB to the retained worker. Worker stderr rejected the
mismatch: capacity worker requires the declared hard memory cap and zero swap.
Exit1,OOMKilledfalse,cleanupverifiedtrue; outer376.218476ms, exact mechanical
command10.146594709s. No run directory, query or SAT search started. Native
admission refused missing terminal without wall censoring. Actual relay and
receipt digests and verbatim output passed director assessment. This is an
implementation error, not a mathematical result or an8GiB search failure.

K4096-v13 was not compiled or executed. Its planned work is superseded by
K4096-v14 with the corrected declaration. K2048-v14 is a separately frozen
repeat; no previous outputs are overwritten. Same800000conflicts/4800worker
seconds/5400command seconds,8GiBbytes8589934592,MemorySwap equal memory/no
extra swap. All original query/worker/input/CPU/isolation/correctness gates
remain. K2048 once -> assess -> K4096 once. No heavy task work during timing.
New two-arm worker maximum9600seconds. Cumulative maximum24000across five
actual-or-newly-planned attempts, counting the aborted v13's full4800 envelope;
observed costs are separate and no unused historical allowance is inferred.
No automatic timed retry; preserve and assess any further failure.

Guard-v14 SHA25602adb545e54f51f35988efcce7f9a5b1be9a83e08db1a91dc4e66f2f3647011b.
It derives the worker MEMORY_MIB argument from the same frozen byte limit
used for Docker memory/swap and native resource checks:4GiB->4096MiB,
8GiB->8192MiB. Worker is unchanged, not rebuilt; no encoding/search changes.
Native22unit+7execution-card tests pass; regression checks the actual worker
argument vector, retained policy/seed/solver caps, position/count and declared
MiB converted to bytes against both allowed cgroup envelopes and both K values.
No claim that unit tests establish a successful worker launch: the new actual
live worker config and full audit are required. Keep v12/v13/v14 ELFs and logs.

Commit clean source/addendum before new native freeze/cards; bind this addendum,
both original protocols and the assessed prior refusals. v14 specs use this
addendum's protocol hash, explicit memory8589934592 and cpu_reservation[14,30].
Native director compile -> exact mechanical stdout -> director assessment at
unchanged HEAD. All report/rawS3/freshGET/payload/native replay/repack gates
remain, including preservation of current S3 access failures. A successful
bounded8GiB terminal does not repair4GiB execution or establish cold/rho gains.
