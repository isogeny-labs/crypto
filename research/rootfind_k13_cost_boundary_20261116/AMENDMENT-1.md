# Amendment 1: encoded target and rho identity

Filed after the first two analyzer invocations failed and before any result
file was produced. The committed [plan](PLAN.md) and its input, row selection,
metrics, and controls remain intact. [pcc:unverified]

- The first invocation used a set containing `workload.target`, which is a JSON
  list and therefore unhashable in Python. The source now converts each target
  pair to a tuple solely for the equality check.
- The second invocation found `factor_base: null` on the `rho-strong` reference.
  Rho has no factor base by design. Check that this field is null for rho, and
  require one identical `points_sha256` among the other seven arms within each
  matched cell. This replaces the plan's too-broad factor-base sentence without
  changing the workload or excluding the reference.

Neither failed invocation wrote `SUMMARY.json`. The exceptions remain described
in the final report. No scientific trial or timing execution was launched.
[pcc:unverified]
