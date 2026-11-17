# Q16 direct-cover isogeny frontier: frozen replay protocol

The earlier exploratory calculation identified a finite exception set. This
protocol fixes the independent, repository-retained replay before its run; it
does not claim the exploratory draw was preregistered.

Use `K=GF(2^12)` and `L=GF(2^24)` with Sage's Conway moduli, `q=16`, and the
first Sage embedding `K -> L`. Enumerate all 8,190 ordinary normalized source
models `Y^2+XY=X^3+aX^2+b`, where `a` has absolute trace 0 or 1 and `b` runs
through `K*`. Represent the two twist classes by `a=0` and `a=32` in Sage's
Conway polynomial basis. Key a model by `(Tr(a), integer_mask(b))`.

Build three direct genus-three V4 screen sets from their exact coefficient
criteria in the triangle, Hess, and nonsplit reports:

1. Triangle: `Tr(a)=Tr(b^d)` with `2(q+1)d=1 mod (q^3-1)`.
2. Hess: `Tr(a)=0` and `b=(uv)^2` for nonzero `u,v` in `ker Tr_(K/k)`.
3. Nonsplit: `Tr(a)=1` and `b=(A A^(q^3))^2` for nonzero `A` in
   `ker Tr_(L/F_(q^2))`.

Check the union count against the proven 6,158 direct-screen models. Compute
each model's elliptic order with the exact trace-of-inverse Walsh transform
from the earlier full-cohort audit. Because two elliptic curves over `K` are
`K`-isogenous exactly when their orders agree, the union's order set defines
the entire `K`-isogeny closure of **these three screens**. Retain every model
outside that closure and the complete order histogram. Independently check
one representative per outside order using Sage's elliptic cardinality routine.

For each exceptional order, record its largest prime subgroup and a
deterministically selected `G,Q=7G`; check their orders and source relation.
Compare each resulting genus-three Weil polynomial with the live LMFDB
`av_fq_isog` class and inspect prior repository work and indexed literature.
An existing Jacobian in the isogeny class plus the simple-root gate proves
existence of *some* subgroup-preserving isogeny, but does not supply its
coordinates or an attack cost. Only evaluated maps and complete same-target
GHS/rho runs can establish a new vulnerability.

After the exact-class replay, run a separate exhaustive genus probe using
Magma 2.30-1's documented `WeilDescentGenus(E,k,c)` intrinsic. For the three
fixed representative equations in `frontier_q16.json`, assert that the
Conway root `w` has multiplicative order 4,095, then enumerate
`c=w^i` for every `0 <= i < 4095`. Record the genus histogram and the
number of intrinsic rejections separately, checking that they sum to
4,095 for each curve. This is a complete parameter sweep for those three
exact representatives and that intrinsic; it is not a classification of all
possible cover maps for their isogeny classes.
