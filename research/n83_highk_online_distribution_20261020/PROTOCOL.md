# N83 factor-base online distribution screen and fresh one-target holdout

The admitted eight-point high-K Tier 1 panel found a cold-process winner but
left its online selector `BOUNDED_UNKNOWN`: target-to-target variation was
larger than the differences between its K4300 and K4405 arms. This experiment
uses more fresh public points to choose which exact factor base merits a new
single-target holdout. The screen records a separate online interval and
correctness witness for every point. Its shared setup makes the screen a
**secondary target-distribution study**; the eight fresh one-target holdout
runs are the primary speedup evidence.

## Fixed instance and candidates

The curve is `EC1N83Ckb1a1haacb69ee9f37`, the exact N83 `a=1` Koblitz
representation with subgroup order `8569786107849059`. Execute the IC source
from commit `e7634a81c22b60ad37e5d6bdb817a3b84867ae86`, SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The five candidate manifests, source hashes, base dumps, and flags are fixed
before dispatch:

| Arm | Actual factor-base points | Folded columns | Candidate ID |
| --- | ---: | ---: | --- |
| K3072 root11 | 509,952 | 3,072 | `IC1N83Ckb1a1fb509952PDP4rootRCguidedLAgaussTDdirectISO0h2e63425c7f7e` |
| K3500 root4 | 581,000 | 3,500 | `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea` |
| K3300 wide3 | 547,800 | 3,300 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` |
| K4300 wide3 | 713,800 | 4,300 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77` |
| K4405 wide3 | 731,230 | 4,405 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7` |

The fixture generator fixes 192 **screen** points under
`n83-online-distribution-screen192-20261020-v1` and eight **holdout** points
under `n83-online-distribution-holdout8-20261020-v1`. Each scalar is
`1 + LE(SHA256(domain || u32le(index))) mod (r-1)`; the public point is
`[scalar]G`. Scalar sidecars stay outside timed intervals. The screen target
SHA-256 is `0617ee4c55b08e3b4379d99490cad9c249c50ef2b0cc6ab321fe63a0bb1ba0b3`,
and the holdout target SHA-256 is
`e7dcb0fe5779f847d381017b3cb5fb7dd46b6a4a7e834611d200d7cbd6244f68`.
The answer SHA-256 values are in `fresh_manifest.json`. The exclusion audit
found zero overlap with 31,600 prior public points and no overlap between
the two new streams.

## Secondary screen

Use one frozen Tier 1 RunPod allocation. Split the 192 screen points into
disjoint ordered blocks 0–95 and 96–191. Run the five IC candidates once per
block, in the table order for block 0 and reverse table order for block 1.
Each IC child starts with empty setup caches, constructs its exact base,
index, factor-base logs, and then processes the block's 96 public points.
Preserve **every per-point** target interval, exclusive phase times, scalar,
four-point relation, base hash, resource receipt, and failure. Do not divide
process wall time by 96 or use batch throughput as a one-target result.
Precompute and cold-process times are separate screen diagnostics.

Each point has one independently verified same-point rho run, seed
`2026120200 + point_index`. Run three identically seeded rho calibration
children before and after each IC child. The before/after median difference
divided by their mean must be at most 20% for both blocks. Freeze affinity,
cgroup quota, memory, provider machine ID, source, binaries, target order,
candidate order, and child caps in a dispatch manifest before measurement.
The IC child caps are 900 seconds for K3072, 1,200 seconds for K3500 and
K3300, and 1,500 seconds for K4300 and K4405; rho and calibration children
have a 120-second cap each.
Require zero cgroup throttling, CPU steal, memory-limit denial, and OOM;
retain any failure, timeout, noisy block, or incomplete point as recorded.
An unadmitted screen selects no candidate.

For each unordered pair of candidates, count the 192 per-point wins and
losses from the recorded online intervals; an exact equal time is a tie and
is excluded from that pair's binomial count. Use the exact two-sided sign-test
tail under probability one half. Correct the ten pairwise tests with
Bonferroni `0.05/10 = 0.005`. Select a unique arm only if it wins more often
than **each** of the other four with corrected p-value below 0.005 and has
the smallest geometric mean online time. Preserve the geometric means,
medians, win counts, exact p-values, paired log differences, and cold costs.
Otherwise record `selected_candidate_id: null` and `BOUNDED_UNKNOWN`.
The screen is candidate selection evidence; its shared setup and target
count must be named whenever its timings are reported.

## Fresh primary holdout

Run the holdout only if the secondary screen is fully admitted and selects
one candidate. For each of its eight previously unused points, start a new
IC child with empty caches and exactly one public target, and solve the same
point independently with rho seed `2026120200 + 192 + point_index`.
Alternate IC/rho order by point index. Run
three same-seed A/A calibration children before and after every pair. Freeze
the selected candidate ID, exact base, target, source and resource envelope
before the first holdout child. The online IC clock begins with the first
target-dependent query after preparation and ends after recovered-scalar
verification. Record cold setup and index construction separately. The rho
clock covers the one target's walk through independent scalar verification.

Admit the holdout only if all eight IC and rho answers independently replay,
all same-point pairs and 20% A/A gates pass, exact base hashes match the
manifest, and every resource/telemetry gate passes. Report each paired
`rho_online_ms / IC_online_ms`, the median ratio, and the included timing
interval. A holdout speedup requires all eight ratios above one and a
positive lower 95% paired-difference confidence bound. A failed gate leaves
the result `BOUNDED_UNKNOWN` with a null winner. This holdout validates the
selected candidate against rho; a later fresh paired candidate comparison
is required to establish its ordering against every other base.
For the eight paired differences `rho_online_ms - IC_online_ms`, use the
sample mean minus `2.3646242510103 * sample_sd / sqrt(8)` as that lower bound.

## Capacity, cost, and archive

The new Community Pod must expose at least four CPUs with four CPUs of quota,
at least 24 GiB plus 256 MiB of cgroup memory, and enough disk for all raw
receipts. Pin the lowest four exposed logical CPUs, cap each IC child at
24 GiB RSS, and record the exact GPU/CPU class and topology. Accept the
eleven GPU type IDs in `runpod_provider.GPU_TYPES`: RTX A2000, V100 PCIe,
RTX 3070/3080/3080 Ti/4070 Ti, RTX 4000 Ada/4000 SFF Ada, and RTX
A4000/A4500/A5000. All arithmetic is on the four frozen CPUs; the GPU is
an allocation class. The selected Pod must cost no more than `$0.20/hour`.
The first three class requests returned no eligible Pod on 2026-10-20; this
pre-dispatch amendment broadens capacity without changing the source,
targets, CPU/memory envelope, or statistical gate. Cap the provider
allocation at seven hours and the producer at six hours 40 minutes. Reserve
at most `$1.40` for this allocation, `$2.07` for the completed high-K Pod,
and `$1.50` for earlier attempts; their conservative combined upper bound is
`$4.97` against the owner's `$5` ceiling. Stop on completion or cap, verify
the archive SHA-256 locally, then terminate the stopped Pod. No API key is
placed on the Pod. Store a hash manifest for every raw file, the provider
snapshot, run status, failed rows, local independent replay, and the sealed
selection and holdout decisions.
