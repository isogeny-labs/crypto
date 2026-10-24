# R02 additive native NFS stage

Recorded before NFS control or production search, 2026-10-22.
The higher-bound ECM stage has completely factored the P-224 block 5736
extension-twist order, with native prime certificates and exact product
replay. Seven of nine targets are complete. That 99-digit residual no
longer needs the proposed SIQS fallback; no SIQS run was performed.

For each of the two lineages still partial after its original R02 stages,
attempt the **largest remaining composite** using native YAFU NFS, source
commit `c7d90283972820f99b3b4920bfc1eff05c1f6342`. Preserve its exact current
partition and decimal cofactor before starting. Skip a lineage already
complete. Stop searching it as soon as a fully certified factorization
exists; do not restart the NFS allocation on a smaller cofactor within R02.

Use two sieving threads and two linear-algebra threads per NFS worker,
fixed RNG seed `20261022,362436069`, a native NFS limit of 1800 seconds,
and an outer 1860-second bound with SIGINT and a 30-second termination
grace. At most two factor workers are active across all R02 methods.
When both are NFS workers this allows four CPU threads; when one is the
earlier ECM/PARI worker it allows at most three. Keep the native savefiles,
job files, polynomials, relation data, logs and exact executable/siever
hashes so interrupted work can be resumed later under a new allocation.
No paid or remote machines are launched.

Before production, factor a known-answer integer
`nextprime(10^42+20261022) * nextprime(2*10^42+20261022)` through the same
explicit NFS entry point with two threads, the same seed and a 300-second
native / 360-second outer bound. Prove the control primes with PARI first;
check the NFS output by exact division and native prime certification.
If the control fails, retain it and fix setup before production. A control
failure is not a negative production result.

Treat YAFU's prime/probable-prime/composite labels as unverified hints.
Apply proper divisors to the original target partition, not merely to a
standalone cofactor, then use `certify_parts.gp` and separate
`verify_factors.gp` replay. No complete claim without both prime proofs
and the exact original product. A bounded incomplete NFS run remains
incomplete even if it has collected relations or polynomials.

This adds a complementary factoring method on the same exact input
lineages. It is not a performance comparison or an inference about why
the historical curves were skipped. Reports must include setup errors,+native alarms and missing search results. Original frozen protocols and
historical artifacts remain unchanged.
