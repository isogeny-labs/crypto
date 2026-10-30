# Original-E0 constructed kernel: native API conformance passes

Native build/test exits0/0 and all15tests pass. Exact fixed public parameters
come from the earlier fixture-subgroups-enumerated-20261025 receipt. That
recorded ambient order is an input; no new curve cardinality is claimed.

Reference lifts of x0..63 alone generate2116points and must fail the complete
4232-point constructor. Adding reference lifts of trace_one XOR x0..63 yields
the complete4232distinct-point census. Every point independently satisfies the
curve equation and [4232]P=O in BinaryCurve arithmetic.36fixed known-target
query/recovery cases also independently agree. Known supplied logs are test
inputs; no unknown logarithm or factor-base relation is discovered.

The failed original positive fixture is kept in the sibling first-attempt
receipt. Here it is an explicit negative control alongside the full positive
case. This is development API correctness; no query-yield, speed, full price,
new scientific run or general formal counting claim is made. Ordinary Cargo
and full-library/hosted checks remain separately scoped.
