# R119 handoff

R119 translates both F4 roots of the R118 repeated lambda edge on all four x=0 base places. It is a fixed, exact local calculation on the degree-112 marked23 source, not a global map construction. Protocol and source were committed at `9d4bdc601` before data. The result is SHA-256 `712278837f84ed1eba630a5aa368a534be7bdd381196ff27ff3fd1a0f52ffa71`.

The new lead is y1/e1: a one-root correction of local order 5 and a seven-root correction of order 23/7 near each F4 residue root. The y0 places and y1/e2 retain eight-root repeated edges at orders 1 and 2. Next, translate those repeated edges further, use the simple y1/e1 branch and tame degree-7 branch as controls, then evaluate N51 on the normalized branches.

Fresh replay, changed-source rejection, independent F4 bit-plane evaluation, Lean finite polygon checks, and a visually inspected PDF passed. No producer, verifier, Lean, or rendering failures were observed.
