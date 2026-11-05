# R129: direct uniformizer at the degree-7 double root

Exact local computation. Pin R127–R128 results and R128 source by SHA-256.
Use the full R128 expansion `F(s,u)=f23(a+s,90+u)` over F128. Since the
coefficient of `s` at `(0,0)` is 84, solve `F(s(u),u)=0` in `F128[[u]]`
through `u^127` by a complete fixed-point lift; verify the residual and
the first nonzero coefficient. Independently reconstruct F from the tracked
R51 source and solve coefficient by coefficient. Only if both match and the
leading term is `6u²`, certify the single normalized base branch with
uniformizer u and `v_u(x-a)=2`. Combine it with R127's two simple factors
for the complete degree-7 base fiber accounting.

Controls: pinned hashes, full residual, independent source expansion and
root, changed-input rejection, byte replay, and Lean finite leading and
degree arithmetic. No lambda or N51 valuation is claimed. Local exact
process with zero cloud cost; source/protocol published before result data.

Prior sources: R51, R127–R128, [Stacks normal curve/DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y),
and [Stacks simple-root lifting](https://stacks.math.columbia.edu/tag/04GE).
