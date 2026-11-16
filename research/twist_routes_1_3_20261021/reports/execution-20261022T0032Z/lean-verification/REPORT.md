# Kernel-checked polynomial and point-group lemmas

Run `lean-verification`; source freeze `0145df8ff32a104d186c31eca2f04c9ee256b21d`. Host `a81fcf6dc864`, Debian 13 x86_64, managed four-CPU virtual host, L0. Workloads use public synthetic fixtures or public-parameter checks. No wall-clock or full-cost improvement is admitted.

Raw outputs: [run directory](../../../lean/receipts/). Commands, binary/input hashes, seeds, resource limits and statuses are in [execution receipts](../../../execution-v1/) and the versioned protocols in the study root. No independent-host replay is claimed.

## Recorded observations

| Metric | Count / recorded value |
|:--|--:|
| audited theorems including generated definitions | 13 |
| compiler trusted proofs | 0 |
| explicit theorems | 9 |
| nonstandard axiom failures | 0 |

![Recorded count outcomes; the editable data are PLOT_DATA.json.](METRICS.pdf)

Lean 4.34.1 and the locked Mathlib commit build the five exact cleared polynomial identities and four trace/eigenspace statements on the actual Mathlib Weierstrass point group. The auditor checks nine explicit theorems plus four generated defining equations on standard foundations only. The concrete Frobenius/twist embedding, odd subgroup, field/chart/Jacobian maps and cost/complexity bridges remain open. See the exact proof map in lean/README.md; none of these missing bridges is an axiom or a completed theorem.

The chart has no statistical interval: census counts are exact on the stated finite sets; solver completion counts describe these bounded processes only. Missing or failed computations stay unresolved. Full original acceptance criteria and proof/measurement gaps remain in [the requirement matrix](../../../EXPERIMENTS.md).
