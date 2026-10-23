#set document(title: "Run report: accounting v2 study sessions")
#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1 / 1")
#set text(size: 9.5pt)
#set par(justify: true)
#set heading(numbering: "1.")
#show table: set text(size: 7.3pt)
#show table: set par(justify: false)
#show table: it => block(breakable: false, it)
#set table(stroke: 0.4pt + luma(150), inset: 3.5pt)
#show figure.caption: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Run report: accounting v2 study sessions] \
  Study `research/ecbench_accounting_v2_20261023` \ Runs started 2026-10-23 08:15 UTC (batch 1: 08:15 to 08:22; amendment A1 batch after the audits)
]

Descriptive only: what ran, on what, with what result recorded. Interpretation is in the study's `REPORT.pdf`. Observations only: no speedup or significance statement is made here.

= What ran

- Command for each session: `ecbench run --wait --allow-busy --cpus auto --quiet --spec SPEC --out sessions/NAME` (scripts `analysis/run.sh` and `analysis/run_a1.sh`). Audit: `ecbench verify --dir D --replay-all --exit-code` (`analysis/audit.sh`), receipts in `audits/`.
- Binary: branch `./target/release/ecbench`, built at head `7ed39a752`, sha256 `e265bbc00a91...`; the main binary `ecbench-base`, sha256 `a59c4014ff52...`, only for the `*-main` sessions. Host capsule: `host.txt` (env class ECBENV3had995aa2df0f, Intel Xeon 2.10 GHz, 4 logical CPUs, 1 NUMA node, virtual machine, Linux 6.18.44).
- Specs and sha256: `SPEC_SHA256.txt`. Committed specs (unchanged): `check-prime-j0-floor-v2`, `smoke-prime-v2`, `smoke-koblitz-v2`, `claim-koblitz-strong-v2-n13`, `-n19`. New specs: `specs/*.json`.
- Workloads and seeds: one algorithm seed per (workload, round) shared by every arm (spec.rs); target seeds are in each spec (`20261023`, `7`, `202610121301`, `202610121901`, `202610123701`).
- Isolation: the specs ask for L2 (counts need none). The host was a shared 4-vCPU VM with other agents building and testing (load average 2 to 4), so most records earned L1; the level counts are in the table below.

= Runs requested and completed

Every session completed; every record verified; no failure, timeout or interruption. Audit column: SHA-256 of the receipt of the `--replay-all` audit (`audits/NAME.verify.txt`). `j0-frozen-main` is archived (`archive/`) because the branch binary's audit rejects it by design (25 problems: "used 6 automorphisms, the curve has 2"); the main binary's audit of it passes. Every other session: audit OK, 0 problems.

#table(columns: (2.2fr, 1.7fr, 0.9fr, 1fr, 1.2fr, 1.7fr, 1.5fr),
  [*session*], [*session id*], [*status*], [*verified of records*], [*binary sha256*], [*isolation level counts*], [*audit receipt*],
  [`claim-n13`], [ECBS1h0417f722e88], [complete], [256 of 256], [e265bbc00a91], [L0x64 L1x130 L2x62], [b11339d68b29],
  [`claim-n19`], [ECBS1h7f68751e557], [complete], [256 of 256], [e265bbc00a91], [L0x64 L1x150 L2x42], [82049f3d016c],
  [`frozen-koblitz-branch`], [ECBS1h5466ade4b06], [complete], [160 of 160], [e265bbc00a91], [L1x160], [ff75f66a1635],
  [`frozen-koblitz-main`], [ECBS1hb8266d1bd07], [complete], [160 of 160], [a59c4014ff52], [L1x24 L2x136], [2a85c3324dfb],
  [`frozen-large-prime-branch`], [ECBS1hf87ec503d14], [complete], [32 of 32], [e265bbc00a91], [L1x16 L2x16], [772fd2b210ba],
  [`frozen-large-prime-main`], [ECBS1he690ca5be93], [complete], [32 of 32], [a59c4014ff52], [L1x4 L2x28], [da9bc0fdce2d],
  [`frozen-prime-branch`], [ECBS1h349b75d5e73], [complete], [240 of 240], [e265bbc00a91], [L1x240], [64609a593829],
  [`frozen-prime-main`], [ECBS1hc0d0d5b26f8], [complete], [240 of 240], [a59c4014ff52], [L1x240], [7d4f536c6773],
  [`j0-frozen-branch`], [ECBS1h26edfacea04], [complete], [72 of 72], [e265bbc00a91], [L1x72], [34b1a7f77011],
  [`j0-frozen-main`], [ECBS1h1c109f746e6], [complete], [72 of 72], [a59c4014ff52], [L1x72], [8f7c547b28e6],
  [`j0-v2`], [ECBS1h70fa091addf], [complete], [144 of 144], [e265bbc00a91], [L1x144], [9d56b9ffa662],
  [`pairs-koblitz`], [ECBS1h001cebd5ff1], [complete], [360 of 360], [e265bbc00a91], [L1x360], [7bd0076d541b],
  [`pairs-large-prime`], [ECBS1h8ce582cbde1], [complete], [48 of 48], [e265bbc00a91], [L1x16 L2x32], [85ad837c7ac5],
  [`pairs-prime`], [ECBS1h5c19d0d7e48], [complete], [480 of 480], [e265bbc00a91], [L1x480], [e97bb70352ca],
  [`smoke-koblitz-v2`], [ECBS1h00aa071a40d], [complete], [48 of 48], [e265bbc00a91], [L1x40 L2x8], [c388521dc72b],
  [`smoke-prime-v2`], [ECBS1hdbf5bc198da], [complete], [72 of 72], [e265bbc00a91], [L1x64 L2x8], [e1610031ce56],
  [`strong-n37`], [ECBS1h61cd8e1c170], [complete], [200 of 200], [e265bbc00a91], [L1x122 L2x78], [6a8bb3a3da97],
)

= Measured values

#figure(image("run_mean_s_per_arm.svg", width: 100%), caption: [Mean S per arm, `pairs-prime` ECBS1h5c19d0d7e485 and `pairs-koblitz` ECBS1h001cebd5ff12, with the `ecbench table` 95% interval (32 verified runs per arm: 8 targets, 4 rounds). Blue: v2 ids; grey: frozen ids; orange: A/A control. Data: `run_mean_s_per_arm.data.json`.])

#table(columns: (1.2fr, 1.5fr, 2.2fr, 0.8fr, 0.9fr, 1.5fr),
  [*session*], [*arm*], [*method*], [*verified*], [*mean S*], [*95% interval*],
  [`pairs-prime`], [`rho-neg-v2-aa`], [`rho.negation_v2`], [32/32], [1.263], [[1.085, 1.453]],
  [`pairs-prime`], [`kangaroo-v2`], [`kangaroo.vow_v2`], [32/32], [2.004], [[1.518, 2.492]],
  [`pairs-prime`], [`kangaroo`], [`kangaroo.vow`], [32/32], [1.970], [[1.485, 2.458]],
  [`pairs-prime`], [`bsgs-neg-v2`], [`bsgs.negation_v2`], [32/32], [1.080], [[0.885, 1.263]],
  [`pairs-prime`], [`bsgs-neg`], [`bsgs.negation`], [32/32], [1.046], [[0.853, 1.227]],
  [`pairs-prime`], [`bsgs-il-v2`], [`bsgs.interleaved_v2`], [32/32], [1.244], [[0.930, 1.554]],
  [`pairs-prime`], [`bsgs-il`], [`bsgs.interleaved`], [32/32], [1.209], [[0.897, 1.518]],
  [`pairs-prime`], [`bsgs-v2`], [`bsgs.textbook_v2`], [32/32], [1.578], [[1.382, 1.761]],
  [`pairs-prime`], [`bsgs`], [`bsgs.textbook`], [32/32], [1.543], [[1.351, 1.724]],
  [`pairs-prime`], [`rho-plain`], [`rho.plain`], [32/32], [1.506], [[1.196, 1.878]],
  [`pairs-prime`], [`rho-neg`], [`rho.negation`], [32/32], [1.382], [[1.192, 1.609]],
  [`pairs-prime`], [`rho-neg-v2`], [`rho.negation_v2`], [32/32], [1.263], [[1.085, 1.453]],
  [`pairs-koblitz`], [`rho-frob-v2-aa`], [`rho.signed_frobenius_v2`], [32/32], [1.122], [[0.903, 1.608]],
  [`pairs-koblitz`], [`bsgs-neg-v2`], [`bsgs.negation_v2`], [32/32], [0.998], [[0.820, 1.199]],
  [`pairs-koblitz`], [`bsgs-neg`], [`bsgs.negation`], [32/32], [0.917], [[0.740, 1.118]],
  [`pairs-koblitz`], [`claw-v2`], [`claw.pair_table_v2`], [32/32], [1.243], [[1.154, 1.340]],
  [`pairs-koblitz`], [`claw`], [`claw.pair_table`], [32/32], [1.163], [[1.073, 1.260]],
  [`pairs-koblitz`], [`rho-strong-v2`], [`rho.signed_frobenius_strong_v2`], [32/32], [0.901], [[0.815, 1.007]],
  [`pairs-koblitz`], [`rho-strong`], [`rho.signed_frobenius_strong`], [32/32], [3.941], [[3.809, 4.082]],
  [`pairs-koblitz`], [`rho-frob`], [`rho.signed_frobenius`], [32/32], [1.141], [[0.916, 1.634]],
  [`pairs-koblitz`], [`rho-frob-v2`], [`rho.signed_frobenius_v2`], [32/32], [1.122], [[0.903, 1.608]],
)

Mean S is in GAE per $sqrt(r)$ (`ecbench table`). The interval is the harness's recorded one; where two arms' intervals overlap it says nothing about their difference, which only the paired `ecbench compare` outputs (`analysis/compare/`) address. All S values here carry the harness's `lower_bound` flag where an arm leaves counted work uncharged.

= References and controls

The reference arms are `rho.negation_v2` (prime) and `rho.signed_frobenius_v2` (Koblitz); every `pairs-*` and `strong-n37` session has an A/A repeat of its reference, which equals the reference on every count (paired ratio 1.000 [1.000, 1.000]). Wall time was recorded but not used: solves are 0.1 to 1.5 ms, below the harness's 1 ms admission minimum, at L1.

= Raw records

`sessions/NAME/records.jsonl`, `session.json`, `plan.json`, `host.json`, `spec.json` for each session above; `analysis/summary/NAME.json` (jq over the records), `analysis/table/NAME.md`, `analysis/band/NAME.txt`, `logs/run.log`, `logs/run_a1.log`.
