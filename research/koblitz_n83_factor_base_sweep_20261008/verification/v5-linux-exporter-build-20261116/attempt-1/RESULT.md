# V5 Linux exporter build: host admission stopped

The [frozen build protocol](../PROTOCOL.md) and [launcher](../run-build.sh)
were committed at `5ae32d781cbd` before this attempt. The launcher ran
at approximately 2026-11-16 14:03 UTC on the custom `n83-arm64` Colima
host. It exited **93**, with the exact [stderr](stderr.txt) `boot below 2 GiB`.
The first host gate precedes source copying, Docker launch, or Cargo build.
The intended pinned exporter source was
`cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.

| Observation | Evidence |
|---|---|
| Command and exit | [command](command.txt), [exit](exit.txt), [stderr](stderr.txt) |
| Post-stop boot space | 1,150,912 KiB available in a later [disk snapshot](post-stop-df.txt), below the 2,097,152 KiB gate; this is not asserted as the exact value at launch |
| Colima profile | [status](colima-status.txt): running ARM64 profile |
| Container inventory | [empty post-stop inventory](containers.txt); no build container was created |
| Source/output paths | Both were absent immediately after the stop; no snapshot, compiled worker, or build log was produced |

The [admission diagram](admission.svg) and [PDF report](REPORT.pdf)
show the attempted path and the stop. This is a **host-capacity admission
failure**, not a compiler result, solver result, or total-runtime
measurement. The earlier local Colima inventory used the default home
and missed the running custom profile; the custom socket was subsequently
verified, and the boot-space gate was the actual stop in this packet.
The full-library protocol retains its separate, larger disk floors.

The next attempt needs an observed increase in boot-volume headroom and
a versioned new packet or amendment before launch. Preserve this exit 93
without relabelling it as a completed Linux exporter build.
