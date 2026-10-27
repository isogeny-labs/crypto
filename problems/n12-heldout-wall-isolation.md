# N12 held-out paired wall comparison lacks an isolated host window

- **Status:** `open`
- **Kind:** resource limit / measurement-environment obstruction
- **Affected work:** [frozen N12 paired wall protocol](../research/factor_base_heldout_wall_20261025/PROTOCOL.md) and [null-result packet](../research/factor_base_heldout_wall_20261025/REPORT.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-25

## Problem

The 420-target N12 candidate/rho wall comparison has no valid measured ratio because the shared benchmark lock did not become available by the authorized 09:03 UTC gate on 2026-10-25. The protocol required one isolated 12-process schedule with two rho A/A passes, five alternating candidate/rho pairs, source verification for 5,040 target-arm observations, and I/O pressure at most 5% at its boundaries. None of the timed target processes ran.

## Evidence and uncertainty

- **Observed:** The initial `tools/isolated_bench.py run --cpus 0,16 -- /bin/true` preflight exited 1 because `/tmp/crypto-bench.lock` was held. At 09:03:06 UTC, `flock -n` again exited 1 while unrelated twist benchmark PID 3986803 owned the lock; I/O `some avg10` was 4.70% at that instant. Earlier samples repeatedly exceeded 5%, including 66.75% at 08:44:46 UTC and 59.36% at 09:00:05 UTC. The lock owner’s workers were observed in uninterruptible filesystem waits (`folio_wait_bit_common`). See [preflight receipt](../research/factor_base_heldout_wall_20261025/results/preflight.txt) and [obstruction note](../research/factor_base_heldout_wall_20261025/OBSTRUCTION.md). The native release binaries compiled successfully, but no isolated target solve was launched.
- **Derived or proved:** No mathematical obstruction is established. The wall ratio is null under the frozen protocol.
- **Suspected explanation:** Shared-filesystem stalls contributed to I/O pressure and lock occupancy; the cause of those stalls was not established in this study.
- **Missing capability:** A quiet, exclusively reserved host window for the complete schedule under the existing 2-logical-CPU, 4-GiB, 3,600-second active-runtime boundary.
- **What this evidence does not establish:** It does not compare candidate and rho wall costs or change the separately verified operation-count result.

## What would unblock it

- [ ] A new authorized execution window with `/tmp/crypto-bench.lock` available and `tools/isolated_bench.py` quiet-host preflight passing on the same host.
- [ ] I/O `some avg10` at most 5% before and after the schedule and every arm, an uncontended isolation receipt, and a complete 12-process run whose 5,040 target-arm rows pass the frozen native validator.

Those conditions would establish a host-specific paired wall ratio with an A/A noise floor. They would not transfer that ratio to another hardware class or replace operation-count pricing.

## Next discriminating step

- **Action or proof obligation:** Execute the frozen [run script](../research/factor_base_heldout_wall_20261025/run_wall.sh) in a separately authorized quiet window, then run the native validator and report all five round ratios and A/A spread.
- **Inputs and controls:** Exact 420 IDs, `fb1-any-j32` selector, source-negation rho, and source SHA-256 hashes fixed in the protocol.
- **Execution limits and stop condition:** Keep the protocol’s 2-logical-CPU, 4-GiB, 3,600-second active-runtime boundary, one schedule, no target retuning or retry selection. Stop on the first failed arm or pressure gate.
- **Outcomes and next actions:** A valid schedule yields a paired wall report; a failed gate leaves wall ratios null with recorded rows and receipts.
- **Recheck trigger:** A user-authorized quiet host window with the lock and I/O gates satisfied.

## Resolution and history

- 2026-10-25: The authorized 09:03 UTC start gate closed with the lock occupied. Unresolved. The standardized packet records zero timed target rows and null wall ratios; the independent-host operation-count result remains separate.
