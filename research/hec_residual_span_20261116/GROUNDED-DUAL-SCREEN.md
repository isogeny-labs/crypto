# Grounded harvesting: exact dual signatures and the limit of a scalar filter

[pcc:unverified]. Author-derived linear-algebra screen for the current generic affine-forest collector, not a new attack, independent proof admission, or a performance result. The all-grounded policy's complete-cost regressions are retained in native-grounded-harvest/RESULTS.md. This note defines a stronger criterion for a future selective policy rather than fitting a cut to those outcomes. No scientific execution or new measurement.

## 1. Grounding does not create new degrees of freedom

Let z comprise all factor-base variables and target k, with accumulated consistent system A z=b over F_ell. A grounded point has an already-derived affine expression in the original small-base variables, k and a constant. Introducing a symbol P=h(z) and then substituting it into the SAME previous equations is a conservative extension: eliminating P returns the same solution set. It adds no new information about k.

New geometric principal relations using P can add information; the previous qualification demonstrates this possibility. Their independence must be assessed against A. A growing eligible point catalogue alone is not a growing space of unknowns or a rank gain.

Choose one solution z0 and a full-column-rank matrix N whose columns form ker A. Every current solution is z=z0+N theta. A candidate equation r z=c becomes

    (rN) theta = c-rz0.

For a consistent enlarged system, the row increases coefficient rank exactly when rN!=0. If rN=0, r annihilates ker A; therefore r belongs to rowspace A, so consistency also forces c-rz0=0. Conversely, r outside rowspace A has a vector w in ker A with rw!=0, hence rN!=0. This proves the criterion without a distributional assumption.

For a block R, its exact added rank is rank(RN), at most dim ker A. The number of promoted points or geometric relations does not change that bound. The target is determined exactly when its coordinate functional tau=e_k N is zero. After inserting R, target determination is equivalent to tau vanishing on ker(RN), or tau belonging to rowspace(RN). These are coefficient criteria under the maintained consistency premise; native kD=T is still required for an accepted scalar.

## 2. Forest factorization of the signature

In the current binary affine forest, let S denote the original m small variables plus k. Each LP vertex satisfies

    X_v = w_v R_component + h_v(S) + constant.

A grounded component has no R_component term. Let B S=d be the independent small-system constraints and let N_S span ker B. If q ungrounded components remain and rho=dim ker B, a full homogeneous parameterization has q independent root coordinates plus rho small-system coordinates. Its dimension is q+rho. This follows by assigning every ungrounded root freely and solving the small system; the forest relations then determine all other LP variables uniquely.

A grounded point's dual signature lies entirely in the rho-dimensional small part and equals h_v N_S. An ungrounded vertex additionally has its weighted root-coordinate signature. Thus the exact signature of a new principal row is the sum of its signed point signatures and target signature, with multiplicities retained. No source/target logarithm or hidden projection is supplied by this construction.

This is a dual form of the SAME elimination problem, not free rank information. Constructing N_S, evaluating sparse h_v N_S, processing root coefficients, and updating signatures after a rank change all cost work. Cached signatures from an old nullspace cannot be used unchanged after shrinking the solution space.

## 3. One target-direction signature is insufficient to reject rows

A tempting cheap filter chooses only one w in ker A with w_k=1, then discards rows with rw=0. A nonzero result certifies that the row is independent, but a zero result does NOT certify dependence or uselessness for target recovery.

Concrete consistent example over F_101: variables (x,k), initially no equations, actual values (5,7), and chosen w=(0,1). First candidate is x=5. Its signature rw=0, yet it is independent. Second candidate is x+k=12. Keeping both determines k=7. Discarding the first leaves x+k=12, under which k remains free. Hence a scalar-signature discard policy can lose exactly the constraint needed for eventual target recovery, even though it retains the row that kills the current chosen target direction.

The same issue applies to ungrounded-root directions: a row may be useful for grounding a component while annihilating one selected small-system direction. Single-vector screening can be a heuristic selection rule only with an explicit coverage/fallback argument; it cannot be advertised as an exact dependence filter.

## 4. When an exact cheap signature becomes possible

If the FULL remaining nullity is one, a single nonzero null vector is a complete basis. Then rw=0 exactly characterizes dependence, and rw!=0 removes the last free direction. If w_k!=0, that row determines the target (subject to native verification). If w_k=0, the target was already determined; it is not the unresolved target case.

More generally, rho small plus few ungrounded components can make short block signatures attractive. An exact policy must retain enough signatures to span every relevant homogeneous direction or prove that omitted directions cannot affect target determination. Grounded point count alone does not justify a one-dimensional approximation.

## 5. Prospective construction requirements

Before implementing another promotion selector, specify an exact representation of N_S or a certified equivalent target quotient, plus costs for: basis construction/update, per-point signature formation, topology discovery, base-field scanning and residual splitting, weighted row substitution, and final target recovery/verification. Compare that complete work to the current static forest, including cases where no cheap low-nullity phase occurs. Do not declare a signature evaluation free because it replaces a Gaussian update elsewhere.

A concrete falsifier for a proposed exact filter is any retained raw prefix where it labels r dependent but rank([A;r])>rank(A), or where it drops a row needed by the independent target-rank reference. The F_101 example above is already a falsifier for the single-target-vector zero-signature discard rule. For an implemented full-basis method, require rank(RN) to match independent coefficient-rank differences, target criterion to match native target solving on held-out prefixes, and complete costs to improve before claiming speed. Source/target/model identity and negative membership controls remain.

This supplies an exact design test and prevents an invalid cheap filter; it does not establish a lower bound on all adaptive collectors, nor a 10x mathematical breakthrough. Generic genus2/3/4 and full runtime remain in scope. Existing one-core promotion and weighted-graph rank notes are prior art/context, not independent validation of a new algorithm.
