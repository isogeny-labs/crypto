#set page(paper: "a4", margin: (x: 17mm, y: 15mm))
#set text(font: "New Computer Modern", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= Koblitz pair-index component timing
#text(size: 8pt, fill: rgb("555555"))[Run `run-20261116T141419Z` · 2026-11-16 UTC · shared-host L0]

This is a descriptive run report only. It records observations from one local process. It does not review a hypothesis or establish a complete-call ECDLP performance claim. The study has no assigned EXP, HYP, or GOAL identifier.

== Run and identity
One of one focused release tests passed. Seven untimed warmups preceded 101 measured repetitions per arm and curve: 1,515 timed rows across K13, K19, K23, and five arms. The run exited zero without timeout. Public residuals used Q = [641]G. Pair attempts were 272 / 4,830 / 18,906; retained first keys 261 / 4,800 / 18,842; duplicate keys 11 / 30 / 64; fixed precomputed probes 2,112 / 8,896 / 17,600. Exact curve, base, pair and probe digests are in `RESULTS.json`.

The hash reference matched the production u64-key MixHasher and a*a+1 reservation. Hash A/A was an identical control. All duplicate pairs passed full group-equation checks; every arm returned the same first witness and checksum. These are harness checks, without independent replay.

#figure(
  image("chart.svg", width: 100%),
  caption: [Recorded median fresh-index build plus fixed-key lookup, nanoseconds. Each arm has 101 repetitions. Panel scales differ. Run `run-20261116T141419Z`.],
)

== Recorded component medians
#table(
  columns: (1.7fr, 1fr, 1fr, 1fr),
  inset: 4pt,
  stroke: rgb("cccccc"),
  [Arm], [K13 ns], [K19 ns], [K23 ns],
  [Hash], [5,667], [37,708], [112,416],
  [Hash A/A], [5,667], [37,334], [111,541],
  [Direct flat u32], [4,666], [31,500], [226,749],
  [Paged], [8,791], [45,000], [118,083],
  [Inline], [7,208], [47,500], [125,541],
)

The A/A absolute median differences were 0 / 374 / 875 ns. The observed direct-flat medians are lower than hash on K13 and K19, and higher on K23. Paged and inline medians exceed hash in all three cells. The full recorded minimum/maximum ranges and separate build and lookup medians are in `RESULTS.json` and `report.md`.

== Reproduction and limits
Protocol `46faae79f8`; control amendment `71d5af799b`; corrected source `1b91f79a5c`; clean tree at run `92bf1257275f`. Executable SHA-256: `4f8e218be771a8cf58cc1005c9aa49561ae85701d3846f5af0f266ca862ec7ff`. Exact command, stdout/stderr hashes, S3 checkpoint and host are in `RUN_RECORD.json`. Host: Darwin 27.2 arm64 Mac16,7, 14 logical CPUs, Rust 1.99.1. Whole-process observations: 0.38 s wall, 0.12 s user CPU, maximum RSS 50,593,792 bytes; these are not per-arm values.

Timed sections exclude EC pair sums, target and probe-key generation, relation rank and linear algebra, verification, process startup, and per-arm memory. The direct-flat standalone build differs from the hash-then-convert production candidate. The matched F4/F5/F6 complete-call panel did not run here. The first compile-only check failed on a source type mismatch and was retained as `build.*`; corrected `build2.*` passed before timing. Independent replay is absent. No canonical performance graph or leaderboard changed because this run supplies no admitted complete-call evidence.
