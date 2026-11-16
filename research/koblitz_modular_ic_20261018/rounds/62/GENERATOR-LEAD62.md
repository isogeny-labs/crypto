# Direct generator decomposition for larger separable sources

PROPOSED, not executed or formally proved in this note. This is a route to
the larger marked23 degree112 source and selected order991 action lane without first
inverting a full squared-basis matrix. It must be checked against the complete
435-function degree32 cohort before scaling. It does not replace that cohort.

Start with the retained R61 square-decomposition design and the actual
FIELD61 equations. In characteristic two, split the coefficients of a
separable primitive polynomial P(t) over F2(u), then group even and odd
powers of t to obtain

    P(t) = a(t)^2 + u b(t)^2 + t c(t)^2 + u t d(t)^2.
    q = P'(t) = c(t)^2 + u d(t)^2.

At a root, require q nonzero and construct its actual field inverse. Then

    t = ((a c + u b d)/q)^2 + u ((a d + b c)/q)^2.

The identity follows by multiplying the equation
t q = a^2+u b^2 by q, and checking
(a^2+u b^2)(c^2+u d^2)=(a c+u b d)^2+u(a d+b c)^2.
All signs are plus in characteristic two. This is a direct algebraic
derivation, not a novelty claim. The rational coefficient split remains
N D = E^2+u O^2, with roots E/D and O/D; see
../61/SQUARE-DECOMPOSITION61.md and the native coefficient API documented
by https://www.singular.uni-kl.de/index.php/singular.pdf.

For the actual degree32 tower, the first source equation gives
v=v^2+u(u+1)^2, hence the generator pair (v,u+1). Coefficients of the
degree16 lambda equation are linear in v. Decompose those coefficients
through this first pair, group lambda parity, and apply the formula to
lambda. Verify the entire polynomial grouping and derivative identity in
the actual field, including q times its inverse equals one. Never substitute
a scalar fiber, truncated expansion, or a guessed equation for these checks.

Pairs represent g^2+u h^2. Their arithmetic is:

    (g,h)+(G,H) = (g+G,h+H)
    (g,h)*(G,H) = (g G+u h H,g H+h G)
    inverse(g,h) = (g/f,h/f), where f=g^2+u h^2 is nonzero.

Use these operations on the canonical polynomial representation of every
input. Bind all435 inputs back to R61, compare BOTH roots to the cached
matrix method, require all435 whole-source identities, and retain exact
1/u/u^2 controls and a changed-root negative. A fresh protocol must freeze
this source and all donor bytes before execution. No timing or asymptotic
improvement is established by this proposal; source-specific expression
growth and inversion cost must be recorded at larger degrees.

Formal obligations: prove the generator formula and pair arithmetic under
explicit characteristic-two and nonzero-denominator premises. Separability,
irreducibility, source-model identification, nonsquare-u, completed normal
orders, global holomorphic regularity and target order11 action remain
their own obligations. This construction supplies a field operator to the
Cartier and supported-unit saturation lanes; it does not prove their
geometric premises.

## Concrete larger-source binding
R52/CM.md identifies the larger field as a degree7 marked23 base followed
by a monic separable degree16 lambda extension, hence total degree112.
The number991 denotes the selected factor action, not a source degree.
Use R52/source-data/marked23-field51.sing for the exact degree7 equation
and b23/c23. First apply the derivative formula to its y generator over
F2(x); then evaluate all seven base coefficient powers by pair arithmetic
and apply the same formula to the retained lambda polynomial. R51's
structured source certificates must bind that polynomial to the original
QR presentation; R52's3810 finite-fiber checks establish the stated field
argument, and must not be rerun just to build this operator. Normalization
and the selected991 action remain independent construction tasks.
