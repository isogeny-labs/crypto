# Prospective warmup-only preflight

This is a 36-cell software/runtime preflight, separate from the future 360-cell
main comparison. It uses all 12 prescribed public targets, three logical arms
and one round. All rows are labeled warmups; no observation is pooled into the
main panel. The exact plan, config, pre-call manifest and copied executable
identities are published before these calls. [pcc:unverified]

The two source-separated worker variants pass 47 native checks each. Seven
copied native driver tests pass; an additional audit test harness is empty.
Strict all-target Clippy passes. Driver controls cover schedule completeness,
unique arm/case/seed/round identity, one warmup/nine measured slots in the main
schedule, arm coverage, median/range calculation, exact fractional JSON
preservation, wrong native identity rejection and incomplete-flow retention.
[pcc:unverified]

For every cell, the unchanged scale worker constructs the same public target
without a supplied scalar, solves, verifies and checks corrupted-scalar rejection.
The driver invokes an external Native verifier and retains both raw outputs.
The analysis checks full certificate bytes, structural counters and CSV/certificate
metadata across arms. The separate OpenSSL driver then checks each certificate's
fixed public-point binding, `rP=O` and `kP=Q`, and requires wrong-scalar and
consistent substituted-target rejections. This point audit does not independently
prove primality, Hasse order, full relations or execution provenance.
[pcc:unverified]

Limits: one worker; fixed CPU affinity 4 for workers/external Native replay;
120s per worker, 60s per replay, 300s whole preflight; one attempt, zero retries.
The worker has 4 GiB virtual-address limit. The retained whole-cgroup guard is
15,032,385,536 bytes and includes other retained research. No build/test runs
during preflight. Every output directory is new; no failed call is replaced.

Logical arms a/c share the identical parent executable; b uses the candidate.
Every native worker argv still says `u`. Its raw fields `replicate=0` and
`warmup=false` are preserved. The collector's instance ID includes case and seed;
collector round 0 is the warmup. These two seeds are distinct targets.

After terminal execution, record wall/runtime estimates and dry analysis before
freezing the main protocol. The fixed proposed main census remains 360 calls:
36 warmups and 324 measured observations (nine per arm/target). Its proposed
deadline ceiling is 1800s; require the preflight-based estimate plus 25% margin
to fit, or retain the feasibility gap without reducing the requested census.
L0 clocks describe this host and guide implementation work; no ranked timing,
total-operation price or exponent conclusion follows.

The new local preflight does not consume or relabel the preserved round25 cloud
attempt. Execution remains local, S3 excluded, with zero additional cloud spend.
The main panel is not yet frozen or started.
