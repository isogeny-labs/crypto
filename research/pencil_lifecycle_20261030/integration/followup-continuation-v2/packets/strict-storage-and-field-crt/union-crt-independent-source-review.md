# Independent source-fidelity review: union and CRT certificates

Producer revision: 929635eec1, PR3228. Native pin: 55b56b1e04f2c4763fa1090d99961864dff221e8. Reviewer /root/quintic_fidelity_review. Read /var/tmp/union-crt-review-plan.md before sources. No sibling review consulted. Context inherited, so no mutually blind claim. Prior strict-storage producer and review artifacts were not changed.

## Result

Source fidelity is **faithful_scoped** for the eleven word-union declarations and three CRT certificate declarations, as limited by REPORT.md. No arithmetic-sign, orientation, or mutable-write mismatch found. Fresh reviewer machine status is **not_verified**: this task inspected archived named readbacks and source, without running another Lean replay. No complete validated_scoped formal review or end-to-end execution is asserted.

## Joint A: signed union potential and anchor transport

The reduced equation cu*Vru+cv*Vrv=a+b*k, with cu and cv in {1,-1}, yields Vru=(-cu*cv)*Vrv+cu*a+cu*b*k. unionPot uses exactly these literal Int8/UInt64 operations. reversePot keeps s and replaces offsets by -s times the offsets, which is correct because s squared is one. The negative-sign cases are covered by hypotheses and sign lemmas; invalid zero sign is excluded, not silently treated as valid. No Int8 overflow occurs within +/-1 products/negation. Canonical coefficient assumptions support signed modular negation; subtraction transport additionally requires 0<r and r<=2^63, preventing overflow in the selected word formula. Prime modulus is present wherever the graph state requires Field structure; simple union/reversal identities need less and correctly use weaker assumptions.

For a fixed child value x+y*k, solving child=s*parent+pa+pb*k gives parent=s*(x-pa)+s*(y-pb)*k. transportPair implements this formula, not the opposite difference. Coefficients are proved canonical by transportPair_correct.

## Joint B: graph mutation

joinWord models parent[child]=parent, pot[child]=pot, and the optional fixed[parent] update. It leaves the child fixed value present, matching native source. When both endpoints have fixed values, the modeled fixed array remains unchanged: native anchor(parent,a,b) dispatches to constraint without overwriting the existing fixed value, so the modeled writes remain correct while the returned constraint is deliberately outside this theorem.

GraphUnion.Valid is precisely Represents AND Anchored. It is not acyclicity, root uniqueness, reachability or termination. The proofs need the assumed true reduced root equation; they do not derive it from the preceding find calls. They need not assume distinct roots to preserve those equations, but that weaker premise does not establish the native distinct-root branch's forest invariant. The report correctly excludes size-vector mutation/overflow, counters, returned constraints, preceding-find composition and forest termination. Canonicality preservation is a separate joinWord_good theorem with Good graph and FixedGood hypotheses; forward/backward validity alone do not supply those hypotheses automatically.

## Joint C: CRT certificates

Using A=qa-qb, B=ra-rb, Ub=X^2+qb*X+rb, H=(B-A*qb)-A*X, expand (A*X+B)*H. The coefficients are -A^2 on X^2, -A^2*qb on X, and B^2-A*B*qb on the constant term. This equals norm-A^2*Ub, with norm=B^2-A*B*qb+A^2*rb, exactly matching the source norm formula.

For C=sb-sa and D=tb-ta, multiply (C*X+D)*H and subtract (s0+s1*X). The result is -A*C*(X^2+qb*X+rb) when s1=C*B-A*D and s0=D*(B-A*qb)+A*C*rb. The coefficient orientation matches native b.v-a.v. Source-substitution theorem uses these differences literally and supplies Ub divisibility with explicit witness C(-A*C).

These are polynomial identities involving no division or inverse, so arbitrary CommRing is justified, including rings with zero divisors. Equal support makes A=B=norm=0 but leaves the identity valid. Nonzero norm with zero slope is possible and covered by a control; no guard implication is claimed. A field/invertible-norm corollary is needed for inversion/interpolation, and a scalar nonzero alone does not justify inversion over a general ring. REPORT.md explicitly leaves coprimality, interpolation, product divisibility, quotient/leading coefficient, batching, and output equality open. No Jacobian-class addition result is implied.

## Evidence inspected

Word-union inventory has eleven main names and four controls; replay2/WordUnionReadback.stdout lists exactly those fifteen with only propext/Classical.choice/Quot.sound. CRT FastCRTReadback.lean.txt and its archived stdout list three main names and five controls with only those standard axioms. CRT lacks a separate inventory.txt, but its eight exact names are explicitly listed in the readback source and output; this does not invalidate the source result. No independent replay was performed, so producer receipts remain producer receipts.

Sources read: review plan; REPORT.md; word-union inventory, WordUnionPotential/Transport/State/Global, GraphUnion Union helper, word-union readback output; FastCRTCertificates/Controls/Readback and archived readback output; native SignedGraph.edge/anchor/constraint and optimized quadratic-addition coefficient formulas. Repository AGENTS and review-evidence Lean-review instructions retained from earlier review in this session. No novelty claim made; existing graph algebra and polynomial-ring expansion suffice for these identities.

## Next technical obligations

Compose the find outputs and union arithmetic; prove distinct-root update preserves a terminating forest; connect the returned constraint to its native word computation; keep size arithmetic/counters separately bounded. For CRT, derive field interpolation under invertible norm, then product divisibility and exact reduction/leading-coefficient facts. These are separate mathematical components, not inferred from this certificate packet.

No repository edits, scientific trials, cloud execution, or budget changes were made.
