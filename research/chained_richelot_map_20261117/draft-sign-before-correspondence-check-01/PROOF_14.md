# Algebraic obligations for the actual joint-chart correspondence

This is a written argument using the cited complete-curve/Picard theory,
followed by finite implementation obligations in PROTOCOL_14. It is not a
new Lean construction or signed execution verdict. [pcc:unverified]

Let f=lambda*product(q_i), Delta=det(q_i), and
H=product(B_i)/(lambda*Delta³), where B_i=q'_j*q_k-q_j*q'_k.
Choose a,b,c in the base field with b²=f(a),c²=H(a),bc nonzero. Unit13's
coefficient identity gives normalized new quadratics Q_i and

    F(w)=w^6*f(a+1/w)/b²=product(Q_i),
    raw_new_target=(bc)²*G(w),
    G(w)=w^6*H(a+1/w)/c².

The normalized old source is f/lambda. Write L²=lambda and let the old
constructor's raw target be lambda*H=T²*G_old. Choose s²=T²/lambda, so
G_old=H/s². The complete source and target chart isomorphisms are

    I_S:(x,y)->(1/(x-a), y/(b/L)*(x-a)^(-3)),
    I_T:(x,Y)->(1/(x-a), Y/(c/s)*(x-a)^(-3)).

Their regularity at both pole branches and both infinities follows from
the explicit local expressions in unit12 PROOF_12, including the inverse
charts. Pushforward preserves principal divisors and composes, hence each
induces a Pic^0 group isomorphism (corrected Galbraith8.3.8/8.3.10).
[pcc:unverified]

For the raw Richelot ordinates, unit13's bracket/determinant covariance
transports the old target ordinate by `w³*(b/L)`. Dividing by the new
constructor's square root T_new gives

    phi_new I_S = [kappa] I_T phi_old,
    kappa=(b/L)*(c/s)*T/T_new.

The square of this scalar is1 because T²=lambda*s² and
T_new²=(bc)². In the fixed characteristic101 instance, kappa is consequently
1 or100, determined by the recorded b,c,L and the two canonical constructor
square roots before comparing any divisor. It is not an empirical choice
of the dual sign. Hyperelliptic ordinate negation induces group negation.
The protocol checks the complete balanced maps, including w=0/infinity
and closed supports; coefficient equality alone is not its acceptance.
[pcc:unverified]

The actual correspondence is Kunzweiler Proposition8:

    q1(x)h1(z)+q2(x)h2(z)=0,
    y_raw*Y_raw=q1(x)h1(z)*(x-z).

The existing completed evaluator handles the finite fibre, degree drops,
infinities and balanced base correction. Exchanging the two curves changes
`x-z` to its negative. The existing reversed evaluator's final group
negation is therefore retained; no new sign is chosen from the test data.
For a (2,2) isogeny of principally polarized Jacobians, the polarized
adjoint psi satisfies psi*phi=[2] and phi*psi=[2]. This is the polarization
identity phi^vee*lambda_B*phi=2*lambda_A, conjugated by lambda_A^{-1}.
It is the classical obligation checked here for the existing code, not a
consequence inferred globally from finitely many assertions.
[pcc:unverified]

The conjugacy also predicts the actual return chain

    psi_new = [kappa] I_S psi_old I_T^{-1}.

Thus applying the fixed return chain to phi_new(D) must give[2]D. Unit14
will execute that chain and compare its canonical output with the new
dual, separately from the coefficient checks and pointwise/closed-fibre
comparison. This exercises a chain of implemented maps on actual classes.
[pcc:unverified]

For a prime q different from2 and an observed nonzero G with[q]G=0,
psi(phi(G))=[2]G implies phi(G) is nonzero, and homomorphy gives[q]phi(G)=0.
The restriction of phi to<G> is injective: phi(D)=0 implies[2]D=0, and2 is
invertible modulo q. It preserves and reflects scalar relations on that
cyclic subgroup. Since109*113=12317 and2*6159=12318, the scalar6159 applied
to the adjoint recovers each directly checked109/113-torsion input. The
runtime tests subgroup membership and the declared known scalar cases;
they do not assume a new all-class enumeration or perform a DLP solve.
[pcc:unverified]

Finally, the old closed-fibre evaluator reduces the first correspondence
modulo U=x²+a*x+b to A(z)x+B(z), takes
R=B²-a*A*B+b*A², recovers x=-B/A when A is a unit modulo R, then recovers
the ordinate when its denominator is a unit. The identity

    (A*x+B)*(B-a*A-A*x)+A²*U=R

gives the finite-algebra isomorphism and preserves repeated-support jets.
Nonunit/degree-drop cases use its explicit original fallback, not division
by zero. `CLOSED_FIBRES.md` supplies the written bridge; its underlying
polynomial and Cantor engines are shared. Agreement of these routes is a
finite same-engine control, not an independent full arithmetic replay.
Prime-input closed-route operands lie in the cubic subfield by closure,
which motivates a separately frozen later specialization, without a cost
or speed conclusion from this unit. [pcc:unverified]
