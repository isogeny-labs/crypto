# Rational maximal isotropic 2-kernel admission

All finite gates pass. The two current genus3fixtures have no rational maximal
isotropic2kernel. Thus neither admits a rational polarized degree8quotient of
this type over its existing base field, regardless of tractability. This is
stronger than merely finding no Smith pair partition. It does not exclude
odd-degree isogenies, general correspondences or other IC mechanisms.

| Genus | Prime | Branch Frobenius cycles (infinity included) | Stable full kernels | Stable pair kernels | Rational 2-torsion dimension |
|---|---:|---|---:|---:|---:|
|2|1009|1,1,4|1|1|1|
|2|16381|1,1,4|1|1|1|
|3|127|1,3,4|0|0|1|
|3|1009|1,3,4|0|0|1|
|4|31|1,1,2,6|3|1|2|
|4|127|1,3,6|0|0|1|

Known quotient/kernel theory, not novelty or a measured speedup. A positive
count supplies only an abstract-label kernel, not explicit root coordinates,
a destination curve or divisor map, same-field descent, or complete solve.
For genus4a principally polarized quotient need not be a Jacobian. The
odd-prime target subgroup would be preserved by a suitable explicit2-power
map, but none was constructed or benchmarked in this gate.

## Independent algebraic obstruction for the zero counts

For branch-cycle lengths c_i, the characteristic polynomial on J[2] is
product_i(T^c_i+1)/(T+1)^2 over F2: take the even-subset submodule and
quotient by the all-ones class. Let P=T²+T+1. It is irreducible and
self-reciprocal over F2. Different reciprocal-primary components are
orthogonal for the Frobenius-invariant symplectic form; the P-primary part
is nondegenerate. Any stable Lagrangian splits across primary components
(by polynomial projectors), and its intersection with a nondegenerate
self-reciprocal component must have exactly half that component's dimension.

If P occurs with odd multiplicity m, its primary component has dimension2m.
Every invariant subspace of that component has even dimension, since all
composition factors have degree2. A Lagrangian intersection would require
odd dimension m. This is impossible. No semisimplicity assumption is needed;
composition-factor dimension also applies to repeated factors.

For cycles(1,3,4), the polynomial is (T+1)^4 P; its Pmultiplicity1rules out
both genus3kernels. For cycles(1,3,6), it is (T+1)^2 P^3; multiplicity3rules
out the second genus4fixture. For(1,1,2,6), it is (T+1)^4 P²and this
obstruction is absent; enumeration finds3kernels. For(1,1,4), it is(T+1)^4.
These explicit polynomial identities and the module argument corroborate
absence independently of exhaustive Lagrangian enumeration. They do not
turn even multiplicity into a sufficient criterion.

## Verification and finite scope

Scientific source58eb56e46dc16f56cd4ea8acd22a254c92fc8061;
protocol9d02c6ef4c; pre-run checkpoint20261115T123803Z-58eb56e46dc1.
No timing study. Native Rust uses cached library only for prime-field
polynomial arithmetic; subspace enumeration is separate F2arithmetic.

Geometric totals for g1..4are3,15,135,2295, matching product(2^i+1).
Distinct pair-generated spaces are3,15,105,945; every one belongs to the
full enumeration. All22genus3branch-cycle types match Smith Lemma1's
tractable table. Independent pair-set invariance matches subspace stability.
The(7,1)control correctly has2stable fullkernels but0pairkernels, showing
that tractable failure is not automatically rational-kernel absence.
Fixedtorsion dimensions independently match finite-factor-count minus1.

Finite-factor degrees use gcd(x^(p^k)-x,f) and divisor subtraction. Inputs
are squarefree and degrees sum correctly; infinity is included as a1cycle.
All six existing family0fixtures are retained without selection or rerun.
CHECKS.txt records all32cases and terminal counters; stderr is empty.

## Reproduce

Build crates/hec-native-ic/Cargo.toml with stable Rust and resolve the emitted
rlib. Compile check.rs using --edition2021 -O, --extern hec_native_ic=RLIB
and -L dependency=DEPS; invoke binary with
research/hec_point_sieve_20261115/fixtures as its sole argument. SOURCE.txt
and hashes.sha256 pin binary, source, protocol and fixtures. This gate is not
automatically executed by existing native CI. Source/PDF/SVGare retained.

Primary Smith Sections3,9and Lemma1were read in the extended arXiv paper;
PROTOCOL.md records KB/source assumptions. Current absence of kernels is an
exact finite-field claim, not a claim that no newer isogeny construction
exists. No benchmark or prior closed record/default/standing changes.
