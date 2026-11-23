# Sign-preserving two-primary projection

Derived construction, 2026-11-07. Domain: caller-certified odd prime p>3,
nonzero field operands, and the inherited conditional normalized transport law.
This is a multiplicative value projection inside F_p, not a field homomorphism
or a characteristic-changing map. No elliptic-curve logarithm table is built.

Write p-1=q*m with q=2^v2(p-1) and m odd. Define pi(z)=z^m.
Cyclicity of F_p^* implies that pi has image order q and kernel order m.
It is multiplicative and pi(-1)=-1. Hence if a normalized source ratio is already
known to be +1 or -1, projection preserves its sign. [pcc:unverified]

This image is minimal among multiplicative homomorphic quotients preserving
-1: a cyclic quotient that retains the unique involution must have odd kernel,
so its image order is divisible by q. Equivalently, a power map z -> z^d
preserves -1 exactly when d is odd, in which case
(p-1)/gcd(d,p-1) is divisible by q. This does not claim minimality among arbitrary
non-homomorphic encodings. If m=1, the projection discards nothing. [pcc:unverified]

## Transport and monomial simplification

From W_next*W_previous = D*A^2*W_current^2, multiplicativity gives
U_next = pi(D)*pi(A)^2*U_current^2/U_previous with U=pi(W).
Inverse in the image is exponent q-1; exponents reduce modulo q.
All zero/anchor handling remains external and unchanged. This primitive does
not yet implement the elliptic source cursor. [pcc:unverified]

Use the inherited partition h=(r-1)/2=2wk+2t+e (t denotes the tail half-length;
it was named s in the predecessor), with e in {0,1}, and
G(n)=(n-1)n(n+1)/3. G(n) is an even integer: the numerator is divisible by six.
The monomial powers are

    delta = 2w[kw(w+1)+wk(k-1)] + 2kG(w) + 2G(t)
    alpha = 4wk + 2t + e
    beta  = 2wG(k)
    gamma = 2t.

Both summands inside the brackets are even; G(w), G(t), G(k) are even.
Consequently delta and beta are divisible by four, and gamma is even.
With T_index the inherited normalized division value: [pcc:unverified]

- q=2: pi(W)=pi(T_(r+1)/2)^e. If e=0, this monomial image is identically1.
- q=4: pi(W)=pi(T_(r+1)/2)^(2t+e mod4) * pi(T_(t+e))^(2t mod4).
  Omit the tail factor when t=0. Neither the seed c nor T_(2w) is needed
  for this projected monomial.
- Larger q: reduce the same powers modulo q; no uniform small-image claim.

These are symbolic eliminations in a prospective projected source, not measured
runtime savings. The current primitive retains residues modulo p, so it has not
implemented a bit-packed image representation. Projection exponentiation and
geometry-residual work must be included when evaluating its cost.

## Validity and addition boundaries

Projection cannot replace the existing rejection of non-sign ratios. In F7,
2 is neither +1 nor -1 but pi(2)=2^3=1. The API deliberately returns a
`sign_candidate`; a consumer needs either the original full-ratio check or a
proof that the source always returns a sign on its accepted domain. [pcc:unverified]

Quadratic characters alone fail for p=1 mod4: in F13, (-1)^6=1, whereas
pi(-1)=(-1)^3=-1. The two-primary image retains the missing sign. [pcc:unverified]

Nor can pi be pushed through arbitrary additions in polynomial/resultant
computation: in F7, pi(1+1)=1 but pi(1)+pi(1)=2. A projected monomial still
needs the geometry residual's projected value; obtaining it without building the
full residual is a separate construction. [pcc:unverified]

## Evidence and next construction

lib.rs implements nonzero projection, image products/powers/quotients and the
transport step, with cross-field operand rejection. The native development
suite checks multiplicativity and recurrence commutation for all nonzero input
pairs over p=5,7,13,17,509 using deterministic delta/anchor choices; this is not
an exhaustive four-variable recurrence check. It checks power-map image-order
minimality for exponents1..p-1, signs, invalid domains and the validity collision.
Five methods pass. Commands/compiler/hash/exit records are in receipt.json and
raw test.log. These are producer tests, not independent review. [pcc:unverified]

Next: implement a projected monomial cursor for q=2 and q=4 alongside the
full-value source, compare signs while retaining full validity checks, and
account for projection work. Then investigate a projected geometry residual.
No scalar recovery or complete-cost comparison was run in this component;
the original4563-case cohort remains unchanged.

Sources checked: predecessor monomial and ratio-sign derivations/source;
Andrew Sutherland, MIT18.782 Lecture3, sections3.1-3.2,
https://math.mit.edu/classes/18.782/2013fa/LectureNotes3.pdf (read2026-11-07),
for Euler's criterion and cyclicity. The projection follows standard cyclic-group
algebra; no novelty-priority claim. KB unavailable in this runtime (no MCP tools;
repository wrapper previously reported no executable). The prior-art search was
limited to these relevant local constructions and the primary lecture notes.
