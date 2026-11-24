# Written proof chain; no new Lean endpoint

1. Structures: smooth projective geometrically integral curve/function
   field, divisors H,T,D,A; ordinary function-field polynomial; total local
   order in the regular surface E x A^1.
2. Existing laws used exactly: principal divisor degree0, L(B) definition,
   nonarchimedean valuation, local regular parameters and their graded
   polynomial domain. The short weighted-Gauss cancellation proof is
   included in DERIVATION.md, not asserted as a checked Mathlib endpoint.
3. Instance obligations: H=5O+R, xy pole5O, phi_R poleO+R, R finite/s!=0;
   complete message spaces from genus1 pole orders. No native compiler,
   [8]-fibre or physical-base enumeration is formalized.
4. Discriminator: repeated graph division changes (A,s) to (A-H,s-1).
   The inequality deg(B)+l<=36u/5<8u forces each factor, not just the
   first. At d<6s there is no polynomial; d=6s is universal only.
5. Missing bridges: no independent/formal verifier of this written theorem;
   original exceptional charts and nonuniform multiplicities outside scope.
6. Product successor: local order of a product of nonconstant affine forms
   equals the count of zero factors. Hasse jets give252 equations in five
   unknowns, but raw alias/jet semantics, a compact exact product circuit
   and its coefficient solver/cost remain separate obligations.

No Lean source is edited, no source-only theorem is reported as compiled,
and no generic law is copied into a study namespace. A future formalization
must follow the owning-library/instance gate before source changes.
[pcc:unverified]
