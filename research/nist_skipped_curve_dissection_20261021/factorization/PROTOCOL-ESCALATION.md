# ECM escalation after the PARI stage

The 600 second PARI search completed six targets and left three extension-
twist orders partial. ECM at B1=100000 has already produced proper factors
of both remaining P-224 targets. Those factors must pass native certificate
verification before being treated as prime.

Complete the original first ECM allocation of up to 100 curves per target,
continuing on a smaller composite cofactor when a proper factor is found.
Count attempted curves by the `Using B1=` records, including curves that
find a factor before stage 2. Do not restart the full 100-curve allocation
after every split.

For each target still composite after that allocation, use another attempt
at **B1=1000000**, up to **200 further curves**, with a **600 second process
limit per cofactor attempt**, two simultaneous factor workers and ECM
stage-2 memory capped at 1024 MB each. Save residues, curve parameters,
exact input and raw stdout/stderr. A successful split may be followed on
its smaller composite cofactor using only the remaining curve allocation.
All factors and products remain subject to independent native verification.

This is an authorized bounded continuation on the existing exact targets,
not a change in the scientific denominator or acceptance criteria. A
timeout or an exhausted curve allocation leaves the corresponding
cofactor unresolved. Further search after these allocations must be a new
recorded attempt with its own bounds.
