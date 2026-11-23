# Necklace-Reduced x-Only Sibling k-Sum Tables with Free Setup Collisions (`2026-11-18`) [pcc:unverified]

- **Protocol (frozen before measurement at commit `bac5b145b5`):** [`PROTOCOL.md`](PROTOCOL.md) (`REGISTRATION.sha256`) [pcc:unverified]
- **Results report PDF:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)) [pcc:unverified]
- **Descriptive run report PDF:** [`reports/run-20261118T2145Z/run-report.pdf`](reports/run-20261118T2145Z/run-report.pdf) [pcc:unverified]
- **Data & receipts:** [`summary.json`](summary.json) (sessions `ECBS1h9130ca664e3d` through `ECBS1h022bf6240f56`, binary `0dd4f18943744e11546be87399393397648f8f7c5ecd3e92109d5a1f2c89107f`, `git_dirty = false`) [pcc:unverified]
- **Figures:** [`fig-band-speedup.svg`](fig-band-speedup.svg) / [`fig-band-speedup.png`](fig-band-speedup.png) and [`fig-bitop-and-binary.svg`](fig-bitop-and-binary.svg) / [`fig-bitop-and-binary.png`](fig-bitop-and-binary.png) (generated from `summary.json` by [`plot.py`](plot.py)) [pcc:unverified]

## Headline Measured Outcomes (`AGENTS.md` §5a Band & `bitop/v1`) [pcc:unverified]

**Track:** Generic and Frobenius-orbit decomposition ($\tilde{\Omega}(\sqrt{r / n})$ on Koblitz curves, $\tilde{\Omega}(\sqrt{r})$ on generic binary curves; bounded by Pollard rho in asymptotic exponent). All 2,992 measured executions across 9 held-out sessions solved `[k]G == Q`, reproduced identically under `ecbench verify --replay-all`, and achieved `bitop/v1` `complete = true` (`unpriced = []`). [pcc:unverified]

| Session | Curve Slug | $\log_2 r$ | Strictest IC Incumbent | Incumbent $S(0 / 0.1 / 1)$ | `cand-kf4-c2-free` $S(0 / 0.1 / 1)$ | Speedup $w=0$ [95% CI] | Speedup $w=1$ [95% CI] | L0 Solve Wall Ratio |
|:--|:--|--:|:--|--:|--:|:--|:--|--:|
| `ECBS1h9130ca664e3d` | `icv1-f2m29-tm40309-30c52b96` | 15.37 | `inc-gen-layout-c2` (`kf3`: 7.719) | `1.052 / 1.675 / 7.284` | `0.665 / 1.244 / 6.455` (`kf5-c1`: `0.327 / 0.594 / 3.003`) | **1.58×** `[1.54, 1.62]` (`kf5`: **3.22×**) | **1.13×** `[1.12, 1.14]` (`kf5`: **2.43×**) | `1.06×` (`kf5`: `1.20×`) |
| `ECBS1hb7bd5c42674b` | `icv1-f2m23-t5197-69e76b73` | 21.00 | `inc-gen-layout-c6` | `1.012 / 1.352 / 4.412` | `0.522 / 0.728 / 2.581` | **1.94×** `[1.65, 2.28]` | **1.71×** `[1.55, 1.89]` | `1.39×` |
| `ECBS1hb366f3156d09` | `icv1-f2m23-tm5197-1f85e9e1` | 22.00 | `inc-kf3` | `1.044 / 1.363 / 4.230` | `0.445 / 0.621 / 2.201` | **2.34×** `[1.82, 3.02]` | **1.92×** `[1.62, 2.28]` | `1.73×` |
| `ECBS1h3f13fef4265b` | `icv1-f2m37-tm534059-32aad96b` | 27.78 | `inc-gen-layout-c13` | `0.765 / 0.964 / 2.753` | `0.326 / 0.435 / 1.414` | **2.34×** `[1.98, 2.77]` | **1.95×** `[1.73, 2.19]` | `2.34×` |
| `ECBS1h764e82418bf0` | `icv1-f2m41-tm2308219-7f48b14a` | 39.00 | `inc-gen-layout-c90` (`c158`: 2.223) | `1.964 / 2.364 / 5.959` | `0.322 / 0.425 / 1.346` | **6.09×** `[5.32, 6.97]` | **4.43×** `[4.00, 4.91]` | **7.14×** (`366 -> 56 ms`) |
| `ECBS1h0a25307fd6a4` | `icv1-f2m53-tm56619371-dac20a85` | 44.26 | `inc-gen-layout-c220` | `2.856 / 3.431 / 8.602` | `0.241 / 0.314 / 0.968` | **11.84×** `[10.00, 14.03]` | **8.89×** `[7.84, 10.08]` | **14.75×** (`4334 -> 266 ms`) |

On Cohort B (generic random-binary Table B primary curves `icv1-f2m21-t2591-de54d72e`, `icv1-f2m24-tm3047-2d0c0e8c`, `icv1-f2m27-t12543-569dca8b`), `BinaryGroup::sibling_pair` (`cand-ksum5-sib-free`) cuts counted cost by 10.6%–22.0% at $w = 0$ and 6.9%–17.6% at $w = 0.1$ (`1.12×–1.28×` speedup, `1.14×–1.17×` wall-clock speedup), strictly outranking `inc-ksum4-prune` across $w \in [0, 1]$ while tying `inc-ksum5-prune` at $w = 1.0$ on `B27` ($R_1 = 1.000$) due to `free=1` mod-$r$ pre-checks at matched base size $N = 12$. Canonical scoreboard files (`docs/ic/LEADERBOARD.md`, `docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`) were checked and left unchanged pending second-host replay validation (`docs/VALIDATION_GATE.md`). [pcc:unverified]
