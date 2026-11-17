# Four-coordinate Kummer repair closure

The new field theorem establishes that the literal four-term row action preserves nonzero vectors whenever its matrix squares to cI with c nonzero. It also proves that applying the action twice gives a scalar multiple of the input and that the sixteen cross-product comparator is preserved and reflected for nonzero inputs.

## Source binding

snapshot/walk-source.rs.txt:48–50 implements linear by four modular multiplies and four successive modular additions starting from zero. RepairClosure.linear uses the exact row/column ordering and parenthesization in a field. Source lines 77–95 try ordinary xadd, then transform x and diff with each action, call xadd again, and return linear(z,act) without a second nonzero check. The source xadd returns Some only after testing its intermediate output nonzero. preserves_nonzero supplies the field-level missing postcondition for a successful repaired branch under squareScalar and c≠0.

This proof does not establish that a branch succeeds, that all xadd inversions are valid, or that returned coordinates represent the intended sum. It proves the nonzero postcondition of a successful field branch. Native mul is narrow u64 multiplication. The separate comparator bridge uses canonical inputs and p≤2^32; this packet does not derive the machine execution semantics of the entire linear fold from that bound.

## Exact theorem scope

linear_comp proves composition against the explicitly expanded four-term matrix product. twice derives M(Mx)=cx from sixteen entry equalities M²=cI. preserves_nonzero and nonzero_iff then use c≠0. twice_same establishes the double action's cross-product relation. same_preserved is a forward field consequence of proportional nonzero inputs (its matrix hypotheses are retained but unnecessary); same_iff uses the invertibility condition for reflection. Zero vectors are excluded from the scalar-comparator interpretation.

chart_basis_change separately proves the corrected N16 column relation: C2[:,2]=-C1[:,2]-σ C1[:,3], all other columns equal, implies C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+σa0). This does not identify C1 with C2 or permit substituting C2 in the degree-one chart C1(0,1,x,x²). A negative rational control demonstrates the wrong degree-one substitution can change the projective point.

## Prior evidence, without rerun

Copied N16 CHART_NONVANISHING.md and REPORT.md are historical sources. They report nonzero scalar-square matrices for 105 recorded actions, corrected degree-two C2 and degree-one C1 determinant checks, and declared-chart action certificates. This task did not rerun those validators, authenticate their raw tables, or bind their matrices to runtime acts. Historical numerical counts are references, not new outputs.

To instantiate this theorem for a runtime action, bind its exact model, modulus, matrix entries and recorded nonzero square scalar; establish field/word correspondence and canonicality; and connect that action to the declared chart and intended geometric translation. Xadd geometry and chart interpretation remain independent premises. Nonvanishing alone does not supply them.

## Checks and retained failures

The packet builds Repair, RepairControls and RepairReadback with the pinned Lean toolchain. It audits ten main declarations and nine rational controls. Controls include a doubled nontrivial permutation with M²=4I, literal transformed values, two repairs, c=0 failure, zero-comparator vacuity, zero input persistence and wrong degree-one basis substitution. They use kernel tactics; no native evaluation or scientific computation is performed.

Attempt1 failed because squareScalar needed explicit unfolding for rewriting. Attempt2 failed on vector notation parsing. Attempt3 retained unreduced Fin conditionals in the basis-change proof. A control build during attempt3 used the earlier Repair.olean and is explicitly superseded: the final sequence rebuilds Repair first, then all controls/readbacks, and checks pre/post hashes. All failed sources and logs are retained. Cached dependency binaries remain trusted; no independent kernel-binary checker was used.

This is a producer packet. Independent source-fidelity review and publication binding remain pending. No repository or cloud state was changed, no scientific validator was rerun, and no measurements or cost ratios were generated.
