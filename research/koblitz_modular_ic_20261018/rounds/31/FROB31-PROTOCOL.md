# FROB31 prospective closed-point and formal-divisor refinement

Read only the accepted REVERSE31 point records. Their original-equation,
unit, torsion and diamond checks are controls and will not be rerun. For each
record factor its actual quadratic L over the retained coefficient field,
require product/separability completeness, and treat EVERY irreducible factor.
Use descent coordinates (b,c,Z,(g1+bZ)/A), obtained by dropping the explicitly
recorded AS auxiliary T terms from the twist record. Untwisted coordinates
describe the original curve; twisted coordinates describe its diamond twist.

On each residue field compute all ten diamond translates. Repeatedly square
the complete coordinate tuple, find its least Frobenius period at most2d,
and partition the translates by actual Frobenius equality. Closed-point degree
means this least period, not the kernel-field degree. Evaluate the integral
group-ring operator (1-T^5)(5-sum(i=0..4)T^(2i)) on these CLOSED POINTS by
aggregating its signed coefficients per Frobenius class. Report zero or
nonzero as a FORMAL DIVISOR, never as a Picard class. This computation cannot
prove nonprincipality, modular-factor image or transfer saturation.

Positive controls: Frobenius closes by2d and every translate is assigned to
one complete class. Check degree invariance under diamond, each coefficient
sum totals0, and formal support/weights explicitly. Fixed deterministic finite
enumeration; no sampling or performance assertion. A/A/statistics/rho not
applicable. Native4GiB/1200seconds per degree; same durable gate, no retries
on mathematical failures, preserve all native outputs and exact source.
