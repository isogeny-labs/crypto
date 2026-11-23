# Endpoint record join

The actual ecbench CLI gains `endpoint-pairs --dir D --source-curve SLUG_A
--destination-curve SLUG_B --arm ARM [--plan-only]`. This is a structural join,
with original workload IDs, execution positions and raw sealed record bytes.
It does not call a solver or admit a comparison. Cost ratio, wall ratio and
conductor gap are explicitly null. [pcc:unverified]

## Requested scope and component boundary

| Requirement | Evidence obligation |
|---|---|
| At least 80-bit prime conductor gap | Explicit quotient/map and endpoint conductor certificates; remains open |
| Several fields and dozens of targets | Join the frozen two-field plan without manufacturing observations |
| Both endpoint ECDLP and scalar measurements | Actual isolated sessions, correct transport and replay; remains open |
| IC comparison | Compatible priced implementation and matched reference; remains open |
| Missing, failed and warmup runs | Preserve all planned pairs and each selected original outcome |
| Statistical and wall results | Separate censoring, pricing, target-cluster, A/A and isolation admission; remains open |

## Contract

The command rebuilds the spec and compares its full expanded plan to the input.
Only `paired_curves_v1` is accepted. It pairs one selected arm by exact subgroup
order, public target seed/index, round and warmup flag. It also checks shared
algorithm seeds and planted scalars. Selected endpoint sets must match exactly;
unknown or identical selectors and ambiguous executions reject.
[pcc:unverified]

In observation mode, the input must have a finished or interrupted session.
The plan, host and records file digests, session/spec identities, executable
hash, record count/status counters, last sequence and each raw record seal
are checked. Every input row, including other arms/curves, must match its
planned execution/workload/method/run ID, seed, environment and binary.
Unexpected, duplicate, changed or malformed observations reject. Invalid
units and nonfinite or negative reported costs reject. Missing, timeout,
wrong-answer, exhausted, error, crashed and warmup rows remain explicit.
Lower-bound and unpriced flags are retained in the original JSON, never
converted to a ranking. Hash consistency is not authentication or an audit.
[pcc:unverified]

The original record is embedded as `RawValue`: decimal tokens and record
identity survive serialization. A typed parse is used only for validation;
parsed floats are never used to reseal or rewrite evidence. `plan-only`
accepts no session and no nonempty records, so it cannot hide observations.
It reports missing observations rather than invented timings.
[pcc:unverified]

No complete-case estimator, statistical confidence interval, speed ratio or
wall admission is implemented here. Future statistics must use the established
ratio-of-totals and target-cluster protocol while retaining censoring and
pricing bounds; the new join supplies the two different workload identities.
Matching order/scalar/seed does not certify an isogeny or an identical random
walk. Arbitrary coordinate models and A/A controls keep their own IDs.
[pcc:unverified]

## Sources checked

KB unavailable in this runtime, as recorded in the lane log. Bounded primary
repository sources: `docs/ecbench/README.md`, measurement and extension skills,
`spec.rs` paired scheduling and target law, `runner.rs` file/hash/run conventions,
`record.rs` raw-line seal and pricing flags, `report.rs` loader, `compare.rs`
admission/within-workload matching, `stats.rs` ratio-of-totals estimators, and
the frozen `paired-schedule-20261118` protocol. The design reuses those contracts
and makes no new mathematical or solver claim. [pcc:unverified]

## Reproduction

The focused integration target includes the actual CLI module and links the
actual crypto library. It uses synthetic codec fixtures labelled as such,
without solver calls or timing APIs. The full frozen plan uses original
native ecbench workload generation. Build and qualification receipts identify
source/input/executable bytes and distinguish failed attempts. No EKS or local
ECDLP benchmark session is part of this component. [pcc:unverified]

See [RESULTS.md](RESULTS.md), [summary.json](summary.json) and the inspected
[report PDF](reports/report.pdf). Producer evidence PCC-20261118-c121f7 is
unsigned/not covered by a signed independent verdict. [pcc:unverified]

Build the focused target from the repository root with Rust 1.100 and the
pinned CLI lock, then pass its executable and the resulting ecbench executable
to qualify.sh in a fresh output directory:

```sh
cargo test --manifest-path research/prime_conductor_gap80_20261031/wide-harness-20261118/cli/Cargo.toml --locked --offline --test endpoint_pairs --no-run
bash research/prime_conductor_gap80_20261031/endpoint-join-20261118/qualify.sh CLI TEST_EXECUTABLE FRESH_DIRECTORY
```

Development builds used CARGO_PROFILE_DEV_DEBUG=0 and
CARGO_PROFILE_DEV_INCREMENTAL=false, not a measured worker image. Frozen
pre-call manifests identify the exact executables and qualifier. [pcc:unverified]

Static checks: focused integration/module clippy and rustfmt pass. Whole CLI
clippy exits 101 on three unchanged warnings (two pdp3 chunk-loop warnings
and an existing CLI type-complexity warning), confirmed against tracking
commit b367b5f373. The root-library test-suite duplicate-definition blocker
remains the earlier unchanged failure. No global passing suite is asserted.
See integration.json and retained stderr. [pcc:unverified]

The deterministic runs.tar.xz archive retains the raw qualifier directories;
extraction hashes and byte-identical repacking pass. Exact named PCC subjects
remain loose; no prior archive, proof subject or ledger entry is rewritten.
[pcc:unverified]

Current qualified development executables remain in target-v5 outside Git.
Earlier failed-call executable identities are retained in SHA manifests, with
source snapshots and commands; those overwritten development executables were
not separately archived. Rebuilding them is distinct from replaying saved
binary bytes. No signed or archived-binary independent replay is asserted.
[pcc:unverified]
