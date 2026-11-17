# Geometric quotient continuation: normalized Weil functions

Original objective: construct the polarized quotient for an actual P-192 d7 mixed3 kernel, identify a same-field nonhyperelliptic Jacobian, obtain a quartic and executable divisor map, then test destination-geometry restricted-base relations. A Weil-function layer is a prerequisite, not the completed quotient.

Sources checked: Tian, arXiv:2007.03172, sections2.1--2.2 and4.1 (determinant Weil functions, odd-level symmetric normalization, invariant theta section and quotient moduli); the existing kernel/source witnesses and native arithmetic. KB unavailable: no loaded server and repository wrapper reports no executable. Existing theta_g.rs implements powers-of-two theta isogenies, not this odd-kernel input. No priority claim.

This first development step uses exactly kernel1 from the eight recorded P-192 kernels, unchanged source model and degree8 field. No paid job, curve search, timing or performance campaign. Derive a curve function h=A(x)+B(x)y with divisor3D-9infinity for each nonzero order3 class D=(u,v): lift v modulo u^3, solve a9-by7 homogeneous coefficient system with degA<=4, degB<=1, and check A^2-B^2f=c*u^3 exactly.

For generic degree3 test class Z=(U,V), Tian's determinant ratio becomes the x^2 coefficient of (V+v)/u modulo U. Multiply its cube by the resultant Res(U,A+B*V). Use -D for the Weil function indexed by D. Singular chart evaluations must reject explicitly, not return a numerical default.

Normalize using Tian section2.2 at odd level3. For raw f_P, use alpha_P=f_P(-Z)/(f_P(Z)f_P(Z+P)); check the normalization equations and symmetry on separate inputs. Form theta(Z)=sum over K of alpha_P*f_P(Z).

Acceptance: all26 exact norm identities; full27-by27 normalized theta-group multiplication table on at least two disjoint generic classes; symmetry, level-subgroup invariance and Frobenius covariance; nonzero theta value; changes to h, normalization and kernel isotropy must reject. Fixed inputs are sums of three distinct source-cover points among the first128 affine x values; at most16 candidate test classes, no bound extension without a recorded amendment. A failure remains in evidence. Outputs are unsigned producer evidence. This step does not determine whether the quotient is a product, hyperelliptic Jacobian or plane quartic.

Next stages retain the original field/descent and map obligations: build level2 quotient Weil functions, compute moduli/theta constants, reconstruct a candidate quartic, check smoothness/descent/twist and actual kernel/map, and use a restricted destination factor base.
