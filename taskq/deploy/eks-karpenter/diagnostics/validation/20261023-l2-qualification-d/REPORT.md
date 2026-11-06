# L2 qualification D did not execute

**The 1,800-second provisioning deadline expired without a container starting.**
AWS rejected worker launches with `MaxSpotInstanceCountExceeded` under the
unchanged 96-vCPU account quota. The Job failed at 01:20:15 UTC on 2026-10-23.

The frozen 600-round c8a workload is unmeasured: zero L2/replay/A/A results and
zero S3 measurement payloads. This is a capacity failure, not an L2 verdict.
Its acceptance criteria remain 3,606 verified L2 records, 3,600 native replays,
600 admitted A/A pairs and 19 S3 payload hashes. No measurement was retried.

See [verification](verification.json), the [diagram](flow.svg) and
[inspected PDF](report.pdf). Check `sha256sum -c runs.tar.xz.sha256` and expand
`tar -xJf runs.tar.xz` for the original protocol, Job manifest, AWS/Kubernetes
failure events, S3 listing and receipts. The protocol itself was uploaded to S3
before launch; there are no benchmark outputs to claim as saved.

The [separate counter diagnostic](../20261023-cpu-accounting/REPORT.md) supports
the v5 candidate fix but does not replace this qualification. Promotion, final
PR review/CI, merge and the main-only GitHub Action remain pending. No algorithm
performance or security claim is made; canonical algorithm graphs are unchanged.
