# R87: full relative-equation preservation on the diamond open

R50's accepted inverse-base model is the two-equation algebra
`E(V,z)=0`, `P(T,V,z)=0`, with `P` monic of degree 16 in
`T=lambda`. The retained source has 145 polynomial terms. On
the registered open, R85 gives `z1=1/h`, `V1=N/h²`, where
`h=z+V`, `N=V+zV+z²`. R86 gives
`T1=A/D`, where `A=T+(1+z)²g`, `D=z²g`, and
`g=1+z+z²+V`.

For each source monomial `T^a V^b z^c`, the frozen program
constructed `h^(64-c-2b) N^b D^(16-a) A^a`. It checked the
nonnegative exponent bounds for all 145 monomials and summed
the resulting 4,612-term polynomial. Singular reduced the sum
to zero modulo the original `(E,P)` and returned a complete
two-generator ideal lift whose product reproduces the full sum.
Replacing `T1` by `T1+1` gave a nonzero remainder. The
corrupted difference was evaluated exactly using the supported
positive `T` exponents 1,4,8,16 and characteristic-two
Frobenius powers.

The single frozen attempt `relative87-01` passed with exit zero,
exact marker, zero Singular `?` errors, seven historical Git
source-byte checks and four donor hashes. This certifies the
degree-16 relative equation under the rational diamond
substitution wherever the registered denominators are units.
The R85 base equation and R86 affine update are separate accepted
inputs. The remaining geometry is to give an explicit inverse
upper-field map and certify regularity of all 32 normalized RR
generators on both affine charts, including exceptional branches.

The accompanying Lean file proves the shift identity for exponents
2,4,8,16 in any characteristic-two commutative ring. These lemmas
justify the corrupted-control difference construction; the
4,612-term ideal membership and full lift are native certificates,
not Lean theorems. Sources checked: R50 `RELATIVE-PROTOCOL.md`
and exported `relative-infinity50.sing`, R85 `CM.md`, and R86
`CM.md`. This round asserts no new modular subgroup transfer.
