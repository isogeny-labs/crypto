# N83 free-selector known-witness search

- **Status:** investigating
- **Kind:** empirical solver bottleneck; internal cause unknown
- **Affected work:** [native N83 continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-phase-20261019-v3/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-19

## Problem

The original public N83 known-witness SAT control fixes eight x-coordinate
blocks but leaves representative and Frobenius selectors free. It does not
return within its 5,000-conflict / 180-second envelope on this native host.
This prevents treating that control as a passed test of free-selector N83
witness discovery. A distinct fully specified witness is not a substitute
for this search predicate.

The primary curve is `icv1-f2m83-tm6151469093347-debefd74`, a=0 over the
polynomial basis x^83+x^45+x^2+x+1, subgroup order 2417851639230796216685689.
The public five-term witness uses full 1182/866 representative domains;
fixture SHA256 is
`fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538`.

## Evidence and uncertainty

- **Observed:** v1 reached fixed_units_installed at 113.114676 ms, with
  101636 variables, 714736 clauses and 4980 XORs, then hit outer wall
  180537.788512 ms (exit 137, cleanup verified). No terminal counters exist.
  See the [original compact control record](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-phase-20261019-v3/records/native-phase-20261019/exec/witness/outer.json)
  and the [visual report](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261019T0140Z/REPORT.md).
- **Observed:** separately versioned v2/v3 controls additionally pin the
  unique representative index and seven-bit phase of every known summand.
  They return a verified SAT model with 0 conflicts. Independent native
  arithmetic replays all selector phases and the five-point group sum.
  A forged phase is rejected after coherent stream-custody updates.
- **Observed:** m5/m6 ordered/unordered free-selector solves pass in the
  exact small-field original-group assertion test. This is n7 coverage,
  not a successful free-selector search on N83.
- **Unknown:** final v1 conflicts/models, time in initial propagation versus
  CDCL search, and whether the fixed-coordinate assumptions propagate through
  the one-hot/phase encoding within the original cap.
- **Not established:** UNSAT, absent decompositions, a mathematical
  obstruction, natural yield, rank gain or a complete cold winner.

## What would unblock it

- [ ] Return and independently verify the original fixed-coordinate,
  free-selector witness under its declared search/resource envelope, with
  original-group and selected-domain replay. Do not add selector knowledge
  without declaring a separate control workload.
- [ ] If a correction is proposed, preserve the failed baseline and freeze
  its matched native baseline/candidate validation before measurement.

This would establish this specific known-witness search control. Natural
relation yield, useful rank and full cold runtime would remain separate.

## Next discriminating step

Instrument initial level-zero propagation and CDCL entry/return in
[`Solver::solve_with_lazy_clauses`](../src/cryptanalysis/sat.rs#L1766), then
freeze one original control diagnostic under the same
5,000-conflict / 180-second, 1 CPU/4 GiB envelope. Retain the fully specified
control as a separate encoding check. The source already records clause/XOR
propagation and analysis timers, but the retained driver summary does not
export them; those costs cannot be reconstructed from these logs. A trace
that never reaches CDCL localizes initial propagation; a returned capped
search supplies counters; a
verified model passes this control; a failure or censoring preserves the
unknowns. This step is proposed, not launched or scheduled here. Apply
normal protocol/card/budget gates before execution.

## Resolution and history

2026-10-19: Original control remains censored. Fully specified selector
controls pass, and both retained natural K2048/K4096 trials have verified
1,000-conflict zero-relation terminals. The bounded natural terminal
observability problem is resolved independently of this control gap.
