# Primary-source follow-through: degree evidence and cofactor plumbing

This review checks interpretation of the existing campaign and its conditional
query support. It adds no experiment, theorem, novelty claim or admitted cost.
The originally requested larger cells and complete-price comparisons remain
unresolved. The active scientific replay and its frozen protocol are unchanged.

## Sources actually read

- Huang, Kosters, Yang and Yeo,
  [On the Last Fall Degree of Zero-Dimensional Weil Descent Systems](https://arxiv.org/abs/1505.02532v2):
  introduction/Theorem1.1, definition of the bounded closure and section6.
  Downloaded PDF SHA256
  `cffdbb8868ac4a4ee10ef4034136f862f41a77a00e67cdd6970397809f6fe36f`.
- Kosters and Yeo,
  [Notes on Summation Polynomials](https://arxiv.org/abs/1503.08001v3):
  introduction, Proposition4.2, Corollary4.11, Remark4.12 and the opening of
  section5. Downloaded PDF SHA256
  `b5ea5e4c5de7dcacf579e283168d9463b2b6977b031002ddee134a3a0d7868eb`.
- Existing source mechanisms: `src/cryptanalysis/koblitz_index_calculus.rs`
  trace-constraint option and `trace_bit_is_a_group_homomorphism_including_torsion`
  test; `src/cryptanalysis/ic_boundary.rs` subgroup-only base. The latter uses
  a trace membership test only for cofactor2 and scalar multiplication for
  the other cofactor cases.

These exact public PDFs were read locally. No third-party paper bytes are
republished with this note. Earlier SEC1 review remains scoped as recorded in
[the implementation note](PROJECTED_QUERY_LAW_IMPLEMENTATION_20261026.md).

## Consequences for the selected leads

Theorem1.1 requires an underlying zero-dimensional radical ideal and an
injective coordinate projection. Its bound still depends on the original
system's last fall degree, degree, variable count and number of solutions.
Section6 explicitly explains why the theorem does not directly determine
ECDLP summation-system complexity: the underlying multivariate system without
field equations need not be zero-dimensional. Adding field equations makes a
finite system, but does not by itself bound the theorem's other parameters
independently of the growing curve. Our finite low-degree observations therefore
remain empirical observations. Full closure at growing sizes, complete branch
coverage, solution extraction/lifting and all costs remain required.

Kosters--Yeo derive a linear trace consequence for ordinary binary curves.
It explains why the first fall degree of the descended third summation
polynomial is usually2, even when the degree reached by a Groebner solver is
higher. Their reported n40 computation exceeds38GB and has degree at least5;
their section5 describes a completed n45 example at degree5 using about126GB.
Those are the authors' finite experiments, not new campaign measurements or a
universal lower bound. A first fall, last fall, refutation degree, solving
degree and complete relation cost must remain distinct in this campaign.
The NP-completeness statement for growing summation-polynomial arity is also
not a proof that ECDLP is NP-complete or that our fixed cells are hard.

The repository already implements the trace consequence as an optional
additional equation; it retains the original equations. It also has the
cofactor2 subgroup-membership specialization. No duplicate trace mechanism was
added and no speedup is attributed to this review. In particular, the
cofactor4232E0 fixture cannot inherit the cofactor2 membership shortcut.
The new kernel constructor is ordinary supplied-generator subgroup generation,
conditional on certified ambient order. It adds no relation oracle, logarithm
discovery, random sampler, full IC cost or actual E0 certificate.

## Remaining primary-source access gap

OpenAlex identifies Gaudry's DOI10.1016/j.jsc.2008.08.005 and its HAL/publisher
locations. The official publisher PDF endpoint returned403 in this follow-up;
the earlier HAL document request also returned403. This does not establish
anything negative about that method. This review still does not claim to have
read Gaudry's full paper; no new conclusion or novelty claim depends on it.
The measured historical quartic row and extrapolation retain their existing
source-specific evidence, separate from this literature note.

## Newer repository evidence: four-summand narrow engines

The current base also contains the separate
[amendment2](../ic_m3_algebraic_vs_mitm_20261014/AMENDMENT_2.md) and its
[native analysis](../ic_m3_algebraic_vs_mitm_20261014/amendment-2/ANALYSIS.md)
from mergedPR2335. Their Git blobs at source974f2e9cd46b are respectively
`3affa96d426383874a564b75b4f3a45990d01fe2` and
`25f2f059beb1881cde580944a1526b6a3316f4ac`.

This is existing source-reported evidence, not independently rerun here. Its
registered `wide-vars` build extends the original narrow Boolean F4/F6/SAT
interfaces to128mask bits. F4 and F6-IC finish the n23/n29 four-summand cells
(70/90unknowns); n31 remains a censored lower bound, and SAT's slope remains
undetermined. The report fits wall-clock cost per relation over its six
completed cells:0.644/0.645versus0.298for its same-cell MITM relation floor.
That floor is not Pollard rho. These different inputs, bases, controls and
units are not a measurement of our GP-plus-closure stack or a full IC comparison.
They are a concrete reason to keep full relation cost and growing parameters
as the decisive tests instead of treating width support as a speed improvement.

The library's128-bit Boolean `Mask` feature is separate from the quartic
u32/u128 field/scalar core. The present campaign already has its own explicit
u128 Boolean-chain/sparse-closure and HKY frontends under
`examples/ic_algebraic_followup/{wide,hky}.rs`; its original default64-bit
refusal remains historical evidence. No claim that all current Boolean engines
still cap at64is made. Wider masks neither fix the existing64-bit checkpoint
format nor resolve the requested large-memory jobs, complete higher-degree
closures or eight-target wide quartic pipeline.
