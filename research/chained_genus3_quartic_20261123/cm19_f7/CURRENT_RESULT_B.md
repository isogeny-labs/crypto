# Genuine CM19 F7 plane quartic: E-cube isogeny class qualified

Source0b9b506bc035d8ada6a111702b2ae3145e65deb6, all18 complete frozen
inputs read back before execution. All7 strict phases0, guards[], unchanged
one-worker2GiB/120s limits. Full3373B native stdout SHA256
6bcb41419d3d400fadb41b5f77355d04b3f979b30bd337f21e78a633c0b9db08
at first-B-native/quartics.stdout; all38 small original records retained.
Unsigned native development qualification. [pcc:unverified]

The separately fixed primary-derived curve is

```text
C7:(Z^2+6X^2+3XY+Y^2)^2=Y(X^3+2XY^2+6Y^3).
E7:v^2=x^3+2x+6; v=Z^2+6x^2+3x+1.
```

All15 homogeneous quartic coefficients agree with2 times Ritzenthaler2010
Example1's rational model after X=V+3W,Y=W,Z=2U. An actual primary W4
coefficient mutation fails, and pristine restoration passes. The actual
branch polynomial h=[2,4,0,0,1] has gcd(h,h')=1 and independent Sylvester
nullity0; x2 singular control fails and h restoration passes. This is a
smooth nonhyperelliptic genus3 plane quartic. [pcc:unverified]

The direct expanded evaluator and elliptic-fibre counter independently give

| Model | N1 | N2 | N3 | Rational Jacobian order |
| --- | ---: | ---: | ---: | ---: |
| C7, kappa1 | 17 | 65 | 236 | 1331 |
| Cover twist, kappa3 | 5 | 65 | 380 | 275 |
| E7 | 11 | 55 | 308 | elliptic order11 |

Native integer Newton/division checks give

```text
C7 P ascending=[343,441,336,153,48,9,1]=(T^2+3T+7)^3.
Twist P ascending=[343,-147,84,-15,12,-3,1].
```

Both have the actual elliptic quotient factor PE7; both altered N3+3
mutations reject that factor and pristine restoration passes. Paired twist
identities and all3 actual field pair/triple/inverse/Frobenius controls pass.
The original250 F5 family and its zero PE-cube result stay unchanged.
[pcc:unverified]

In F49=F7[b]/(b2-3), the first gamma is encoded7, with gamma2=3,
gamma7=-gamma and gamma49=gamma. The actual geometric cover-twist map
z'=z/gamma sends all17 C7(F7) points to17 distinct valid twist points,
has an actual inverse, and Frobenius equals the deck involution. Exactly16
images are not F7-fixed; the branch point[1:1:0] remains fixed. Complete
source/image/Frobenius point lists are printed. This is a curve-map descent
control, not an already constructed Jacobian4-torsion map. [pcc:unverified]

These counts identify an unpolarized E7-cube isogeny class. They do not
identify the actual principal polarization with a specific CM19 Hermitian
lattice, build a nontrivial mixed kernel, evaluate a quotient/target map or
establish an ECDLP advantage. The primary source's polarized/base-field/Serre
bridge needs independent validation. The first format failure is retained
in full; no compiler/native call preceded its formatting-only successor.
Original4656584B ELF is temporary custody-pending. Independent replay,
generic Mathlib, actual J4/target descent and complete strongest-reference
cold costs/normal CI remain open. [pcc:unverified]
