# Follow-up: a larger same-session A/A for the aimed-gather timing (protocol)

Written on 2026-10-29, **before any measurement in this follow-up**. It
amends nothing in [`../PROTOCOL.md`](../PROTOCOL.md) and rewrites no record
of the first session ([`../runs/time/`](../runs/time/)); both sessions are
reported side by side.

## Why

The first session (2026-10-23) measured a paired collect-phase ratio of
1.111 [1.065, 1.160] over 30 A/B rounds (28/30 above 1). The declared rule
also asks the interval's lower end to clear the A/A interval's upper end,
and the A/A had only 5 pairs: its interval was 0.944–1.087 for collect and
0.953–1.121 for the whole set-up. The verdict was **not established**. The
question here is whether the gain survives a noise floor measured with as
many rounds as the effect itself, in the same session.

## Arms (fixed, by hash)

The same two binaries the first session measured, unchanged:

| arm | source | `sha256` |
|:--|:--|:--|
| baseline | `d34b9b390` (main before #2448) | `209b7b741d469182…` (full hash in `runs/arms.sha256`) |
| candidate | `6bc18857b` (the change, merged in #2448) | `431a9d98aa827c44…` |

`run.sh` refuses to start unless both hashes match `runs/arms.sha256`,
which is written from the first session's binaries before the first run.

## Workload and instrument (unchanged from the first session)

- `ic price --params research/ic_tool_program/rule/v3/runs/k0n53/T01.params.json --json`
  (`icv1-f2m53-tm56619371-dac20a85`, rule v3 T01).
- Every process under `tools/isolated_bench.py run --wait --cpus 3
  --max-other-cpu 0.3` with `RAYON_NUM_THREADS=1`. A launch refused for CPU
  pressure is retried and counted; it is not a measurement.
- Metrics per process: the median over the price's repetitions of
  `phases_ns.collect` and of `total_ns`.

## Schedule

42 rounds. Each round runs three processes: baseline `a`, baseline `q` and
candidate `c`. Round `r` uses permutation `(r − 1) mod 6` of
`[a q c, a c q, q a c, q c a, c a q, c q a]`, so over 7 full cycles every
label runs in every position equally often.

## Statistics (as the first session declared them)

- Per round: A/B ratio `a / c`; A/A ratio `a / q`. Both are within-round and
  same-session.
- Median of the 42 per-round ratios, with a 95% percentile bootstrap over
  rounds: 2,000 resamples, the same LCG as `../summarise.sh`, seed
  20261029.
- A round with a contended (`run.contended == true`) or failed process is
  dropped from **both** statistics, and counted.

## Decision rule (unchanged)

For each metric (collect and whole set-up) separately, the native effect is
**established** in this session only if the A/B interval's lower end is
above 1 **and** above the A/A interval's upper end. Otherwise it is reported
and not claimed. The rule is applied once, to the full 42 rounds; there is
no early stop and no extension.

## Identity

Every one of the 126 processes must report `status`, `all_verified: true`
and the recovered logarithm `518509155375`. Any mismatch voids the session.

## Route

Run locally on this session's container, not on Iso Lab (AGENTS.md §9a).
The reachable EKS request paths run `ecbench.spec/v1` specs only, and an
ecbench spec compares methods inside one build. This comparison is between
two builds, and its A/A needs all three processes of a round on the same
host minutes apart; two jobs could land on different hosts and break that
pairing. The arbitrary-command route (`isolab_k8s_run`) needs the
harness-host MCP, which this container does not have. The host manifest is
recorded in `runs/host.json`. The first session ran on a container in the
same environment, but the kernel differs, so the first session's ratios are
not pooled with these.

## Not in scope

Instruction counts and perf shares are unchanged by this follow-up (same
binaries) and are not re-measured.
