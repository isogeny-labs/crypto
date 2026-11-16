#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Restricted pivot reuse through compression
2026-11-02 · Derived schedule / static checks · Numerical trials: zero

*Construction.* Removing a proved-zero prefix shifts each selected index by
j and restricts its cached column to the tail. The first column remains
nonzero, so the reduced deterministic selector chooses the same scalar/pair
pivot, including the secondary-diagonal branch.

#image("pipeline.svg",width:100%)

*Equality gate.* The independent checker verifies the compressor's skinny
factor identities and output generators. Equal displacement at fixed dimension
and distinct companion wraps determines the same operator. The helper creates
the rebound selection only after that check. The checker copy is byte-identical
to the frozen source. This runtime gate has not yet been executed on instances.

#table(columns:(2fr,2fr),inset:5pt,
 [Reduced selector branch],[Source queries removed],
 [First diagonal nonzero],[1],
 [Secondary diagonal or pair],[2],
 [Added compression-check products],[2qg(s+t)],
 [Added check modular reductions],[q(s+2g+t)])

Here q is restricted dimension, s input width, g first factor rank and t
compressed width. The checker uses plain integer products and reductions;
these derived schedules are separate from Meter field arithmetic. Each
removed source query has 4t+8 instrumented native products in the preserved
kernel, with dimension-dependent operand sizes. No net runtime gain is claimed.

*Preservation and costs.* Compression producer, pivot choice, paired formulas,
Schur path, zero-padding frames, early witness rank semantics and root checks
remain. Canonical cached-column tests, zero-prefix scans, metric-view matching,
(k+1)q column copies and metric slices are charged as native work. The original
search stays in the deletion trace; rebound search queries are zero.

*Provenance.* Source/meter/wrap/signature, dimensions, column shape, zero prefix,
nonzero first tail and metric view are checked. A one-use receipt reserves
rebinding before arithmetic. Compression equality is checked independently;
deletion and original caches still require trusted internal provenance.

*Validation.* Two full static substitution audits cover the checker copy and
complete driver integration; five Python files parse. Static inspection excludes
helper source calls and places verification before operator/selection creation.
Numerical, corruption, heap and performance trials remain zero. Funded tests
must compare pivots/root outputs, corrupt certificate components, cover scalar
tails and alternating frames, and independently check counters and lifetimes.

*Sources / next.* DERIVATION.md, reuse.py, driver\_reuse.py,
compression\_certificate.py and integration.json. Bostan et al.2017 supplies
the standard displacement conventions. Next consider sharing the checked
compression boundary and measure prefix incidence when funding permits.
