#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Public sign source beyond projection order four
T-1823 · 2026-11-07 · derived normalization and native producer checks

The value-only projected source now supports the full dyadic part q of p-1, subject to the retained prime, coprime-subgroup and word-size conditions. It replaces the shortcut r²=1 modulo q with e=(r²)⁻¹ modulo q. This broadens the source domain; it does not lower its square-root geometry cost.

#image("flow.svg", width: 100%)

*Identity.* Let beta(T)=W(r+1)² W(2)/W(r+2). Ward and the division chain rule give beta([k]T) W(k)^(r²)=beta(T)^(k²). Projecting and raising to -e yields C([k]T)=pi(W(k)) C(T)^(k²). Since e and the projection exponent are odd, C(-T)=-C(T). The product over the scalar half-set therefore transforms by the quadratic character of k.

*Implementation.* Use beta(T)^(-e) as a full-field seed; only its projection is the normalized point function. Project the block half-product at the end. The value-only division cache also checks r-torsion. No derivative seed, radical extraction, extension-field logarithm or scalar label occurs in runtime. Geometry/resultant work, inversions, powers and setup remain charged. The specialized q=2,4 quotient-residual route remains available.

*Complete source checks.* 1,108 nonzero points match scalar Euler signs, direct projected half-products and normalization covariance. Twenty-four identity-crossing windows pass. The new projection orders are 8,16,32,64,256. A wrong e=1 control differs on 266 point-function values; this is not a count of failed final half-products.

#table(columns: 5, inset: 4pt,
 [p], [curve (a,b)], [r], [P], [q / e],
 [17], [(2,2)], [19], [(0,6)], [16 / 9],
 [41], [(1,5)], [47], [(0,13)], [8 / 1],
 [97], [(0,5)], [79], [(1,43)], [32 / 1],
 [193], [(2,7)], [191], [(0,59)], [64 / 1],
 [257], [(1,7)], [281], [(1,3)], [256 / 145])

*Recovery.* All 617 targets on these five groups recover, with 5,929 source calls. Ordinary-envelope and workspace-adaptive routes agree in output and query fields. The inherited 63 public checks also pass with 511 calls. The normal build has four dependency inputs and excludes derivative jets and old source implementations. Original 4,563-case cohort unchanged. No timing study or cloud launch.

*Evidence and next step.* Native source, recovery and metadata builds passed; warnings retained. Inputs, commands and hashes are recorded. Producer evidence has no independent signed verdict. Lauter-Stange2008 Theorems3-4 provide the established identities; URLs and full local derivation are in DERIVATION.md. KB unavailable. Next extend quotient-class arithmetic beyond q=4 while retaining this general dyadic source as a reference. Coprimality and r!=p remain explicit conditions.
