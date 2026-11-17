#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Complete native wiring: controlled validation

1 November 2026. The native runner now connects canonical coverage tracking,
codomain moment assembly, complete point evaluation and known-answer
logarithm recovery on both endpoints. Two ordinary-curve fixtures pass
independent PARI comparison. These are controls, not the requested 80-bit
crossing or performance measurements.

#image("complete-pipeline.svg",width:100%)

== Actual execution

Over p=71, degree 19, q=3: all 9 half-kernel representatives are covered,
the quotient has short coefficients 52 and 17, six evaluated inputs match
PARI, and two scalar pairs recover on both endpoints.

Over p=6666679, degree 10009, q=37: all 5004 half-kernel representatives
are covered, the quotient has short coefficients 5344836 and 3950273,
36 evaluated inputs match PARI, and 32 scalar pairs recover on both
endpoints. The source coefficients in both fixtures are 5 and 5.

The 42 checked inputs include repeated bases and six kernel/infinity
controls. The runner checks ordinary trace, exact kernel order, equal group
order, nonzero transported bases, subgroup order and known-scalar
homomorphism identities. Raw per-target coordinates and both recovered
answers are retained; no wall-time or peak-RSS measurements were taken.

== Coverage, failures and provenance

The controller rejects incomplete schedules, gaps, duplicate rectangles,
wrong extents, missing leftovers and wrong leftover ordinals. It refuses
quotient/image assembly until all expected contributions are present.
Kernel collisions use polynomial gcd checks; infinity and kernel inputs
map to infinity. Bounded jets call the existing native log-norm helper.

An initial q=2 assumption for p=71 failed the subgroup-divisor check.
Independent PARI found group order 57=3×19. The corrected q=3 run passed;
the failure and both successful replay versions are retained. Native source,
binary and raw receipt hashes are in `../complete-pipeline/checks.json`.

== Target status and remaining work

The actual degree 729845429276318409687041 is unchanged. Its fixed stored
prefix covers only 4096 baby roots; 427154957034 are required. The inventory
gate returns exit 3 with null quotient, images and endpoint timings. It is
not a timed failed construction or a proof of infeasibility.

The complete-fixture runner regenerates bounded blocks. Production durable
S3-manifest coverage and restart are not yet integrated; total work/I/O
feasibility remains unestablished. Complete target data and a feasible
native schedule must precede the requested paired EKS timing protocol.
Only PARI elllog is exercised here; no index-calculus arm is implemented.

Sources: archived full-map/Sage 10.9 contract, existing CM CONSTRUCTOR.md,
FLINT polynomial APIs and PARI elliptic-isogeny/logarithm APIs. Detailed
commands, identities, raw records and exact limitations are in
`../complete-pipeline/README.md`. No conductor-gap or benchmark row is
promoted by these fixture controls.
