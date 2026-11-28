# V5 Linux exporter build: disk-floor stop

The frozen [attempt-4 amendment](../AMENDMENT-4.md) and
[launcher](../run-build-attempt-4.sh) were committed at `3c16b3dfe617`
before execution. The source snapshot was pinned to
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`; the image was
`sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
The Linux ARM64 container used Rust 1.99.1, one Cargo build job, a
12 GiB memory cgroup, zero swap, four CPUs, no network, and
`--release --locked --offline`.

**Observed:** The [child receipt](child.receipt) spans
2026-11-16 14:28:27–14:46:54 UTC (18 min 27 s). Offline dependency
resolution passed and Cargo compiled the library; the
[build log](build.log) ends with library warnings. The
[disk watcher](disk.tsv) recorded 3,393,656 KiB available on SSD990-2
at launch, 2,527,196 KiB at 14:46:39, and 1,893,556 KiB at
14:46:49. At that final sample it crossed the frozen 2,097,152 KiB
floor and stopped the container. The [outer receipt](outer.receipt)
records `disk_stop=true`, child exit 137, and successful inspection
and cleanup. [Docker inspection](container.inspect.json) reports
`OOMKilled=false`, exit 137, and the pinned resource/network limits.
The [post-stop container inventory](post-stop-containers.txt) is empty.

The build generated 1,395,996 KiB of target output at the
[post-stop size check](input-output-size-kib.txt). No exporter binary
was present. This is a **disk-capacity stop**, not a compiler result
or a memory OOM. There was no factor-base export, solver call, relation
measurement, or total IC timing in this attempt.

The [disk plot](disk.svg) renders every recorded SSD990-2 sample
against the fixed 2 GiB threshold; the [flow diagram](admission.svg)
shows the reached and censored stages. The [inspected PDF](REPORT.pdf)
contains both. [File hashes](sha256.txt) cover the raw receipts and
post-stop checks. The frozen numeric IC scoreboard and canonical
performance graphs were checked conceptually and left unchanged:
this attempt has no IC performance result to add.

**Next discriminating step:** Build the same source and image on a
qualified host with at least the recorded target footprint plus a
2 GiB scratch floor, while retaining the current 12 GiB memory,
no-network and offline gates. A successful binary build would permit
the separately frozen factor-base export and paired solver controls;
it would not by itself establish an end-to-end factor-base winner.
