# V5 Linux exporter build: passed after boot-output mount

The [attempt-5 amendment](../AMENDMENT-5.md),
[cache-preservation amendment](../AMENDMENT-5A.md), and
[launcher](../run-build-attempt-5.sh) were committed before execution.
The source snapshot remained
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`, and the image remained
`sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
The Linux ARM64 container used Rust 1.99.1, one Cargo build job, a
12 GiB memory cgroup, zero swap, four CPUs, no network, and
`--release --locked --offline`.

**Observed build result:** The [child receipt](child.receipt) spans
2026-11-16 14:57:56–15:15:23 UTC (17 min 27 s at one-second
resolution). Cargo reported `Finished release ... in 17m 26s` in the
[build log](build.log). The [outer receipt](outer.receipt) records child,
inspection and cleanup exits all 0, with `disk_stop=false`.
[Docker inspection](container.inspect.json) reports exit 0,
`OOMKilled=false`, and the pinned resource/network limits. The
[post-run container inventory](post-run-containers.txt) is empty.

The resulting [Linux ARM64 ELF](binary.file) is 9,884,664 bytes.
Its SHA-256 is
`87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`,
verified against the [retained binary](koblitz_n83_factor_base_export)
and [build output receipt](binary.sha256). The binary was **built**;
its V5 route, factor-base import, solver, and target verification were
not executed in this attempt.

The [mount setup](setup/guest-mount.txt) records the writable boot
virtiofs output path. The incomplete attempt-4 build cache was
[preserved on the boot volume](setup/build-cache-after-kib.txt) and
was not reused. Across all 105 [ten-second disk samples](disk.tsv),
the minimum boot-volume availability was 14,733,820 KiB and the
minimum SSD990-2 availability was 3,579,248 KiB, both above the
2,097,152 KiB stop floor. The [plot](disk.svg) and
[flow diagram](admission.svg) appear in the inspected
[PDF](REPORT.pdf). [Raw file hashes](sha256.txt) permit receipt replay.

This is an **engineering build gate**, not a factor-base or IC runtime
comparison. No new base was generated or uploaded. The selected
minimum cold single-target IC runtime remains unmeasured. The canonical
IC scoreboard and performance graphs remain unchanged because this
run adds no relation, rank, target or algorithm timing result.

**Next gate:** Run a frozen no-work binding check with this exact
source/binary and a fresh readback of the retained K600 object. Then,
subject to the existing host/input controls, run the constructed
positive and ordinary public-point arms with native replay. A binary
build alone does not admit a factor-base winner.
