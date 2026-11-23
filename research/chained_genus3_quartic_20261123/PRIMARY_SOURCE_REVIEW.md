# CM19 source review and separately motivated F7 successor

Read before the first new scientific native call. The fixed250-equation F5
family stays exactly as published. These are written derivations and primary
source leads; no new counts, polarized-module computation or Jacobian map has
run. All claims unsigned. [pcc:unverified]

Sources checked (KB): KB/Conductor unavailable, vault-entering wrapper not
invoked. Exact repository successor af2a50 and primary sources below were
reviewed. No novelty clearance or literature-absence claim. [pcc:unverified]

1. Alekseenko, Aleshnikov, Markin and Zaytsev,
   [Optimal curves over finite fields with discriminant -19](https://arxiv.org/pdf/0902.1901),
   v2, especially Sections2,4,5. Its finite-field discriminant means
   floor(2 sqrt(q))^2-4q. At q=5 that is-4, while the selected elliptic
   curve's endomorphism-order discriminant is-19. The optimal-curve theorem
   is not a direct existence theorem for our F5 source.
2. Ritzenthaler, [Explicit computations of Serre's obstruction for genus3
   curves and application to optimal curves](https://doi.org/10.1112/S1461157009000576),
   LMS J. Comput. Math.13(2010),192-207, Theorem1.1, Section2 and Example1,
   pp204-205. A geometrically Jacobian PPAV can have a quadratic descent
   obstruction; an abstract polarized E^3 is not automatically the base-field
   Jacobian. An actual smooth plane quartic avoids declaring this bridge.
   Example1 supplies an explicit CM19 quartic and a Q-descended model.
   [pcc:unverified]

The Example1 elliptic model is y^2=x^3-152x-722. Its Q-descended quartic is

```text
F(U,V)=U^4+(1/9)V^4+(2/3)U^2 V^2-190V^2-570U^2
       +(152/9)V^3-152U^2 V-1083.
```

The primary discriminant of -F/2 is2^19*19^7. Its geometric/CM Jacobian
relation must still have its reduction and base-field twist checked. Mod5,
the elliptic model becomes y^2=x^3+3x+3, the nonsquare2 twist of our
y^2=x^3+2x+1. This difference motivates retaining an empty F5 outcome;
it is not a proof that the prescribed F5 family is empty. [pcc:unverified]

A separately motivated single F7 follow-up, to freeze only after recording
the original family outcome, is obtained as follows:

```text
F mod7=U^4+4V^4+3U^2V^2+6V^3+2U^2V+6V^2+4U^2+2
       =(U^2+5V^2+V+2)^2-(4V^3+V^2+4V+2).
X=V+3, Z=2U.
C7: (Z^2+6X^2+3XY+Y^2)^2=Y(X^3+2XY^2+6Y^3).
```

The last homogeneous polynomial equals2F after substitution. A later
separate native gate should check this coefficient identity, smoothness,
actual elliptic order/trace and direct/fibre counts over F7,F49,F343.
For E7:y^2=x^3+2x+6 the prospective trace is-3 and order11;
PE7^3 has ascending coefficients[343,441,336,153,48,9,1], hence
prospective C7 counts17,65,236 and #J=1331. These expectations are
acceptance controls, not executed results. Do not label a point-count
isogeny class as an actual polarized E7^3 identification. [pcc:unverified]

After a genuine source gate, possible mixed-kernel work still needs actual
polarized arithmetic and a target-Jacobian/descent bridge. Exact CM19
Hermitian-form arithmetic could supply a separately frozen local-module
lead, but it would not by itself implement a map of the actual Jacobian.
No computational advantage follows from any small finite example.
[pcc:unverified]
