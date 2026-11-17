#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Native mixed-stride shared source window
2026-11-03 · Exact geometry contract checks · One shared conversion

The native window now preserves the original three initialization branches,
updates lanes with prepared affine strides, and converts all finite lanes in
one batch. It accepts public shifted points; no scalar labels enter the engine.

#image("pipeline.svg", width: 100%)

*Checked behavior.* At source26e63a0422bc, one standalone rustc1.99.1 build and
one invocation passed all4 methods. The four original windows at O,P,2P,-P
use64 public shifts and match1426 saved finite slots exactly in offset, lane,
multiplier, projective coordinates, affine coordinates and inverse-Z. Identity
snapshots and initialization modes agree. Nonempty windows use one conversion;
the identity-only singleton skips it. Prepared reuse, bad prefixes and mixed
exceptional cases also pass.

*Accounting.* Target-independent stride preparation includes a separate
conversion. A nonempty target window converts N finite lanes with7N-3 field
products and one inversion, preserving the original method. Public-prefix
validation adds group/curve/cross-product work and must be charged. Complete
snapshots, allocations and copies remain. No timing or total-cost gain is measured.

*Public boundary.* Prepared state borrows its Curve and is immutable during
window calls. Supplied points are checked as Q,Q+P,... without per-shift
inversion. Prime and subgroup premises remain caller obligations; arithmetic
is limited to p at most half the u64 maximum. Output slots are ordinary data,
not independent certificates.

*Next.* Join normalized seed/progression and resultant evaluation to the
checked scalar/jet cores. Preserve product-tree and multipoint structure rather
than replacing it by a direct quadratic product. Then connect the native decoder
and admit the unchanged full4,563-case runtime/cohort.

*Scope and evidence.* No sign oracle, resultant, scalar recovery panel or cloud
job ran. Source/reference/fixture locks, original-file hashes, projection script,
receipt and raw logs are preserved. DERIVATION.md gives the exact scope and costs.
The previous native grid prefix is byte-preserved in this component.

*References.* Original mixed-projective-strides-v2 window and mixed addition;
checked native grid; #link("https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html")[EFD Jacobian formulas].
No new group formula or fastest-operation claim is made.
