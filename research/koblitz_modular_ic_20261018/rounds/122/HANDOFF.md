# R122 handoff

R122 closes the last repeated local edge in the four x=0 base-place program. Its corrected source was committed at `42032ff4b` before the accepted replay. Result SHA-256: `49710620cb89fb763fe13f952f64cfdc725094d87786c991d844a331fdd840fa`.

The y0/e3 edge splits one V correction of order 2 and seven of order 2/7. Together with R119–R121, each of the four base places now has a simple and a tame-seven positive branch near each of the two F4 lambda residues. The next job is a normalized branch census and N51 valuation, using exact local expansions or valuation bounds with explicit cutoffs. Do not infer the global divisor solely from the polygons.

The first diagnostic run failed an overstrict distinct-conjugate assertion; `delta=1` is Frobenius-fixed. The corrected source was committed, then replayed. Independent F4 table arithmetic, conjugacy, byte replay, changed-input rejection, generated Lean finite polygons and visual PDF inspection passed.
