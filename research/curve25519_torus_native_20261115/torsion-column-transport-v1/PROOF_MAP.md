# Instance gate and fidelity map

Applied lean-instance-gate before the first Lean edit. No new generic group
law will be restated as a study theorem. Multi-declaration plan: existing
Mathlib homomorphism laws first; anonymous assembly probe second; actual
native-curve/group instance only after an explicit implementation bridge.
There is currently no such native point representation bridge. [pcc:unverified]

1. Structure above: Mathlib AddMonoidHom over additive groups; subtraction,
   addition and zero are inherited group operations. Rational cofactor fibres
   would instantiate its kernel fibres for multiplication8.
2. Existing laws used as stated: map_sub, map_add, sub_eq_zero, add_zero.
   No new kernel-fibre or group-cardinality theorem is constructed locally.
3. Object: the assembly variable psi is an AddMonoidHom; the eventual desired
   object is the original native Curve25519 scalar8 map. Its equivalence to
   Mathlib's point group and enumeration of its rational kernel are missing.
4. Assembly probe: two anonymous examples, subtraction-to-zero iff equal
   images and zero-image translation. Type-check in the existing pinned
   received-ideal Lake environment, not bare Lean. Actual-native instantiation
   is NOT supplied by those examples.
5. Missing components: native Field/Curve group equivalence, subgroup/cardinal
   hypotheses, exact rational8-torsion enumeration and original map/compiler
   refinement. Native checks remain data, not those formal bridges.
6. Inheritance: difference image--map_sub; zero difference--sub_eq_zero;
   translated image--map_add; zero summand--add_zero. Native operations have
   no formal inheritance line yet; full instance acceptance stays open.

| Written claim | Formal artifact | Difference / boundary |
| --- | --- | --- |
| All rational raw fibres form a kernel coset, assuming original group/order | anonymous homomorphism assembly only | no Curve25519 instance or eight-element kernel proof |
| At most32original-map encodings/image | written inverse formula plus fixed native enumeration | no formal field/root-cardinality/map theorem |
| Exact rank/kernel on fixed native matrices | invertible-operation replay and dense kernel product | producer native checks, not Lean checker-soundness theorem |
| Scalar recovered from informative tag witness | native group replay if one exists | no arbitrary solver/cost/growing-family theorem |
| Explicit inverse8 coefficient for actual ell | TorsionColumn.actual_inverse8_bezout | closed Nat identity only; native group instance absent |
| Explicit inverse8 coefficient for toy13 | TorsionColumn.toy_inverse8_bezout | closed Nat identity only; native group instance absent |

The anonymous probe is not a named theorem endpoint. A successful probe only
checks existing-law assembly under abstract group hypotheses. Two separately
named instance obligations, actual_inverse8_bezout/toy_inverse8_bezout, check
the concrete Nat Bezout coefficients for the original ell and13. Their floor
is Lean's decidable Nat equality, not a new generic group law. The native
constructor explicitly checks its computed inverse against these coefficients.
The focused project gate passed those two numeric endpoints and the
anonymous module together; native group/map/finite-alias completeness still
has no formal inheritance line. Exact declarations/configuration/toolchain
and actual gate standing are recorded separately. [pcc:unverified]
