# Factor-base-reduced S3: preregistered continuation

2026-10-25; starting head115963f0bc4f7c61a8d4a46d55a05fbcadfa075b.
Run `reduced-s3-20261025`. Extend PR2381 normally; no second writer or PR.

## Hypothesis and new lead

Reduce the actual S3 polynomial modulo the two monic factor-base membership
equations **before** constructing Macaulay matrices. This preserves the ideal
but changes the supplied generators and potentially their highest homogeneous
parts. Test whether an actual twist comparison can then have both more useful
rows and a strictly smaller homogeneous cutoff or certified solver-relevant
degree. No positive outcome is assumed. This is finite generator preprocessing,
not a new prime-field IC asymptotic claim; the lead atlas's known prime-field
Semaev limitations remain in force.

For a two-root base with sum s and product t, use
gx=x^2-s*x+t and gy=y^2-s*y+t. The unique bidegree<(2,2) remainder is
h=A*x*y+B*(x+y)+C, with

    A=s^2-4*s*z-2*z^2-2*a
    B=-s*t+2*z*t+s*z^2-2*a*z-4*b
    C=t^2-2*t*z^2+a^2-4*b*z.

The candidate exact identity is

    S3 = (y^2-2*z*y+z^2)*gx
       + ((s-2*z)*x+z^2-t)*gy + h.

Prove it and the equality of generated ideals in Lean where feasible, with
a source-bound native identity certificate and false identity/source controls.
An identically zero remainder, nonzero constant and degree drops stay explicit.

## Frozen population and comparisons

Use every size2 case in the preceding frozen124-case S3 study: all88 eligible
small-field subgroup cases from the942-model population, plus all10 size2
F43 subset policies. Keep the exact curves, valid generators, targets and
signed factor-base points. Reject missing/extra input cases. The F43 subgroup
generator is the already proved image U=(24,26), not the older EC1 generator.

Compare all88 small-field main/twist prefixes. For F43, compare the full
Cartesian product of its ten main and ten twist bases (100 comparisons).
Thus there are188 comparisons. Check every nonzero subgroup target, every
available matched/inverse-transported base control, and exact equality of
accepted coefficient sets before/after reduction. No discarded nulls/losses.

For each arm/target, independently reconstruct the actual group points and
signed relations. Record before/after coefficient rank and all distinct rows.
A useful row is an actual rank increase, not a raw root or a zero row modulo r.
The first lexicographic comparison/target satisfying a joint useful-rank gain
and lower homogeneous cutoff is the primary proof fixture if one exists;
otherwise retain the bounded negative and choose a different next lead.

## Degree measurements and controls

For the reduced INPUT generators compute exact homogeneous leading parts,
their degree-D Macaulay matrices and Hilbert function for D=0,...,8. Report
the first empty graded piece under this explicit definition, not an inferred
F4/F5 runtime. For the affine generators build all degree<=D multiples up to8,
starting at their actual maximum degree, and record closure and unit-refutation
degrees as in the old protocol. Null caps remain null. Keep the unreduced
profiles as a separate reference; distinguish solving an instance from merely
refuting an unsuccessful target. If a particular solver bridge is not proved,
state that gap rather than relabel closure as its solving degree.

C++ may reuse unchanged prior producer arithmetic; independent Rust may reuse
the prior independent auditor arithmetic, with generic polynomial division and
batch RREF differing from the producer's closed remainder formula and incremental
elimination. Check rehashed false remainder, false degree, false useful rank and
missing-case controls. Keep actual point lifts and the previously verified
scalar-preserving maps; never use a discrete-log table to define a transfer.

## Budget, evidence and decision

Each native command is capped at600 seconds. No timing/speed result, paid
compute, benchmark/leaderboard promotion, registry change or new external
artifact upload is authorized by this protocol. Keep compact case records,
independent receipts, failures, source/input pins, Lean builds/axioms, and an
inspected report/PDF/diagram under fresh paths. Preserve all previous records.
Run relevant native replay/controls, proof gate, library and workflow metadata
checks before publishing implementation. Existing mathematical, Conductor,
review, CI/access and conflict/integration gates stay open.

A joint finite gain advances the explicit example and its degree statement;
it does not by itself establish stationary yield or lower complete work. A
pullback-reproducible gain is attributed to base/preprocessing choice and may
still count as a verified engineering improvement when its costs are measured.
An equal/no-gain result closes this bounded candidate and selects the next
encoding or proof item; it does not end the investigation.

The actual pre-edit shared Conductor command exited127 (CLI unavailable),
with no reservation or replacement coordinator claimed. Lead-atlas matches
were read before selecting this bounded preprocessing test. Python is limited
to metadata/presentation, never research arithmetic, harnesses or verifiers.
