# Blocked x-only BSGS versus batched rho

Status: protocol, harness, seed custody, and independent verifier complete;
admitted measurements blocked.

This study will compare the existing paired x-coordinate BSGS kernel with the
existing single-thread batched negation-map Pollard rho implementation on fixed,
public, prime-order toy curves. The primary workload is one cold target with all
solver setup charged. A separate secondary panel may reuse one BSGS table across
64 targets; it is not evidence about the cold one-target problem.

The frozen design is in [`PROTOCOL.md`](PROTOCOL.md), with pre-timing
corrections to the admitted CPU envelope in
[`PROTOCOL-ADDENDUM-1.md`](PROTOCOL-ADDENDUM-1.md) and the lossless seed-custody
encoding in [`PROTOCOL-ADDENDUM-2.md`](PROTOCOL-ADDENDUM-2.md). The randomness
contract is in [`seed-manifest.json`](seed-manifest.json). The complete compact
mapping in [`derived-seeds.json`](derived-seeds.json) was independently
expanded and checked [pcc:unverified]: 16,360 unique derived seeds, 19,680 logical records,
403,302 canonical bytes, and SHA-256
`89a9d8982cf867a9174953b1682006c50cbadd30e4d65316acf258c21ea5f311`.
No timing result has been collected or interpreted.

## Implemented controls

- `native/` includes the exact pinned solver source files and exposes only the
  five public fixtures. Its timing commands require a clean exact commit, the
  frozen protocol and addenda, the canonical seed mapping, one Rayon thread,
  and a release build before they can start.
- `verifier/` uses separate `u128` affine arithmetic. It checks the curves,
  targets, returned scalars, event schedule, phase sums, solver configuration,
  censoring, A/A gates, and the frozen bootstrap analysis without importing
  either solver.
- Root `ecbench` has fixed cold adapters for the paired BSGS and rho arms. They
  reject curves outside the registered panel and retain unpriced native work
  explicitly rather than inventing operation counts.

## Admission status

The current `ecbench.spec/v1` format has one global round and target shape, so
it cannot encode the unequal 512-pair primary panel, two 64-pair A/A panels,
the custom seed derivation, and the 16-by-64 shared-table panel in one exact
study. A fixed study wrapper or planner extension is still required before an
honest `isolab-spec.json` can be filed.

The repository's recent trusted-main checkpoint and experiment workflows also
fail AWS OIDC role assumption after the repository moved organizations. An
authorized operator must repair that trust and record budget headroom before a
run. Local timing is not a substitute for the required native-L2, replay, A/A,
checkpoint, and benchmark-record admission.

## Sources checked

- `src/cryptanalysis/bsgs_fast.rs`, especially `BsgsFastPlan`, the paired
  kernel, candidate verification, and table accounting.
- `src/cryptanalysis/prime_fast.rs`, especially `RhoConfig::tuned`,
  `rho_parallel`, collision verification, and `RhoOpCounts`.
- `research/lit2026_w11_generic_engineering_20261011/README.md`, which measured
  the paired BSGS kernel against its chain predecessor but did not time it
  against rho.
- `research/ic_e222_m221_20261106/interleaved_rho_bsgs/{PROTOCOL,README}.md`,
  which supplies the balanced scheduling and A/A pattern for different
  order-6 multi-target algorithms; its timing rows are not reused here.
- `docs/experiment-seeds.md` for domain-separated public seed custody.
- The repository knowledge-base wrapper was invoked, but no `research-kb`
  executable or MCP endpoint is available in this runtime. No novelty or
  absence claim is made from that failed search.

A Daybreak-model audit checked the repository history and found no existing
same-host benchmark of these exact two implementations. The connected Daybreak
Defense Factory currently reports exhausted scan allowance, so no new Factory
scan was launched. Neither fact is signed validation or an execution result.
