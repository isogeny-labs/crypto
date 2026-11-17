# Full plane-to-integral-v defect length, before data

Consume the actual accepted finite and infinity relative charts. Extract the
v/V-linear coefficients and compute the finite-chart defect R/(A,B), as
proved in LOCAL-GEOMETRY.md. Compute only the z=0 contribution of the infinity
chart, by dimensions of R/(Ainf,Binf,z^k), k=1,2,4,..., stopping at the first
equal successive dimensions (at most12 steps). Equality implies stabilization
of multiplication by that power on the finite module; this is the complete
z-primary length, including nilpotents. Preserve every recorded truncation.

The actual checked gluing is diagonal on the plane basis t^i (i<32), with
weights4i, and on the refined basis t^i v^j (i<16,j<2), with weights4i+2j.
Their pushforwards to P1 therefore split as sums of O(-weight). Computing
Euler characteristics gives arithmetic genera1953 and961 and total defect
length992. Require the independently computed chart lengths to sum to992.
This is an exact check on the whole plane-to-refined defect, separate from
the remaining normalization of the refined model. The retained smooth genus61
would leave normalization defect900; this producer does not compute that
normalization. No timing/statistical claim. Same4GiB/1200s, seed50/one thread,
serial CAS. Require exit0, no error lines and DEFECT50_ALL_CHECKS_PASS;
preserve failures/partial exports and original limits. Export actual ideals,
standard bases and lengths for later independent replay.
