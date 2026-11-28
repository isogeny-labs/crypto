# EC graph-cycle V4 rank-input repair

Date: 2026-11-25. Conductor T-133 claimed. V1–V3 source, protocols and raw receipts remain unchanged. V3 passed all 152 source and 136 generic, 136 component, 126 cycle complete-row EC identities, then exited 101 while checking an extracted target scalar. V3 stdout SHA-256 `eeede8cde889c68e85103d6dff40f7fdca71d6dab794930ba5344428aee83391`; stderr SHA-256 `5e8725eb693d348e4ec59122300c41d848703f9a3ecb6c9e2eead3bb2633f589`. The rank adapter had copied small coefficients and target coefficient but omitted source RHS; this made every candidate scalar zero.

V4 source SHA-256 `00fe6655315bd90ecb073b11c6c7272f3d6a4fc989911761a738a2464c18683c` adds `b[RHS]=x[RHS]` before making base/target rank views. All curve, 152 source rows, graph/ordinary/component filters, coefficient operations, witness checks, and caps are identical to V3. Input `WS68_NATURAL_PARTIAL.stdout` SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3`. The local-only run uses the prior user authorization after checkpoint gate exit 2 for missing `.resume/RESUME.md`.

Required checks: all 152 source point identities; all complete-row point identities in each arm; target scalar point check; then report ranks, target, graph closures and all counted arithmetic/scans. Any mismatch retains V4 raw output and requires a separately frozen repair.

`rustc -O research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V4.rs -o /tmp/ec_graph_cycle_v4_20261125.bin`

`timeout 60 /tmp/ec_graph_cycle_v4_20261125.bin > research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V4.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_V4.stderr.txt`

## Post-run receipt, appended after execution

V4 compiled with one Rust edition warning about array `into_iter`; compile exit 0. The single capped run exited 0. Stdout SHA-256 `c2ce74cf71a86cc0da3cb6a3aec6af5f46b3778e1de336e8961264c446f39329`; stderr was empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; exit receipt SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`.

All 152 source identities and all 136 ordinary, 136 component, and 126 cycle complete-row identities passed independent EC point checks. Each arm has base rank 8, augmented rank 9 and extracts 701, checked against Q. There is one graph component. The cycle arm has 16 tree edges, 109 closing cycles, 10 nonclosing cycles, 257 path edges and 2,291 path adjacency scans. Filter operation counts: ordinary 4,862 modular adds, 5,278 multiplies, 16 inversions, 2,312 cell scans; component 4,862 adds, 5,278 multiplies, 16 inversions, 2,040 cell scans plus 270 graph edge visits; cycle 6,682 adds, 13,364 multiplies, 257 inversions and 3,784 cell scans. Rank-stage counts are in the raw stdout. On this one fixed source stream, cycle closure retains target coverage but produces fewer complete rows and uses more counted filter arithmetic than ordinary elimination. Its filter source is bounded and exact; this does not establish performance on another curve or charge the producer's 512 candidate point computations and table construction.

Earlier failures remain distinct: V1 compile exit 1, stderr SHA-256 `770b3ef11abb217e12a30bd1365944329d3265963fa14af004cbcaf2c9282457`; V2 run exit 101, stdout SHA-256 `4eecfe78c1ad4e49c090889d32bb7f0a8213c92b4850dd0e6010620fdafe3930`, stderr SHA-256 `00f88dd519e6c89fef0dd21537ba9f89101e714619b7d7c52328069fae83746e`; V3 run exit 101, stdout SHA-256 `eeede8cde889c68e85103d6dff40f7fdca71d6dab794930ba5344428aee83391`, stderr SHA-256 `5e8725eb693d348e4ec59122300c41d848703f9a3ecb6c9e2eead3bb2633f589`. Their outputs are not used for the V4 comparison.
