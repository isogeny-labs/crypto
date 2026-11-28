#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Weighted orbit modes and direct phase extraction
T-1811 · 2026-11-07 · producer checks, unsigned

The compensated modes satisfy $F_e = A x^e H_e(v)$, where $A=m chi(b)$ and $v=x^m$. Roots-of-unity filtering proves the identity for both parities of $d=(r-1)/m$. For $d>=2$, the zeroth mode bounds the image candidate count by $floor(d/2)$.

If the image is known and $H_1(v) != 0$, recover $x=F_1/(A H_1(v))$, then verify the public-point equation. A zero denominator remains unresolved. The second mode supplies a division-free phase-cancellation constraint.

#image("flow.svg", width: 100%)

*Exact finite checks.* 44 small configurations: 28 even-d and 16 odd-d; 15,321 mode identities; 4,704 conditional phase extractions; 822 zero-denominator cases retained. Four larger coefficient compilers checked at r=65,537. Independent small dense powers and streamed larger references are distinguished in DERIVATION.md.

*Public recovery.* Four prescribed radii, no scalar labels at runtime. Of 63 fixtures, 46 recover: 26 phase extractions and 20 endpoints. All 17 unresolved cases belong to the frozen F509/r479/m2 panel. Source-call count is 527, including target validation; source setup and final verification are separately required. These are exact counts, not timing measurements.

#table(columns: (1fr,1fr,1fr,1fr), [Fixture], [Cases], [Recovered], [Unresolved], [F7/r13/m3], [13], [13], [0], [F7/r13/m6], [13], [13], [0], [F509/r479/m2], [20], [3], [17], [F701/r683/m11], [17], [17], [0])

*Costs and scope.* Scalar product/remainder trees compile selected coefficients; conservative field work is O(M(m+d) log(m+d)+log r). Each orbit still queries m source signs. Source domain restrictions and the original 4,563-case cohort are unchanged. No full formal proof or signed independent review is supplied.

*Next:* compose mode constraints and direct phase extraction with retained-sign fiber recovery; investigate the remaining quadratic-image ambiguity. No automatic fallback was added to hide the unresolved cases.

*Evidence:* modes.rs, receipt.json, check.stdout, validation-01 and validation-02 snapshots. Sources: Bostan–Gaudry–Schost (2007), specfun.inria.fr/bostan/publications/BoGaSc07.pdf; FLINT gr_dft transform conventions. KB unavailable; bounded local and primary-source fallback recorded.
