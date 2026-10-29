# Aimed gather, follow-up: a larger same-session A/A (2026-10-29)

[`PROTOCOL.md`](PROTOCOL.md) was committed and pushed (`5d726412b`) before
the first run. Report PDF: [`report/report.pdf`](report/report.pdf). The
first session is [`../README.md`](../README.md); its records are unchanged.

**Class: engineering.** A native component speed-up of IC relation
collection. Every counted operation is identical between arms, so GAE
records and every IC row's counted columns stand.

## Result

42 rounds of baseline `a`, baseline `q` and candidate `c` of `ic price`
(rule v3 `k0n53/T01`, curve `icv1-f2m53-tm56619371-dac20a85`), the order
cycling through all six permutations; 126 processes under
`isolated_bench --cpus 3`, none contended, none failed, no round dropped (71
launches refused for CPU pressure and retried).

| metric | A/B median (main / candidate) | A/B 95% | rounds > 1 | A/A 95% | verdict |
|:--|--:|:--|--:|:--|:--|
| collect phase | 1.093 | 1.090 – 1.160 | 38 / 42 | 0.977 – 1.030 | **established** |
| whole set-up | 1.042 | 1.021 – 1.116 | 29 / 42 | 0.941 – 1.035 | not established |

- **Collect phase:** median 230.3 ms and 235.2 ms (main, `a` and `q`)
  against 212.5 ms (candidate). The rule declared on 2026-10-23 holds: the
  A/B lower end clears 1 and the A/A upper end.
- **Whole set-up:** not established; 1.021 is below the A/A's 1.035. The
  set-up includes phases the change does not touch.
- **Identity:** all 126 processes recovered `518509155375`,
  `all_verified: true`.
- The A/B interval is lopsided because the per-round ratios cluster just
  above the median; its lower end still clears the A/A upper end by 0.06.

![per-round collect ratios](report/per-round-collect.svg)
![both sessions](report/sessions-intervals.svg)

## Limits

- One host class (an x86-64 cloud VM with AVX-512), one size (n53), one
  params file. Not on Iso Lab: see `PROTOCOL.md`, "Route".
- The leaderboard's `icv1-f2m53-tm56619371-dac20a85` one-target row (rule
  v7, S timed) was measured before this change and is not re-priced here.
- No IPC: no hardware PMU.

## Reproduce

```sh
# ic-baseline at d34b9b390 and ic-candidate at 6bc18857b:
cargo build --release --bin ic --bin isolated_bench
sha256sum -c research/ic_aimed_gather_20261023/aa2_20261029/runs/arms.sha256   # in the binaries' directory
BIN=<dir> bash research/ic_aimed_gather_20261023/aa2_20261029/run.sh
bash research/ic_aimed_gather_20261023/aa2_20261029/summarise.sh
python3 research/ic_aimed_gather_20261023/aa2_20261029/report/plots.py
```
