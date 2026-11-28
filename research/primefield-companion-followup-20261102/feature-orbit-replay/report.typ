#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= One character residue per retained orbit
T-1815 · 2026-11-07 · exact native producer checks

The lifted candidate ideal is stable under multiplication by the selected root of unity. Its character residues obey
$ R_(a zeta^j)(X)=zeta^(j h) R_(a) lr((zeta^(-j)X)), quad h=(r-1)/2. $
One modular power therefore supplies an orbit; coefficient factors advance the residue. The character factor remains essential for odd d. Stability is required of the original lifted ideal, not the shrinking candidate polynomial.

#image("flow.svg", width: 100%)

*Exact checks.* 5,838 residue coefficient equalities;5,838 replay-prefix equalities;1,110 wrong-character-phase controls;356 zero-containing subsets. Invalid stability, orbit ordering and cap controls reject. Validation01 passed and is preserved before the final coefficient-factor implementation in validation02.

*Public equivalence.* All63 inherited public targets recover identically under both adapters, with matching routes, lifted degrees, retained counts and query fields. Each uses511 source calls. The candidate computes78 retained residue powers for180 retained constraints. Extra ordinary signs retain their earlier replay path.

*Cost distinction.* Shared residues use the original degree N. The reference exponentiates on successively smaller degrees. Fewer exponentiations therefore do not establish less total field work or elapsed time. Charge coefficient rotations, gcds, inversions and an additional O(N) workspace; keep both paths available.

*Evidence.* generation.json binds the previous source, preserved reference and generated adapter. Validation receipts pin dependencies and retain raw output. DERIVATION.md proves ideal stability, residue transport and prefix equivalence. The original4,563-case cohort and source restrictions are unchanged. No cloud or timing study, full formalization or independent signed verdict.

*Next:* derive a degree-aware selection rule or a cheaper shrinking stable envelope before asserting full-cost improvement. Source batching remains separate.

Prior work: local weighted-mode and bounded image-pullback components; FLINT nmod_poly conventions at flintlib.org/doc/nmod_poly/index.html. KB tools unavailable; primary/local fallback recorded. Quotient-ring substitution is standard algebra.
