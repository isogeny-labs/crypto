# Primary-source baseline correction

FLINT commit0fc4cb181f71be6ef48ae2f9a75cbbbf45d2ba1f explicitly documents fq_poly_roots with multiplicities as a wrapper over gr_poly_roots_finite_field. Source src/gr_poly/roots_finite_field.c, blob07b10ba5c4ed3d5e2c7f4bd8c074562c6b43b56f, first performs squarefree factorization and then reuses an initial trace/Legendre split to identify field-linear factors and partition them simultaneously.

For a binary squarefree polynomial g, T=X+X^2+...+X^(2^(n-1)) modulo g satisfies T^2+T=X^(2^n)+X. Therefore gcd(g,T) and gcd(g,T+1) are exactly the two disjoint field-root factors, already partitioned by trace. A separate complete Frobenius test followed by another first trace split repeats work. For odd q and nonzero roots, X^((q-1)/2) equal to1 or-1 similarly identifies and initially partitions all field roots; zero roots are handled separately. Standard squarefree factorization also groups different multiplicities into smaller factors before this step.

This is known production prior art, not a new algorithm discovered here. The planned Python direct/radical comparators are useful controls but do not by themselves demonstrate superiority over the strongest specialized native root finder. Before any such claim, add an explicit initial-split comparator and a matched native FLINT interface if available. python-flint was not installed in this runtime; no installation or FLINT timing occurred. This limitation does not block local proof or the already prepared diagnostic panels.

Primary URLs:
- https://github.com/flintlib/flint/blob/0fc4cb181f71be6ef48ae2f9a75cbbbf45d2ba1f/src/gr_poly/roots_finite_field.c
- https://github.com/flintlib/flint/blob/0fc4cb181f71be6ef48ae2f9a75cbbbf45d2ba1f/doc/src/fq_poly_factor/index.md

The genuine matched all-roots F4/F5/F6 release gate remains separate and unchanged.
