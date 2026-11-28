# Exact H6 unit elimination: source qualification first

Continue local Rust work in lane PR #5094. Fiber-pilot-01 is consumed and
its raw references stay byte-equal. Do not execute saved F4 fibers under
this source-only plan. No paid compute or changed production default.

Check the actual last equation for the exact shape
`-2*b + 2*r*s + c = 0`, with b=c0.im, r=t2.re,s=t2.im.
In odd characteristic b=r*s+c/2 is a global substitution, without an
exceptional denominator or discarded branch. Substitute into the other
six equations, preserving their scalar, then fix c0.re. The result has
six variables and degree at most four. Projection/lifting is an exact
bijection; degree growth may increase F4 cost.

Implement an opt-in Rust module beside the existing constructor. Count
coefficient products/additions in construction, specialization and lift;
report them separately from the existing F4/root counters. Map/allocation,
word and evaluation coverage remains partial. Explicitly reject malformed
pivots, fields, shapes and coordinates. Preserve original norm/support and
Jacobian checking after lift, cooperative deadlines and attempted work.

Before any saved fiber call, qualify substitution against original numeric
equations on synthetic off-variety points including nonzero Im(H6), retained
positive normalization/support/group replay, malformed-pivot/refusal cases,
six-variable known-shape full/plain/traced roots and expired deadlines.
Build/lint exact source with pinned native dependencies; scoped compilation
does not imply a clean whole-root build.

After qualification, freeze a separate protocol with exact source/binary
and input hashes: same369(control),0..7 development, unit-plain then
unit-traced18 calls, existing frozen full references for validation only.
Do not re-solve the unchanged baseline. Positive mismatch stops remaining
calls; all later failures are retained. Prediction: lower variable count
may lower recorded solver products; falsification is equal/higher products
or failure to reproduce complete references. No wall-speed claim. Limits
8GiB/360s process and10s shared per case, native local CPU0. Final protocol
and source publication must precede this count pilot.

Sources checked: existing jv_nagao_unit_{fast_,}elimination.rs, H6 norm
identity and fiber-pilot-01 outputs, prior primary Joux–Vitse and Sparse
FGLM review. KB unavailable; no novelty claim. Tracing and unit substitution
cannot change which development functions have admissible graph support.
