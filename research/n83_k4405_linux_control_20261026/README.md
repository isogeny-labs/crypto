# K4405 current-source Linux control

This wrapper carries the exact a1 K4405 `wide3` full-process control from the
local [resource receipt](../n83_k4405_current_source_full_control_20261026/README.md)
to a Linux Pod with a clean 64 GiB-class memory cgroup. It uses fixture zero
from the earlier public N83 target set. The sealed cross-family points are
reserved for the later paired Tier 1 comparison.

The source hash, prior source flags, target and answer sidecar are checked
before execution. The Linux executable is built from that source, copied to
`output/executed_binary/koblitz_orbit_dlp_fast`, checked byte for byte, and
launched from that retained copy. `config.json` records its platform-specific
hash and executed path; `receipt.json` records the retained copy's hash. Copy
the whole output directory for independent replay. The worker records
cgroup v1 or v2 memory, swap, CPU quota, CPU statistics, memory events,
affinity, and a 0.5-second time series. The v1 path uses the memory+swap
counter or `memory.stat`'s `total_swap` and rejects a Pod that exposes
neither. A new cgroup OOM or limit-denial counter marks the receipt
`CGROUP_OOM`. The wrapper stops at 1,200 seconds, 48 GiB cgroup use, or
2 GiB swap use. Admission requires a finite cgroup limit of 56–128 GiB,
at least four effective CPUs, and at most 8 GiB initial cgroup use.

**Run only after an independent provider deletion watchdog is armed and its
deadline and cost envelope have been verified.** This process guard stops the
worker, not Pod billing. On 2026-10-26 the approved GPU pair was out of
capacity; starting a CPU Pod requires a separate approved substitution. No
Pod was started by adding this wrapper.

On a qualifying Pod, pin the checkout to the reviewed commit, retain its
`git rev-parse HEAD`, `rustc -V`, `cargo -V`, and `sha256sum Cargo.lock`, then:

```sh
export CARGO_TARGET_DIR=/workspace/n83-k4405-target
cargo build --release --locked --example koblitz_orbit_dlp_fast
python3 research/n83_k4405_linux_control_20261026/run.py \
  --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
  --output /workspace/n83-k4405-control --preflight
python3 research/n83_k4405_linux_control_20261026/run.py \
  --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
  --output /workspace/n83-k4405-control
```

The exact curve is `EC1N83Ckb1a1haacb69ee9f37`. The frozen candidate is
`IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h76ff6ddee28c`.
The wrapper's `PASS_PROCESS_EXIT` status only means the producer exited zero;
rank, relation verification, base identity and scalar replay require a
separate independent audit of its emitted artifacts before promotion. A
cap, telemetry failure, OOM or other failure remains a retained result row.
