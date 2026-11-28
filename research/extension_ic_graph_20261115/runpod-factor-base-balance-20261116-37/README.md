Seven public-input Rust workers recover10013957165807875 on p1009 seed359,
L263807419874170961. The copied qualifier passes all successful graph
certificate/target/checkpoint/gate/prefix checks. PCC-20261116-7fa174 remains
unverified. [pcc:unverified]

| Arm | Small base | B values | Sources | Observed incomplete primitive bitops |
|---|---:|---:|---:|---:|
| Original IC | — | — | — |3241959371026|
|75%, stride32|387|500736|519|4634237434948|
|75%, stride128|387|499712|517|4628340096078|
|85%, stride32|439|365568|537|4152078192556|
|85%, stride128|439|380928|568|4234576160840|
|95%, stride32|491|303104|522|4171549682926|
|95%, stride128|491|362496|620|4560298544450|

All graph arms use the same517point full base, two accepted descents and
969residuals; every arm performs one exact homogeneous rank checkpoint.
The independent qualifier reconstructs all structural-gate prefixes using
BFS+dense LP rank, verifies bound predicates/schedule/known charges, replays
all affine/checkpoint combinations, determines the actual scalar, and checks
common descent prefixes and same-fraction original-source prefixes.
[pcc:unverified]

Observed relation work decreases as the small fraction grows, while LA
work increases. Shorter spacing reduces collection at85% and95%; at75%,
stride128 happens to stop1024B values earlier. Thus spacing is not monotonic.
85%/32has the lowest observed graph subtotal in this cohort, while original
IC's subtotal remains below every graph arm. Native container/PRNG/RREF/DSU
coverage remains incomplete: all arms are unrankable, complete totals,
pricing bands and ratios null. No full-cost, wall-speed, T0best-IC/T1rho or
cross-cohort composition is admitted. [pcc:unverified]

Sourcef3e76f16a85c30cc4018183b864083fe030e6307 precedes the sole Runpod call.
One cold round, zero warmups/retries;7/7complete/recover, no caps/exclusions,
worker/copied qualifier exit0,215.886s whole practicality only.4vCPU/8GiB
cpu3c/0.12USDh/pinned Rust image/actualCPU1.297committed source blobs and all
portable pre/preworker/post/qualified hashes pass. Collection precedes
owned-pod DELETE204/GET40418:57:55UTC; no owned paid pod remains.
[pcc:unverified]

[Final one-page report](report.pdf) was visually inspected. Exact PDF/SVG/
150dpiPNG/native figure copies are bound by `figure-copies.sha256`. The
initial clipped figure caption was fixed only in post-call presentation by
increasing SVG height; initial source/figures remain hash-bound locally in
`presentation-initial-custody.sha256`. Frozen scientific code/data/checker
were not edited. [pcc:unverified]

Exact original76096Bsummary,3434418Bexport and six graph raws498081–593059B
remain untruncated under
`/workspace/ic-runpod-runs/runpod-factor-base-balance-20261116-37/returned/task/run`.
`local-custody.sha256` binds originals, immutable source/binaries, sealed
bundle and returned archive. The small `presentation.json` is a separately
named/schema-tagged post-call Rust field projection; it does not replace or
rewrite the original summary. Original7395BIC raw and other small metadata
fit Git. Native export claims no unavailable canonical-schema conformance
or independent signed execution verdict. [pcc:unverified]
