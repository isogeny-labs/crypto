# H116 pointwise rational reconstruction v2 protocol

Protocol `H116-POINTWISE-RATIONAL-RECONSTRUCTION-002` is frozen after the
retained v1 failure and before any v2 implementation or execution.

V1 incorrectly equated `ker(V-1)` with `ker(pi-1)=E(F_p)`.  The observed data
instead match a Frobenius eigenspace: on `ker(V-1)`, the relation `V+pi=[t]`
predicts `pi=[t-1]`.  For the cyclic kernels tested here, the number visible
over degree `d` is therefore predicted to be
`gcd(N,(t-1)^d-1)`.  V2 freezes that prediction; it does not retrofit v1.
[pcc:unverified]

V2 no longer constructs a denominator only from roots visible in the current
field.  It jointly solves for a degree-at-most-`N` numerator and a monic
degree-`N-1` denominator from `2N` pointwise equations.  The first and last
canonical `2N`-sample windows are solved independently by Gaussian
elimination.  Both candidates must agree, descend coefficientwise to `F_p`,
remain identical across extension degrees, match every regular sample, and
have the predicted observed poles without cancellation.

The raw capacity result guarantees at least `2N` regular samples in every
cell.  The tight `p=5,d=2` cell again has separation margin one.  Numerator and
denominator constant mutations, plus the exact-cardinality boundary, remain
frozen controls.

This is a corrected finite-panel reconstruction.  It does not prove the
generic degree/cyclicity model, scheme-level identity, local-series transport,
or any ECDLP complexity claim. [pcc:unverified]
