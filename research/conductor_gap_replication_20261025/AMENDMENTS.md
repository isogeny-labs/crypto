# Pre-data amendments

## 2026-10-25, v2: CM-map eligibility and execution preflight

No primary solver measurement had run when this amendment was written.
Protocol v1 and source commit `6e30fc1de378ee764f4a63abfddc26298748b01c`
remain available. `prepared-b/` is the rejected v1 fixture set; its specs
were never run. The `check-bases` preflight failed with `this trace has no
required rational twist kernel` before it could certify the direct-map
control. Its fourth class also had trace one (an anomalous curve). This
set is retained as failed preparation, not used to infer solver behavior.

Panel B v2 uses `prepared-b-v2/`. In addition to every v1 admission rule,
exclude t=1 and require `(t - N + 2) mod (2q) = 0`, with
`q=(abs(D0)+1)/4` (5 or 11). This selects Frobenius eigenvalue -1 on the
canonical small CM kernel, making that kernel rational on the quadratic
twist. This q is distinct from the large conductor prime N. The large-N
kernel-field degree criterion is unchanged. Exact scalar/recurrence/map
base equality remains mandatory; the congruence alone is not accepted as
an executable-map certificate. Trace scanning, prime bounds, counts, seeds,
metrics and decision rules are unchanged. No selection uses solver costs.

The controlled-experiment skill requires a runtime preflight and running
the positive control first. These had not been operationalized in v1.
Before the main campaign, freeze and run a separate 128-row calibration:
four curves and four targets in each A size class (classes 2 and 5, chosen
by largest subgroup in that size class), with rho.negation, rho.plain and
BSGS negation; one crater model and four targets in each B CM field
(classes 0 and 2), with rho.negation, scalar, recurrence and direct-map IC.
Use target seed 2026102520 and measurement seed 2026102521, otherwise the
same settings. Calibration is excluded from every primary estimate.
Record fixed/session and per-row costs and estimate the full campaign and
replay time before starting. Preserve the calibration even if it fails;
sample counts and the inference rules will not be tuned to its outcomes.

Main A ordering is now: all eight control sessions, exact replay of these,
then a positive-control gate requiring all 8,192 answers verified and the
whole-class-bootstrap 95% lower endpoint of rho.plain/rho.negation above
one. On failure do not start the primary. On pass run the primary/BSGS
sessions and replay them, then B and its replay. The existing shared
90-minute admission guard includes controls and replays. No concurrent
builds, tests or other solver work. The final A/A and coordinate controls
remain required for any equivalence verdict.

Bootstrap seeds are explicit: level l uses 2026102504+l, secondary overall
2026102509, positive control 2026102515, A/A 2026102516, coordinate model
2026102517. All use 10,000 whole-class draws and the v1 percentile rule.
These are clarification of the native analyzer, not outcome-based tuning.

The calibration supports a runtime feasibility check, not a power claim.
For the primary equivalence test, interval containment, rather than a
rank-test resolution formula, determines achieved precision. Eight classes
may yield inconclusive intervals; no assertion of pre-established power
or representative sampling of ordinary curves is made.
