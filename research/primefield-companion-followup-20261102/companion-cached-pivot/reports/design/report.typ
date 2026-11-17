#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Reuse pivot-search queries in column transport
2026-11-02 · Derived operation schedule · Syntax/static checks only

*Construction.* The scalar-pivot search already queries columns until the
first nonzero c\_j=Ae\_j. Retain that column and the established zero prefix
to avoid duplicate work in the two-query column-swap primitive.

#image("pipeline.svg",width:100%)
The three middle boxes are mutually exclusive branches. All continue to
the same row-transport and solve/witness path.

*Identity.* For j>0 and u=e\_0−e\_j, A u=−c\_j. The second generator
correction −A Z\_b u equals c\_(j+1)−c\_1 for an interior column and
−c\_1+b c\_0 for the last column. Here c\_0=0, and c\_1 is cached when j=1
or already known zero when j>1. Thus the interior branch needs only the
following column; the last-column branch needs no query. For n=2,j=1,
both corrections are the same cached negative column.

*Derived transport counts, excluding query internals.*
#table(columns:(2fr,1fr,1fr,1fr,1.4fr),inset:5pt,
  [Branch],[Old queries],[New queries],[New products],[New add/sub],
  [j=0],[0],[0],[0],[0],
  [j=1 interior],[2],[1],[0],[2n+1],
  [j>1 interior],[2],[1],[0],[n+1],
  [Last j>0],[2],[0],[0],[n+1])

For every old nonidentity branch, explicit scalar work was two products
and n+1 add/sub operations. The first interior branch now adds n
subtractions. Search query count and entry-test order are unchanged.
Copying the canonical wrap b eliminates multiplication by one. Generator
width remains s+2, including a zero correction when applicable.

*Integration and provenance.* A separate solve\_cached.py changes only
the search return and transport call, plus one import. Normalized AST
comparison checks every other executable statement against the preserved
original solver. The internal cache is tied to the source object/signature
and is consumed once. It retains one length-n vector. In-place source
mutation and forged cache objects are outside this internal contract; the
cache is not an independently verified public certificate.

*Scope.* Nonsingular nodes have j=0, so this optimization targets singular
intermediate systems with zero-prefix columns. Query vectors differ from
the reference: native bit-length volumes and runtime must be evaluated,
not inferred from call counts. No numerical or performance experiment ran.

*Next checks.* All branch boundaries, n=2, wrap values, pivot rows,
wrong-source/reuse/malformed cache controls, exact scalar/query counters,
reference generator equality and both solve/witness outcomes. Preserve
the full original cohort; charge cached vectors and audit storage.

*Sources.* cached\_transport.py, solve\_cached.py, check\_preparation.py and
DERIVATION.md; original companion-rank-solver column-swap identity.
Bostan et al. (2017), Section 6, DOI 10.1137/16M1062855 supplies prior
structured-solve context; no randomized or fast bound is imported.
