# N83 free-selector known-witness search

- **Status:** resolved for the exact original known-witness control
- **Kind:** bounded control verification; measured XOR propagation cost
- **Affected work:** [native N83 continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-phase-20261019-v3/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-19

## Problem

Resolved by the [native post-solve export](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-selector-export-20261019/RESULT.md):
the original free-selector workload returns SAT at787conflicts /167.34s
worker wall under unchanged5000conflicts /180s,1CPU/4GiB. Actual model
representative indices/phases and original-group sum replay independently;
a coherently rehashed phase forgery is rejected. This resolves this control
only. Natural yield/rank and full cold-runtime selection remain open.

The original observed problem and failures below remain historical:

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

- [x] Return and independently verify the original fixed-coordinate,
  free-selector witness under its declared search/resource envelope, with
  original-group and selected-domain replay. Do not add selector knowledge
  without declaring a separate control workload.
- [x] If a correction is proposed, preserve the failed baseline and freeze
  its matched native baseline/candidate validation before measurement.

This would establish this specific known-witness search control. Natural
relation yield, useful rank and full cold runtime would remain separate.

## Next discriminating step

The boundary diagnostic and separate post-solve export are complete. Initial
propagation ends at192.850022ms, CDCL enters at193.136020ms, and SAT returns
at167259.951266ms; returned timers record167102070169ns XOR propagation
and70147211ns clause propagation, with35307decisions /36757XOR passes.
These observations localize cost to XOR work during search. They establish
no speedup over the older censoring; all observations are L0 and the extra
export adds no solver assumptions. Next is a frozen native XOR-engine
profile/candidate comparison, while positive natural-relation/rank/scalar
replay remains a separate implementation gap. Neither is launched here.

The following was the preregistered next step before those runs:

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

2026-10-19: Boundary source5e9ee5d4 returns original SAT at787conflicts,
but its frozen auditor deliberately stops without decoded-selector export.
Separate sourcecbe8e44da adds only post-solve model export and independently
audits the same workload at787conflicts within180s. Both runs and the
earlier180s censoring remain intact. The fully pinned gate also passes;
its initial native receipt-checker float-roundtrip defect is preserved and
fixed with a byte-preserving native report and regression test.

2026-10-19: Original control remains censored. Fully specified selector
controls pass, and both retained natural K2048/K4096 trials have verified
1,000-conflict zero-relation terminals. The bounded natural terminal
observability problem is resolved independently of this control gap.

## Indexed XOR continuation (2026-10-19)

The [native matched comparison](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-xor-indexed-20261019/RESULT.md)
retains the exact solved control and independent actual-model/group replay.
Seven verified observations per native arm preserve all search signatures;
five dense/indexed wall ratios9.95866828830–12.1636332262 are descriptive,
and counted word updates fall394.704355222×. CryptoMiniSat5.17.0 remains
faster (candidate wall / CMS algorithm CPU6.27411646961–6.34929032028,
different timers). Next-round goal: reduce repeated decided-row scans with
backjump-safe invalidation and fresh differential/external comparison.

Native natural K2048/K4096 each completed1000 and separately10000 conflicts
with zero models, relations and rank. Their query/custody audits pass;
this does not establish UNSAT or absent decompositions. Natural positive
relation/rank/scalar verification, useful yield and full cold-runtime base
selection remain unresolved. The known-witness control stays resolved.
