#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)

= Run report: Part A sessions A-n11, A-n13, A-n17 (launch 2026-10-12 02:30 UTC)

*Descriptive run report* (research-visuals, "Every experiment run"): what ran and what was recorded; interpretation is in the study README.

== Sessions

- `A-n11`: session `ECBS1hb842db9ff0ac`, spec `ECS1ha99bdb902e95` (pdp\_split\_slots\_20261012 Part A, n=11); status `complete`, 48 executions planned, status counts `{"verified": 48}`; commit `94428d2ae7d85c8f5db78530298bb92d741c8fe4` (dirty: False), binary SHA-256 `226a31f06997964c…`; targets: 8 per curve, seed 202610121101, rounds 1, isolation required L0.
- `A-n13`: session `ECBS1h2c0810d330ce`, spec `ECS1h8c7e4bddda48` (pdp\_split\_slots\_20261012 Part A, n=13); status `complete`, 48 executions planned, status counts `{"verified": 48}`; commit `26233e4b77f046e4fbd237b93df9117dcd0e6e35` (dirty: False), binary SHA-256 `226a31f06997964c…`; targets: 8 per curve, seed 202610121101, rounds 1, isolation required L0.
- `A-n17`: session `ECBS1haa051de697f4`, spec `ECS1h60403669a052` (pdp\_split\_slots\_20261012 Part A, n=17); status `complete`, 32 executions planned, status counts `{"verified": 32}`; commit `26233e4b77f046e4fbd237b93df9117dcd0e6e35` (dirty: False), binary SHA-256 `226a31f06997964c…`; targets: 8 per curve, seed 202610121101, rounds 1, isolation required L0.

== Recorded values (medians over verified runs; per-workload values in ANALYSIS.json)

#table(columns: 8, align: (left, left, right, right, right, right, right, right),
[curve], [arm], [verified], [median S (recorded)], [median F4 XORs], [median systems], [median priced S], [priced ÷ strong rho (min–max)],
[icv1-f2m11-t67-05f5aa36], [clo-chain-g4], [8/8], [16.339], [2.577e+08], [25.5], [22449.379], [1133.383 (75.7–1.49e+03)],
[icv1-f2m11-t67-05f5aa36], [clo-unsplit-g4], [8/8], [16.339], [7.808e+06], [23.0], [695.666], [35.705 (2.52–52.2)],
[icv1-f2m11-t67-05f5aa36], [rho-frob], [8/8], [2.700], [—], [—], [2.700], [0.133 (0.00732–0.162)],
[icv1-f2m11-t67-05f5aa36], [rho-strong], [8/8], [19.743], [—], [—], [19.743], [1.000 (1–1)],
[icv1-f2m11-t67-05f5aa36], [std-chain-g4], [8/8], [18.814], [6.264e+08], [53.5], [54535.836], [2636.196 (202–3.68e+03)],
[icv1-f2m11-t67-05f5aa36], [std-unsplit-g4], [8/8], [18.814], [1.992e+07], [53.5], [1753.772], [85.119 (6.71–120)],
[icv1-f2m13-t181-515ee569], [clo-chain-g4], [8/8], [13.364], [8.391e+08], [36.0], [43211.085], [3025.808 (2.31e+03–4.05e+03)],
[icv1-f2m13-t181-515ee569], [clo-unsplit-g4], [8/8], [13.364], [1.688e+07], [36.0], [882.070], [60.298 (48.6–80.5)],
[icv1-f2m13-t181-515ee569], [rho-frob], [8/8], [1.799], [—], [—], [1.799], [0.125 (0.107–0.146)],
[icv1-f2m13-t181-515ee569], [rho-strong], [8/8], [14.318], [—], [—], [14.318], [1.000 (1–1)],
[icv1-f2m13-t181-515ee569], [std-chain-g4], [8/8], [16.887], [1.486e+09], [70.5], [76539.088], [5341.416 (2.86e+03–8.25e+03)],
[icv1-f2m13-t181-515ee569], [std-unsplit-g4], [8/8], [16.887], [3.912e+07], [70.5], [2030.543], [141.705 (75.6–219)],
[icv1-f2m17-tm101-00378d4e], [clo-unsplit-g4], [8/8], [3.631], [1.509e+10], [156.5], [1.151e+05], [25257.008 (2.07e+04–3.72e+04)],
[icv1-f2m17-tm101-00378d4e], [rho-frob], [8/8], [1.002], [—], [—], [1.002], [0.213 (0.141–0.305)],
[icv1-f2m17-tm101-00378d4e], [rho-strong], [8/8], [4.223], [—], [—], [4.223], [1.000 (1–1)],
[icv1-f2m17-tm101-00378d4e], [std-unsplit-g4], [8/8], [5.359], [4.052e+10], [398.0], [3.091e+05], [68366.748 (4.53e+04–1.25e+05)],
)

`S` = charged group-addition equivalents / √r. F4 arms: solver word XORs counted, not charged, so recorded S is a lower bound. Priced S adds XORs × the curve's pinned `ns_per_word_xor` (`docs/ic/calibration.json`, calibrated on another host) / √r: a sensitivity, and one that understates F4's per-call work. Table arms (`mitm-frobenius`) are fully charged. Wall time is descriptive only (L0).

#figure(image("fig-ecbench-s.svg", width: 100%), caption: [Recorded values from ANALYSIS.json; layout `reports/plot_ecbench.py`.])

== Raw records

`research/pdp_split_slots_20261012/sessions/A-n11/records.jsonl`, `research/pdp_split_slots_20261012/sessions/A-n13/records.jsonl`, `research/pdp_split_slots_20261012/sessions/A-n17/records.jsonl`; audits under `audits/`.

A session marked dirty ran with untracked or modified files in the checkout (drafts of this study's own reports and protocols); the binary SHA-256 above is the one `BINARIES.sha256` records, and `ecbench verify` replays against it.
