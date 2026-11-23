# Recover the predicted quartic-to-elliptic cover

2026-11-16,frozen before affected map data. Preserve the timed-out seven-variable inverse attempt. This is a different construction of the elliptic-subgroup transfer,using exact differential integration and rational reconstruction,following the formal-series approach in Tian arXiv:2007.03172 section4.3. KB wrapper unavailable. It does not replace the original full-J(H) isogeny/dual acceptance.

Let F=phi*pi^* and T=pi_*psi for the polarized3-quotient. Then TF=[6]. Duality predicts that the corresponding morphism f:C->E has degree6 and f^*=F. Its differential is obtained from the existing candidate quotient differential D=sqrt(s)*M0 as3*l_H*D^-1. Derive the source normalization from the frozen Hamiltonian flow: the native source basis is X^j*dX/(2Y),because dx=2Y*h has trace velocity h. For c=1024 the cover identity gives l_H=(0,16/c^2,16/c) for omega_E=dx/(2y). Require this cleared rational differential identity natively. Use the canonical base-quartic basis(X,Y,Z)*dx/f_y,as in the checked target flows. Preserve the conditional normalization argument; a global curve-map equation must settle whether this candidate differential really integrates with the declared degree.

Choose the canonical Fp square root of s (p=3 mod4,verify its square) and the first retained base-field quartic point R with f_y(R) and the differential's linear numerator L(R) nonzero. Translate the target E so f(R)=O. No alternate point or differential rescaling after a failed mathematical check.

Use112-term native Fp series. Lift y(x_R+t) by Newton iteration and check the quartic to full precision. Integrate omega=L/f_y to u(t). Construct the Weierstrass Laurent series X_E(u)=u^-2+c2*u^2+c4*u^4+...,with c2=-a/5,c4=-b/7 and recurrence from X_E''=6X_E^2+2a. Work with W=t^2*X_E and check the scaled elliptic equation for its first96 coefficients.

Use22 monomials x^i*y^j,j<4,i+j<=6,for H0(C,O(6)). Solve the64 training coefficient equations t^2*A-W*B=0 on44 numerator/denominator coefficients. The predicted degree6 map gives rank34/nullity10. Require that rank,all32 unused coefficients for every kernel basis vector,and consistency of all ratios A/B modulo the original quartic. Select the first kernel vector; require B nonzero. No extra degree,precision or scalar search after data.

Define the rational Y_E by the exact differential identity:
V=B*(f_y*A_x-f_x*A_y)-A*(f_y*B_x-f_x*B_y),Y_E=V/(2*L*B^2).
Require V^2=4*L^2*B*(A^3+a*A*B^2+b*B^3) in Fp[x,y]/(f_C),with nonconstant X_E. Verify a changed numerator rejects. Retain all coefficients,point,differential,series and linear ranks. A formal fit alone is not a map result.

For degree,homogenize A,B to degree6 and compute the Hilbert dimensions of S/(f_C,A,B) in degrees14 and15; require both12. On the smooth quartic,the common base divisor has degree at most24. The base-point-free pencil exact sequence has H1(kernel)=0 in degrees>=14,since its kernel degree is4(n-12)+deg(Base)>=8>2g-2. Therefore these dimensions equal deg(Base). Then deg(X_E)=24-12=12 and deg(f)=6. This argument and the native calculation have producer standing; retain the exact matrix ranks and assumptions.

Only after the global identity and degree pass: recover closed fibers,implement pullback/norm on classes,compare against the original source quotient coordinates,check public points and collect destination restricted-base relations. A successful curve backmap alone does not complete those steps or the full source-Jacobian isogeny.
