# Public 3-torsion gluing-module filter

Use the already frozen E16 whole curve/twist orders for the ten ordinary toy
fixtures and five NIST P-curves. The GP source contains their exact integers.
Recover p=(#E+#twist)/2-1 and t=p+1-#E. Over F3, enumerate all 2x2 matrices M
with det(M)=-1 and test M*A=-A*M for A=[0,-p;1,t]. Retain every row, including
zero counts. An invertible anti-intertwiner requires t=-t modulo 3. Cross-check
the toy positive/negative classification against the existing E11 module panel.

This is a companion-matrix applicability filter. It does not construct torsion
bases, a principally polarized quotient, a genus-two curve, divisor maps or a
decomposition algorithm. A positive matrix result is not an existence/cost
certificate for the required geometric construction. No NIST DLP or speedup is
tested. Keep the concrete field/Frobenius/Weil-pairing bridge explicit.

Freeze this protocol and the exact GP source before execution. Use native
PARI 2.17.2 in one process, a 4 GiB address-space bound and a 15-second hard
limit. Preserve stdout, stderr and status; a failed computation remains
unresolved. No wall time is admitted.
