# Independent full-algebra substitution control

Direct generation succeeded from source819d8e856bbf/checkpoint20261116T025753Z. Before any direct solve, independently parse its twelve equations using the existing exact verifier with only variable names adapted. Load original D1 ordered parametrization, repeat every original residual, and reconstruct all six symmetric coefficients in its full degree12algebra. Compute the new saturation inverse exactly modulo the original eliminant. Require all twelve direct equations vanish identically in that complete algebra. Reject adding one to the interpolation constant. This verifies inclusion of the entire known inverse fiber, not absence of extra direct solutions or a runtime gain.

Freeze generated system, control/verifier/quotient sources, original equations/parametrization and binary; commit/checkpoint/gate before one execution. Retain failure; do not solve until control passes. No new expected scalar or generated-input change.
