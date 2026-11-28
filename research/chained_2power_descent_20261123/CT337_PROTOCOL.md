# CT337 preregistered plan — bielliptic quotient pushforward on genus-two divisors

**2026-11-23 UTC.** Source-only, bounded finite-field positive control; [pcc:unverified]. This extends the already published CT336 Prime31 curve and does not claim the separately owned F3/E7 native Richelot/Cantor lane, original 695/701 norm-point reuse, or benchmark resources.

## Frozen source and problem

Base commit of this lane before this protocol: `258000400d474fd5257c9e638ca9d8792cbeb6cc`. Use p=2147483647, C: y²=∏_{i=1}^6(x-i), h=7/2, t=x-h, u=t², (a,b,c)=(1,9,25)/4. The CT336 quotient maps are Eplus:(u,y), Eminus:(u,ty). For actual elliptic addition turn Eminus into a cubic via X=1/u and Y=ty/u², giving Y²=(1-aX)(1-bX)(1-cX); u=0 is the identity on this cubic. Preserve CT336 source and its five image records unchanged.

## Mathematical deliverable

For a generic degree-two affine Mumford pair U(x)=x²-sx+z, V(x)=Ax+v0 with U | (V²-f), put B=2h-s, C0=h²-hs+z, D=Ah+v0, S=B²-2C0. When B*C0≠0 and the image support is separable, derive explicit output quadratic support and ordinate-polynomial records on the two cubic quotients:

- Rplus(Z)=Z²-S Z+C0²; Vplus(Z)=-(A/B)Z + D-A*C0/B.
- Rminus(X)=X²-(S/C0²)X+1/C0²; Vminus(X)=D/(B*C0)+[A-D/B-D*S/(B*C0)]X.

Check Rplus | (Vplus²-Eplus) and Rminus | (Vminus²-Eminus), in addition to U | V²-f, by polynomial long division without using a root factoring oracle. These are **effective finite-divisor image records**, not fully reduced divisor classes, and normalization/subtraction of the domain base divisor and elliptic group addition must be accounted for separately.

Construct a deterministic public panel: the first 36 rational source points from x=1,2,..., y the canonical square root modulo p (p≡3 mod 4), including six branch points; all admissible distinct-point pairs; and the first 20 irreducible supports U=x²-d (d scanned upward from 2, nonsquare) for which an Fp² ordinate sqrt exists. Independently derive the extension field y=c+e*theta, theta²=d, by norm and base-field modular square roots. Compare the generic output polynomials to a separately written Python reference.

## Group structure and exceptional controls

For each rational source point P with sigma(t,y)=(-t,y) and rho(t,y)=(-t,-y), use actual cubic EC additions to check that:
- pi_plus(P)+pi_plus(sigma P)=2pi_plus(P), pi_minus(P)+pi_minus(sigma P)=O;
- pi_plus(P)+pi_plus(rho P)=O, pi_minus(P)+pi_minus(rho P)=2pi_minus(P).
Treat these as degree-two fiber pushforward controls; do not claim a native Cantor test or full Jacobian composition.

The three pairs W1+W6, W2+W5, W3+W4 above the branch roots must all map to zero after subtracting the two infinity points. They give explicit candidates for the order-four product-isogeny kernel; retain proof obligations about distinctness and the isogeny degree. Exercise both exception boundaries: B=0 (colliding quotient x-support), and C0=0 (map to cubic identity for u=0). Reject deliberately mutated signs and quotient polynomial coefficients. A zero, missing, ineffective or vacuous negative control is a failure.

## Execution bounds, evidence and gates

One coordinator. No native parallel-agent slots or Rust compiler were available at planning time; no EKS, paid jobs, CI rerun, benchmark timing, locked S3/vault retrieval or norm branch edit is authorized by this protocol. The earlier local Python reconnaissance was exploratory only, and must NOT be represented as a preregistered frozen run. Freeze an exact standalone JS source blob and raw JSON on this PR, execute it once, then check the frozen outputs through separate Python arithmetic in this container. Record exact executed commands, branch head, source hash, results, controls, no-signer status and remaining dependencies. A second program executed by the same coordinator is **not** a signed independent PCC verdict. Preserve branch draft/stacking and all review/check budgets.

The mathematical literature and repository sources already examined: CT335/CT336 reports; Shaska, *Genus two curves covering elliptic curves* (arXiv:1209.3187); Bruin/Flynn/Testa on (3,3)-isogeny; explicit curve/quotient literature. No novelty assertion.

**Next distinct gate:** native source-bound Cantor/Jacobian equality, ramified and projective infinity cases, dual composition and independent validated public toy logs on an unowned approved worktree. The original owner-reserved complete cold comparison remains separate and higher priority.
