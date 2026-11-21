#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Workspace-adaptive polynomial projection
T-1822 · 2026-11-07 · derived algebra and unsigned producer checks

The adaptive recovery constructor now selects any legal positive polynomial block width under its short-table cap. A square-root-sized table is no longer required. The quotient-annihilation certificate and recovered image polynomial remain unchanged.

#image("flow.svg", width: 100%)

*Declared objective.* For prefix L, degree bound d=min(n,L), width q, let a=ceil(L/q) and b=ceil((d+1)/q). The short powers, projection and certificate require at most q+a+b-2 quotient products and exactly a projection convolutions. Select the legal q minimizing q+2a+b-2; ties prefer less table storage. This is a structural call bound, not a total-field-operation or timing optimum.

*Derived continuum.* Put K=floor(W/n), at least one. Across doubling prefixes stopping below4t, projection and certification cost O((sqrt(t)+t/K) M(n)+nt). Including setup and recurrence synthesis gives O(M(n)(log m+sqrt(t)+t/K)+nt+t²+t log r). Table storage is bounded by W words and O(n sqrt(t)); other arrays and backend workspace are separate. Validation, output, parent checks and source evaluation remain additionally charged.

*Native checks.* 23,408 planner configurations satisfy admissibility, objective optimality and tie handling. 1,091 arithmetic configurations match direct power sums and full Horner remainders across workspace choices. Twenty-two below-one-row cases reject. A named F7 example formerly rejected by the square-root table cap now completes with one row and the same exact recurrence.

*Recovery integration.* The 6,522 small cases and three larger cases preserve complete image, envelope, moment and rejection-count results under both generous storage and exactly one row. All 63 public recoveries preserve scalar, route and query fields, with 511 source calls. The adapter uses a declared 64-coefficient-word short-table allowance on these fixtures. This does not measure total memory. Original 4,563-case cohort unchanged; no cloud work.

*Evidence and next step.* validation-01 compiled and passed without assertion failures; warnings retained. Receipts bind source dependencies, compiler and commands. Producer evidence has no independent signed verdict. See DERIVATION.md for assumptions and proof. Official FLINT modular-composition documentation supplies background; its generated-prose notice is recorded, and local identities are checked against exact references. KB unavailable. Next account for field arithmetic and source cost when choosing among recovery constructors.
