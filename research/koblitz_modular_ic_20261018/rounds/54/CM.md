# R54: actual normal-RR fiber, binary closed points and restricted obstruction

The supersingular QR fiber is now identified directly with the actual normal
RR algebra. The four closed source points have explicit binary degree-20
residue fields and complete function evaluators. Their four-step orbit traces
are annihilated by the sign-minus projection; this family does not add
directions through that projection.

## Direct source-bound identification

Reuse the complete 32-generator normal algebra and all 528 multiplication
columns. Before specialization, the original global PID products bind the
source embedding and actual descended nu. At
\(a^5+a^2+1=0\), use the actual base value
\(V_0=(1+a^4+a)/(a+a^2)\) and \(b=1+1/a\). The source multiplication
matrices satisfy the unit, base and commutator products. The idempotent is
\[
 E=(M_V+(V_0+a^2)I)/a^2.
\]
Its rank is 16. All 16 columns \(\lambda^i\nu^j E(1)\), for
\(0\le i,j<4\), have rank 16 and lie in its image. A full inverse gives
\[
 KF=E,\qquad FK=I_{16}.
\]
Both complete multiplication intertwining products hold, and both descended
QR equations vanish as full matrices on the summand. Altering a selected
basis entry breaks the inverse product. These checks identify the actual
normal-RR fiber, rather than only reproducing a scalar presentation.

The source-bound border equations simplify to
\[
 \lambda^4+b\lambda+b^2=0,\qquad
 \nu^4+b^3\nu+b^4(\lambda+1)=0.
\]
Both full ideal products also identify these equations with R53's retained
descended presentation. All matrices and source witnesses are serialized.

## A primitive coordinate and four degree-20 source points

Set \(w=\nu/b\). The full forward/backward ideal products give
\[
 \lambda=w^4+w+1,\qquad
 P(w)=w^{16}+(1+b)w^4+bw+(1+b+b^2)=0.
\]
The derivative is the nonzero constant b. P factors into four irreducible
quartics over F32. Independent Frobenius/gcd tests verify each factor; their
full product is checked. All five coefficient-Frobenius conjugates give four
distinct irreducible degree-20 norm polynomials over F2. Coupled (a,w)
prime data is retained even though no projected norm collision occurs here.

For each prime, the evaluator gives all 32 normal-RR basis values. Its unit,
lambda, nu and V labels are checked, as are all 528 table products. The 2,112
products have full factor-times-quotient witnesses. A changed point value
fails the unit control. These are source-bound closed points, independent of
constructing classes as forward images; a Jacobian reverse map is separate.

The fresh flat-field stage independently replays all 2,112 serialized products
and verifies both levels of irreducibility. Each coupled ideal
\((a^5+a^2+1,g_i(a,w))\) has an explicit two-way ideal certificate with
\((N_i(w),a+\rho_i(w))\). The recovery polynomial degrees are 16,16,18,18.
All 128 RR values become polynomials of degree below 20 in w over F2, with
full source conversion products and changed-value controls. This provides
four concrete binary-field evaluator inputs rather than only tower fields.

## Exact restricted Frobenius/deck obstruction

On this 16-dimensional F32 fiber algebra, the complete matrices give
\[
 T=\mathrm{Frob}_{32},\quad T^2=S,\quad T^4=I,\qquad S(w)=w+1.
\]
The orbit-trace operator \(U=I+T+T^2+T^3\) has rank 4. The sign difference
\(I-S\) has rank 8, and its complete product with U is zero. A changed trace
is rejected. Thus these closed-point orbit traces are sign-invariant, and
their sign-minus projected columns vanish. This is a finite-family result;
it is not a claim about the global collector rank or the selected order-11
Jacobian action. Any use on a particular target factor needs that action's
actual sign and Frobenius premises.

## Formal and workflow boundaries

Five authored Lean claims cover basis injectivity, recovery on the projector
component, exact range identification, orbit-trace invariance, and the
conditional anti-trace identity. Compilation and the separate axiom audit
exit 0. Lean generated `orbitTrace.eq_1`, so six declarations were audited,
all with zero failing and zero trusted. The protocol's five claims are all
present; the extra declaration is a definition equation, not a sixth research
claim. Arrays, field arithmetic, polynomial presentations and the retained
normal-RR/function-field bridge remain external premises.

All five workflows are terminal: four accepted and one rejected. The first
primitive attempt used the reserved Singular identifier `degree`; point
checks were skipped and its late marker is rejected. The corrected attempt
changes only local names and completes every check. Native error logs and
all partial outputs are retained. Immutable copied drivers prevent live Bash
source edits from changing an invocation.

## Sources and next construction

Reviewed the retained R50/R51 normal RR algebra, R52's complete nu action,
R53's full versus compact fibers, native linalg.lib constant-rank routines,
and the standard [trace criterion](https://stacks.math.columbia.edu/tag/0BJF).
Independent irreducibility uses the [Frobenius/gcd criterion](https://fredrikj.net/fdoc/html/algorithms/polynomials/irreducibility.html);
no complexity or performance claim from that documentation is used.

Global F2 target identities, the actual level-11 factor map and intended
order-11 action remain construction tasks. The new finite-fiber results do
not replace them. NEXT.md preserves these tasks together with remaining
completed-local-ring and cusp geometry, polarization, full quotient saturation, the larger degree-112
source/991 action and arbitrary reverse-divisor classes.
