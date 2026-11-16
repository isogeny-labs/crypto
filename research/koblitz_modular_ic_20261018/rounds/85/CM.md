# R85: exact quadratic-base diamond overlap

On the accepted inverse-base source equation
`E=V²+z²V+z+z³=0`, set `h=z+V`,
`H=1+z+z²+zV`, and `N=V+zV+z²`. The frozen Singular
run verified the full identities `h²+zH=E` and `N+h+1=H`.
For the forward rational coordinates `z1=1/h` and
`V1=N/h²`, it cleared denominators in `E(z1,V1)` by
`h⁶` and verified an exact polynomial lift into `(E)`.
The corrupted numerator `N+z` failed the same ideal test.

On the principal overlap where `E=0` and `z*h` is a unit,
`h1=z1+V1+z1²=H/h²=1/z`. Thus the proposed inverse
returns `z` from `1/h1`; then `V=z+h` returns the original
`V`. This certifies the quadratic-base coordinate round trip,
not yet the lift to all 32 generators of the upper source field.

The native attempt passed exit, exact marker, zero native `?`
errors, eleven historical source-byte pins and five donor hashes.
The pinned Lean file proves the two characteristic-two polynomial
identities and their consequence on `E=0`; all three theorems
compiled and passed the axiom audit with zero failing and zero
trusted. The native transformed-equation lift and corruption
control are not claimed as Lean theorems.
