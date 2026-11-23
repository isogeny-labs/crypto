# Pic64 counter coverage gap found in this run

The optimized setup's FqCounter is exactly zero despite enumerating3783projective candidate points and evaluating66conic value vectors. This is expected from the source and is not zero field work:

- native-smith-transport/pic.rs:627 std_values is explicitly uncounted and calls mulu/powu.
- pic.rs:605 eval_f is an uncounted helper, used by rational_points_bruteforce at1405.
- pic.rs:592 normalize_point calls invu/mulu, also uncounted.
- mulu, invu, powu and constructor/free-function helper paths must be audited before promoting any total count.

The original exhaustive arm also executes these helpers, and its counted Pic group verification remains a real extra workload. Nevertheless the reported Pic-counter ratio is not a setup-time ratio or a complete field-operation ratio. Both native16-column vectors and Pic vectors are preserved without blind summation. Incremental rank collection adds ell-field work that was absent from the exhaustive setup.

A successor must instrument omitted scalar paths without double-counting counted wrappers, preserve arithmetic/class keys/answers/control flow under paired calibration, and separate constructor/model search/transport/setup/online/verification phases. Native Fp convolution products and REDC vectors, ell operations and host overhead remain distinct units. Complete matched original-IC/rho comparison remains unperformed. Do not promote this engineering component result to the10xgoal.
