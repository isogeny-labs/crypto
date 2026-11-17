#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

= IC collection, aimed gather: a larger same-session A/A (2026-10-29)

*Study:* `research/ic_aimed_gather_20261023/aa2_20261029/` · *Protocol:* `PROTOCOL.md`, committed and pushed (`5d726412b`) before the first run · *Arms:* the first session's binaries, by `sha256` (`runs/arms.sha256`): main `d34b9b390` and the change `6bc18857b` (merged in \#2448) · *Workload:* `ic price` on rule v3 `k0n53/T01.params.json`, curve `icv1-f2m53-tm56619371-dac20a85` · *Host:* one x86-64 cloud VM, Intel Xeon @ 2.10 GHz, 4 vCPU, AVX-512, kernel `6.18.44-fc-v185`, no hardware PMU (`runs/host.json`).

== Question

The first session (2026-10-23) measured the collect phase 1.111× faster over 30 paired rounds ([1.082, 1.149] with the corrected bootstrap below; published as [1.065, 1.160]), but its A/A had 5 pairs, with an upper end of 1.087. The rule declared before measuring needs the A/B interval's lower end above 1 and above the A/A interval's upper end, so the verdict was *not established*. This session measures the noise floor with as many rounds as the effect, in the same session.

== Design

42 rounds. Each round runs baseline `a`, baseline `q` and candidate `c`, in the order given by cycling through all six permutations, so each label runs in each position seven times. Per round: A/B = `a/c`, A/A = `a/q`. Every process ran under `isolated_bench --cpus 3`, `RAYON_NUM_THREADS=1`. The statistic and rule are the first session's: the median of per-round ratios, a 95% percentile bootstrap over rounds (2,000 resamples, a fixed LCG, seed 20261029).

== Result (measured)

#table(columns: 6, align: (left, right, left, right, left, left),
  [*metric*], [*A/B median*], [*A/B 95%*], [*rounds > 1*], [*A/A 95%*], [*verdict*],
  [collect phase], [1.093], [1.068 – 1.136], [38 / 42], [0.977 – 1.026], [*established*],
  [whole set-up], [1.042], [1.020 – 1.082], [29 / 42], [0.946 – 1.015], [marginal (+0.005), not claimed],
)

#block(stroke: 0.6pt + rgb("#c44e52"), inset: 6pt, radius: 3pt)[*Correction.* The bootstrap as first run (in `summarise.sh` and the first session's `../summarise.sh`) stepped its LCG in jq's floating point; the rounded states are almost always even, so 99.6% of resample draws fell on odd-numbered rounds. The independent validator found it. The intervals above use the same constants, seed and resample count with the step done exactly and each resample index taken from the generator's high bits; a first correction (exact step, index `s mod n`) still drew every resample half odd and half even rounds, a defect found in review (`correct_bootstrap.sh`, `summary-corrected.json`, where both are kept); `CORRECTION.md` lists the as-run intervals beside them. As run, the collect interval was 1.090 – 1.160 against an A/A upper end of 1.030, and the whole set-up was not established.]

#figure(image("per-round-collect.svg", width: 100%), caption: [Per-round collect-phase ratios, A/B (green) and A/A (blue), with each median and its corrected 95% interval (`runs/`, `summary.json`, `summary-corrected.json`).])

- *Collect phase: established.* The A/B lower end, 1.068, clears 1 and the A/A upper end, 1.026, by 0.042. It also held under the as-run generator, and under 500 of 500 independent seeds (validator). Median collect time: 230.3 ms (main `a`) and 235.2 ms (main `q`) against 212.5 ms (candidate).
- *Whole set-up: marginal, not claimed.* Corrected, 1.020 clears the A/A's 1.015 by 0.005; as run, it did not. The generator was fixed after the data were seen, so this verdict is not claimed. The set-up includes phases the change does not touch.
- *Isolation:* 126 processes, none contended, none failed, no round dropped. 71 launches were refused for CPU pressure and retried; a refusal measures nothing.
- *Identity:* all 126 recovered `518509155375` with `all_verified: true`; an independent replay matched every non-timing field in both arms.

#figure(image("sessions-intervals.svg", width: 100%), caption: [Both sessions' A/B and A/A intervals, corrected. The first session's A/B effect is the same size; its 5-pair A/A was too wide to rule on, and its verdicts do not change with the correction.])

== What this changes, and what it does not

- *Class: engineering*, a native component speed-up. On the measured params every counted operation is identical between arms; identity was checked on these params only.
- *The board's row is not re-priced here.* Rule v7 times S, so a faster collect phase should eventually lower the `icv1-f2m53-tm56619371-dac20a85` one-target row (IC 8.146 against rho 1.159 S, 7.03×). Re-pricing it needs rule v7's 64-target protocol and is not claimed.
- *Not pooled* with the first session: different kernel, so a different host state. The first session's records are unchanged.
- *Not measured:* other sizes' native time, other hosts, IPC (no PMU). The run was local, not on Iso Lab: the reachable EKS paths run ecbench specs only, and this build-against-build A/A needs each round's three processes on one host (`PROTOCOL.md`, "Route").
