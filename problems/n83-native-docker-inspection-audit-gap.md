# Native N83 Docker inspection and terminal-audit gap

- **Status:** resolved (bounded reacquisition only; root cause unknown)
- **Kind:** operational/tool failure with unknown cause; missing terminal-state measurement
- **Affected work:** [400K continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/PROTOCOL.md), [corrective preregistration](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/INSPECTION-V11.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-19

## Problem

The initial400K K4096 worker reported a terminal bounded result, but a failed
Docker inspection left its final exit/OOM state and complete outer audit
unavailable, so that attempt cannot be admitted as a complete verified run.

This concerns the retained public synthetic primary a=0
icv1-f2m83-tm6151469093347-debefd74 over x^83+x^45+x²+x+1, subgroup
2417851639230796216685689; single public_x_hash query seed2026100801,
m5/identitymask0, K4096,400000conflicts/2400seconds, guarded1CPU/4GiB,
full core14/30. It does not reopen the historically resolved
[1000-conflict internal-phase observability](n83-native-k4096-terminal-observability.md)
or establish a mathematical obstruction.

## Evidence and uncertainty

- **Observed:** K4096-v10 source d216680177, guard76155c48… reported
  UNKNOWN_solver_cap,400000conflicts/zero models/relations/rank. Its card
  stopped at command1, exit1,1598.372932784seconds, with empty-stderr Docker
  inspect failure. No audit.json/outer.json/final state exists. Original
  failure output, worker summary/events and cleanup snapshot remain raw.
- **Observed:** a later exact-name Docker ps query exited0 with empty stdout;
  task-owned container absence was confirmed without stopping peer jobs.
- **Observed:** after director assessment, [v11](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/INSPECTION-V11.md)
  raises only read-only inspect wait15→60seconds, adds exit-status diagnostics,
  never retries RPCs, and passes19unit+7card tests. Other RPC waits remain15s.
  The separately frozen K4096-v11 repeat passes native query/custody audit
  and exact director checks, exit0/no OOM/no wall censor/cleanup verified.
- **Derived or proved:** no universal daemon-reliability or root-cause result.
- **Suspected explanation:** RPC stall/15-second timeout. The v10 Docker exit
  status was not retained, so timeout is an inference, not a preserved exit124.
  The earlier stronger timeout wording is corrected in the new report;
  frozen preregistration and raw failure are preserved unchanged.
- **Missing capability:** the original missing final state cannot be recovered
  from a removed container. Its worker-reported clock is not promoted to a
  complete-run clock. New errors retain process status for future diagnosis.
- **Not established:** a complete original-v10 run, absence of future Docker
  failures, UNSAT/absent decompositions, natural yield, best base, scalar
  recovery or a full cold IC/rho winner.

## What would unblock it

- [x] Preserve failed v10 records, exact custody and cost; separately assess
  before another plan, without fabricating terminal state or silently retrying.
- [x] Freeze a bounded corrective attempt with same worker/query/inputs/CPU/
  resource envelope and400K/2400s, explicitly accounting for all3attempts
  under maximum cumulative worker allocation7200s and3000s corrective card.
- [x] Obtain complete original-group/query/source/ELF/stream native audit and
  director receipt/digest verification, exit/OOM/censor and cleanup evidence.
- [x] Publish all3raw attempts and verify fresh GET, every payload hash,
  corresponding exact-driver native replays and deterministic repack.

These criteria reacquire one completely audited bounded K4096 outcome.
They do not retrospectively admit v10, prove a root cause, supply an A/A
series or solve the cold-runtime/natural-yield research obligations.

## Next discriminating step

- **Action:** recheck any new RPC failure against the retained fresh S3 proof linked below. For any
  future inspect failure retain its new exit-status diagnostics and actual
  elapsed records before choosing another bounded plan.
- **Inputs/controls:** unchanged retained worker3a31bd2b…, same exact public
  query/panel/fixtures, strict full core14/30 and native replay predicates.
- **Limits/stop:** the [correction](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/INSPECTION-V11.md)
  permits one explicit K4096 repeat, no further automatic timed retry.
- **Outcomes:** complete proof resolves bounded reacquisition; any new
  orchestration failure stays unadmitted and requires assessment. Any positive
  model remains fail-closed pending independent group/rank/scalar verification.
- **Recheck trigger:** fresh publication proof or a new RPC failure.

## Resolution and history

2026-10-19: initial v10 terminal audit missing despite worker-reported capped
result. Separate v11 repeat passes locally and on fresh downloaded-record native replay; all172payload hashes and deterministic repack pass. Bounded reacquisition is resolved; original v10 remains unadmitted and root cause unknown.
[Report and inspected PDF](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261019T2015Z/REPORT.md),
[native summary](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/SUMMARY.json),
[publication/proof](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-400k-20261019/PUBLICATION.md).
Completing this record or merging the PR does not establish wider research claims.
