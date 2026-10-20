# Native N83 800K-conflict runs exceed 4 GiB

- **Status:** investigating
- **Kind:** observed resource limit; unmeasured higher-memory outcome
- **Affected work:** [800K protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/PROTOCOL.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-20 UTC

## Problem

The retained native worker cannot yet provide admitted 800000-conflict
terminals for either public N83 base under the frozen4GiB memory limit:
both measured attempts ended with Docker OOMKilledtrue/exit137.
Curve a=0 icv1-f2m83-tm6151469093347-debefd74, field x^83+x^45+x²+x+1,
subgroup2417851639230796216685689; retained query seed2026100801,m5,identitymask0.

## Evidence and uncertainty

- Observed K2048-v12 last progress789000conflicts; outer2856001.340487ms.
- Observed K4096-v12 last progress702000conflicts; outer2575759.292233ms.
- Both:4GiB/no swap,800000conflicts/4800worker seconds, strict core14/30,
  exact frozen sourcef94d5da32ecd9b6a8956749978cd5a771482a0ed, recorded exit137,
  OOMKilledtrue, cleanupverifiedtrue; native guard refused search admission.
  Actual card/relay/receipt digests and verbatim relay passed assessment.
- [Records and publication](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/RESULT.md).
  Native SAT-trace checks report valid nonterminal prefixes, terminal counters null.
  Progress snapshots are not terminal counts or relation-rate evidence.
- Suspected memory growth within the SAT worker is not profiled here; exact
  allocation cause is unknown. More memory has not yet been shown sufficient.
  No mathematical obstruction, UNSAT or unknown-scalar recovery follows.

## What would unblock it

- [x] Separately frozen8GiB/no-extra-swap attempts reach800000conflicts
  with complete native terminal, original-group/query, custody/resource and
  cleanup audits; retain every original failure and all new costs separately.
- [ ] Fresh downloaded-record replay and exact payload/archive verification.

This would resolve bounded terminal execution under8GiB only, not prove the
4GiB issue fixed, establish useful natural yield, rank, cold runtime or rho gain.

## Next discriminating step

[Explicit memory addendum](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/MEMORY-V13.md):
one separately assessed repeat per base, same worker/query/panels/conflict/wall
caps and CPU/isolation gates,8GiBmemory/no extra swap. New maximum9600worker
seconds, cumulative19200across all4attempts. No automatic timed retries.
A complete terminal supports only its recorded bounded result; another OOM,
wall censor or infrastructure failure stays unadmitted and needs a new protocol
before another attempt. Recheck after these independently assessed attempts.

## Resolution and history

2026-10-20 UTC: both4GiB attempts preserved and assessed;8GiB repeats planned,
not yet executed at preregistration. Resource obstruction remains unresolved.

2026-10-20 UTC correction before further search: K2048-v13 exited immediately
because the launcher supplied4096MiB while Docker imposed8GiB. Outer376.218476ms,
command10.146594709s,exit1,noOOM,cleanupverified,no query/SAT start; exact relay
passed assessment. Guard-v14 corrects the worker declaration from the same
frozen bytes used for Docker. K4096-v13 never compiled/executed; its planned
work is superseded byv14. [Corrective addendum](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/MEMORY-V14.md)
preregisters two corrected8GiB attempts, cumulative maximum24000worker seconds
across five actual-or-planned attempts. The resource obstruction remains open
until full terminals and independent downloaded-record replay pass.

2026-10-20 UTC observed execution update: corrected v14 attempts both complete
800000conflicts at8GiB with native PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY,
zero models/relations/rank,exit0,noOOM,no wall censor and cleanupverified.
Both exact card/receipt/verbatim-relay assessments pass. This establishes
bounded8GiB terminal execution only. Original4GiB failures remain unadmitted;
fresh archive replay/storage custody is tracked in the publication and the
current S3 access obstruction remains open. No minimum-memory or speed claim.
