# First original-E0 API fixture: incomplete supplied span

Native build exits0; tests exit101 (14pass/1fail). The positive fixture initially
supplied only reference lifts at x0..63. `certify_from_points` correctly returns
IncompleteKernelSpan { found:2116, expected:4232 }. The raw failing output,
exact source hash and source patch are retained unchanged. This is an input
span failure in a development test, not a new scientific timeout or a solver
failure. No ambient order, scientific binary or protocol changed.

The subsequent distinct attempt adds an explicit trace-one coset to the
positive fixture and retains this proper span as an asserted negative control.
No test is skipped and complete4232-point independent checking is still required.
