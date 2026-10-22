# Reproduction

From the repository root:

```sh
cd research/kummer_differential_collector_20261022
TMPDIR=/var/tmp cargo build --release --locked --offline --bins
cd ../..
research/kummer_differential_collector_20261022/target/release/setup
research/kummer_differential_collector_20261022/target/release/uncached
KUMMER_ARMS=uncached research/kummer_differential_collector_20261022/target/release/analyze
MPLCONFIGDIR=/var/tmp/kummer-mpl uv run --with matplotlib python3 research/kummer_differential_collector_20261022/plot.py
```

When the optional cached arm finishes, run its binary and then repeat the
native analyzer without `KUMMER_ARMS` to include both complete arms. Python
is used only to lay out the PDF and graph from the native result files.
The analyzer also emits `replay-certificate.json` and `validator-report.json`
with SHA-256 pins for all raw inputs. The archived rho was collected in a
separate process, so these artifacts document the result but do not yet
satisfy the leaderboard's same-process rho gate.

The setup binary reconstructs and matches each frozen root set, theta tuple
and torsion matrix before emitting counts. It also recomputes each source
target's transfer and `k·P=Q`. The collector recomputes every candidate
scalar through signed graph elimination and checks its transferred group
identity and planted scalar. The independent Rust analyzer recomputes
`k·P=Q` on the original elliptic curve over `Fp²`, joins the archived rho
control and recalculates every cost cell from raw operations.
