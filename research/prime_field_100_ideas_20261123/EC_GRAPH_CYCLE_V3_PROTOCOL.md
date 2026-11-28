# EC graph-cycle V3 exact-row repair

Date: 2026-11-25. Conductor T-130 claimed. V1 and V2 sources, protocols, raw stdout/stderr/exit remain unchanged. V2 source `b71a8547...` compiled but the run exited 101 on `generic row 1` point replay: left A(546,1005), right A(590,88). V2 stdout SHA-256 `4eecfe78c1ad4e49c090889d32bb7f0a8213c92b4850dd0e6010620fdafe3930`, stderr SHA-256 `00f88dd519e6c89fef0dd21537ba9f89101e714619b7d7c52328069fae83746e`. The 152 source point witnesses passed before that failure.

V3 source SHA-256 `60b2f217fb4fa8821c870e6b867e04d45598e2b86ab60a8c70e4e88b696151db`. It changes both dense row-operation loops from `col..C` to `0..C`; pivot selection remains only among the sixteen large columns for filtering. Every row coefficient and RHS now participates in normalization and elimination. The input, graph construction, controls, caps, curve, and candidate stream are unchanged from V1. Input stdout SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3`.

Run once under the user's authorized local checkpoint bypass; `scripts/checkpoint.sh gate` exited 2 because `.resume/RESUME.md` is missing. First require all 152 source identities and all generic, component, and cycle complete-row EC point identities. Only after these pass, compare rank, target and costs. On any mismatch, stop and preserve V3 raw receipts. Commands:

`rustc -O research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V3.rs -o /tmp/ec_graph_cycle_v3_20261125.bin`

`timeout 60 /tmp/ec_graph_cycle_v3_20261125.bin > research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V3.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V3.stderr.txt`

Rust binary remains local only. Full producer setup and candidate costs in `WS68_NATURAL_PARTIAL.stdout` must be charged to any end-to-end interpretation; this run measures the filter on fixed rows only.
