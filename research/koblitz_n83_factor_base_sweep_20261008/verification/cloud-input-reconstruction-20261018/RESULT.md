# Exact K2048 input reconstruction on the resumed cloud checkout

The primary `a=0`, public-x-hash, seed `2026100801`, K2048 factor base was
reconstructed from clean PR #2036 source
`8f971585692c702d2804668b322e2440177f80d2`. The 10,313,647-byte archive
matches the retained compressed BLAKE3
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`
and SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`
exactly. Plain-byte and point-set BLAKE3, policy, seed, all 339,968 points,
2,048 orbit columns and 4,094 scanned x candidates also match the
[retained manifest](../remaining-budget-k2048-20261010/manifest.json).
See [comparison](reconstruction-comparison.json).

The [generic multi-limb replay](generic-replay.json) checked every point
and all representatives against the producer's distinct arithmetic backend.
This is same-host, same-repository replay, not external independent-host
validation. A [negative control](negative-control.json) changed the first
coefficient from 1 to 2, recomputed compressed/plain BLAKE3, object size,
filename and S3 URI, and was rejected at the semantic point/orbit/label
check. The original archive was preserved.

## Execution and accounting

The [pre-run protocol](reconstruction-protocol.json) bounded the native
build at 600 seconds, construction at 240 seconds and replay at 240 seconds.
Both workers used 230-second internal guards. Cargo reported 220 seconds
for the successful offline release build. Observed external wall intervals
were 4.628 seconds for construction, 2.924 seconds for replay, and 2.171
seconds for forged-input rejection. These are local recovery observations,
not isolated throughput comparisons or improvements over prior timings.
Build, construction, validation and failed-access preparation remain separate
charges; no campaign-wide remaining allowance is inferred from these intervals.

The diagnostic supervisor applied single-CPU affinity, a 4-GiB `RLIMIT_AS`
and external wall guards. This is a native process envelope; it does not
establish the original hard Docker memory/no-swap/network isolation. Child
rusage is cumulative across exec, so worker-only CPU time and peak RSS
remain unknown. The receipts label this limitation explicitly.

Raw archives and the worker remain under
`/home/ec2-user/scratch/n83-resume-20261018/`; compact review receipts are
retained here. No new S3 GET succeeded and no S3 write was attempted. The
existing S3 storage receipt remains historical; this recovery does not
recertify current S3 availability.

## Remaining requirements

| Requirement | Status and evidence |
| --- | --- |
| Recover exact retained K2048 input | Verified byte-for-byte reconstruction and generic replay |
| Frozen finite design integrity | PASS; 26,939,278,080 declared addresses, not executed experiments |
| Recover later K2048 100,000-conflict run | Retained receipts recovered from upstream; source, manifest and query replay pass; original binary/live cleanup not rechecked |
| New retained-base natural solver sweep | Not run: original guarded runtime and current S3 access unavailable |
| Natural yield, useful rank and complete cold runtime selection | Unresolved; selected best remains null |
| Current source regression validation | See VALIDATION.md |

During recovery, PR #2036 advanced to
`63844b3b2b674d31ef07e8dec8d0f63ffe66bc52`, publishing the later K2048
search and K4096 construction receipts. The [receipt-recovery check](upstream-recovery.json)
verified all 31 K2048 and 26 K4096 manifest file hashes, all five K2048
source snapshots against the frozen run commit, worker receipt consistency,
and independent original-curve replay of its public query using the exact
reconstructed archive. The result is `UNKNOWN_solver_cap`, 100,000 conflicts,
zero models and zero relations. The original static worker binary and live
Docker cleanup were not rechecked here; this is not a full rerun of the
original auditor. K4096 archive bytes were not reconstructed in this rung.
The [problem record](../../../../problems/n83-resume-evidence-and-execution-access.md)
tracks the remaining execution and evidence gates.
