# Exact quintic finite-field root kernel and discriminant dispatch

Scope: every distinct finite-field root of a supplied monic quintic, with its
exact original multiplicity. This is a conditional univariate kernel, not a
solver for the original coupled339-variable system. Nonsplit quintics can
still have field roots; no failed full-splitting test becomes an empty output.

The direct comparator computes R=gcd(H,X^q-X); since X^q-X is squarefree and
has exactly the field elements as roots, R is precisely their squarefree
product. The stronger comparator first computes the radical recursively,
including pth-root recursion when the derivative vanishes. Both feed the
same exact factor extractor and return original multiplicities by division.

Binary candidate uses the C105 Delta proof. On Delta!=0 it follows the direct
method. On A=B=0 it returns t and every field root of the exact Q factor,
including the case Q is irreducible. On the other Delta=0 branch it returns
sqrt(B/A) and every field root of the reconstructed cubic J. These are exact
factor identities; no root or repeated/nonrational factor is discarded.

Binary extraction splits squarefree linear products using trace polynomials
Tr(beta*X). Nondegeneracy of the trace pairing and a complete field basis
separate every two distinct roots, guaranteeing separation within n probes
per factor. Odd extraction uses the standard random-polynomial quadratic
character split; a128attempt cap raises incomplete, never empty/complete.
Every reported root has its original multiplicity checked. Independent tiny
oracles enumerate the entire field and divide literal linear factors.

All methods share the SAME binary byte-squaring/Euclidean-inverse arithmetic;
this optimization is verified against independent old arithmetic. It cannot
manufacture an algorithm gain by slowing a comparator. Binary moduli receive
exact Rabin irreducibility checks. The163/233/283/409/571 polynomials are the
standard binary curve field representations; this establishes field context,
not ordinary/Koblitz curve equation solving. Odd prime presets come from the
already source-identified NIST data; extension controls include characteristic3.

Cold panels freeze input polynomial bytes before timing, alternate method
order and run each call in a fresh one-core child with inherited4GiB limits.
Field construction/verification, root extraction, multiplicities and JSON
output count in process wall time. Repetitions and memory are recorded, but
there is no isolation/A-A certificate or matched genuine F4/F5/F6 comparison.
Full-width small-degree kernels need not meet the one-second hardness floor.
The210/120second panel budgets preserve every completed row on timeout.
Original completion/count fields stay false/null, even for complete local
kernel rows. Standard squarefree/Frobenius/trace factorization is prior art;
the candidate's novelty and original end-to-end benefit remain unestablished.
