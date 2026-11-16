# Public F631 input-coordinate gate

Use the already frozen anchors01 calibration and fresh alignment matrix from hec_calibration631_20261115. Read source public D1 and B, kernel0 and alpha0 through the existing F631 source evaluator. Do not refit the matrix, rerun a calibration fiber, supply expected images, or scan scalar multiples for a DLP.

For each original input C, try offsets 1 through 4 in order: evaluate C+offset*base and its adjacent translate by base. Apply the recorded rank-8 matrix. Accept the first pair with both source charts supported and nonzero primary coordinate zero-index entry. Retain all exclusions. Assert the source base is killed by the registered prime. Emit two coordinate vectors and the offset for each input into a new directory. Failure at the fixed cap is retained without changing the policy.

Freeze source, lock, all reused source dependencies, setup inputs, calibration, matrix and release binary hashes; commit and checkpoint/gate before one execution. Record stdout/stderr/exit. This gate establishes supported input coordinates only, conditional on the existing finite alignment evidence. It does not prove inverse classes, a global map, DLP recovery or a runtime gain. Actual inverse generation must replace the old F127 constants and control points separately.
