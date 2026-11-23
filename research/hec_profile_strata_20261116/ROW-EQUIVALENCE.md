# Exact row equivalence for nonbranch multiplicity inputs

This is a post-screen derivation from classical principal-divisor and Mumford reduction facts, not a new DLPalgorithm or a tenfold proof. It strengthens REDUCTION.md's surviving-weight residual identity.

Let D=sum mi*Pi be the prescribed signed NONBRANCH effective input, degreeM. Suppose phi=a-b*y satisfies its local conditions, poleorder n<=M+g, and phi=h*phi0 after primitive cancellation. LetD'=sum ki*Pi be the surviving prescribed input; L=D-D'has0<=multiplicity<=ord_(x-xi)h at eachsupportpoint. The finitezero divisor V(h) of the polynomial h can be written

    V(h)=L+conjugate(L)+V',

where V'is an effective vertical divisor, possibly including ramifiedpointpairs and nonrational conjugatepairs. Writing div0(phi0)=D'+E0, withE0thedegree<=gprimitive residual, and div0(phi)=D+E, gives

    E=conjugate(L)+E0+V'.

Because degree(E)=n-M<=g, reduceEby removingverticalpairs. The remainder is the unique reduced Mumford representative Qofclass-D. E0is semireduced by primitivity (no conjugatepairs, branchmultiplicity<=1). Thus after removing V', additional cancellations can occur only between rational nonbranchpoints in conjugate(L)andE0. They preserve the merged signed place vector; branchpoints ofE0areunchanged. SinceLhasnonbranchsupport, signed(conjugate(L))=-signed(L). Consequently, INCLUDINGbranchcoordinatesofE0,

    signed(D)+signed(Q) = signed(D')+signed(E0)

as INTEGRALcanonical signed rational-place vectors whenever theysplit. No modularscaling, droppingbranchcolumnsorzero-logassumptionisneeded. If either residual has nonrational factors, these cannotcancelagainst rationalconjugate(L), so Qsplits overFp iffE0does. Inputbranchmultiplicitycases are outside this lemma; do not extend the exact integer-vectorstatement to them automatically.

The constructive consequence is an alternative row generator: add weighted original anchor places with Cantor, negate/reduce, split its degree<=gMumfordu, and merge originalinputweightswiththoseorientedroots. This produces the same admitted row/smoothnessdecision as primitive rational interpolation for these inputs. It may have a different arithmetic cost and deduplication policy. It does not improve useful-relation distribution or establish novelty or end-to-end speed; existing complete native/compact controls remain the performance reference.
