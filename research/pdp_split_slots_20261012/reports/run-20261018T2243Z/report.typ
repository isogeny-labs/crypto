#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)

= Run report: Part C session C-n17 (m = 2 on the dimension-9 invariant base)

*Descriptive run report* (research-visuals, "Every experiment run"): what ran and what was recorded; interpretation is in the study README.

== Sessions

- `C-n17`: session `ECBS1h87f9b53a44cc`, spec `ECS1h41ea97461839` (pdp\_split\_slots\_20261012 Part C, n=17 invariant base, m=2); status `complete`, 48 executions planned, status counts `{"verified": 48}`; commit `87f3609c950abd758476238c059c1554a3c3689e` (dirty: True), binary SHA-256 `226a31f06997964c…`; targets: 8 per curve, seed 202610121301, rounds 1, isolation required L0.

== Recorded values (medians over verified runs; per-workload values in ANALYSIS.json)

#table(columns: 8, align: (left, left, right, right, right, right, right, right),
[curve], [arm], [verified], [median S (recorded)], [median F4 XORs], [median systems], [median priced S], [priced ÷ strong rho (min–max)],
[icv1-f2m17-tm101-00378d4e], [ic-mitm-frob], [8/8], [17.099], [—], [—], [17.099], [4.179 (3.65–4.53)],
[icv1-f2m17-tm101-00378d4e], [ic-sat-xor], [8/8], [3.430], [—], [—], [3.430], [0.857 (0.711–0.917)],
[icv1-f2m17-tm101-00378d4e], [pdp-m2-g0], [8/8], [3.432], [1.650e+09], [21.5], [12593.320], [2919.302 (1.77e+03–3.45e+03)],
[icv1-f2m17-tm101-00378d4e], [pdp-m2-g4], [8/8], [3.416], [2.426e+07], [19.0], [188.531], [45.930 (32.1–63.4)],
[icv1-f2m17-tm101-00378d4e], [rho-frob], [8/8], [0.937], [—], [—], [0.937], [0.232 (0.155–0.27)],
[icv1-f2m17-tm101-00378d4e], [rho-strong], [8/8], [4.098], [—], [—], [4.098], [1.000 (1–1)],
)

`S` = charged group-addition equivalents / √r. F4 arms: solver word XORs counted, not charged, so recorded S is a lower bound. Priced S adds XORs × the curve's pinned `ns_per_word_xor` (`docs/ic/calibration.json`, calibrated on another host) / √r: a sensitivity, and one that understates F4's per-call work. Table arms (`mitm-frobenius`) are fully charged. Wall time is descriptive only (L0).

#figure(image("fig-ecbench-s.svg", width: 100%), caption: [Recorded values from ANALYSIS.json; layout `reports/plot_ecbench.py`.])

== Raw records

`research/pdp_split_slots_20261012/sessions/C-n17/records.jsonl`; audits under `audits/`.

A session marked dirty ran with untracked or modified files in the checkout (drafts of this study's own reports and protocols); the binary SHA-256 above is the one `BINARIES.sha256` records, and `ecbench verify` replays against it.
