# R78 actual full-source u=0 fiber, frozen before native execution

Bind the R77 quadratic infinity coordinate `U=t^2 V` to the R50
finite-`u` chart on the overlap `t=u=1/z`, `V=z^2 v`; hence `U=v`.
Load the retained R50 complete degree-16 relative equation over
`F2[u,v]/(v^2+v+u^3+u)`. Specialize **every term** at `u=0` and check
the exact polynomial `lambda^16+(v+1)lambda^8+v+1`. Split by `v=0,1`
and check `F_0=(lambda^2+lambda+1)^8`, `F_1=lambda^16`.
Require total fiber dimension 32, each quadratic-component dimension
16, the expected reduced residue degrees 2 and 1, and a wrong-factor
negative control. These are source-model special-fiber facts. The
model is not yet certified normal at `u=0`, so repeated exponents are
not claimed as ramification indices or a complete branch split.

One deterministic exact local Singular run; seed 78, one
Singular/FLINT thread, 1200-second timeout and 4 GiB virtual memory.
Copy and verify source/protocol/driver bytes from committed Git and
all donor hashes. Accept only exit 0, exact marker and zero native
whitespace-question-mark errors. Retain failures and freeze repairs
before a fresh ID. No timing comparison or statistical claim.
