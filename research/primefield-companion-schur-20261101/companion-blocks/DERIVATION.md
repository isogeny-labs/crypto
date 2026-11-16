# General companion block interfaces

The prior REQUIREMENTS derivation is implemented here, with dimension n
independent of the coefficient prime r and canonical distinct wraps a,b.
It uses (a-b)A=sum L^(n-1-j) GK^t R^j, L^n=aI,R^n=bI.
Zero wraps are permitted when distinct. No Frobenius identity, roots of
unity, or assumption n<r is used.

Each direction uses two ordinary packed products per generator pair.
For n-dimensional vectors and width s, a single application costs:

    field products = 2s(n-1)+n
    field additions = s(3n-2)+1
    field inversions = 1
    native integer products = 2s; input encodings = 4s
    packing bytes = 4snd
    product bytes = decoded bytes = 2s(2n-1)d
    coefficient reductions = 2s(2n-1)

Here d=ceil(bitlength(n(r-1)^2)/8). Both zero-wrap multiplication and
zero coefficients remain charged; no special-case operation skipping.
Native operand bit products are separate from these scalar counts.

`principal` appends four boundary outer products to the restricted G,K.
Its own correction work is n field products and n+1 additions. Required
public boundary vectors are obtained by four requests to BoundaryCache:
last full row, first crossing column, last block row, first full column.
Each cache miss executes a full operator query; hits, requests and retained
field slots are reported. No dense matrix is accepted by this constructor.
For the four cuts {1,2,floor(m/2),m-1} and four wrap choices, there are64
requests and10distinct vectors per source context (m>=7), hence54hits.
All10 query costs are charged once; changing new wraps does not invalidate
the cache of the fixed full source operator. Cache slots are not process RSS.

If A is invertible, inverse generators for swapped wraps b,a are
U=-A^-1G,V=A^-tK. `inverse_generators` implements this transport through
supplied oracles, charging s forward and s transpose calls and ns negations.
The validation supplies explicit dense reference inverses; this is a test
of the generator identity, not an efficient inverse synthesis or solve.
Singular oracle failures must propagate and remain recorded.

Source of the formulas: ../companion-matvec/RECURSIVE_REQUIREMENTS.md.
Primary background: Bostan et al. (2017), companion/Krylov multiplication
and structured inversion, https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf.
Finite validation and execution status are recorded separately in
../companion-blocks-validation/. No experiment has run at this initial snapshot.
