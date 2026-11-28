#set page(paper: "us-letter", margin: 0.65in)
#set text(font: "Helvetica", size: 9pt)
#show heading: set block(above: 9pt, below: 4pt)

= Run Report: Generic Prime-Order E(F_p) Research Harness (`run-20261119T0718Z`)

*Claim ID:* `PCC-20261119-85e006` (`[pcc:unverified]`) · *Branch:* `cursor/prime-generic-autolab-20261118` (Round-0 git `94bedd024d60216a091669f1ed83a8561637d607`, base `ad2a014c0022`). \
*Host:* macOS arm64 (`ECBHW1h7d1934655eee`, `ECBENV3h32d7ea52a547`), `rustc 1.99.1`, isolation `L0` (`--cpus none --allow-busy`). Deterministic counted GAE across `w in {0, 0.1, 1}` only; no wall-time claim. \
*Binaries (SHA-256):* `ecbench` `5f53faa52d15...23e4`; `ic` `52d5543d36d7...7bf6`; `ic_autolab` `02df68156ac4...e148`.

== Execution Flow

- *ecbench + ic_autolab:* 42 sealed sessions (5 Round-0 + 22 train + 15 holdout), *9,624 planned, 9,624 completed, 9,624 `[d]G = Q` verified, 0 failed*. All 42 `ecbench verify` replay audits passed (`ok: true`, `0` problems).
- *ic sweep TPE studies (`seed = 20261118`, `--family generic`):* `prime-table` (rungs 16..22: 24 trials, 14 complete, 10 pruned, 0 failed); `prime-rootfind` (rungs 16..22: 24 trials, 13 complete, 10 pruned, 1 exhausted); `prime-algebraic` (rungs 10..16, usable `[10, 12]`: 18 trials, 10 complete, 1 pruned, 7 exhausted).

== Recorded Mean S Across the §5a Pricing Band

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  inset: 3.5pt,
  align: (left, right, left, left, right, right, right, right),
  [*Stage / Track*], [*k*], [*Curve (`ICV1`)*], [*Session ID*], [*Ver*], [*S(w=0)*], [*S(w=0.1)*], [*S(w=1)*],
  [Round-0 `mitm2-n45`], [20], [`icv1-fp20-t727-cd198a38`], [`ECBS1h6a7cf080ed52`], [32/32], [10.345], [11.768], [24.575],
  [Round-0 `mitm2-n160`], [24], [`icv1-fp24-t1059-6df599df`], [`ECBS1h31fedbf2ea3e`], [32/32], [13.300], [15.250], [32.800],
  [Round-0 `mitm2-n402`], [28], [`icv1-fp28-tm3161-380f655a`], [`ECBS1hdd960964ad94`], [32/32], [20.640], [23.749], [51.724],
  [Round-0 `mitm2-n953`], [32], [`icv1-fp32-t74357-b7396dac`], [`ECBS1hcb28a02cb5f9`], [32/32], [31.270], [35.981], [78.379],
  [Round-0 `pkm-iso-t6-g4`], [20], [`icv1-fp20-t727-cd198a38`], [`ECBS1h7ed674fd42dd`], [8/8], [21.681], [669714.8], [6696952.5],
  [R1 `multibase` `wsum-b5-n3`], [20], [`icv1-fp20-t727-cd198a38`], [`ECBS1h5019e97ecdd4`], [48/48], [4.479], [5.010], [9.787],
  [R2 `multibase` `wsum-b12-n3`], [24], [`icv1-fp24-t1059-6df599df`], [`ECBS1h88d91e04d47f`], [48/48], [3.060], [3.422], [6.678],
  [R2 `multibase` `wsum-b20-n3`], [28], [`icv1-fp28-tm3161-380f655a`], [`ECBS1h6219d7548cb5`], [48/48], [3.081], [3.466], [6.936],
  [R6 `multibase` `wsum-b28-n3`], [32], [`icv1-fp32-t74357-b7396dac`], [`ECBS1h80afb0ec9fc2`], [48/48], [2.966], [3.396], [7.269],
  [R4 `generic` `tw2l-x20-j16-n2`], [20], [`icv1-fp20-t727-cd198a38`], [`ECBS1h7ab83998e0bd`], [48/48], [2.123], [2.305], [3.943],
  [R4 `generic` `wwalkGg-x09-sr-n2`], [24], [`icv1-fp24-t1059-6df599df`], [`ECBS1hbda7b82ae434`], [48/48], [1.946], [2.128], [3.771],
  [R4 `generic` `tw2l-x20-j16-n2`], [28], [`icv1-fp28-tm3161-380f655a`], [`ECBS1ha25666791b28`], [48/48], [1.726], [1.897], [3.432],
  [R4 `generic` `tw2l-x20-j16-n2`], [32], [`icv1-fp32-t74357-b7396dac`], [`ECBS1h4ae73e7b227d`], [48/48], [1.833], [2.015], [3.659],
)

#image("band_comparison.svg", width: 100%)
#image("sweeper_rungs.svg", width: 100%)
