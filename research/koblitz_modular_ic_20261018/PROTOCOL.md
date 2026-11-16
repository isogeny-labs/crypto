# Koblitz modular carriers: bounded structural investigation

Frozen before the native run on 2026-10-18. Base revision: 4fdf4f39d.
Request: investigate the Klein-quartic and modular-twist lead for index calculus.

## Inputs and hypotheses

Source family: E_a: y^2+xy=x^3+a*x^2+1 over F_2, a=0,1.
Primary target: n=131, a=0, ICV1 slug
`icv1-f2m131-tm22283658519494248867-115e0dc5`, field modulus
`0x800000000000000000000000000002007`, prime subgroup
`r=680564733841876926932320129493409985129`. No challenge point or
unknown scalar is used. EC1 generator binding remains unresolved in the registry.

H1: twisting conductor-49 f by a character of order 2n, whose value at 2
has order 2n, realizes the a=0 trace-zero polynomial without the separate
conductor-3 sign twist. Its square has order n, so the character-kernel
modular curve still has degree n over X_0.

H2: the existing modular construction can be materially reduced by searching
prime conductors, while retaining the exact Frobenius target.

## Bounded native experiment

Use Rust, exact integers, and no timing comparison. For n=3,5,7,11,13,41,
53,61,83,131 enumerate odd primes ell <= 10,000, ell != 7, with
n | ell-1. Define chi(g)=exp(2*pi*i/m), m=n or 2n, with g the least
primitive root. Record log_g(2), order chi(2), parity, and character order.
Accept only full order at 2. For each accepted conductor compute the genus
of Gamma_H(49 ell^2), H=ker(chi^2), including elliptic-point terms.
Compare the smallest signed result with the historical a=0 recipe using
the first accepted order-n character and the additional level factor 9.
The ell=7 overlap is treated only as the existing Klein positive control,
not by the coprime-level formula.

Independently compare the trace-zero polynomial from division of
X^(2n)-s_n X^n+2^n by X^2-tX+2 with the coefficientwise quadratic norm of
sum_{j=0}^{n-1} alpha^j X^(n-1-j). Do both signs at every listed n.
Check the Klein counts through F_(2^8). Check composite n=9 to distinguish
primitive and full trace-zero components. Validate generalized character
kernels by complete coset enumeration at small levels; do not enumerate
the enormous full-size modular curves.

Success is an exact smaller carrier construction, or a checked reason why
the new variant fails. This is a structural result, not a measured IC gain.
An executable IC advance additionally requires curve equations, maps that
preserve r-torsion, relation generation, descent, and a complete comparison
against the current IC and signed-Frobenius rho baselines. Those costs are
unknown here. The old genus 130--300 window is a historical smoothness model,
not a validated destination solver or a sufficient attack certificate.

Stop after the finite conductor scan and exact checks. No cloud instance,
GPU job, external target, broad curve enumeration, or key-recovery run.
Retain failures and outputs; do not rewrite historical evidence. Produce
source-linked report, diagram, PDF, proof map, obstruction record, and PR.

## Amendment 1: cyclic-cover window, before its native check

The modular scan is complete. Extend the existing degree-131 cyclic-cover
argument from quotient genus <=1 to every quotient, retaining the hypothesis
that the geometrically connected curve and covering map descend to F_2.
Test the derived lower bound g>=394 using Riemann--Hurwitz,
dim(Prym)>=130d, d|130, and absence of rational 131-torsion in
dimension <=3 over F_2 and F_4. The latter follows from #B(F_2)<=5^dim(B)
and #B(F_4)=#B(F_2)*#B^-(F_2). Enumerate all integer parameter tuples
g<=393, b>=0, d|130, R>=0 satisfying the necessary constraints. This
finite check validates the arithmetic implication only; the report must
give the geometric/class-field-theory argument and expose its hypotheses.
Preserve the modular run unchanged, save the new check separately, and
attempt a conditional Lean proof of the numerical implication.
