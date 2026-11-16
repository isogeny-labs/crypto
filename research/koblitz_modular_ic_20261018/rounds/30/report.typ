#set page(paper:"a4",margin:(x:16mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
= Koblitz cover transfers — R30
*Uniform integral recovery and actual ordinary scalar monodromy* \
Exact algebra and native observations; no IC cost, rank or attack verdict.

R30 separates two scale tests. The formal coefficient pattern is checked
through cyclic length 1021, while actual kernels on the existing n=5 ordinary
lane are computed over F2^436. Neither constructs larger-n geometric transfers.

#image("round30.svg",width:100%)

== A universal odd-cycle identity
In Z[w], with w²+w+2=0 and rⁿ=1, set m=(2+w)+7r+(6+2w)r³.
For odd n put S=sum of (-r³)ⁱ for i=0,...,n-1, a=12S, e=w-3,
b=1+2r³ and h=17+a(r+r³). Then (1+r³)S=2 and e²=7(1-w).
#raw("B1 = a - e*a^2*b\nB3 = (1-w)*a^2*b^2 - h*(1-e*a*b)\nB1*m + 7*B3 = 1",block:true)
Lean proves this for every odd n. Compact native certificates check all
augmentation basis vectors for 19 odd lengths through n=1021; their least
scalar is1. Ten even controls have least scalar7 by the evaluation obstruction
r=-1,w=3 mod7. This concerns the displayed formal family, and does not alter
R21's different regular-cycle maps or their denominator5 obstruction.
#pagebreak()
= Actual kernels and the constant-extension distinction
The actual R29 pairs are retained. Native factor products, stable cyclic7/11
kernels, commuting squares and both complete49-level transported triples pass.
A separate enumeration certifies that all common-j and parameter roots are
rational and simple over the retained fields. Compatible middle counts are
complete for these finite ordinary inputs.

#table(columns:(1fr,1fr,1.5fr),inset:7pt,
 [Base field],[Complete middles],[Kernel colors],
 [F2^12],[2],[(3,4) and (4,3)],
 [F2^436],[1],[(3,3)])

Colors are powers of doubling modulo sign among nonzero residues mod11, identifying signs. All six are nonzero,
so the scalar fibers have orbit length5, with absolute F2 algebra dimensions
60 and2180. The larger calculation uses degree-five kernel algebras rather
than explicit generators for the full degree2180 field.

== An inert F2 orbit is not an individual F32 component
For each actual color k at base degree d, the coupled permutation certificate
checks F^d=D^k on a cycle of length5d, with deck shift d*k-inverse modulo5d.
F^5 has five components, each of length d; deck permutes them regularly.
An individual component difference is a nonzero integer vector. The sum of
all five components has zero deck difference. The corrupted-monodromy control
is rejected, and both colors satisfy their determinant character relation.

These are permutation-module statements. Actual divisor classes can still be
principal or have zero image in the desired factor. Conversely, degree460
base cancellation and an inert F2 orbit alone do not certify that an
individually colored F32 collector column is zero. Keep the component and
coefficient-descent information in the reverse evaluator.

The ordinary Hecke source used here is not identified with the new proper
C61 incidence carrier. Its moduli/collector interpretation remains a written
bridge. Complete signed principal divisors, exceptional fibers, intended
Koblitz-subgroup binding and the actual F2 factor map remain open.

== Evidence and immediate continuation
Five native attempts accepted; two Lean compile/audit runs accepted.
Nine named theorems and13 audited declarations include4 generated equations;
0failures and0trusted-native results, Lean4.34.1/mathlib d13f23b. Native sources,
inputs, certificates and audit logs are retained; no earlier raw archive changes.
No timing or complete-root-software pass is claimed.

Next: bind actual F32 component divisors, then evaluate their Picard/factor
images; apply the kernel method to degrees284/83. Further proposals are an
adaptive shift exponent for even cycles and canonical/Cartier constraints
before another map elimination. Larger-n geometric carriers remain a separate
construction task. See LEADS.md, NEXT.md and PROOF_MAP.md for acceptance bounds.

References: R22 RECOVERY.md; R27 CM.md; R29 ACCEPTANCE.md; native
ordinary13_lift.rs; Stacks0AMF, Lemma10.32.4; Sutherland modular polynomials;
Kilford–McMurdy arXiv1110.6801, section2. The elementary nilpotent-unit mechanism
is prior art. No novelty or official research-status promotion is made.
