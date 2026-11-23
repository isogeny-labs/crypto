# Disjoint enrollment preparation packet

This packet preserves the original request bytes, six native preparation outputs, source snapshots, software-test receipt and raw logs, independent source/request review, and root's preparation/check command logs. The independent review and 11 software tests preceded plan generation; they are not relabelled as a review of subsequent empirical results. The subsequent native prepare and solver-free check both completed successfully, as reported by the root executor. Preparation made one plan-only invocation and zero solver executions.

The neutral PDF, SVG and standardized report JSON are in `report/`. The verified partition contains 1,260 workloads, 22,680 planned slots, 18,900 measured arm slots, 3,780 warmups and 6,300 measured pairs. Seven original plans correspond to eight enrolled plans. This is preparation evidence only: scientific reuse admission, clocks, budgets and analysis/export integration remain pending. No new empirical cost result is reported.

`inputs.sha256` binds and verifies the actual external input files; original paths remain intact in the request/certificate. `source.sha256` verifies preserved native preparation sources against the software receipt. `SHA256SUMS` binds the whole local packet. Source files and test receipts are snapshots, not active analysis consumers.

The two mapping JSONL files remain local pending root publication through the maintained artifact-store helper. In particular, `outputs/slot-mapping.jsonl` exceeds 1 MiB and must not be committed as a raw Git blob. Root should publish both mapping files, retaining their paths through artifact manifests. No file has been staged by this packaging task.

The native Rust renderer is in `renderer/`; it reads the successful exact-count receipt and uses native pdflatex with shell escape disabled. Its release build, Clippy check and actual one-page PDF render succeeded. The PDF was visually inspected with no clipping. Renderer checks are separate from the 11 preparation software tests; no additional solver or plan generation was run here.
