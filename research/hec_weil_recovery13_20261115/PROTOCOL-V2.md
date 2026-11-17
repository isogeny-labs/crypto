# Exact fixture integer lift amendment

V1source9e75b695db30 passed once, output retained unchanged. V2 lift.rs copies
that worker and adds a separate exact lift gate. No speed measurement.
Original main.rs and RUN.txt remain unchanged.

Compute residues from the public polynomial first; then derive unique a1 in
|a1|<=floor(sqrt(36p)). Enumerate b congruent r2 with |b|<=15p. Bound
|c|<=floor(sqrt(400p^3)). The enclosing order interval must contain exactly
one ell-multiple, otherwise stop unsupported. Use that multiple to derive c
for each b and check r3. All integer square roots and radical endpoint signs
are exact i128 operations; no floating-point rounding or extension counts.

Reject a candidate only for failed necessary coefficient/congruence or endpoint
conditions. For each survivor require three sign-changing disjoint integer
unit intervals inside [-2sqrt(p),2sqrt(p)]. These certify three distinct real
roots of the monic cubic h; thus the reciprocal degree6 polynomial is p-Weil.
If no such sufficient bracket certificate is found, stop unsupported, never
discard it as non-Weil. Require exactly one survivor, and assert archived
coefficients only after recovery. General repeated/clustered-root cases remain
unsupported, not falsely rejected. Fixture only, not a generic point counter.

Subgroup divisibility premise reuses the public nonidentity ell-torsion checks
of frozen coldbuild13; this worker does not independently implement Jacobian
arithmetic. It certifies the integer lift conditional on that established
public input premise. It neither proves new subgroup membership nor changes
validation standing. Primary theory and general exact criterion are recorded
in the sibling PROTOCOL and Weil research note.

Freeze/commit worker and this amendment, fingerprint optimized binary, checkpoint
before one execution into RUN-V2.txt/RUN-V2.stderr plus EXIT-V2.txt. Require
108coefficient checks, public residues(1,84,46), one exact survivor
(a1,a2,a3,N)=(1,-43,-1224,2057786), implied N1=129,N2=16043.
Retain any failed outcome; no rerun without amendment. No timing or10xclaim.
