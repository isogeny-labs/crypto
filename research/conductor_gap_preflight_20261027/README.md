# Prospective conductor-gap pilot allocation

**Software preparation; execution blocked.** The native planner and auditor
check the complete proposed pilot allocation. They do not construct curves,
evaluate maps, run rho, calibrate power or establish crater equivalence.
The blocked [confirmation card](../../backlog/XP-20261026-conductor-gap-size-confirmation/README.md)
retains those requirements.

| Exact allocation check | Value |
|---|---:|
| Native parameter classes | 128 |
| Rung/discriminant cells, eight classes each | 16 |
| Distinct public class/logarithm pairs | 4,096 |
| Logarithms per class, shared across four crater slots | 32 |
| Separate walk seeds per target/slot | 2 |
| Prospective primary invocations / globally distinct nonzero seeds | 32,768 |
| Complete class/crater-slot blocks | 512 |
| Solver invocations attempted by this tool | **0** |

These are exact prospective allocation counts, not statistical observations.
All 12 native example tests pass, including deliberate target/seed reuse,
incomplete cohorts, modified order, invalid readiness claims and overwrite
attempts. A separate `jq` pass checks the allocation's counts, per-class
target uniqueness/range and global seed/block uniqueness. The retained
134 parameter certificates also reverify to the original receipt hash.
Repository-wide validation has separate limits recorded in
[VALIDATION_CONTEXT.md](VALIDATION_CONTEXT.md); it is not claimed green.

## Files and reproduction

- [PROTOCOL.md](PROTOCOL.md): scope, source review, variable ledger and
  exact allocation contract.
- [Native implementation](../../examples/conductor_gap_confirmation.rs):
  `plan`, `audit`, `preflight`; no execution mode.
- [pilot-allocation.json](pilot-allocation.json): the public proposed pilot
  input, **1,011,591 bytes**, SHA-256
  `bf8ebc6255d660c4b9d34dc9843958eacc4182ee593f85f50e73284c9abe7ea5`.
  Its crater slots are placeholders, not constructed curve identities.
- [Allocation audit](checks/allocation-audit.json),
  [separate count check](checks/jq-allocation-counts.json),
  [blocked readiness receipt](checks/readiness.json),
  [example tests](checks/example-tests.md), and
  [parameter re-verification](checks/parameter-verification.json).

From the repository root:

```sh
cargo test --release --example conductor_gap_confirmation --offline
cargo build --release --example conductor_gap_confirmation --offline
target/release/examples/conductor_gap_confirmation audit --plan research/conductor_gap_preflight_20261027/pilot-allocation.json
target/release/examples/conductor_gap_confirmation preflight --plan research/conductor_gap_preflight_20261027/pilot-allocation.json
# preflight must exit 3 and report blocked, even when allocation_valid is true.
sha256sum -c research/conductor_gap_preflight_20261027/INPUTS.sha256
```

`plan --out NEW_FILE` writes a byte-reproducible allocation to a previously
absent path. It refuses overwrites. Changing targets, seeds or block order
after seeing outcomes is not accepted as the same frozen allocation.
The source CSV's hash is checked before parsing. The six upper parameter
classes stay outside the native pilot, consistent with the original design.

## What this resolves and what remains

The preparation now checks that repeated walk seeds or logarithms cannot
silently inflate the proposed sample count, that every fixed parameter
class is included, and that block order is fixed before outcomes. One
scalar is stored per target for all four slots. Actual G/Q transport must
still be proved on constructed endpoints; distinct numeric seeds do not
prove statistical independence or a transported identical walk.

Exact models/orders/strata and three distinct S2 endpoints, map/dual
certificates, a resumable native measurement executor, independent replay,
same-instrument controls, masking, full accounting and resource admission
remain unimplemented or unverified. The disjoint 32,768-solve pilot and
provisional 1,024,000-solve confirmation remain unlaunched. Confirmation's
final count requires the separate pilot's coverage, variance and joint-power
calibration; 1,000 repetitions is not a guaranteed sufficient sample size.

The measured crater comparison remains inconclusive. The upper 254-bit
subgroup-order certificates are parameters, not completed ECDLP comparisons.
Complete CM setup costs and the prime-field algebraic oracle remain open.
No hardness, production-security or full-cost speed result is added.
