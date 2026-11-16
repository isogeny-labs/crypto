#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= N83 K600 Colima charged-build attempt

*Run:* `run-20261116T0109Z`  ·  *Source:* `519dd25d3d0e`  ·  *Status:* cold Rust stage capped

The frozen no-work admission passed from 01:09:04 to 01:10:02 UTC (58 seconds). It verified clean source, plan and archive hashes, producer ELF, Docker socket and a Linux ARM64 cgroup with 12 GiB memory, zero swap, four CPUs and 512 PIDs. The native charged-build producer then ran from 01:10:18 to 01:31:14 UTC (1,256 seconds outer interval) under the original three-step plan.

#image("build-flow.svg", width: 100%)

The first `rust-binaries` stage had a frozen 1,200-second cap. Its `rust-binaries.json` reports `wall_capped=true`, `exit_code=null`, `status=STOPPED`, and 1,214.178 seconds to receipt start including termination and checks. Source, program and inputs remained unchanged. Docker exit was 1 and `OOMKilled=false`. `failure.json` records `build step rust-binaries stopped`. The solver and runtime-image stages did not start; the K600 worker did not start. The source checkout stayed clean, the named container was removed, and no Cargo/rustc process remained in the VM.

The raw evidence is `admission.log`, `admission.receipt`, `build.log`, `build-child.receipt`, `build-outer.receipt`, `build-container.inspect.json`, `rust-binaries.json`, `rust-binaries.stderr`, and `failure.json` beside this PDF. The charged-build stderr is 15,952 bytes, SHA-256 `1b3703824145f73ec1a82bcd6e7cf1d0ab521855c2a7f1b3ae9b16c0fa01085e`. The incomplete 1.3 GiB Cargo target is a local cache, not a valid build artifact.

The prior bootstrap took 1,446 seconds and remains a separate setup interval. This stopped build supplies no relation yield, rank, descent, matched rho or total cold one-target IC cost. The exact blocker is the cold locked single-job five-example build exceeding the frozen Rust stage cap on this host. A new versioned plan and full source/input/solver/image gates are needed before any worker run or factor-base selection. No canonical IC graph changes.
