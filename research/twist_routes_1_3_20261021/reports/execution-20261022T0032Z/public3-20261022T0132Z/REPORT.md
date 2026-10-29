# Public 3-torsion companion-module filter

Run `public3-20261022T0132Z`; source freeze `36563882e86dacbbcea0d2e473aca236a116ec97`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../execution-v1/runs/public3-20261022T0132Z/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| NIST cases | 5 |
| NIST positive module filters | 1 |
| geometric constructions certified | 0 |
| parameter cases | 15 |
| toy cross check mismatches | 0 |
| toy cross checks | 10 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

PARI enumerates det(M)=-1 matrices over F3 with MA=-AM for the Frobenius companion matrix. All ten toy counts equal the earlier E11 counts. Among five public NIST P-curve order pairs, only P-521 passes this necessary module filter, with four matrices; the other four have none. This is a public-parameter calculation, not a NIST DLP. The actual torsion/Weil-pairing bridge, principally polarized quotient, genus-two model and divisor maps are not constructed or certified. A positive count is a candidate for further construction, not a speedup or novelty result.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
