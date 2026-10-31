#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)

= Run report: interrupted session A-n19-02 (amendment 1 rerun; 24 of 40 executions)

*Descriptive run report* (research-visuals, "Every experiment run"): what ran and what was recorded; interpretation is in the study README.

== Sessions

- `A-n19-02`: session `ECBS1he8dffc3b6a4b`, spec `ECS1h7d3b9a04b90d` (pdp\_split\_slots\_20261012 Part A, n=19); status `running`, 40 executions planned, status counts `{}`; commit `26233e4b77f046e4fbd237b93df9117dcd0e6e35` (dirty: False), binary SHA-256 `226a31f06997964c…`; targets: 8 per curve, seed 202610121101, rounds 1, isolation required L0.

== Recorded values (medians over verified runs; per-workload values in ANALYSIS.json)

#table(columns: 8, align: (left, left, right, right, right, right, right, right),
[curve], [arm], [verified], [median S (recorded)], [median F4 XORs], [median systems], [median priced S], [priced ÷ strong rho (min–max)],
[icv1-f2m19-tm797-9c54981b], [clo-chain-g4], [4/4], [1.879], [4.519e+11], [2218.0], [1.393e+06], [5.783e+05 (3.08e+05–9.37e+05)],
[icv1-f2m19-tm797-9c54981b], [clo-unsplit-g4], [5/5], [1.835], [6.945e+08], [2532.0], [2143.444], [889.211 (417–1.27e+03)],
[icv1-f2m19-tm797-9c54981b], [rho-frob], [5/5], [0.504], [—], [—], [0.504], [0.212 (0.172–0.908)],
[icv1-f2m19-tm797-9c54981b], [rho-strong], [5/5], [2.409], [—], [—], [2.409], [1.000 (1–1)],
[icv1-f2m19-tm797-9c54981b], [std-unsplit-g4], [5/5], [19.230], [1.242e+09], [7959.0], [3850.003], [1597.180 (1e+03–2.36e+03)],
)

`S` = charged group-addition equivalents / √r. F4 arms: solver word XORs counted, not charged, so recorded S is a lower bound. Priced S adds XORs × the curve's pinned `ns_per_word_xor` (`docs/ic/calibration.json`, calibrated on another host) / √r: a sensitivity, and one that understates F4's per-call work. Table arms (`mitm-frobenius`) are fully charged. Wall time is descriptive only (L0).

#figure(image("fig-ecbench-s.svg", width: 100%), caption: [Recorded values from ANALYSIS.json; layout `reports/plot_ecbench.py`.])

== Raw records

`research/pdp_split_slots_20261012/sessions/A-n19-02/records.jsonl`; audits under `audits/`.

A session marked dirty ran with untracked or modified files in the checkout (drafts of this study's own reports and protocols); the binary SHA-256 above is the one `BINARIES.sha256` records, and `ecbench verify` replays against it.
