# Algebraic transfer genus floor over a prime-degree extension

Status: a derived specialization of [Diem, Theorem 3](https://www.esaga.uni-due.de/f/ulrich.goertz/iem-archiv/dissertation_diem.pdf), with the elementary trace criterion written out here. It has not been independently formalized in Lean or given a PCC signed verdict. [pcc:unverified]

Let `k=F_q`, `K=F_(q^ell)` for an odd prime `ell`, and E/K ordinary with trace `t=q^ell+1-#E(K)`. Set `A=Res_(K/k)E`. Its Frobenius polynomial is

    F_A(X)=X^(2ell)-t X^ell+q^ell.

Define `S_0(u)=2`, `S_1(u)=u`, and `S_m(u)=u S_(m-1)(u)-q S_(m-2)(u)`. Then A is simple if and only if `t` is not `S_ell(u)` for an integer `u` with `|u|<=2 sqrt(q)`. For `ell=7`,

    S_7(u)=u^7-7q u^5+14q^2 u^3-7q^3 u.

For q=2, the exceptional traces are `0, ±13, ±16`; ordinarity makes t odd, leaving `±13`. For q=4, the exceptional traces are `0, ±87, ±128, ±251, ±256`; ordinarity again leaves `±87, ±251`.

[`TraceExceptions.lean`](TraceExceptions.lean) kernel-checks the five q=2 evaluations of `S_7(u)` and reports no axioms for that finite theorem. It does not formalize the prime-degree irreducibility criterion, Diem's simplicity theorem, or any curve geometry.

**Argument.** Let pi be a root of `X^2-tX+q^ell`. It is nonreal for ordinary E. By the binomial irreducibility criterion for odd prime ell, `F_A` is reducible exactly when pi is an ell-th power `beta^ell` in `Q(pi)`. Such beta is an algebraic integer with conjugates of complex absolute value `sqrt(q)`, norm q, and integer trace u in the Hasse interval. Then `t=beta^ell+bar(beta)^ell=S_ell(u)`. Conversely, such a u gives a quadratic factor `X^2-uX+q` of `F_A`, because its roots have ell-th powers among the roots of `X^2-tX+q^ell`. Irreducibility of the Frobenius polynomial is equivalent to simplicity for this ordinary abelian variety. Diem's theorem supplies the same conclusion through the endomorphism algebra of the Weil restriction.

If `t` is not exceptional and an algebraic homomorphism `phi:A -> J_Y` over k is nonzero, simplicity makes `ker(phi)` finite and `dim image(phi)=ell`. Hence `genus(Y)>=ell`. In particular, any GHS conorm-norm transfer that is nonzero on a source subgroup has genus at least ell. The implication does not cover arbitrary abstract homomorphisms between groups of rational points, nor does it say a genus-ell Jacobian or efficient attack exists.

For n=7 and a hypothetical genus-seven Y with J_Y isogenous to A, the L-polynomial is `1-tT^7+q^7T^14`; therefore `#Y(F_(q^m))=q^m+1` for `1<=m<7`, `#Y(F_(q^7))=q^7+1-7t`, and its number of degree-seven places is `(q^7-q)/7-t`. At q=2, this is `18-t>=0`. The q=2 census below gives t<=17 for all W1 examples, so this necessary place-count test eliminates none of them.
