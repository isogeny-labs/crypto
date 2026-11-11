# Non-split kernel/codomain checkpoint and next map plan

Producer commit cryptanalysis0348e90f0654255917c0d6878168ebbda782ffd5;
`experiments/chained-transfer-reference/src/nonsplit.rs` SHA-256
`6ac0dbe9ebe814a5cdb898c3483684a5fb5306845ef08fd4453b363953a3df4c`.
The unchanged native runtime remains crypto75a737b8. Full written assessment,
editable diagram and inspected two-page PDF:
[NONSPLIT_KERNELS.md](https://github.com/isogeny-labs/cryptanalysis/blob/0348e90f0654255917c0d6878168ebbda782ffd5/experiments/chained-transfer-reference/NONSPLIT_KERNELS.md),
[PDF](https://github.com/isogeny-labs/cryptanalysis/blob/0348e90f0654255917c0d6878168ebbda782ffd5/experiments/chained-transfer-reference/nonsplit-report.pdf).
All results have unsigned/unverified standing. [pcc:unverified]

## Accepted bounded evidence, not an operational map

Predeclared E19/F17 gives C:y²=x⁶+2x²+2 and285 rational Jacobian classes.
Its branch roots in F17³ have Frobenius(012)(345). All15 matchings yield3
stable splittings:opposite pairs are degenerate; cross pairs have δ11/6 and
raw targets[10,6,14,2,12,10,9]/[10,11,14,15,12,7,9]. Both are smooth, descend
to F17, have leading9 and monic normalization w=3Y. EachJac has285 rational
classes. They are equivalent by z↦−z,w↦w. A complete4896-matrix PGL2 audit
finds no F17 curve isomorphism to source, without asserting unpolarized
Jacobian nonisomorphism. Producer claim42713c remains unsigned.
[pcc:unverified]

E109/F101's cubic root has nonsquare norm55. The written Euler/norm argument
gives a sextic6-cycle; only opposite-root pairing is stable and its linear
coefficient column is zero, δ0. This excludes only a direct nondegenerate
rational Richelot genus2 neighbor for that specific quadratic cover. Other
covers/degrees, product paths and transfers are not excluded. [pcc:unverified]

Final guarded native25/reference40 tests,1 old benchmark ignored, formatting,
strict Clippy and guards pass; producer claim3c51f5/receiptnonsplit-pinned-02.
Five core Lean finite-table/algebra lemmas pass5-declaration gate, claim465142;
local PCC check passes without signed standing. Geometry/finite-field norm
theory remain written inputs. Total40 written lemmas; gates22+8+7+12+6+5.
[pcc:unverified]

## Frozen next implementation and acceptance criteria

Use first nondegenerate complete-order pairing[(0,4),(1,5),(2,3)],δ11.
Source raw scale1, target raw w=3Y. Implement Kunzweiler Proposition8 on
balanced classes, using existing prime balanced arithmetic as the outer
reference. Factor coefficients lie in F17³; rational class support of degree≤2
splits over F17². Their compositum F17⁶ suffices: each target fiber remains
a quadratic Mumford pair and needs no point-root choice or F17¹² arithmetic.

Suggested extension arithmetic: F17³ uses the existing irreducible
q=T³+2T+2 and FpPoly operations. Adjoin s²=δ_NR with the existing Fp2
nonresidue. It stays nonsquare over the odd-degree cubic field. Store a+b*s
with a,b in the cubic quotient; inversion uses(a−b*s)/(a²−δ_NR*b²).
Check compatibility with existing Fp2 embeddings, inverses and Frobenius.

For a finite source point, use the quadratic
`U(z)=G1(x)H1(z)+G2(x)H2(z)` and
`V(z)=G1(x)H1(z)(x−z)/(y*target_scale)` modulo U. Retain degree-drop fibers:
one finite point plus the appropriate target infinity, or a double infinity.
The raw infinity sign is `−G1(x)*lead(H1)/y`, divided by target_scale.
Handle zero-y branch cases using the completed correspondence, not division
by zero. Source infinities use
`U∞=lead(G1)H1+lead(G2)H2`,
`V∞=lead(G1)H1/(source_cap*target_scale)`.
Prove/validate any degree restrictions used for this fixed model.
[pcc:unverified]

With ψ(P)=[fiber(P)−K_target], the two source infinity images are negatives.
For canonical source finite degree m and plus count n, evaluate
`sum ψ(Pfinite)+(2n+m−2)ψ(∞+)`. This accounts for the balanced base K;
do not copy the old Weierstrass-base translation formula. Require output
coefficients to descend to F17 and validate the canonical prime-field class.
Individual ψ(P) need not descend: cyclic factor permutation can change its
base translation by2-torsion. Check descent after the complete balanced class
sum, including infinity terms; do not discard valid intermediate extension
classes merely because their coefficients are not in F17.

For the dual reverse G/H, read raw source ordinate3Y and output source scale1,
then negate for the positive[2] dual convention. Check both dual norms[2]
on **all285 rational classes**, paired additivity and bijective rational
images. Then check every19 source scalar through quadratic pullback/R,
nonzero order19, reversed-chain/source norm[4] and inverse5 modulo19.
Maps must never use scalar labels. These are prospective criteria; no actual
non-split correspondence is implemented by this checkpoint. [pcc:unverified]

After F17 validation, CT15 predeclares shift s=2 on the existing E109 source:
C:y²=f(t²+2)=t6+6t4+13t²+56, with norm(α−2)=−f(2)=45 square. This is a
prospective route around the unshifted6-cycle restriction; preserve that
negative and assess actual stable kernels/maps on the shifted input. No
shifted map or family claim has yet been tested. [pcc:unverified]

Keep every failure, degeneracy and model equivalence. No solver/cost/prime-field
descent claim follows from this arithmetic work. S3/live-budget/KB/vault and
crypto frozen-ledger/archived CI blockers remain. Do not repeat completed
baseline/full-library runs without changed inputs or resolved causes.
