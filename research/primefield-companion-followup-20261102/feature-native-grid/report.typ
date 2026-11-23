#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Native prescribed elliptic source grid
2026-11-03 · Exact public-generator checks · Individual-grid component

The native grid preserves the original public multiplier layout, Jacobian
addition/doubling and one-inversion affine conversion. Its inputs are the public
curve and point plus the source partition. It performs no unknown-scalar search.

#image("pipeline.svg", width: 100%)

*Construction.* Starting at the half-order multiple and a public stride,
the code builds the original two progressions and optional tail point. Identity,
vertical-pair, doubling and equivalent-scaled-coordinate cases are handled
explicitly. The original partition and public point are validated.

*Checked evidence.* At source6c5f5b983d62, one standalone rustc1.99.1 build and
one invocation passed all3 test methods. All71 slots across the three original
grids match saved labels, Jacobian coordinates, affine coordinates and inverse-Z
values exactly. Independent affine known-multiplier calculations agree. Additional
checks cover exceptional points, rescaling, singleton conversion and rejection.

*Conversion cost.* For N nonempty finite points, prefix and reverse products
plus coordinate conversion use7N-3 field products and one inversion, excluding
validation/group construction. This preserves the original batch method; it is
an algebraic count, not a measured speed improvement. Allocation and native
integer costs remain. Empty/infinite batches are rejected.

*Scope.* The backend supports p up to half the u64 maximum; caller supplies
primality and subgroup order. Fixed checks do not exhaust all points or field
sizes. No fresh sign oracle, recovery cohort, cloud job or benchmark ran.

*Next.* Expose projective-only output and mixed-stride shared-window updates,
then connect division/seed recurrences and the resultant stage. Repeating
individual conversions must not replace the original shared-window accounting.
The complete4,563-case cohort remains unchanged and unlaunched.

*Evidence.* Source and input locks, fixtures.tsv, slots.tsv, native build/test
receipt and logs, and the byte-preserved original geometry source. See
DERIVATION.md for exact formulas, costs, test limits and next integration.

*Reference.* #link("https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html")[EFD Jacobian formulas]
and original shared-projective-grid/geometry.py. Established formulas are reused;
no fastest-formula or novelty claim is made.
