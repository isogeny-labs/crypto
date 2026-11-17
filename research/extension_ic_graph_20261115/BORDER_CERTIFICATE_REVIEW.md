# Review pointer: commuting quotient graphs as a replay certificate

Read-only review during stage19 compilation,16November2026. No implementation
or trial selected. [pcc:unverified]

Sources checked (KB): MCPabsent; wrapper queries on border-basis commuting
matrices/ideal-equality certificates and non-GB quotient-graph/spurious-root
failures unavailable(no executable). Receipts /tmp/ic-border-certificate-kb-search{1,2}.log.
Read nativeS4border/normal-form construction and residual-walk note sections
11.12–11.14, including the corrected negative lazy-elimination ceiling.
Those records concern constructing the old Macaulay quotient; they do not
price a replacement for stage18's exhaustive replay certificate. They remain
negative prior evidence, with no implied new advantage here. [pcc:unverified]

Primary: Kehrein–Kreuzer, *Characterizations of Border Bases*, Definition15 /
Proposition16 and field/order-ideal definitions, [author PDF](https://www.symbcomp.fim.uni-passau.de/fileadmin/dokumente/fakultaeten/fim/lehrstuhl/kreuzer/AlgebraicOil/Publications/049_Kehrein_Kreuzer_characterizations_of_border_bases.pdf),
SHA2568c928f34f99e94670ff62c0553cc9dae5129bcde808d77e31eb11a2d097be1b5,
local /tmp/ic-border-certificate-primary.{pdf,txt}. The formal multiplication
matrices of a complete border prebasis on a finite divisor-closed order ideal
commute exactly when it is a border basis. This concerns the ideal generated
by the border polynomials, not automatically the original S4ideal.
[pcc:unverified]

Possible later certificate, still requiring implementation review: native
replay supplies basis polynomials in the input ideal I. Complete symbolic
reductions of border monomials produce B subset I. If all formal multiplication
matrices commute, they certify the quotient by J=(B). To prove J=I additionally
check every original input f in those matrices on the constant vector:
f(M)e_1=0. Then I subset J, while construction supplies J subset I. Both
inclusions, finite divisor-closed order ideal, every border term, exact
commutation and original input evaluation are necessary. Merely checking one
matrix, its recurrence, apparent roots or a guessed quotient dimension is
insufficient. The empty-order-ideal/unit-ideal case needs explicit handling.
[pcc:unverified]

Existing S4 code already constructs all three borders but extracts using one
multiplication matrix. A later implementation might share those normal forms
with a commuting-graph certificate. Its construction/check/failure costs must
be measured; counts do not establish a speedup. Keep existing exhaustive and
pair-graph modes, with charged fallback on any unsupported or failed certificate.
No novelty, completeness proof for unwritten code or performance claim.
[pcc:unverified]
