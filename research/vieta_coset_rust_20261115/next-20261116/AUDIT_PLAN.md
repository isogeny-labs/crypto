# Prospective exact degree audit

Before executing native/tower_degree_audit.rs: six exact public fixtures, no clock/performance comparison. Bound60s, no retry without a diagnosed code defect, retain failed output.

- F17, two rational (X^2+1)/(2X) maps, terminal4: A2,t6,r14 and A1,t0,r0.
- F17, X^2, terminal4: A13,t0,r0 (nilpotent blocks).
- F17, X^2, terminal9: A9,t0,r0 (nonsplit quadratic blocks).
- F97, cubic rational map (2X^3+6X)/(6X^2+2), terminal29: A2,t6,r14.
- F97, quadratic then cubic rational maps, terminal58: A2,t6,r14.

Every path projector, partition of unity and coordinate identity must match as literal sparse polynomials at degreeS+1. First five cases also compute V_T by closure of the original generators under every permitted variable multiplication, using graded-leading-term sparse Gaussian elimination. All good-block coordinate actions and discarded projectors must lie in V_T; truncated quotient dimension must match the predicted local algebra dimension. The mixed case checks projector certificates only (closure deliberately not in this finite plan).

These checks use direct modular arithmetic, finite fiber scans and polynomial linear algebra separate from the candidate library. They are producer-owned checks; neither independent signed standing nor a proof for all maps/fields follows. The existing pinned hand argument is never rewritten.
