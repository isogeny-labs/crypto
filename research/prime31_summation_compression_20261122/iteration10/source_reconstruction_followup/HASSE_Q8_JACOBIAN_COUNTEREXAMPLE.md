# Explicit genus-two counterexample to the inherited binary-HEC Hasse endpoint

Date: 2026-11-23. Lane: [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977).
Source: frozen candidate blob `616422524711c1dd0e54b6bb6c557348b999e262`, current inherited production blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`; reviewed parent head `f0e7a7dc32bc3a7c954e14cae63014661e4e7c91`.
Status: exact written point-count argument, not an original-Rust build or independent signed attestation. [pcc:unverified]

## Explicit curve and result

Let q=8 and consider the smooth genus-two Artin–Schreier curve over F_8

    C: y^2 + y = x^5 + x^3.

This is a permitted odd-degree binary-hyperelliptic model with h(x)=1, f(x)=x^5+x^3, deg f=5 and exactly one point at infinity. In particular the model is smooth (the affine y-partial derivative is 1), and its genus is 2 because the polynomial has odd degree 5.

**Claim (exact):** #C(F_8)=17, #C(F_64)=65, hence #Jac(C)(F_8)=169. The inherited function hasse_interval returns upper endpoint 81 at m=3,g=2, thus **an actual admissible Jacobian group order lies outside the claimed interval**. The safely rounded integer endpoint (ceil(sqrt(8))+1)^4=256 includes it.

The following supplies a finite-field proof of the two point counts without relying on a numerical solver or unseen experimental artifact.

## First extension: all eight x-values split

For any z in F_{2^k}, the equation y^2+y=z has exactly two solutions iff Tr_{2^k/2}(z)=0 and none otherwise.

For x in F_8, x^8=x; for nonzero x, exponents are reduced modulo 7. Thus

    Tr_{8/2}(x^5+x^3)
      = x^5+x^3+x^10+x^6+x^20+x^12
      = (x^5+x^5)+(x^3+x^3)+(x^6+x^6)
      = 0,

also at x=0. All 8 abscissae have two rational y-values. Including the unique point at infinity gives

    N_1 = #C(F_8) = 2*8+1 = 17.

## Second extension: a fixed-point-free trace-flipping translation

Work in F_64. Define Q(x)=Tr_{64/2}(x^5+x^3), with values in F_2. Select a nontrivial cubic root of unity w in F_4, embedded in F_64: w^2+w+1=0, so w^3=1 and w^4=w.

For any x,z in F_64, the polar bilinear form is

    Q(x+z)+Q(x)+Q(z)
      = Tr_{64/2}(x^4*z+x*z^4+x^2*z+x*z^2)
      = Tr_{64/2}(x*(z^16+z^4+z^32+z^2)).

The second equality uses Tr(t^{2^j})=Tr(t) and the Frobenius adjoints over degree 6. When z=w, w^16=w^4=w and w^32=w^2, so the coefficient in parentheses vanishes. Therefore Q(x+w)+Q(x)=Q(w).

Moreover w^5+w^3=w^2+1 belongs to F_4. Since [F_64:F_4]=3 is odd,

    Q(w)=Tr_{64/2}(w^2+1)
        =Tr_{4/2}(w^2+1)
        =(w^2+w)+Tr_{4/2}(1)=1.

Translation x -> x+w is a fixed-point-free involution of the additive F_64 set (characteristic 2). It exchanges the Q=0 and Q=1 sets. Thus precisely 32 of the 64 abscissae give solvable y^2+y=x^5+x^3, and

    N_2 = #C(F_64) = 2*32+1 = 65.

## Exact Jacobian order

For a genus-two curve over F_q, write t_1=q+1-N_1 and t_2=q^2+1-N_2 for its Frobenius power sums. Newton's identity gives e_2=(t_1^2-t_2)/2. The reciprocal Weil polynomial evaluated at 1 gives

    #Jac(C)(F_q)
      =q^2+1-(q+1)*t_1+e_2
      =(N_1^2+N_2)/2-q.

For q=8, N_1=17, N_2=65, the exact order is

    (17^2+65)/2-8 = 169 = 13^2.

The current code uses s=floor(sqrt(8))=2 and computes hi=(s+1)^(2g)=3^4=81, whereas the conservative integer replacement is (ceil(sqrt(8))+1)^4=4^4=256. This is an explicit **order-bound counterexample**, strengthening the earlier inequality-only [HASSE_INTEGER_CEILING_REVIEW.md](./HASSE_INTEGER_CEILING_REVIEW.md).

## Consequences and guarded next test

1. The old upper interval demonstrably omits a valid group cardinality for a model accepted by the source's documented algebra. This corrects the earlier note's weaker statement that no actual curve-order witness had been supplied.
2. It does **not** show that `divisor_order_bsgs` failed on an evaluated divisor: a divisor of order 13 could still have an annihilating multiple within [lo,81]. No Rust execution, original-root pass, or Prime31 relation-query improvement follows.
3. In a separately authorized root repair lane, retain the exact both-parent union, fix the rounding, and add an original-Rust regression constructing this m=3,g=2 curve. Check that the interval contains 169 and, if a trusted independent Jacobian census is available, check the order without assuming the interval. Preserve all existing tests and source-bound receipts.
4. Re-run full-root tests only under the prior original-source and admission rules. Keep the distinct Prime31 sublinear pair-spectrum query objective open. No merge or ledger promotion is justified by this note.

Proof provenance: coordinator algebraic derivation with explicit Frobenius/trace identities. No native algebra/Rust/validation workers, cargo/rustc, authenticated EKS or independently signed verifier were available. Rust-worker production edits remain deferred; validation package is the exact m=3 counterexample and original-root regression specification. This is not a formalized proof certificate. [pcc:unverified]
