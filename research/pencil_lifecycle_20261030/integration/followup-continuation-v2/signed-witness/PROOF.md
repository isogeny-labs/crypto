# Certificate implications

Let R=[U b c] over the stated prime field, with unknown z=(x,k), and equations Ux+bk+c=0. In the native extraction b=-b_native and c=-a_native.

**Determined.** If lambda R=(0,1,-k0), then every solution satisfies k-k0=0 by taking the same row combination. The strengthened certificate also stores a particular solution z0 with target coordinate k0; direct row sums verify R(z0,1)=0. Thus consistency and scalar uniqueness are both certified within the linear system. The direct checker verifies the combination, particular solution and canonical field domain.

**Undetermined.** If R(z0,1)=0, R(d,0)=0, z0_target=0 and d_target=1, then R(z0+t*d,1)=0 for every t. Thus the recorded linear system has a solution for every target coordinate. These are algebraic assignments of nuisance variables; the certificate imposes no additional geometric constraints.

**Inconsistent.** If lambda R=(0,0,1), then any putative solution would imply 1=0. The synthetic inconsistent control exercises this case; none of the 64 native systems produced it.

The checker does not trust the Gaussian elimination state. It verifies only these equalities by modular multiplication and addition, then checks each extracted row against the original hashed edge/identity event. Independent human/model source-fidelity review is still required; separate code paths are not a non-author review.
