# Exact simplicity criterion and genus floor for the ±9 carrier line

15 November 2026. This is a paper-level proof about algebraic transfer maps. It does not construct such a map or price a discrete-logarithm attack. [pcc:unverified]

## Theorem

Let `A0=Res_(F128/F2) E`, where `E/F128` is either exact ±9 source in `PROTOCOL.md`, and put `A_s=A0 ×_(F2) F_(2^s)` for positive `s`. Then `A_s` is simple over `F_(2^s)` **if and only if** `7∤s`. In particular, for `7∤s`, every nonzero homomorphism of abelian varieties over `F_(2^s)` from `A_s` to a curve Jacobian `J(C)` has finite kernel and requires `genus(C)≥7`. The same genus floor applies to any source elliptic curve over `F_(2^(7s))` isogenous to the selected `E_s`: its Weil restriction is isogenous to `A_s`.

This is a dimension bound for algebraic homomorphisms. It does not exclude a higher-genus GHS cover, an abstract finite-group homomorphism, or another attack mechanism. The known target `Y_s` attains genus seven at the isogeny-class level, so the bound is sharp there.

## Proof

Write `t=±9` and let `α,β` be the roots of `f(X)=X²−tX+128`. The discriminant is `t²−512=−431`, so `L=Q(α)=Q(√−431)`. Since 431 is squarefree and `−431≡1 (mod 4)`, its integers are `(u+v√−431)/2` with `u,v∈Z` of the same parity. Such an integer has norm `(u²+431v²)/4`. Norm two would require `u²+431v²=8`, which has no integer solution. Now `N_(L/Q)(α)=128=2⁷`. If `α=γ⁷` in L, γ is integral because it satisfies the monic polynomial `X⁷−α` over the integers of L, and its norm would be two, a contradiction. Thus α is not a seventh power in L.

For every positive s, `α^s` still generates L. Indeed, equality `α^s=β^s` would make `α/β` a root of unity in L. The only roots of unity in `Q(√−431)` are ±1, whereas `α≠β` (nonzero discriminant) and `α≠−β` (nonzero t). If `7∤s` and `α^s=γ⁷`, choose integers u,v with `us+7v=1`; then `α=(γ^u α^v)⁷`, contradicting the preceding paragraph. By the prime-exponent binomial irreducibility criterion (Capelli), `X⁷−α^s` is irreducible over L. Applying the composition form of that criterion to the irreducible quadratic polynomial of `α^s` shows that

`F_s(X)=X¹⁴−(α^s+β^s)X⁷+2^(7s)`

is irreducible over Q. This is the Frobenius characteristic polynomial of `A_s`: the roots of the F2 polynomial `X¹⁴−tX⁷+128`, raised to the s-th power, are the F_(2^s) eigenvalues. A proper positive-dimensional abelian subvariety over a finite field would contribute a proper factor to this characteristic polynomial. Hence `A_s` is simple.

If `7|s`, write `s=7r`. Raising the fourteen F2 Frobenius roots to the s-th power gives α^r seven times and β^r seven times. Thus the characteristic polynomial is `(X²−(α^r+β^r)X+128^r)⁷`. Also `F128⊂F_(2^s)`, so the Weil restriction splits after this base change into a product of seven conjugate elliptic curves. It is not simple. This proves the exact criterion. [pcc:unverified]

Finally, if `7∤s`, the image of a nonzero homomorphism `A_s→J(C)` is a nonzero abelian subvariety, and its connected kernel is an abelian subvariety of the simple `A_s`. The connected kernel is therefore zero; the image has dimension seven. Since `dim J(C)=genus(C)`, the genus is at least seven. Weil restriction preserves isogenies, so the same argument covers elliptic curves isogenous to `E_s` over the extension field.

## Scope and checks

- The exact ±9 q2 and q4 Weil polynomials and base-change identity are in `THEOREM.md` and the raw replay receipt. SageMath 10.10.rc0 checked the discriminant, norm obstruction, and factorization for both signs at s=1 through 10; its output is in `reports/simplicity-20261115/result.txt`. The first run failed because its validation assertion confused base change with Weil restriction at s=7; `PROTOCOL.md` records the correction and `reports/simplicity-20261115/failed-initial.txt` preserves the failure. These finite checks support the displayed algebra; the all-s theorem is the argument above.
- The irreducibility criterion is standard; a statement is in [Stevenhagen, *Algebra III*, §23](https://pub.math.leidenuniv.nl/~stevenhagenp/GaloisTheory.pdf). The finite-field simple-factor criterion is an elementary consequence of the Tate-module exact sequence; see [Oort, *Abelian varieties over finite fields*, §5](https://math.nyu.edu/~tschinke/books/finite-fields/submitted/oort.pdf).
- No Lean file in this directory formalizes the quadratic ring of integers, Capelli's criterion, or the abelian-variety dimension argument. `CoprimeBaseChangeTrace.lean` checks only finite trace arithmetic. The theorem is awaiting independent mathematical review. [pcc:unverified]

## Relevance to vulnerability bounds

For this exact infinite isogeny line, the smallest **algebraic Jacobian target** has genus exactly seven: the lower bound above and the explicit isogeny-class carrier `J(Y_s)` meet. This gives a rigorous geometric bound on the transfer target, not an upper bound on attack cost. Vulnerability still depends on an evaluable map, kernel behavior on a chosen large subgroup, relation/decomposition/descent cost, and matched rho cost.
