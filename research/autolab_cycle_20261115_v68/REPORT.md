# C68: nonlinear certificates for larger cells

Two parity forms now certify the 128-point cell that defeated the previous linear certificate. Exact Boolean polynomial composition gives Q(g)=1, while Q(0)=0. An odd XOR dependency proves that a single form is impossible on this cell. All 256 subsets of F2^3 matched an independent brute-force search, including zero-containing controls.

Fresh tests used the actual 73-parameter GF(2^83) norm family. Three forms certify a 256-assignment cell; four certify a 512-assignment cell. Their input ANF degrees are 8 and 9. The cells are nested, so their union contains 512 distinct parameters. The restricted quadratic search did not find two-form certificates for these larger cells; it does not prove that none exist.

A separate 16-assignment coefficient cell contains a known supported quintic, verified from its literal roots. Its guard has a zero, and both certifiers correctly refuse to discard the cell. This is a support-valid positive control, not a claim that it belongs to the same fixed norm chart.

All 784 fresh field-evaluation occurrences were charged. Coefficient construction still enumerates each cell. Reusing C67 coefficients for the earlier cell does not remove their construction cost. These results establish exact pruning certificates, not a root-search speedup or a qualified 50x result.

Two frozen 90-second, 4-GiB, one-core experiments ran through checked resume/run-ready. Sources, inputs, coefficients, masks, negative controls and replay are preserved. No genuine F4/F5/F6 ratio, twenty-hard-instance multi-field qualification or novelty clearance is established. The main preimage search remains open.

Next: test translation invariants of the image characteristic polynomial, then check whether their algebraic orbits intersect the actual norm family usefully. A fixed trace coefficient can prevent an apparent symmetry reduction. Sound nonenumerative certificate synthesis remains a separate obligation. The scientific goal stays active.
