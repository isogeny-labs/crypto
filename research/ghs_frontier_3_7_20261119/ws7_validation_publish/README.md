# Workstream 7: independent validation and publication gate

Status: **implemented, unverified against measured workstreams**. This package
adds a fail-closed native gate and a cross-lane evidence schema. It does not
contain an admitted timing session, does not authorize a frontier change, and
does not edit the scoreboard or leaderboard `[pcc:unverified]`.

## Boundary

The only promotion scope represented here is a cold, one-target comparison on
the restricted GHS-weak `E_0/F_(q^3)` family. The matched reference is the
strongest applicable strong-rho implementation on the same point. A passing
bundle cannot be used for generic prime, random-binary, Koblitz, generic
prime-order extension-field, or standard curves.

The native gate requires all four upstream lanes:

1. `ws3_parallel_rho` — production parallel-rho implementation and admitted
   measurement;
2. `ws4_composition` — source-bound baseline/R17/R36/R64 composition;
3. `ws5_matched_ghs` — the complete frozen matched GHS grid; and
4. `ws6_solver_coverage` — the live, source-bound ECBench solver census.

Each must be `verified-complete` with nonempty evidence before the authorizer
can succeed. Partial work remains reportable but cannot be promoted
`[pcc:unverified]`.

## Native gate

The Rust binary has three commands:

```text
cargo run --manifest-path research/ghs_frontier_3_7_20261119/ws7_validation_publish/Cargo.toml -- check MANIFEST.json
cargo run --manifest-path research/ghs_frontier_3_7_20261119/ws7_validation_publish/Cargo.toml -- authorize MANIFEST.json OUTPUT.json
cargo run --manifest-path research/ghs_frontier_3_7_20261119/ws7_validation_publish/Cargo.toml -- self-test
```

`check` exits 0 only for a complete non-test bundle, 3 for a structurally valid
but blocked bundle, and 2 for malformed input or an I/O error. `authorize`
uses create-new semantics and refuses to overwrite an existing output. Its
output binds the exact manifest bytes and source revision. A `test_only`
bundle is always refused by the CLI even when every synthetic gate is true.

The committed [`fixtures/current-blocked.json`](fixtures/current-blocked.json)
is the current honest state. It is expected to exit 3. It has no placeholder
hashes that could be confused with evidence.

## What the gate checks

- exact source revision, executable, image, spec, and protocol identities;
- four-lane census and state;
- cold one-target scope, matched target/reference, and complete phase census;
- at least one warmup and five measured rounds with zero automatic retries;
- L2/L3, at least five A/A pairs, the frozen `[0.9, 1.1]` A/A interval gate,
  and a claimed wall difference outside the admitted noise floor;
- the `0, 0.1, 1` GAE pricing band, table-work pricing, every unpriced counter,
  byte-counter treatment, 600-triple calibration, and identical calibration
  for both arms;
- exact scalar checks, phase sums, replay-all, and a distinct-host replay of
  both arms;
- sealed raw evidence, producer-distinct recomputation, `VALIDATION.json`, and
  CI-main PCC standing;
- benchmark/export/report/visual artifacts and generated frontier,
  leaderboard, browser, dashboard-coverage, and progress-timeline checks; and
- twelve locally hash-bound evidence roles with safe relative paths and exact
  byte counts.

The gate intentionally does not calculate a winner. It authorizes publication
only after the producer and independent tools have already produced the
complete evidence named above.

## Sources checked

The shared research-KB executable was unavailable. The fallback review used:

- `AGENTS.md` sections 5a and 7–8;
- `.agents/skills/ecbench-measure/SKILL.md` sections 4–7;
- `.agents/skills/ecbench-independent-runner/SKILL.md`;
- `.agents/skills/ecbench-bounds/SKILL.md`;
- `research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`
  sections M4 and P0–P3; and
- `research/ecbench_e2e_frontier_20261031/LITERATURE.md` and the restricted
  GHS section of its report.

The unavailable catalog is an evidence-discovery gap, not evidence that no
other validation method exists.

## Files

- [`manifest.schema.json`](manifest.schema.json): portable schema;
- [`src/main.rs`](src/main.rs): authoritative semantic validator/authorizer;
- [`REQUIREMENTS.md`](REQUIREMENTS.md): current requirement ledger;
- [`INDEPENDENT_REPLAY.md`](INDEPENDENT_REPLAY.md): second-host procedure;
- [`PUBLICATION_DRY_RUN.md`](PUBLICATION_DRY_RUN.md): generated-view sequence;
- `fixtures/current-blocked.json`: explicit unmeasured state; and
- `receipts/`: development-only native test receipts once generated.

