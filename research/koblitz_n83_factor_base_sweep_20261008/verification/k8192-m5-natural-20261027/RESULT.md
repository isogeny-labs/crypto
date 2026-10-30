# Primary N83 K8192 five-summand model and natural query

The frozen K8192 orbit-chain model built with **172,918 SAT variables,
3,782,477 clauses and 4,980 XOR rows**. On the fixed public fixture-0
query, one natural relation attempt returned `UNKNOWN_solver_cap` at exactly
1,000 conflicts. It produced zero models, relations and rank-increasing
rows. The worker returned normally before its 600-second wall budget; the
container exit was 0 and `OOMKilled=false`.

| Stage | Observed result | Resource boundary |
| --- | --- | --- |
| Replayed base import and model | `PASS_model_construction_only`; 172,918 variables, 3,534,390 orbit-domain clauses, 3,782,477 total clauses, 4,980 XOR rows | 69.37 s outer; 6,271,631,360-byte cgroup peak under 6 GiB |
| One natural relation attempt | `Unknown` after 1 SAT call and 1,000 conflicts; 0 models, 0 relations | 345.914 s solver; 425.59 s outer; 6,442,450,944-byte cgroup peak; no OOM |
| Relation matrix | 0 rows, 8,193 columns, rank 0; no scalar candidate | Full solver terminal retained |

The [protocol](PROTOCOL.md) froze the exact `a=0`, `public_x_hash`, seed
`2026100801` base, 8,192 orbit columns, 1,359,872 points, `m=5`, identity
mask 0, one trial, 32-model ceiling, 1,000-conflict limit, 6 GiB zero-swap
memory, and source commit `f61b33c1b1bb98efa22177a615270c160859a605`.
The Linux arm64 exporter SHA-256 was
`41c968ee8d82d085b8f9eca03ddcbfed69805e415408f36ddc1bcc58ccdeb898`.
The public fixture pair and sampler seed `2026100901` are bound in the
[config](evidence/config.json) and [summary](evidence/summary.json). The
[model](evidence/model.json), [event stream](evidence/events.txt),
[container state](evidence/container-state.json) and raw stage timing logs
retain the completed and zero-yield rows. Full raw files are at
`/Volumes/SSD990-2/.codex/n83-k8192-m5-20261027`.

This is a local guarded stage diagnostic. One capped attempt cannot
estimate relation yield or establish a fastest base; it also supplies no
final factor-base logs or target recovery time. The existing K4096 m5
attempts used different hosts and limits, so their wall times are not a
paired performance comparison. The next experiment is a same-target,
same-tier paired natural corpus with useful-rank measurements, followed by
matrix, target and rho stages. `total_index_calculus_runtime_ms` and
`selected_best_total_runtime` remain null.
