# Fresh N83 cross-family factor-base comparison under target cancellation

This prospective Tier 1 panel compares four complete N83 index-calculus
pipelines on the same 24 public points under the verified two-worker
cooperative target-cancellation policy. It puts the K2816 and K3072 root11
dynamic bases against the previously replayed K3450 root11 wide3 Bloom and
K3500 root4 standard Bloom bases. The primary objective is one-target online
wall time after reusable setup. Complete fresh-process cost is a separate
cold objective. Each candidate-point pair has its own same-point rho solve.

The executed IC source must be SHA-256
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`.
The four exact candidate IDs, in fixed order, are:

| Arm | Usable points | Folded columns | Exact IC1 candidate |
| --- | ---: | ---: | --- |
| K2816 root11 dynamic | 467,456 | 2,816 | `IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h7a3cca2a9993` |
| K3072 root11 dynamic | 509,952 | 3,072 | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0hde7207572a27` |
| K3450 root11 wide3 | 572,700 | 3,450 | `IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hd7dca5070efb` |
| K3500 root4 standard | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h441d3898d220` |

The last two manifests transfer the exact previously replayed factor-base
geometry and index layouts from source
`8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`
to the current cancellation source. Their base hashes and ordered dumps are
**expected values** to be checked again against every executed cloud arm;
they are not accepted from manifest identity alone. All four manifests
freeze `KIC_TARGET_THREADS=2`, `KIC_TARGET_CANCEL=1`, and
`KIC_INDEXED5_ID30=0`. The wide3 arm uses the five-byte fixed30 indexed
root table and wide3 Bloom filter. The root4 arm uses the four-byte fixed31
indexed table and standard Bloom filter. The root11 dynamic arms use
five-byte load75 tables without Bloom.

The public 24-point file is [fresh_targets.jsonl](fresh_targets.jsonl),
SHA-256
`ac0b84bd916291331e0dd13bfb312aedcd50d7cd3bbc1c3cd5f81eeca22ea73d`.
The verifier-only [answer sidecar](fresh_answers.jsonl) has SHA-256
`2c47e48393346ca742eddff8e0cbfeb59b9bd486b154428f28aee2a90a5fa49b`.
The [generator](generate_fresh.py) fixes scalars by
`1 + little_endian(SHA256("n83-crossfamily-cancel24-20261017-v1" || u32_le(i)))
mod (r-1)` for indices 0–23, computes public points on the exact N83 prime
subgroup, and seals both files. The [exclusion audit](fresh_exclusion_audit.json)
found zero overlap with 31,400 distinct points in prior target files,
including the separate cancellation factorial fixture, and zero overlap
with the live 32-point finalist fixture at commit
`9b9a0f3842de6cf921e38573cb99fcaed2a4683c`.

Use four independent Tier 1 cloud allocations, one per six-point stratum:
indices 0–5, 6–11, 12–17, and 18–23. Every IC arm starts a new process
with empty factor-base, index, and log caches. Rotate arm order by public
point index. Alternate IC then rho and rho then IC by point and fixed arm
index. Use rho seed `2026111000 + 4*point_index + fixed_arm_index`.
Run three independent frozen-point rho calibrations before and three after
each candidate-point pair. Compare the two medians; their absolute difference
divided by their mean must be at most 10%. All 96 IC solves, 96 same-point
rho solves, and 576 calibrations must independently replay before pooling.

Cap K2816 and K3072 IC processes at 900 seconds each, K3450 and K3500
at 1,200 seconds each, and every rho/calibration process at 120 seconds.
Use a 15.25 GiB process RSS cap, 300-minute producer cap, and 350-minute
cloud-job cap. Preserve each timeout, failed replay, resource event, noisy
pair, and global-cap stop as its recorded status with unavailable costs
null. Require four distinct overlapping provider runner allocations; bind
each raw ZIP and every extracted member to provider receipts and a separate
local verifier replay. The host gate records exposed topology, affinity,
cgroup memory, throttling, steal time, OOM events, process peaks, source and
binary hashes, exact commands, event order, and calibration. An admitted
stratum needs all six points and all 24 candidate pairs to pass.

For the primary online selector, compute all six unordered candidate
contrasts using the 24 paired values
`log(second_online_ms / first_online_ms)`. A candidate is selected only if
its geometric mean online time is lower than each other arm's and all three
of its exact two-sided sign-flip p values over `2^24` sign assignments are
below `0.05/6`. Apply the same six-pair Bonferroni gate separately to
complete cold-process millisecond differences. Report paired mean, median,
point win count, descriptive 95% paired interval using
`t(0.975,23)=2.06865761041904`, exact p, all individual timings, five
exclusive online phases, verified relation and rank, actual base size,
index entries/allocation, probe counts, memory, and paired rho/IC ratio.
The four host strata must pass every identity, replay, resource, and noise
gate before either aggregate selector may return a candidate ID; otherwise
the selected ID remains null with `BOUNDED_UNKNOWN`.
