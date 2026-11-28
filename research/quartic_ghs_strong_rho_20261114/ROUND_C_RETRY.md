# Round C: exact look-ahead retry batching screen

Status: prospective deterministic correctness/count protocol, frozen before
candidate implementation or output inspection on 2026-11-18. Round ID:
`ghs-strong-rho-r3-retry-slab2`.

This round tests one bounded implementation lead in the matched strong-rho
comparator. It does not amend Round A, replace the frozen `strong-b8`
reference, qualify wall time, or move a frontier or leaderboard row.

## Baseline and question

The baseline is `origin/main` revision
`ad2a014c0022cf000845014499a9649b6c2cef2d`, with
`strong_rho.rs` SHA-256
`0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`
and `main.rs` SHA-256
`b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8`.

The existing batch-8 walk executes one batch inversion for the common first
candidate and another for every nonempty retry set. The fixed `q=223` Round B
control took 1,487 lane steps in 186 primary rounds, reported 53 look-ahead
retries, and used 231 batch inversions. Thus 45 retry rounds were nonempty;
the remaining 10 walk inversions were scalar cycle additions.

The question is whether two candidate additions per lane can be placed in one
Montgomery batch, then selected with the original sequential rule, reducing
retry batch inversions without changing the walk. This is a bounded screen,
not a claim that fewer inversions imply lower total cost.

## Dependency barrier and bounded alternative

For a candidate `P + M_j`, retry membership is known only after its affine
`x` coordinate has been produced, hashed, and compared with `j`. Advancing
accepted lanes while waiting to fill a later retry batch would move their
canonicalisation, cycle handling, distinguished-point lookup, and table
insertion ahead of lower-index lanes. That can change the first collision and
the answer-producing record. Waiting at the current outer-round barrier keeps
collision order exact but supplies no additions from later rounds to fill the
retry batch. Cross-round compaction is therefore excluded in this round.

The bounded alternative is `retry_prefetch=2`:

1. For every lane at the start of an outer round, compute both
   `P + M_j` and `P + M_(j+1 mod r)` in one batch inversion.
2. Inspect the second candidate only when the first candidate would have
   retried. Hash and look-ahead counters therefore follow the original
   sequential decisions; speculative group and field work is counted in full.
3. If the second candidate also retries, resume the existing pending-lane
   loop at `j+2`. The fallback may execute additional batch inversions.
4. Return candidates in original lane order. The caller's lane-index loop,
   RNG draws, canonicalisation, cycle logic, collision table, and stopping
   condition are unchanged.

The existing arm remains the default. The candidate receives a distinct
development-only arm name and cannot silently replace `strong-b8`.

## Frozen workload and commands

The source-bound standalone driver is built once after the edit with:

```sh
rustc --edition 2021 -O main.rs -o round-c-retry/ghs-strong-rho-r3
```

The controls are:

```sh
./round-c-retry/ghs-strong-rho-r3 selftest
GHS_CALIB_TRIPLES=1 ./round-c-retry/ghs-strong-rho-r3 \
  run 223 1 0 1 ic-repeat strong-b8,strong-b8-retry2
```

The input is the existing public synthetic known-answer cell: `q=223`,
instance seed 1, target index 0, `ic-repeat`, walk seed 1000, 32 partitions,
batch width 8. Calibration and wall time are emitted by the driver but are not
used in this round. Stdout, stderr, exit status, source and binary hashes,
compiler, host metadata, and the failed Conductor/research-KB checks are
retained beside the result.

## Requirements-to-evidence matrix

| Requirement | Evidence | Pass condition |
|:--|:--|:--|
| Exact candidate selection | deterministic helper equivalence tests | baseline and retry2 return the same point and jump for every lane, including lanes requiring at least two retries |
| Same ECDLP trajectory | paired `q=223` JSON records | equal target, answer, steps, DPs, restarts, look-ahead retries, cycles, cycle additions, setup counts, and every native count |
| Complete speculative accounting | paired field records and count identities | candidate charges every computed addition; no field/group work is hidden |
| Inversion objective | paired walk counts | `retry2.walk.batch_inv < baseline.walk.batch_inv` on the fixed cell |
| Total counted cost | paired walk counts | report every field delta; acceptance as a cost optimization requires lower cold `M-eq` for every frozen Round A inversion price, not merely fewer `batch_inv` calls |
| No performance/frontier substitution | report and dashboards | no wall-time, throughput, E2E, rho-frontier, or leaderboard claim from this development cell |

## Stop and interpretation rules

Stop on any lane-output mismatch, native-count mismatch, wrong logarithm,
changed trajectory, missing raw output, or unaccounted speculative operation.
Preserve the failure.

If batch inversions fall but priced field work rises at any frozen comparison
price, report the candidate as a negative bounded screen and do not make it
the default. If all count gates pass and total cold `M-eq` falls across the
frozen price band, the result is only a code/count candidate. Profiling and
matched isolated A/A plus interleaved A/B measurement remain separately
checkpoint-gated before any throughput or frontier statement.

## Pre-implementation checks

- `scripts/research-kb status` and both requested searches failed because no
  research-KB executable was available. No novelty or absence statement is
  based on that failure.
- `conductor check --summary 'Prototype exact GHS lookahead retry batching'
  --scope path:research/quartic_ghs_strong_rho_20261114` could not run because
  `conductor` was not installed. This isolated branch has no shared-worktree
  edits.
- The canonical `crypto-autoresearcher` experiment-protocol skill was
  vault-encrypted in the available public checkout. This round uses the local
  pointer, repository evidence rules, and this frozen deterministic protocol;
  it makes no controlled timing claim.
