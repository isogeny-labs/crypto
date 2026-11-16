# Canonical three-chart union

Compute the homogeneous direction intersection C of all three affine charts.
Extend C to a basis C+E_i of each chart. With D=max_i|E_i|, use shared gamma,
Dextra coordinates t and a two-bit selector with codes0,1,2. Code3is forbidden;
coordinates beyond |E_i|must be zero in branch i. Every valid encoding maps
injectively to exactly one original point. Chart images are disjoint by the
leading flags, so the union is exact. Padding guards are essential to counts.

The piecewise map is p0+Cgamma+E0t+s0(p0+p1)+s1(p0+p2)
 +sum_j(s0*t_j)(E0_j+E1_j)+sum_j(s1*t_j)(E0_j+E2_j).
It is formed as affine combinations of actual primary/selector-product wires,
not independent new old-qbits. Shared XORsynthesis preserves their functions.
Destination guards are appended only after source-node import/pruning so
old/new node number spaces are not confused.

Tiny oracles enumerate every original/encoded point, check invalid selectors,
padding, injectivity, complete union images and complete root sets. Actual
controls cover each stratum and all old predicates, bounds and guards. This
is a necessary prefix-root cover in one fixed norm family, not an original
339-variable all-root certificate. One core4GiB90s,78/84s finalization checks;
known affine decomposition/logic synthesis is not novelty certification.
