# Factor-base and jump sweep, committed-source replay

- **Run ID:** `20261021T1538Z-factor-base-grid`.
- **Source:** `f6a440f0eaef939094062012b7db4aee9f550529`.
- **Host:** `ip-172-31-26-135.us-east-2.compute.internal`, Linux 6.18.54-121.166.amzn2023.x86_64, Intel Xeon 6975P-C, 32 visible CPUs.
- **Isolation:** L0, five independent single-threaded arm processes run concurrently on the same host. Wall time is a diagnostic, not a qualified comparison.
- **Input:** the original 21 exact-order curves × 60 public target workloads, `sessions_projective_chart_v2.tar.zst`, SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
- **Command:** `bash research/four_followups_20261021/run_factor_base.sh` after `cargo build --release --locked --manifest-path research/four_followups_20261021/Cargo.toml --bin factor-base`.
- **Arms:** full-base 8/16/32 jumps; half-base and quarter-base 16 jumps. The target-blind membership rule, seeds, caps and cost interval are frozen in [`PROTOCOL.md`](../../PROTOCOL.md).
- **Requested/completed:** 35 arm-size files × 180 targets = 6,300/6,300 per-target solves, with 6,300 source scalar checks. Failures and exhaustions: zero. Each target is a separate cold solve.
- **Raw:** [`results/fb*-bits*.jsonl`](../../results/) and matching `.time` files. The analyzer's joined per-target rows, price grid and summary are in the same directory.
- **Comparison:** representative complete cold prices and all 36 pricing cells are in the [report](../../REPORT.md); the graph is [`factor-base-grid.svg`](../../factor-base-grid.svg) and the PDF is [`REPORT.pdf`](../../REPORT.pdf). The matched source rho uses the exact same target cohort and frozen source-addition unit.
- **Uncertainty:** one algorithm seed per target, no confidence interval; no isolated wall-speed or fastest-external-tool result.

The pre-commit development run is retained through `results/initial-run.sha256`
and `results/initial-times.tar.gz`. The committed-source replay is checked
against those 35 file digests, not assumed identical.
