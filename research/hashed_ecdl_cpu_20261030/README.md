# Complete CPU hashed-ECDLP optimization

The implementation is in [`crates/hashed-ecdl-cpu`](../../crates/hashed-ecdl-cpu/).
The two completed matched panels each contain 120 executions, all independently
replayed with byte-identical full certificates across arms. The final panel
compares the original CPU oracle with persistent MITM tables and batched
inversions, using a parameter-only curve cache key. Each arm uses the same
shared reduction and native binary within its panel.

| Artifact | Initial cache | Parameter-only cache |
|---|---|---|
| Frozen protocol | [protocol](protocol.json) | [protocol](panel2/protocol.json) |
| Raw rows | [rows](results/rows.csv) | [rows](panel2/results/rows.csv) |
| Checked export | [export](results/export.json) | [export](panel2/results/export.json) |
| Stage summaries | [summary](results/summary.csv) | [summary](panel2/results/summary.csv) |
| PDF | [report](reports/panel1/report.pdf) | [report](reports/panel2/report.pdf) |

Final matched medians (nine repetitions after one warmup per arm/fixture):

| Public known-answer fixture | Original CPU | Cached batch64 | Identical reference repeat |
|---|---:|---:|---:|
| p211, m3, tau40 |0.315901ms|0.283191ms|0.314862ms|
| p65521, m3, tau80 |45.993793ms|3.903194ms|46.098433ms|
| p65521, m4, tau64 |80.536765ms|4.118365ms|80.527314ms|

These are descriptive L0 intervals on an AMD EPYC 9R45 EKS VM, one software
thread and 2 vCPUs/4 GiB reserved. Complete intervals include curve and known-target
construction, all trial/base/table setup, relation collection, elimination,
internal and independent certificate replay, and serialization. Launch and
archive I/O are excluded. The native analyzer checks complete certificate bytes,
not just a digest. Every execution rejects a deliberately corrupted scalar.

`u` is original Enumerate; `v` is cached with batch 64; `w` is cached with batch 1;
`x` repeats the exact `u` binary/path. Batch 1 is faster at the smallest fixture;
no universal optimal batch size is claimed. Preparation, probe and inversion
counters are cumulative. Reference zero counters do not mean zero arithmetic.

The [larger-field protocol](scale/protocol.json) is a separate bounded
public-target feasibility ladder, not pooled with either timing panel. The
job expired at its scheduling deadline; no recovery rows or certificates were
produced. See [terminal receipt](scale/results/status.json) and
[zero-execution export](scale/results/export.json). No larger-field recovery
is established by this attempt.

Runtime image digest `67eea58f…` is from crypto main ancestor `cb2ec6873f30`.
Separately staged standalone CPU binary/source hashes are recorded per panel in
`build.json`; an image source label does not identify those executables.
Raw durable archives: `s3://setec-astronomy-artifacts/validation/hashed-ecdl-cpu/20261030/`.
The two initial zero-data staging failures and all protocol versions remain here.

Reproduce the analyzer using `rustc -O analyze.rs -o analyze`, then
`./analyze RESULTS/rows.csv RESULTS`. Build the native instrument using the
standalone manifest with `--locked --release --example complete`; stage it,
the analyzer, protocol, run.sh and hash metadata on the same CPU runtime. The
script uses a fixed Latin arm rotation, cases 0..2 and rounds 0..9. Do not overwrite
any existing run directory. Typst report sources and generated PDFs are paired.

These results establish exact software behavior on fixed fixtures and expose
CPU stage costs. They do not establish a certified speedup, fastest-tool status,
matched-rho advantage, large-curve attack, or the paper's conditional tensor
contraction hypothesis. External/rho gaps remain unknown. Independent review
and the repository's validation gate remain required for leaderboard promotion.
