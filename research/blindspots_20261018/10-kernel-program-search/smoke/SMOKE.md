# Item 10 smoke test (not a result)

Run before any measured card, with the committed `examples/blindspots_10_kernel_search.rs`
(built with `rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native`), at the smallest
cell `n = 31, l = 9`, 12 evaluations per search arm, 8 + 8 held-out targets:

* `selftest.txt`: the selftest passes (field, curve order, quartic forms against the resultant S_4,
  S_4 on true sums, family construction at n = 31, 37, 41, trace classes and T_eff, kernel against brute force
  on 400 random programs, handicap positive controls).
* `state-a/` (12 evaluations in one command) and `state-b/` (6 + 6 in two commands) are identical apart from
  `wall_ms`: a card boundary changes nothing.
* `search-*-a.log`, `finalists-n31.txt`, `heldout-n31.jsonl`, `ablate-n31.jsonl`, `conserve-17-6.jsonl`,
  `summary.json`, `summarize.txt`: every subcommand runs end to end. The verdict printed there is of 12
  evaluations and 8 targets and means nothing.
