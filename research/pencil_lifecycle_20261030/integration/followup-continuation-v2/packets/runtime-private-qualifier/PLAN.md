# Private qualifier source plan

Task T-1417, 2026-11-03. New copy of runtime-qualifier-v5; preserve original qualifier/gate packets. This is source and mocked validation only. Do not invoke sudo, Docker, systemd, containers, image build/load, cloud or compile the gate. Twelve Python source/mock checks are bounded by 60 seconds and 512 MiB virtual memory. Runtime planning remains 25,200 active-worker seconds ending 2026-11-04T06:00:00Z; this is not admission.

Prospective qualification runs in its own bounded 300-second service, with 30 seconds reserved for cleanup and 20 seconds per active Q case. Assembly is a separate 600-second service; it must stop and prove its cgroup empty before qualification reopens the retained bounded disk. Qualifier uses only the fresh qualification service endpoint. No shared daemon migration/import is implemented here.

Preserve Q0-Q4, owner hash, fixture/gate hashes and review joins, static ELF checks, exact image/member inspection, strict empty writable-layer diff, one container, 1 PID / 512 MiB / 1 CPU and restrictive mounts/capabilities. New gate requires new binary/control/review evidence before admission. Actual runtime and image admission remain false.

Source review should attack endpoint disagreement, daemon replacement, sibling cgroup prefix confusion, PID reuse, inode mismatch, release before ancestry, partial copy exposure, wrong/replayed tokens, late signal/deadline success and old gate evidence substitution. No new scientific data or statistical comparison.
