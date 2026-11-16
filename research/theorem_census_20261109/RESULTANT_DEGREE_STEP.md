# Fixed-size resultants: general degree step and S5

Lean proves the leading coefficient and degree of the actual resultant
step for arbitrary declared size D. With f in R[X] and f.natDegree≤D,
the output in R[Z][T] satisfies the following identities. [pcc:unverified]

$$[T^{2D}]\operatorname{Res}_{D,2}(f,S_3(X,Z,T))=f(Z)^2,
\qquad \deg_T\operatorname{Res}_{D,2}(f,S_3(X,Z,T))\le2D.$$

The coefficient statement works over every commutative ring. If the ring
is a domain and f is nonzero, the degree is exactly 2D. D remains the
declared Sylvester size even when specializing f lowers its actual degree.
These are polynomial statements about the actual determinant, without
finite-field interpolation or a characteristic restriction. [pcc:unverified]

Applying the step to the canonical S4, with three other inputs independent
in MvPolynomial(Fin3,K), gives the canonical S5 with degree8 in its last
input over every field. The two additional inputs are the independent Z,T.
This application proves one chosen S5 input, not all five input positions
or the generic all-m family theorem. [pcc:unverified]

## Skeptical control

Lean also disproves changing the declared size from2to0 after specializing
the incoming polynomial to1. The corresponding actual resultants have
T-degrees4and0, so they cannot be equal, for any curve-invariant parameters
over a domain. This is a counterexample to resetting the operator's size,
not to Semaev's qualified generic degree theorem. The earlier smooth F5
specialization S4(0,1,4,T)=1 supplies a separate actual curve control.
[pcc:unverified]

## Proof structure and source scope

`lean/ResultantDegreeStep.lean` generalizes Mathlib's determinant argument
to individually bounded columns. The D second-polynomial columns have
T-degree≤2 and the two f columns degree0. The top coefficient matrix is
the actual Sylvester matrix of f and (X−Z)². Mathlib's monic-square resultant
identity then gives f(Z)²; the even sign is proved explicitly. [pcc:unverified]

The S5 coefficient ring uses three independent multivariate indeterminates.
An explicit evaluation homomorphism to the earlier nested-polynomial
presentation proves the S3 coefficient nonzero. This avoids a very long
tower of elaborator typeclass instances while retaining independent
variables and the actual canonical S4 determinant. [pcc:unverified]

Read scopes and exact versions are in `resultant-degree-step-sources.json`:
Semaev2015§2/pp2–3 and Kosters–Yeo2015§2/p3; the original Semaev2004 text
remains unread after403/404responses. Related papers do not replace that
source. No irreducibility, zero-sum equivalence beyondm4, solving-degree
or complexity theorem is established here. [pcc:unverified]

The frozen earlier native sources/protocols/raw data are unchanged. This
piece runs no new curve population, statistical study, benchmark, paid
compute or second-host validator. The full fourteen-item request remains
partial; its outstanding obligations remain in README.md. [pcc:unverified]

## Verification

The full tracked census gate passes 291 declarations, 0 failing and
0 trusted-native exceptions; counts include generated declarations.
Packet PCC-20261109-621d56 passes its full local check at18:39:21Z, with
source commit ddc05a59 made before issue. Standing remains unsigned.
Exact hashes and scope are in `resultant-degree-step-verification.json`.
[pcc:unverified]

Supplement IDC1h7dd754d70e343323 accepts 32 derived and 11 extra points for
the fixed D=4 scalar monic-square resultant identity only, with 6 variables
and total degree bound10. Its proof-file pointer matches. The actual
unbounded-D coefficient/degree and S5 conclusions rest on Lean, not these
finite one-host samples. See the [report](reports/proof-20261109-resultant-degree-step/report.pdf).
All failed build attempts are retained. [pcc:unverified]
