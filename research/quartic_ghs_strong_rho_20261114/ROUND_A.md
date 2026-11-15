# Round A: duration-only amendment for the strong-rho boundary

Status: frozen prospectively on 2026-11-14 before any primary EKS output.
Round ID: `ghs-strong-rho-r1-duration-a`.

This amendment changes only the Kubernetes active deadline of the two largest
splits in the [frozen protocol](PROTOCOL.md). It does not change an algorithm,
input, target, seed, repetition count, estimator, confidence procedure,
admission interval, accounting rule, resource request, retry rule, or
publication gate.

## Reason for the amendment

Each seed process executes its repetitions serially, while the three seed
processes run concurrently. At q = 4,194,271 the subgroup has
`ell = 18,446,308,672,764,918,653`, so `sqrt(ell)` is about 4.29 billion walk
steps. Three serial repetitions plus setup, calibration, compilation,
verification, upload, and Spot provisioning can exceed the original one-hour
top deadline even when the implementation is healthy. The same margin is
narrow at q = 2,097,091.

This feasibility assessment was made without inspecting primary round output.
Development smokes remain excluded from the scientific sample.

## Frozen round-A deadlines

| split | q values | active deadline |
|:--|:--|--:|
| low | 4,051 through 262,111 | 3,600 s |
| mid | 524,287 and 1,048,507 | 3,600 s |
| high | 2,097,091 | 7,200 s |
| top | 4,194,271 | 10,800 s |

The deadline includes provisioning. Reaching it is a censored infrastructure
outcome, never evidence for or against either algorithm.

## Unchanged limits and bindings

- Each Job requests 4 vCPU and 4 GiB; aggregate requested CPU is at most 16.
- Launches use two waves because `cpu-spot` admits 8/16/32-vCPU nodes and the
  Job contract requires anti-affinity between experiment Pods. Wave 1 is
  low+mid; wave 2 is high+top only after both wave-1 Jobs are terminal. Thus
  concurrent requested work is at most 8 vCPU/8 GiB and no Job spends its
  active deadline queued behind this round on an already occupied node.
- The three seeds run concurrently; repetitions within a seed remain serial.
- Automatic retries remain zero. The frozen manual-infrastructure-retry rule
  and the USD 5 ceiling across builds, jobs, failures, storage, and transfer
  remain unchanged.
- The primary arm remains `strong-b8`; the exact grid, targets, diagnostic
  arms, calibration triples, 10,000 clustered bootstrap resamples, and
  `[0.75, 1.15]` rho admission interval remain unchanged.
- Scientific source files must retain the #4171 merge hashes:
  `main.rs` =
  `b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`
  and `strong_rho.rs` =
  `73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de`.
  The eventual source revision may add only reviewed launch, validation, and
  report material around those pinned implementation bytes.
- Every request binds this file and `PROTOCOL.md` by SHA-256 and names this
  round ID in provenance.

No request may be generated or launched unless the live budget, all-tenant
pool (8 vCPU/8 GiB available for the next wave), checkpoint, Git custody,
image digest, and source-hash gates all pass. The pool and budget checks are
repeated before wave 2; a passing wave-1 receipt is not reused.
