# E30: small-box incidence encodings and the ordinary-mask control

Question: does coupling two small coordinate boxes produce a lower-degree
membership representation that depends on twist structure? This bounded
correctness pilot measures no solver or DLP cost. It does not replace E26.
Commit the source and this protocol before execution.

Use all ten original ordinary odd-prime-order fixtures. Freeze lambda in
{1,2,3}, h,k in {2,4,8}, I={0,...,h-1}, J={0,...,k-1}: 270 cells. Set
x=r-lambda*u. Accept (r,u) precisely when x lifts on E/Fp and d*u lifts on
the standard twist y²=x³+d²*a*x+d³*b. Keep empty and tiny projected sets.
Do not search offsets, replace fixtures, or generate E28 final labels.

Interpolate the Boolean acceptance table as M(r,u), of bidegree at most
(h-1,k-1), using ordinary Lagrange interpolation over Fp. The incidence
system is product(r-i)=0, product(u-j)=0, x-r+lambda*u=0, M(r,u)-1=0.
The first two polynomials have distinct Fp roots, so the algebraic-closure
projection of this finite system is exactly the recorded set of x values.
Record the coefficients, table, actual source/twist witnesses and extension
point S with trace T and anti-trace U, obtained by halving the two odd-order
components. Check every coordinate and Frobenius equation natively.

Control: after constructing the same base B, interpolate the ordinary mask
M_B(r,u)=1 exactly when r-lambda*u belongs to B. This mask uses no twist
membership. Check both projections equal B and both complete box tables.
Compare direct univariate membership degree |B| with each incidence system's
maximum generator degree. These degrees are descriptors, not complexity
estimates; an auxiliary-variable system can be harder despite lower degree.
Reject a one-unit corruption of each twist-mask constant term.

The independent PARI script recomputes the two curve-lifting conditions,
evaluates every interpolated coefficient table and both projected sets,
and rejects the corrupted mask. Transport plus the native CAS error gate
must pass before this replay is accepted. Preserve failure outputs.

Run `followup incidence-pilot` into a fresh directory with the frozen fixture
manifest. One worker, 4 GiB address-space bound, 3,600-second native cap and
600-second PARI cap, L0, no admitted wall time. Source, binary, script and raw
hash receipts are required. Recompute/record all summary statistics in Rust.

This is elementary finite-grid interpolation, not a novelty claim. A later
experiment must compare complete Gröbner bases, solutions, construction and
all solving costs for the exact same projected base, including the ordinary
mask. A degree reduction alone cannot establish a twist-specific advantage,
a calibrated IC speedup or a NIST attack. This pilot leaves that requirement
open and its backlog card blocked and unfunded.
