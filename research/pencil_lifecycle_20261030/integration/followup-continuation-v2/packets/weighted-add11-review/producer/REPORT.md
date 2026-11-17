# Weighted distinct add11: normalized field validity

Five Lean declarations now compile and pass separate axiom readback. For nonzero input weights and distinct normalized affine x coordinates, the actual weighted add11 field formula returns nonzero weights and preserves normalized Mumford validity. This extends the reviewed unit-weight result to arbitrary nonzero weights at field-value scope.

## Source expression and normalization

For weighted a,b define xa=a.x/a.z² and ya=a.y/(a.z⁵*a.w), likewise xb,yb. Rust cross values are A=a.x*b.z², B=b.x*a.z², Y=a.y*(b.z⁵*b.w), Z=b.y*(a.z⁵*a.w). The literal field output is q=-(A+B),r=A*B,s=Y-Z,t=A*Z-B*Y,z=a.z*b.z,w=(A-B)*(a.w*b.w). The Lean definition preserves this association (squares as self-products; field powers for pow5).

The proof derives A-B=(xa-xb)*(a.z²*b.z²), so distinct normalized x plus nonzero z implies the actual cross-coordinate guard is distinct. All four output normalization identities are derived by field algebra, not assumed: q/z²=-(xa+xb), r/z⁴=xa*xb, s/(z³*w)=(ya-yb)/(xa-xb), t/(z⁵*w)=(xa*yb-xb*ya)/(xa-xb). The existing independently reviewed interpolation/Bézout theorem supplies divisibility of common f-v² by the normalized monic quadratic. Output z,w nonzero follow from explicit input hypotheses. Source inverse guard precedes doubling; cross inequality excludes both.

## Controls and native receipts

A full nonunit-weight fixture over Q uses a=(0,0,2,3), b=(9,486,3,2), normalized points(0,0),(1,1), and f=X⁵-X+X². Both input divisibilities are constructed, and the full theorem applies. Output z=6,w=-216,q/z²=-1 and slope s/(z³*w)=1 are checked. Additional controls cover unit weights, zero input z producing zero output z, and different raw x=4,9 with weights z=2,3 encoding the same normalized x and zero output w.

The first attempt failed because a local variable used the reserved Lean keyword by; original source/errors are retained. Corrected coefficient proof and full theorem stages both exit0. Controls and separate readback exit0. Only propext, Classical.choice and Quot.sound occur in final axiom readback; no sorryAx. Harmless unused/unreachable ring-tactic warnings are retained. Every compile is bounded120seconds with existing pinned dependencies, no installation.

## Machine and group-law boundaries

For the intended native odd-prime path, require5<=p<2³¹, all eight words canonical, Field.n=1,tables=None, nonzero weights, normal execution and counter headroom. Canonical modular operands yield products<2⁶² and addition/subtraction intermediates<2³², providing the primitive no-overflow envelope. This packet does not yet compose the actual UInt64 pow5 and full expression/guard tree with field decoding; that is the next machine-refinement obligation. It also does not prove counter/compiler semantics, the Rust Jacobian::valid bridge, class-sum equality or whole-walk closure. The polynomial f is arbitrary; Jacobian smoothness/degree/characteristic assumptions belong to a later geometric interpretation.

Sources: exact retained gate add11 and normalization code, Field implementation, prior reviewed Add11/interpolation packet and source-bound complete-law literature audits (Ozbay Gurler–Hisil sections2–4). No new group-law or novelty claim. Source hashes are in sources.json. This is producer evidence pending independent source-fidelity review, with no root edits, scientific runs or cost measurements.
