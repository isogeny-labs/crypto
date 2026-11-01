from pathlib import Path
import hashlib,json
D=Path(__file__).parent;h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();gate=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs');field=Path('/home/ec2-user/crypto-pr2304-persist/src/cryptanalysis/three_leads.rs');deps=[Path('/var/tmp/pencil-add11-distinct-independent-review-20261031/replay/Add11.lean'),Path('/var/tmp/independent-same-support-replay-ow8hrtl9/SameSupport.lean')]
(D/'sources.json').write_text(json.dumps({'rust':{str(p):h(p) for p in [gate,field]},'lean_dependencies':{str(p):h(p) for p in deps},'source_lines':{'gate_add11':'511-532','degree_one_normalization':'541-551','degree_two_normalization':'552-567'},'literature_scope':'Prior source-bound Ozbay Gurler–Hisil sections2–4 audits and reviewed Add11 interpolation proof; no novelty claim'},indent=2)+'\n')
r={'schema':'research.formal-component-result/v1','claim_scope':'weighted nonzero-input distinct add11 field-value normalized validity','source_sha256':h(D/'WeightedAdd11.lean'),'declarations':['cross_difference','normalized_distinct_cross','normalized_coefficients','preserves_mumford','distinct_guard'],'compile_stages':{'attempt1':1,'attempt2':0,'attempt3':0,'controls1':0,'readback1':0},'axioms':['propext','Classical.choice','Quot.sound'],'independent_review':'pending','hypotheses':['arbitrary field K','all four input weights nonzero','normalized affine x coordinates distinct','each normalized degree-one support divides common f minus y squared'],'conclusions':['literal field output weights z,w nonzero','four normalized output coefficients equal affine interpolation coefficients','normalized monic output support divides f minus normalized v squared','distinct normalized x implies unequal source cross coordinates, excluding inverse/doubling guards'],'machine_boundary':{'required_external_conditions':['odd prime5<=p<2^31','all eight input words canonical (<p)','Field.n=1','tables=None','normal execution and sufficient counter headroom'],'sufficient_primitive_overflow_bounds':'canonical modular operands multiply below2^62; addition/subtraction intermediates below2^32; these arithmetic bounds are not the completed Add11 word-refinement theorem','not_proved':['actual UInt64 expression/pow correspondence','counter or compiler semantics','Rust Jacobian::valid refinement','divisor-class addition semantics','nonidentity dispatcher or whole-walk closure']},'controls':['full nonunit-weight rational instance with nonzero interpolation slope','unit-weight specialization','zero input z preserves zero output z','different raw x values can have equal normalized x and zero output w'],'failures_preserved':['attempt1 used reserved Lean keyword by as a local name; parse failure and resulting dependent errors/sorry diagnostics retained; final successful readback contains no sorryAx'],'scientific_execution':False,'cost_ratio':None}
(D/'status.json').write_text(json.dumps(r,indent=2)+'\n')
(D/'REPORT.md').write_text('''# Weighted distinct add11: normalized field validity

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
''')
(D/'overview.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="390"><rect width="1100" height="390" fill="white"/><g font-family="sans-serif" fill="#17334d"><text x="25" y="35" font-size="23">Weighted distinct add11: field-value validity</text><text x="25" y="83" font-size="18">Nonzero z/w weights + distinct normalized x + degree-one input divisibility</text><text x="25" y="130" font-size="18">→ A-B=(xa-xb)*(az²*bz²) ≠ 0 → source distinct guard</text><text x="25" y="177" font-size="18">→ exact four normalized output coefficients = affine interpolation coefficients</text><text x="25" y="224" font-size="18">→ reviewed interpolation theorem → normalized output Mumford validity</text><text x="25" y="275" font-size="17">Five declarations; full nonunit/nonzero-slope control; separate standard-axiom readback</text><text x="25" y="320" font-size="16">Next bridge: actual UInt64 pow5 + full arithmetic/guard tree under canonical p&lt;2³¹ bounds</text><text x="25" y="364" font-size="15">Producer packet pending independent review. No counter, class-sum or whole-walk claim.</text></g></svg>''')
(D/'report.tex').write_text(r'''\documentclass[11pt]{article}
\usepackage[margin=0.78in]{geometry}
\usepackage[T1]{fontenc}
\begin{document}
\section*{Weighted distinct add11 validity}
31 October 2026 UTC. Producer field-value proof, pending independent source-fidelity review.

Five Lean declarations compile and pass separate axiom readback. With four nonzero input weights, distinct normalized affine x coordinates and two degree-one Mumford-valid inputs, the weighted source formula produces nonzero output weights and normalized output validity.

\textbf{Literal field expressions.} Put $x_a=a_x/a_z^2$, $y_a=a_y/(a_z^5a_w)$ and likewise for b. Source cross values are
\[ A=a_xb_z^2,\quad B=b_xa_z^2,\quad Y=a_y(b_z^5b_w),\quad Z=b_y(a_z^5a_w). \]
The output is
\[q=-(A+B),\ r=AB,\ s=Y-Z,\ t=AZ-BY,\ z=a_zb_z,\ w=(A-B)(a_wb_w).\]
The proof derives $A-B=(x_a-x_b)a_z^2b_z^2\ne0$, then
\[q/z^2=-(x_a+x_b),\qquad r/z^4=x_ax_b,\]
\[s/(z^3w)=(y_a-y_b)/(x_a-x_b),\]
\[t/(z^5w)=(x_ay_b-x_by_a)/(x_a-x_b).\]
These identities are conclusions, not premises. The reviewed interpolation/Bezout theorem then yields quadratic support divisibility of $f-v^2$. Cross inequality also excludes inverse and doubling guards in their actual source order.

\textbf{Controls.} Over Q, $a=(0,0,2,3)$ and $b=(9,486,3,2)$ normalize to $(0,0),(1,1)$ on $f=X^5-X+X^2$. The full theorem applies after explicit input divisibility proofs. Checks give $z=6,w=-216,q/z^2=-1$ and slope1. Additional controls cover unit weights, zero-z failure and different raw x values encoding equal normalized x.

\textbf{Receipts and failures.} Initial source used reserved keyword ``by'' as a local name and failed; its diagnostics remain. Corrected coefficient and full-theorem stages, controls, and separate readback exit0. Final axioms are only propext, Classical.choice and Quot.sound; unused-tactic warnings remain recorded. Existing pinned dependencies were used, with120-second per-command limits.

\textbf{Next machine obligation.} The intended native mapping requires odd prime $5\le p<2^{31}$, eight canonical input words, Field.n=1, tables=None and normal execution/counter headroom. Canonical primitive products are below $2^{62}$ and add/sub intermediates below $2^{32}$. The actual UInt64 pow5 and complete expression/guard correspondence is not proved here. Neither are native validity refinement, counters/compiler behavior, divisor-class addition or whole-walk closure.

Source hashes, exact hypotheses and receipts: sources.json, status.json, PLAN.md, attempts and readback1/. No scientific run, root edit or cost measurement.
\end{document}
''')
