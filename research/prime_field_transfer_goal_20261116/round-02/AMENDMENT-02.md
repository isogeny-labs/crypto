# Preserve the secp256k1 timeout; partition the same suite by public point

The isolated p17 and p257 calls complete all seven precisions. The isolated
secp256k1 call completes through32 digits and times out at60 seconds before
emitting its64-digit record. This is a resource-limit outcome, not a failed
torsion equation or a completed64-digit check.

Partition the unchanged secp256k1 workload into four calls, one per originally
specified public point. Retain all seven precisions and every r-torsion,
source, recurrence, mutation, negation and generator-addition check. Do not
substitute scalar-generated points. Avoid computing a generator lift at lower
precisions where it is unused by the public-point checks. This changes test
chunking only; the frozen lift evaluator remains byte-identical.

Allow exactly four further calls with the same60-second limit each. No
performance claim, paid execution, timing comparison, acceptance relaxation
or replacement of earlier outputs. If an individual call still times out,
record the remaining tests as incomplete before another implementation decision.
