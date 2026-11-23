# Native scalar controls, before execution

This is a source-formula and parameter-certificate check in Rust BigUint,
not a graph allocation, production Setup call, solver or speed measurement.
The frozen PROOF-MAP remains the original pre-proof plan.

The positive witness uses M=s=2^262150, p=M/2+1, maximum_degree=2,
lambda=262150, sqrt=512, delta=2^235935, u=2, a=4, L=16,
array=64^8, t=2. Acceptance requires exact source normalization,
bit-length lambda, sqrt certificate, delta^10=s^9 certificate,
nonzero-degree branch, every scalar parameter guard, active valuation,
shift/power equality, two independent ceiling expressions, root35 and the
normalized546E+2 identity. Constant34 must fail on this witness.

Seven single-field corruptions must be rejected: lambda+1, u+1, delta+1,
sqrt+1, maximum_degree=delta+1, inactive t=4, and wrong array=64^7.
These controls test source binding as well as the final inequality.

A deterministic bounded scalar sweep uses u=1..12, M=32u..32u+127,
p in {0, floor(M/2)+1, M}, and i=0..u-1: exactly29,952 rows.
All must satisfy35. Retain an exact rational maximum per u by integer
cross multiplication; no floating point or timing comparison. These small
rows satisfy the weaker margin-domain hypotheses, not the full array guard.
They do not prove the universal result; the shared Lean theorem does.

Run three explicit scalar premise-deletion witnesses from RootControls.lean
and the E=32 constant34 counterexample. Each must have the stated remaining
premises and fail the weakened conclusion; no Setup admission is asserted.

Compile and qualify one copied read-only binary with Rust1.99.1 and cached
num-bigint0.5.1/num-traits0.2.19, offline, one build worker. Source and binary
hashes precede calls and must remain unchanged afterwards. The whole-cgroup
stop stays15,032,385,536bytes; phase ceiling600s, native executable ceiling60s.
Any assertion failure, wrong census, physical stop or timeout fails this run.
No retries that erase a failed attempt; preserve outputs with exclusive labels.
Local checkpoint only; no S3, credential use or additional cloud spend.
