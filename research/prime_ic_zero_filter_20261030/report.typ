#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Candidate ideas for algebraic prime-field index calculus
2026-10-30 · proposals and exact finite controls; novelty not established

The proposed bridge combines PKM membership chains with weighted/exact
counting on the actual rational, subgroup-valid zero predicate. A compact
polynomial-value representation alone does not supply this predicate or an
algorithm for locating its zeros. No new IC algorithm, speedup or proof is
claimed. The preceding final-slot oracle remains a generic technique.

#image("integration.svg", width: 100%)

*C1 — Exact predicate contraction.* Feed PKM auxiliary-isogeny membership and
summation into a bounded exact counter. Audit decoded Boolean size, coefficient
algebra, precision and actual intermediate storage. Keep Kummer/Dickson controls
and their existing leading-form obstruction. The integration is unimplemented.

*C2 — Rational-chart-aware witnesses.* Combine count-guided prefix extraction
with a complete chart cover, rational lifts and subgroup membership. Avoid
whole-grid recovery only after demonstrating complete agreement with direct
small-field group addition, including poles and intermediate identity cases.
Saturation alone can discard valid charts. This combination is unimplemented.

*C3 — Target-parametric messages.* Reuse exact target-independent membership
messages across the relation queries of one solve. Existing coefficient
specialization is prior work; a new compact message representation is needed.
A table of all targets or tuples is not such a representation. Charge cold setup.

*Finite controls.* Nine native catalog/control tests and strict Clippy pass.
For the tested order-11 subgroup over F17, S3 values have rank3 while their
zero filters have rank5. These coordinate-field ranks are not useful relation
rank over F11. Small slices do not prove asymptotic rank growth or rule out
compact functional predicates. No performance experiment was run.

*Cost hurdle.* Under explicitly heuristic decomposition and useful-rank
assumptions, the README includes setup, failed queries, witness recovery and
quadratic matrix work. For the hypothetical m=4, B=r^(1/5), the oracle cost
divided by its useful-rank fraction must beat the r^(1/10) scale. This is a
requirement for a future solver, not a demonstrated rate.

*Sources and gaps.* PKM2016 sections3.1–3.4; Hexagon2610.00594v1,
2610.00613v1 and2610.00600v1; Kosters–Yeo1503.08001v3; existing prime-IC/tower
studies; a public S3-value-rank README. Source proofs and external timings were
not independently validated. A relevant2018 primary repository page returned403;
its full text remains unreviewed. Search absence does not prove novelty.
