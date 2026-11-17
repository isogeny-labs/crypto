#set page(paper:"a4",margin:16mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.65em)
#text(size:21pt,weight:"bold")[Target-preserving relation filtering]

Exact algebraic gate · 15 November 2026 · established filtering methods

The next selector must preserve weighted equations and the target coordinate.
Unsigned graph topology alone cannot certify target recoverability over odd fields.

#image("FLOW.svg",width:100%)

*Projection theorem.* Split the relation matrix into off-base columns B and core columns C.
Eliminating the off-base variables leaves all equations yᵀ C x = yᵀ d with yᵀ B = 0.
This is sufficient as well as necessary by finite-field image/annihilator duality.
There are at most M − rank(B) independent surviving constraints. A cycle may pin an
off-base component instead of producing a pure target constraint.

*Safe singleton rule.* A non-target variable appearing in exactly one row can be solved
uniquely from that row for any assignment of the other variables. Remove its row, retain
the pivot witness, and reconstruct in reverse order. This preserves all feasible target
values, including an empty set for inconsistent systems. Never peel the target itself.

#table(columns:(2.1fr,1fr,2.2fr),
 [Gate],[Count],[Outcome],
 [Exhaustive F₅ systems],[384],[All feasible target sets preserved],
 [Inconsistent systems included],[246],[Empty target sets preserved],
 [Consistent sparse prefixes],[8,064],[Same target result before/after],
 [Rows peeled across prefix gates],[28,473],[Count only; no speed inference],
 [Same-support negative control],[2 systems],[Target sets differ as predicted])

*Scope.* Finite synthetic systems over 5, 7 and 101, with duplicate terms, cancellations,
isolated columns, zero rows, target-only constraints and reverse-extension checks.
No real-curve filtering timing or 10× result follows. Existing solve_fast is the large
prefix reference; exhaustive enumeration is the separate small-system oracle.

*Prior art.* Cavallar, Strategies in Filtering in the Number Field Sieve, Section 2.1/2.2;
singleton pruning and structured merging are known. The F₂ dependency setting does not
justify ignoring odd-field weights or a designated target. The Avanzi–Thériault publisher
abstract describes joint harvesting/base-size selection; its full text was not retrieved.

*Cost boundary.* The preceding complete point pilot loses all six fixture medians. Under
its fixed-work diagnostic, free setup plus free forest reaches at most 2.190×. A selection-only
micro-optimization cannot establish 10× under that assumption. A successor needs a joint
useful-relation supply/cost case with weighted rank and all filtering costs charged.

Protocol f847c25ba8 preceded code. Rust execution, exact enumeration and source hashes
are retained beside the report. These are producer checks and a mathematical boundary,
not an independent proof audit, novelty claim or performance comparison.
