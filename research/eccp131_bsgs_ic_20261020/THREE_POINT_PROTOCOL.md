# ECCp-131 checked three-point relation diagnostic

**Frozen before measurement:** 2026-10-21. **Scope:** exact Certicom ECCp-131 group, generator, and published target from `CurveParams::eccp131()` and `ic::params::named("eccp-131")`. This is a bounded relation and cost diagnostic, not a full-subgroup solve protocol.

## Question

Does subtracting one signed small-x base point and querying the existing complete pair-sum key table produce independently checked natural relations, new modular rank, or a target-descent row under a fixed small workload? What is the measured cold process and probe cost relative to the unchanged two-point path on this host?

## Frozen design

- Factor base: first **128** subgroup point abscissae of the exact curve, both signs admitted. The CLI records the base SHA-256 and pair-key count. Keep this digest equal across all arms.
- Arms: `--relation-arity 2` control and `--relation-arity 3` candidate. Both use `--size 128 --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16` and seeds **131** and **137**. Four fresh processes, one per arm/seed, in this order: `(131,2)`, `(131,3)`, `(137,2)`, `(137,3)`.
- Command template: `ic prime --curve eccp-131 --size 128 --trials 0 --controls 8 --setup-candidates 32 --target-candidates 16 --seed SEED --relation-arity ARITY --json`. Use the release binary built from the recorded commit. Capture stdout JSON, stderr, exit status, cold wall, maximum RSS, UTC timestamps, binary and source hashes, host/compiler details, and raw `/usr/bin/time` or Python `resource` data. Each process is limited to 300 seconds; retain timeouts and failures.
- No adaptive base, seeds, candidate counts, or retries. A failed row is retained with its reason; repair of the implementation requires a new protocol and output directory.
- Positive controls: eight planted pair decompositions per process; a separate release unit test constructs a three-term point on exact ECCp-131 outside the two-point key set and checks the returned signed terms. An independent PARI/GP script checks the same existence property from the exact field and curve parameters. The same two-point arm is the negative/control oracle. The published target is always `Q` at descent candidate zero; it is not planted.
- Validity: relation coordinates are rechecked by EC addition in the implementation; every returned scalar requires a separate `[x]G=Q` check. Compare natural setup count, independent setup rank, target count, combined rank, target determination, and wall/probe time. Duplicate or dependent rows do not count as rank progress.
- Report full setup and misses. The pair table and curve validation are charged to process wall. Summaries state median and per-run values; no speedup or full-cost claim follows from L0 wall under concurrent host load.

## Inference and stop

A natural relation with a verified point identity opens an independent replay and rank audit at the same curve parameters. If no natural rows occur, conclude only that this base and candidate budget produced none. A nonzero target row with rank deficiency is recorded as an undetermined target column. A timeout or invalid witness is an implementation failure. The full published-point scalar and complete cold cost remain the exit criteria in [the problem record](../../problems/eccp131-full-parameter-index-calculus.md).

For a base with `B` abscissae, the signed three-point list has at most `(2B)^3 = 8B³` ordered sums before collisions, so uniform nonzero candidates have coverage at most `8B³/(n−1)`. This is only a method-specific support bound; the actual three-point support is not enumerated by this protocol. At `B=128`, zero natural hits are the overwhelmingly expected outcome. The measurement instead validates the exact relation oracle and its per-candidate cost.
