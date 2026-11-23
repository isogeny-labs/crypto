# Reviewed collector follow-ups

The original witness and assembler source reviews are preserved in separate packets. New checker and assembler versions leave the original records unchanged. This integration packet records root replay of the new versions and a separate non-author review/replay of earliest-information certificates.

| Packet | Outcome | Reviewer-facing artifacts |
|---|---|---|
| [First information](../signed-first-information/REPORT.md) | 39 minimal determined prefixes; 25 terminal-undetermined systems; all 39 boundaries at native return attempt | [PDF](../signed-first-information/report.pdf), [SVG](../signed-first-information/flow.svg), [CSV](../signed-first-information/systems.csv), [JSON](../signed-first-information/summary.json) |
| [Witness checker v2](../signed-witness-checker-v2/README.md) | 64 certificates and 6900 rows checked; 11 negative controls reject | [PDF](../signed-witness-checker-v2/report.pdf), [SVG](../signed-witness-checker-v2/review-matrix.svg), [JSON](../signed-witness-checker-v2/check-result.json) |
| [Runtime assembler v2](../packets/runtime-assembler-v2/REPORT.md) | 18 mocked tests pass; final verification expiry/signal guards added | [PDF](../packets/runtime-assembler-v2/report.pdf), [JSON](../packets/runtime-assembler-v2/test-receipt.json) |

The first-information result is a statement about saved linear systems: an isolator and particular solution prove consistency plus uniqueness at the first prefix; the preceding affine family proves every earlier prefix remains undetermined. The direct checker was replayed by a non-author with optimization disabled. Candidate counts are not measured costs. Original software fixtures, full scientific cohort obligations, failed attempts and planning limits remain unchanged.

The runtime correction has separate root source inspection and mocked replay. Whole-process/daemon containment and prospective execution admission remain dependencies. No scientific worker, collector rerun, image build or cloud mutation occurred in this publication task.
