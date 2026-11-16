# Block-tail relation screen worklog

## 2026-11-16T12:47Z — Frozen source and one release run

Protocol commit `fe79c218f3` preceded source commit `d9ba9ca5d6`.
The locked library-only harness was launched once with Rust 1.99.1 and
four Cargo jobs. Exact command: `cargo test --manifest-path
research/rootfind_block_tail_relations_20261116/harness/Cargo.toml
--release --locked --test rootfind_block_tail_relations -- --nocapture`.
Raw `run.stdout`, `run.stderr` and `run.meta` retain UTC 12:47:48–12:47:51Z,
exit zero and `/usr/bin/time -l` counters. One release test passed in
0.11 seconds; this is harness time, not collector time. [pcc:unverified]

The test enumerated K13 fully and sampled K19/K23 under the frozen
nonoverlapping coefficient law. First-hit and block-tail full-rank
target counts were equal: 47/154/659. The tail policy made 67/306/813
more pair lookups and 6/12/10 more pair re-additions through those
targets. Its extra rows did not reduce target queries in any cell.
The test verified scalar 641 for all three arms in all cells. [pcc:unverified]

`derive_results.py` binds the raw output, source commit, test binary,
fixed inputs and operation arithmetic to `RESULTS.json`. The report,
editable diagram, chart and PDF preserve this negative result. The
next action is to avoid integrating the simple block-tail policy and
test only a rank-aware cross-block variant with a frozen probe cap and
complete-cost follow-up. [pcc:unverified]
