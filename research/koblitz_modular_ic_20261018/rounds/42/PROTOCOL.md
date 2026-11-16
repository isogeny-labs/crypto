# R42 fixed larger-size CM residual and unit-recovery experiment

Frozen before affected data. Same manual local Rust4GiBvirtual/1200s build
and run; pinned Lean4.34.1/mathlibd13f23b,-j1-M4096/1200s when proofs follow.
No new cloud job, duplicate Diem/freeze restart, extra claim, agent, cap
increase or merge. Conductor R42 scope clear. Preserve sealed R33–R41.

Literature before method: read R15 one-prime ideal CM construction, R18
joint systems, R39 twist correction and R41 source actions/NEXT. Molin–Page
Theorem37 is the retained construction reference; its primary endpoint
timed out this session. No new modularity theorem or geometric map is claimed.
The n7 split and level7p carrier construction are already known here.

## Fixed inputs and scope

Six cases from literal root polynomial data: n5 both trace signs at11;
n11/trace-1 at conductor23 (order529) as a collision control;
n11/trace+1 at991; n83/trace-1 at2417851639230796216685689;
n131/trace-1 at680564733841876926932320129493409985129.
Conductor primes11,23,499,263 and historical genera61,309,54781,45589
come from CM15. Do NOT replace large target moduli by convenient small primes.
These are retained subgroup-order inputs; no new primality theorem is
claimed. Exact unit certificates do not require a primality assumption.

Recompute the integer trace recurrence and trace-zero cardinality, require
the retained values and target divisibility. Check all finite-conductor
units, primitive generator, both roots of alpha²-alpha+2, their discrete
logs, the norm-2 relation and odd conductor index.

Construct a target Frobenius unit root by computing X^n modulo
X²-trace*X+2 and target modulus. If its linear coefficient is a unit, solve
exactly and verify the root, n-order and all inverse identities. Only for
the fixed small control may an exhaustive root search be used. Never search
a large coefficient box. Put zeta=-unit_root; verify all2n powers distinct,
zeta^n=-1, zeta^2!=1 and every primitive character embedding.

Use a CRT generator diamond (1 mod7, primitive modp), not diamond2.
Record the exponent and exact character order of2 separately. Delta is
the order2 element of the full character group; in particular it must not
be silently replaced by diamond2^n when diamond2 has only ordern.

## Native arithmetic and complete finite enumeration

Use arbitrary-precision native Rust. Every modular multiplication is checked
against independent repeated-doubling/addition reduction. Every required
inverse uses an exact extended-GCD Bezout certificate and is re-multiplied.
Keep both actual CM roots and all primitive2n character embeddings,
under BOTH untwisted and quadratic-twisted Frobenius conventions.

Export every row (CM root, character embedding, generator/2 diamond,
both Frobenius roots and determinant). Enumerate all joint collisions,
all rational Frobenius1 systems, multiplicities, generator/inverse and
diamond2 conventions. An equal rational system is an exact polynomial
separation obstruction at that system, not a global map nonexistence proof.

For disjoint rational branch sets, export exact unit-gap Bezout certificates
and affine two-line projectors when each branch has one system. Verify
projector values and sum identity in both diamond conventions. If a gap
is nonunit, export its gcd obstruction. If there is only one rational line,
label it explicitly rather than inventing a second component. Keep empty
or collided outcomes. Fixed controls must reproduce the old n5 untwisted
collision and twisted7/8 values, plus the untwisted n11 conductor collision.
Reject corrupted roots, inverse coefficients and a wrong delta surrogate.

## Acceptance and limits

Exit0, exact marker, full six-case/two-twist outputs and all arithmetic/
control checks. Preserve failure/timeout evidence, no automatic retries.
No randomness, statistical sample, A/A or timing comparison: finite exact
identity/enumeration experiment. The six cases are fixed, not an exhaustive
search for curves. Primitive input primes n,p are trial-checked; large
target primality is a retained input boundary, not inferred from unit tests.
The modular-CM interpretation is the retained written bridge.

These are rational residual/local-unit recovery certificates. They do not
compute the full integral Hom lattice, quotient saturation, polarization,
actual larger-n collector images or a chosen Koblitz model isogeny.
No IC relation rank, unknown-scalar recovery or speed claim.

## Additional global denominator check, before data

For u=D^i,v=D^j, the two twisted/untwisted CM branch quadratics have
resultant 2(u-v)^2(2u²+3uv+2v²). Verify that identity algebraically in the
native coefficient calculation, and measure the equivalent Bezout
denominator (v-u)*2*(2u²+3uv+2v²) at EVERY primitive generator value D.
Export zero/nonunit witnesses, or the complete product norm modulo the
actual target modulus with an exact inverse Bezout certificate.
This distinguishes a rational-only separator from a global joint residual
obstruction; the twist does not change this global denominator.
A unit norm is a finite-algebra gate, not an evaluated larger-n map.
No global inverse polynomial or full integral Hom-lattice certificate is
claimed from the scalar norm alone.
