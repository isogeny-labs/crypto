# V5 Linux exporter build: offline dependency miss

The [versioned capacity amendment](../AMENDMENT-2.md) was committed at
`d1ecd2440873` before attempt 2. A [prelaunch disk snapshot](prelaunch-df.txt)
showed 11,650,712 KiB available on the boot volume and 5,659,720 KiB
on SSD990-2. The unchanged [launcher](../run-build.sh) then copied its
source snapshot, checked critical files against the clean checkout, and
entered the pinned Linux ARM64 Rust 1.99.1 container.

The [child receipt](child.receipt) records 2026-11-16
14:13:19–14:13:21 UTC, exit **101**. The complete [build log](build.log)
shows Cargo's offline resolver could not find `aws-credential-types` in
the image registry. The [outer receipt](outer.receipt) records no disk
watchdog stop; [Docker inspect](container.inspect.json) records exit 101,
`OOMKilled=false`, 12 GiB memory, zero swap, four CPUs, and no network.
Container inspection and cleanup both exited zero. The [disk samples](disk.tsv)
and [source attestation](source-attestation.json) are retained.

This is **dependency-cache incompleteness in the pinned toolchain image**.
No Rust compilation completed, no Linux exporter binary was produced,
and no solver work occurred. The [diagram](admission.svg) and inspected
[PDF report](REPORT.pdf) show the reached stage and stop.

The host's configured Cargo registry is a 196 MiB external cache that
contains the locked `aws-credential-types` package and index entry.
The next discriminating step is a separately frozen offline attempt that
copies that cache to a new writable output path, verifies its content
identity, and mounts it as the container registry. It must retain the
same source, image, network ban, resource limits, and a fresh output path.
A missing additional package remains a distinct stopped result; an
offline resolver pass would permit the bounded build, not an IC result.
