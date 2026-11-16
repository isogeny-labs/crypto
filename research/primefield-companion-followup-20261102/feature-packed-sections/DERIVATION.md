# Packed univariate quotient sections

Let r>=7 be a caller-certified prime, g(W) monic of degree d>=1 over F_r,
s nonzero in F_r, 2<=m<r, and h=(r-1)/2. Put

    Q = F_r[W,V]/(g(W), V^2-WV+1),
    P(V) = V^d g(V+V^-1),
    M(X) = s^(2d) P(s^-1 X^m).

P is monic of degree2d with constant coefficient1. Consequently V is a unit
modulo P. Substitution W=V+V^-1 induces an isomorphism F_r[V]/P -> Q:
it respects g; conversely V in Q is a unit with inverse W-V and P(V)=0.
The maps are inverse on their generators. No reducedness assumption is used.
Eliminating V via X^m=sV now identifies Q[X]/(X^m-sV) with F_r[X]/M.
M is monic of degree2md, so the latter has unique representatives below that degree.

Compute R=(X-1)^h mod M by binary powering. Split its coefficients by
residue modulo m: R=sum_e X^e S_e(X^m). Then evaluating S_e(sV) in Q
recovers exactly the e-th section returned by the original quotient-ring
powering. Uniqueness follows from the free basis1,X,...,X^(m-1) over Q.
The balanced decoder splits S=A+Z^k B and evaluates A(sV)+(sV)^k B(sV),
sharing cached powers across sections. These are ordinary polynomials;
leading zero coefficients do not change their meaning.

## Repeated factors are essential

Use the raw homogeneous lift, not the endpoint-corrected support lift.
For g=W-2, P=(V-1)^2 and M=(X^m-s)^2. Replacing this with X^m-s
would discard the nilpotent direction and need not reproduce the sections.
The checked r17,m3,s2 modulus is X^6+13X^3+4. The controls also include
(W-2)^2 and W^3. No squarefree operation occurs in this encoding.

## Capacity and costs

The existing output threshold2md is checked before lift or modulus allocation.
This is not a peak memory limit: the modulus has2md+1 coefficients; products,
remainders, the raw lift, decoder vectors, cached pairs and copies also cost
space and arithmetic. Setup includes lift construction and2d+1 scale products.
The current routine also performs the last unused scale update; it is charged.
No vector of h+1 coefficients is constructed.

There are O(log h) modular polynomial products of degree O(md). If modular
multiplication/reduction has regular cost M(md), powering costs O(M(md)log h).
Balanced decoding uses quotient multiplication, reductions and shared powers;
under regular multiplication bounds it costs O(m M(d)log d) arithmetic,
besides output/storage/copies and encoding. This conditional arithmetic bound
is not an observed timing crossover. The original implementation instead
explicitly loops over m^2 section pairs for each product. The new encoding
makes the native polynomial multiplication backend available across that axis.

## Integration and qualification

The optional UnivariateSectionImage adapter replaces only the scale*V section
calculation. Constant phase recovery still uses the original generic sections;
capacity failure still selects the existing stream fallback. Anchor, zero-mode,
canonical filtering, public source, replay and final point verification remain.
Fixture scalar enumeration is confined to the correctness oracle.

Sources checked: predecessor powered.rs, quadratic.rs, homogeneous.rs and
oriented-image.rs at754f7d221f02a9e6cec45e3e18ecb52e10eea9ea; NIST DLMF
https://dlmf.nist.gov/18.5 equations18.5.1-2 for reciprocal coordinates.
The ring elimination and multiplicity argument above are supplied explicitly;
DLMF is not cited as establishing this algorithm. KB unavailable: no MCP tool
and scripts/research-kb status reports no executable. No novelty claim.

The remaining initial phase-domain expansion grows with L. Next investigate
using oriented relative constraints before that expansion, while keeping their
construction and completeness explicit.
