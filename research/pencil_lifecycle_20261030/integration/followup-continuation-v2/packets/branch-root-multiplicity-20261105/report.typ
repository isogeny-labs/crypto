#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Base root multiplicity at the actual branch stalk
[pcc:unverified] Seven prospective Lean declarations construct the base-polynomial transport. No compiler, tests, axiom readback or scientific execution was run. The predecessor source and independent review remain unchanged.
#image("multiplicity.svg",width:100%)
== Construction
Actual branch residue evaluation of affine polynomial g equals g(a). Localization at its kernel makes an element a unit exactly when its residue is nonzero; branchRingEquiv transports this criterion to the actual weighted-Proj stalk. Hence E(g) is a unit iff g(a) is nonzero. No arbitrary unit premise is supplied.
For nonzero g, Mathlib's exact monic-division factorization gives g=(X-a)^m t with m=rootMultiplicity(a,g) and t(a) nonzero. Map through E; E(t) has order0 and E(X-a) has order2. Additive multiplicativity and the power rule give order(Eg)=2m in Nat-infinity. Finiteness is established before conversion to Nat.
== Norm support consequence
The prior actual support-stalk sum2 forces each support polynomial nonzero: zero would contribute infinity. Transport each of the three orders and cancel the factor2 to obtain base multiplicities summing to1. This supplies the previously missing normalization bridge at source level; it does not double Mumford multiplicity.
== Countermodels and scope
The zero-polynomial condition is essential: rootMultiplicity(a,0)=0 but valuation(E0)=infinity. Nonzero constants give0; powers(X-a)^m give2m. Prior source fixture f=X⁵-X at a=0 has base support multiplicities(1,0,0) and stalk orders(2,0,0). These are symbolic checks, not executed tests. Simplicity/DVR hypotheses cannot be dropped.
Exact declarations and source pins are in declarations.csv and input-bindings.json. Primary APIs: Polynomial.Div factorization and residual nonvanishing; localization unit criterion; actual branch parameter order; AddValuation.map_pow; ENat finite conversion. Seven requested print-axioms lines are not run receipts. Remaining checks are exact closure binding, independent review and authorized kernel/axiom validation. Infinity and global divisor assembly remain separate.
