# Branch coefficient replay: prospective source review

The unchanged branch-coefficient theorem and its proposed replay were reviewed against the frozen original source and prior scoped review. The review found no blocking source defect. No compiler, experiment or candidate payload was run by this reviewer. The current packet remains prospective until publication, live campaign accounting and coordinator release.

The theorem assumes a field, a separable normalized quintic, a branch root, a nonzero coefficient determinant, two support divisibility congruences, and a vanishing product of the two supports and residual at that root. It concludes that all three polynomials are nonzero and their root multiplicities sum to one. The implementation invokes the earlier support multiplicity theorem with its hypotheses, rather than assuming the conclusion or dropping separability.

The independent rational fixture has F=X^5-X, u=X^2+1, v=X^2-1, residual U=-16X and determinant b=-4. Direct symbolic inspection gives (-25X^3)F+(5X^4-4)F'=4, a nonzero Bezout constant supplying separability over Q. At a=0 the supports have nonzero values and the residual has a simple root. Equal supports give determinant zero; a=2 gives F(2)=30. These are source-level calculations, not a compiler result.

The planned replay has 18 ordered Lean calls: ten absent canonical prerequisites, four retained norm/branch dependencies, the target theorem, independent controls, axiom readback and the expected mathematical failure. Thirteen named declarations are audited. Every direct import was checked against an earlier fresh module or an exact bound cached import. Packet file hashes and Python syntax were checked without executing either runner.

The expected-failure control asserts a=2 is a branch root. The runner requires one unsolved-goals error with a False goal, rejects syntax, import, resource and elaboration errors, and requires empty stderr. This is a prospective expectation and could fail elaboration; any such outcome must be retained as a failed attempt.

The adaptation retains current-host ownership, invocation-specific cleanup, pinned runtime and dependency pre/post checks. It uses one CPU, 8 GiB and a 3600-second maximum reservation split 900/1800/900 for precheck/compiler/postcheck. The campaign ceiling remains 43200 worker seconds and the deadline remains 2026-11-10 05:11:39 UTC. The release template intentionally lacks publication, live lease and current debit bindings. Those must be supplied after the preceding payload is terminal.

The exact source and freeze hashes, import mapping and limits are recorded in review.json and import-review.json. This review does not establish successful elaboration or an official mathematical-status promotion. Shared-cache pre/post checks preserve the stated custody boundary but do not exclude transient concurrent changes.
