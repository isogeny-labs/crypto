# N83 root11 finalists on four RunPod Tier 1 Pods

This provider adaptation executes the already sealed 32-point K2816 versus
K3072 comparison from `../n83_root11_finalists_tier1_20261017/PROTOCOL.md`.
It keeps the public points, exact IC1 manifests, two-arm schedule, same-point
rho references, six calibration solves around every pair, caps, independent
arithmetic replay, and exact paired selector unchanged. No result from the
queued GitHub Actions run may be used to alter this RunPod protocol or its
inputs. A GitHub result and a RunPod result are separate Tier 1 panels; wall
times are compared only within the four matched RunPod allocations.

## Provider and resource envelope

Allocate at most four fresh RunPod Secure Cloud CPU Pods, one per stratum,
using CPU flavor `cpu3m`, four vCPUs, 32 GB RAM, image
`runpod/base:1.0.2-ubuntu2404`, a 30 GB ephemeral container disk, SSH on port
22, and no persistent volume. The live RunPod CPU catalog on 2026-10-17
reported `HIGH` availability and `$0.055` per vCPU-hour for this flavor,
or `$0.22` per Pod-hour. Recheck the price and exact resources before each
allocation; if a Pod differs, stop and retain its provider receipt as an
infrastructure outcome. The allocation plan ends all study Pods within 300
minutes of the first creation. Four Pods at the quoted compute rate would
cost at most `$4.40` over that interval; the requested total spending ceiling
is `$6` including storage. A failed, delayed, or noisy Pod stays in the raw
record and is terminated under the same limit. The older running account
Pods are outside this study and are not changed.

The operator must keep the API key only in memory and delete all new study
Pods on completion, cancellation, producer failure, or at the 300-minute
deadline. Before execution, record every Pod ID, a provider start snapshot,
the source commit, the exact price, and the deadline. At the end, capture a
provider end snapshot before deletion, then independently confirm that the
four Pod IDs have left the active allocation list. Preserve API responses
with only identity, resources, cost and SSH port metadata; never archive the
API key or Pod environment. The current RunPod API does not offer a verified
provider-enforced expiry for these Pods, so loss of operator connectivity can
extend billing beyond the intended limit. The user must explicitly approve
the exact allocation and this residual risk. Committing this protocol alone
does not authorize allocation.

## Reproducible execution and admission

On every Pod, check out the same immutable source commit and install Rust
1.93.1 with the minimal profile. Build and test both examples with
`cargo --locked --release`; retain toolchain versions, binary SHA-256 values,
build logs, and the Git commit. Run the inherited preflight before any timed
work. `entry.py` adds the authentic RunPod Pod ID and source commit to the
existing host receipt without changing the candidate producer or its timed
interval. Each Pod executes its designated eight-point stratum and runs the
inherited arithmetic verifier. Archive the complete `results/` directory
as a ZIP on the Pod, transfer it over the provider-advertised SSH endpoint,
and retain the ZIP SHA-256 and transport transcript.

`provider.py` binds four distinct Pod IDs, stable machine IDs, live resource
and price receipts, the source commit, and overlapping producer intervals to
the extracted results. `aggregate.py` checks every ZIP member, independently
reruns the inherited verifier from the archived bytes, enforces identical
candidate bases across strata, and applies the unchanged 32-point selector.
Every point needs a replayed IC and same-point rho result, both candidates,
and a passing 10% median calibration gate. One missing, failed, timed-out, or
noisy stratum keeps the panel `BOUNDED_UNKNOWN`; it is not a zero-yield result.

## Frozen identity and decision

`dispatch.json` pins the fixture and source digests, candidate IDs, provider
adapter files, flavor, hourly price ceiling, absolute allocation time and
total spend ceiling before any Pod is created. The primary result is the
preregistered two-sided exact sign test on 32 paired one-target online
intervals: at least 23 strict wins by one arm, no ties, p at most 0.05,
and all replay and Tier 1 gates. Complete cold process time is selected by
the same rule separately. A result on this pair does not establish a global
optimum over other untested factor-base constructions.
