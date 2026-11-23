The research objective is faster exact relation solving on ordinary elliptic curves over prime fields, starting with p=2^31-1. The current strategy combines a full Dickson fibre with rational auxiliary S3 equations. The exact representation and factor-base construction have written proofs; the sublinear empty-query operation remains the central open step. [pcc:unverified]

# Current checkpoint

[Research PR4977](https://github.com/isogeny-labs/crypto/pull/4977) contains the reusable S3 pencil analysis and a stronger field-family parameter argument, with separate independent mathematical reviews. It also preserves the actual original-root build failures and the next proof obligations. Earlier research and prerequisite PRs4928,4941,4951 and4962 are merged.

# Rational quadratic system

For each internal parent point, orient its children with a slope and require rational auxiliary abscissae:

```math
\begin{aligned}
\lambda_v^2&=x_l+x_r+x_v,\\
2y_v\lambda_v&=(x_l-x_v)(x_r-x_v)-(3x_v^2+a),\\
y_w&=\lambda_v(x_w-x_v)-y_v.
\end{aligned}
```

With the Dickson ladders, this gives mk+3m-5 variables and mk+3m-4 quadratic equations for m>=2 and k>=1. The freely ordered comb is complete under the stated prime-order conditions; a fixed balanced tree is not. Field-valued roots and signed witnesses remain distinct from algebraic-closure roots and multiplicities. [pcc:unverified]

# Reusable S3 pencil

Writing w(u)=(1,u,u^2), the S3 value matrix factors as W C_r W^t and has rank at most three. The relation query requires zeros of its columnwise products. In the split quotient Fp[V]/H(V), that product has exact formal target degree2D-1 and leading unit-4f(V)Hprime(V)^2. A dense reusable coefficient rectangle allocates2D^2 field slots; this is a representation-specific cost, not a general lower bound. [pcc:unverified]

Every nondegenerate partner quadratic already has rational roots on the actual odd-order pair spectrum. A nonsquare-discriminant filter removes no components. The remaining operation is the exact intersection withX, including a correct answer for an empty target. [pcc:unverified]

# Available factor-base sizes

For the stated supplied odd-order curve and power-of-two fibre sizeN, at least1/6 of regular fibres have at leastN/4 liftable coordinates. Chang's existing smooth-modulus prime theorem strengthens the selected-field construction: every fixed0<beta<5/12 can support a full fibre of sizeTheta(p^beta). Thus all factor-base exponents needed by the conventional four-summand model with0<=tau<1 fit within the field-size argument. Source-curve construction, proper-subgroup density, relation rank and the actual query algorithm remain separate. [pcc:unverified]

# Native evidence

The latest[original-root run](https://github.com/isogeny-labs/crypto/actions/runs/45113871392) attempted all five correctness command steps but failed compilation in an unchanged binary-hyperelliptic module. Zero verifier tests or new identity issuances executed. The source-bound artifact, compiler logs and preceding failures are retained in the PR. Written proofs and parser reviews are not counted as native test passes. [pcc:unverified]

# Next decision

For an actual pair-spectrum sizeD=Theta(B^2), construct a target-independent exact index with near-linear setup inD and empty-inclusive query costD^theta for a fixedtheta<1/2. Include rational witness recovery, zero subtotals, failed targets, storage, signed output and any certificate cost. The rank-three value representation identifies the structure to exploit; it does not establish this bound. [pcc:unverified]

The existing research task continues the same objective, preserves adverse results and advances one concrete mathematical or correctness gate at a time. No asymptotic relation-solving speedup is asserted at this checkpoint. [pcc:unverified]
