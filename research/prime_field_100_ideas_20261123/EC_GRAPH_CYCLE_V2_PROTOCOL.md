# EC graph-cycle V2 compile repair

Date: 2026-11-25. Conductor T-126 claimed. V1 protocol and source remain frozen. V1 `rustc -O` failed, exit 1, because `[Option<i64>;3].into_iter()` yielded `&i64` on this Rust edition; its stderr SHA-256 is `770b3ef11abb217e12a30bd1365944329d3265963fa14af004cbcaf2c9282457`. V1 stdout/stderr/exit remain at `EC_GRAPH_CYCLE.{stdout,stderr,exit}.txt`.

V2 changes exactly `mul(d,g)` to `mul(*d,g)` in final answer verification. Its source SHA-256 is `b71a8547857477c86a8d7e2e65e72beb13e75a56a8bfe27189c7149dc3f45f23`. It embeds the same WS68 partial input SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3` and retains every mathematical input, bound, control, witness check and cost counter from V1. Local-only checkpoint bypass remains under the user's explicit instruction; the latest gate failure is the missing `.resume/RESUME.md`. One compile and one run, with separate V2 raw receipts:

`rustc -O research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V2.rs -o /tmp/ec_graph_cycle_v2_20261125.bin`

`timeout 60 /tmp/ec_graph_cycle_v2_20261125.bin > research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V2.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V2.stderr.txt`

On any V2 failure or disagreement, stop and preserve exact raw artifacts.
