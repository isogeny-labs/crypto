# R36 production integration: initial-walker joint-table reuse

Status: selectable production candidate implemented; deterministic control
passed; isolated timing and promotion remain unmeasured
`[pcc:unverified]`.

The final q=223 correctness replay is bound to exact commit, source closure,
reproducible binary, and raw record by
[`receipts/commit_bound_manifest.json`](receipts/commit_bound_manifest.json).
Its strict cross-row validation is recorded in
[`receipts/commit_bound_validation.txt`](receipts/commit_bound_validation.txt).

## Change

The composed strong-rho source now accepts the diagnostic suffix
`-walker-table`. It reuses the cold width-two table for initial walkers one
walker at a time. Coefficient sampling, infinity rejection, canonicalization,
and walker initialization retain their baseline order. Rejected attempts keep
all field work charged.

The existing `strong-b*` arms remain unchanged. The candidate is selected as,
for example, `strong-b8-walker-table`; the counter-free counterpart is
`fast-b8-walker-table`.

## Deterministic q=223 control

Command:

```text
GHS_SOURCE_REVISION=worktree-r36-joint-walker-table rustc --edition 2021 -O research/quartic_ghs_strong_rho_20261114/main.rs -o /tmp/ghs-r0-r99-composed
/tmp/ghs-r0-r99-composed selftest
/tmp/ghs-r0-r99-composed run 223 1 0 1 ic-repeat strong-b8,strong-b8-walker-table,fast-b8-walker-table
```

The self-test returned `SELFTEST OK`. Baseline and candidate recovered scalar
`1765657` and retained 1,487 steps, 76 distinguished points, zero restarts, 53
lookahead retries, two cycles, ten cycle additions, all walk/verification
field counters, and all native counters `[pcc:unverified]`.

The candidate changed setup counters as follows:

| Counter | Baseline | Candidate | Candidate − baseline |
|:--|--:|--:|--:|
| `fq_mul` | 56,484 | 48,433 | -8,051 |
| `fq_inv` | 511 | 288 | -223 |
| `fq_add` | 204,142 | 178,576 | -25,566 |
| `f3_mul` | 6,437 | 5,847 | -590 |
| `f3_sq` | 2,346 | 1,979 | -367 |
| `f3_inv` | 511 | 288 | -223 |
| `batch_inv` | 33 | 258 | +225 |
| `group_add` | 1,455 | 1,224 | -231 |

These are producer-side deterministic implementation counters, not a wall-time
or end-to-end speedup. Development `wall_s` fields are retained in the raw
JSONL but excluded from the disposition. The candidate remains
`implemented-unmeasured` until the frozen admission and isolated A/A/ABAB
gates pass `[pcc:unverified]`.

Raw output: [`receipts/r36_q223.jsonl`](receipts/r36_q223.jsonl).
