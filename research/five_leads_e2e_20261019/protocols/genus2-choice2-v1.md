# Genus-2 two-choice collector: frozen before measured cohort

Question: does component-directed choice of the next walk state lower the
complete cost to a useful target dependence, relative to the v2 collector?

Use the seven original exact-order specs in
`research/three_leads_e2e_20261018/genus-order-fixtures-v1/order-genus-{16,20,24,28,32,36,40}.json`.
Retain every curve, sixty targets per curve, target seeds, measurement seeds,
one round and trial caps: 21 curves / 1,260 one-target workloads. Add
`ic.three_leads_genus2_choice2_v1` alongside the unchanged incumbent, source
plain/negation rho and destination generic rho. No candidate selection is
based on these held-out targets. The fixed two-choice rule is the hypothesis;
there is no claimed prior training win.

At each continuation evaluate two distinct seeded jump choices. Both point
additions, scalar labels, splitting tests, roots and valid new relations are
charged. Insert both valid rows immediately; choose the next state by the
pre-insertion component score (cycle closure first, then component size), with
stable first-choice ties. This is a rank heuristic, not a proof that a cycle
solves. Only the original signed elimination or a charged group identity
with a nonzero target coefficient extracts a scalar. Generic identity returns
are separately disclosed. Duplicate walk states trigger charged fresh starts.
The cap counts prospective divisors, so it is the same candidate-trial budget
as the incumbent, not twice that budget. Do not replace an exhausted workload.

Charge construction, both transfers, jump generation, every restart, relation
search, roots, graph/scalar work, component-policy probes and source answer
verification. Retain M/S/I and every native counter. Report the twelve prices
S/M={0,.5,1}, I/M={1,10,100,1000}, plus the original native sensitivity band
{0,.1,1} source-addition equivalents symmetrically for every arm. Ratios are
ratios of complete costs on matched targets; report per size and curve as well
as the whole cohort, all failures and source/destination memory diagnostics.

Every successful scalar must be checked on the source in the adapter and by
the ecbench parent; audit and replay all records. Preserve old-method outputs.
Operation counts can run at L0; no qualified wall-time or fastest-tool claim
without L2/A/A and the strongest available external comparator. A loss is a
valid end-to-end result, and an IC improvement does not imply beating rho.
