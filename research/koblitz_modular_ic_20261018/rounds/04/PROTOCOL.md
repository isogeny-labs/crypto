# Round 04 protocol: explicit quotients and n=5 modular relations

2026-10-18. Parent f5b90fb286dde2e8b0fa15b127a37514a8d64258.
The user requests explicit polarized quotient geometry, an evaluated n=5
F2 modular-factor map and direct relations, with failures and Lean proofs.
Research arithmetic, certificates and ledger assembly are native Rust.
Prior round evidence is immutable. No timing protocol is authorized here.

## Q1: complete short-vector and quotient-lattice computations

Use round-03 GL2b's retained kernels and first anti-isometry of every one
of its fourteen nonempty gluing cases. Construct a basis of the overlattice
Z^10 + graph/ell. Check the integral descended alternating form, determinant
one, actual Frobenius integrality and F^T J F=2J. These are prime-to-two
polarization/Tate data, not automatically a global complex uniformization
when the elliptic cofactor is supersingular. Keep that distinction explicit.

For the determinant-19 A5 Hermitian forms, replace the coordinate box with
exact inverse-Gram coordinate bounds and an exact rational LDL enumeration
of every vector of norm at most three. A limit of 50,000,000 recursive visits
per distinct form is an incomplete run, never a global minimum assertion.
Retain counts and witnesses separately from prior bounded results.

## Q2: one explicit principally polarized quotient

Construct the natural determinant-five quotient A5(E) x E -> Res(E),
(a,Q) -> a-diag(Q). Give actual equations for the Weil-restriction affine
chart over F2 using the pinned F32 basis. Supply chart/domain exceptions,
group operations and the inverse on all F2 rational points. Evaluate both
signs, every point, every addition, and all eleven subgroup inputs.

The geometric kernel consists of (diag(T),T) for T in E[5], of order25.
Check the kernel and its group arithmetic over F_(2^24), where both signs
have full five-torsion. Find a degree-24 irreducible modulus by increasing
odd low masks up to4095 with exact Rabin checks, then use deterministic
point search up to4096 x-coordinates and exact cofactor projection. Preserve
failure if these limits do not yield a basis. Verify finite coordinates with
the existing binary-curve library where applicable. State the written
polarization-pullback identity and formalize its algebraic dependencies.
This quotient is a fivefold; it is not called a genus-five Jacobian.

## M1: explicit X0(49) coordinates over F2

Use Kilford--McMurdy, arXiv:1110.6801, section2 equations(1),(3),(5),(6),
and the integral change x=X-2,
y=(x^2+7x+7)*Y+4*x^3+22*x^2+35*x+7 from the minimal elliptic model.
Prove the polynomial identity and reduce it in characteristic two, without
reducing a formula with a denominator2. Evaluate both degeneracy parameters,
j-maps and involution on every point through field degree10. Record cusps,
exceptional denominators and removable singularities explicitly.
This evaluates a modular base map, not yet the twisted A5 map.

## M2: level-11 Hecke correspondence and twisting bridge

Obtain the classical degree-11 modular polynomial from a primary source;
freeze its exact input and parse/reduce it in Rust. Enumerate the explicit
X0(49) point pairs through degree10 and test the j-level correspondence.
Retain multiplicities/level-49 ambiguity; a j-neighbor alone does not identify
an actual transported cyclic49 subgroup or a twisting projector. Freeze any
subsequent transport, projector or relation extension before execution.
The actual requested F2 modular-factor divisor evaluator stays an open
requirement until its divisor inputs, exceptional cases, subgroup image and
relations on that destination are evaluated and checked.

## Formal proofs and delivery

Add Lean lemmas for the central quotient and modular-coordinate identities,
with geometric hypotheses explicit. Build and audit the pinned project,
preserve failed attempts, and add the new replay to CI. Ship a proof map,
ledger, diagram, inspected PDF, deterministic raw archive, exact replay and
updated open-problem/canonical summaries. Run native and root library tests.
Do not substitute a same-field cover control for the modular-map requirement.

## M3 amendment: evaluate actual degree-eleven transport and twisted images

Before execution, restrict the first transport experiment to X0(49)(F64).
For ordinary underlying j, reconstruct both successive cyclic7 kernels
using their target j-values and the exact division polynomial. Enumerate
monic degree-three kernels with prescribed coefficient T satisfying
T^2+T=b+b_target; test divisibility, separability and doubling invariance.
Keep ambiguous or exceptional cases as unsupported records.

For underlying trace +/-1, select its quadratic twist of order66 overF64.
Construct full eleven-torsion overF_(2^30), using a Rabin-verified modulus
(first odd low mask <=4095). Embed F64 exactly, find one rational eigenline
and one complementary point by deterministic x-search <=4096 and cofactor
projection. Diagonalize Frob^6 to eigenvalues1,9 on the order11 group.
Enumerate all12 cyclic subgroups. Transport both cyclic7 kernels through
the actual Velu x-map; transport the11 kernel through the first7 map as a
commuting-square check. Reconstruct every target X0(49) point from its
three j-values, including exact equation checks and denominator failures.
Check the sum of all twelve target points against the integer a11 computed
by counting the conductor49 elliptic curve overF11.

For each supported level point, collect the10 nonfixed Hecke images into
five even sums and signed odd differences, subtract the augmentation mean
integrally as 5*D_i-sum(D). Apply the six-term twisted Frobenius trace.
Verify the fixed-tuple condition for both signs, exact order11 or zero,
F32 descent, and the quadratic-twist isomorphism using s^2+s=1.
Record zero images and unsupported branches, not only nonzero images.
This evaluates a proposed twisting correspondence on explicit modular
closed-point inputs. Distinguish its modular-character identification,
optimal quotient/saturation, reverse divisor map and direct principal
relations as separate requirements. No surjectivity is assumed.

## M3b/M4 amendment: paired level bases and genuine modular-unit relations

M3's supported inputs give nonzero projected order11 points, but their
independently chosen level bases do not yet share a fixed Weil pairing.
Before any cross-point relation, compute the exact Weil pairing by Miller
functions with deterministic auxiliary points (<=1024 x trials), verify
alternation and a bilinearity control, and rescale the complementary basis
to a single primitive eleventh root inF_(2^30). Preserve M3 unchanged.

For each supported source enumerate the five symplectic level bases modulo
simultaneous sign, then identify closed degree-six points using canonical
Frobenius(P,Q)=(P^2,Q^2/2), with simultaneous-sign equivalence. Verify the
transported tuple's compatibility with this actual modular Frobenius action.
This gives a concrete modular closed-point factor base, not an abstract
Frobenius-module label.

Test principal relations from functions pulled back fromX0(49): lines and
verticals on its elliptic model whose divisor support is completely in the
supported base-point set. Their full inverse-image divisors also include
non-F64 level points: explicitly enumerate/determine those missing orbits
before admitting a relation. A partial rational fiber is not a principal
relation. If rational fibers alone cannot close the divisor, record that
failure and keep the direct relation collector open. No planted scalar,
Hecke zero-image identity or supplied matrix row is a principal relation.

## M5 amendment: a level-coordinate interior-function test

M4's full-fiber pullback relations have rank4 on544 closed columns. Test
x(P)+1, a level-coordinate function depending on the nonzero11-torsion
section, to seek a row that is not constant along the base fibers.
Construct psi11(x=1,b) exactly as a polynomial in b in Rust, retain its
multiplicities, and check its degree. For each root b inF64 count the
known modular points in the current factor base satisfying x(P)=1.
The complete ordinary zero fiber has length degree_b(psi11(1,b))*56*11:
56 cyclic49 subgroups and11 complementary symplectic bases per P modulo
sign. Record all unrepresented b roots and missing level49/11 orbits.
Admit a principal row only if this complete zero divisor is represented
and its poles are resolved. A vanishing image of a partial row is not an
admission criterion. Preserve an incomplete-fiber outcome as a failed
collector extension, with the exact missing degrees.
