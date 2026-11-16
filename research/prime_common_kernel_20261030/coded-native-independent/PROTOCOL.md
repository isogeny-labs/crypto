# Fixed development correctness protocol

Frozen before compilation, 2026-10-30 UTC. Tiny native exact-arithmetic
unit only; no timing, scaling fit, search, benchmark or ECDLP claim.

Literature/read first: the written candidate
`/workspace/audit/coded-operator-kernel-20261030.md`, sections1–3, the
pinned Oct25 manuscript D.4.3–D.4.6, and the candidate's cited syndrome
projection construction. The mathematical independent specification check
is provided by the parent's existing gauge-countercheck child. This
unit tests the specified finite specialization, not its general proof.

Parameters are fixed: a=3, r=2, L=8, alpha=16, binary syndrome columns
1,2,3. Enumerate x,y in0,...,7, retaining H(x xor y)=0. Expect16 slots,
two partners perx, and8slots for either fixed first-register bit0.

Compute all ordinary Lagrange cardinal values on S={-8,...,7} at t=1
and T=17 by exact rational products

    ell_q(T)=product_(s in S,s!=q) (T-s)/(q-s).

No coefficient or valuation is inserted in place of this product.
Set f*=ell0, fj=sum_(v2(q)=j)ellq for j=0,1,2 and add ell(-8) to f0.
Check cardinal specialization at1 and selector sum1 at1 and17.

For each of four fixed scalar fixtures, compare all256entries of the
direct16-by-16 projected A tensor A with the diagonal paired-flip formula:

1. (1,2,3,4).
2. (1/3,-2/5,3/7,-4/9).
3. (0,-1/2,2/3,-3/4).
4. The actually computed selector vector at17.

Here A=f*I+sum fjXj is built explicitly as an8-by-8 matrix, and direct
entries are A[xrow,xcol]*A[yrow,ycol]. The comparison expression is
f*²I+sum fj²Uj, with Uj computed by actual paired-bit flips. Preserve
every compared entry. Separately check that the actual node1 selector
matrix equals U0 on all256entries, which flips the first orientation bit.

For each fixed bit0 side, extract its actual8-by-8 crossing submatrix
B at17. Compute every rational entry divided by256, check odd
denominators and reduction modulo2 equal to I8, and compute rank8 by
independent binary elimination. Compute detB by exact rational Gaussian
elimination and its2-adic valuation64. These constitute the finite
certificate that all eight Smith orders are8; no Smith-normal-form
implementation is claimed. Check observed selector valuations directly
from their exact numerator/denominator: f* order4 and f1,f2 order>=5.

Two frozen negative controls: replacing U0 with I must fail the active-node
matrix equality; deleting the f*²I contribution must make the normalized
crossing matrix singular modulo2. Expected control failures count as a
successful discrimination, not a successful candidate construction.

Implementation: standalone C++20/GMP, strict warnings as errors, no random
seed. Keep original source, protocol/source hashes, compile log, run log,
raw slot/selector/matrix/check TSVs and any failures. No runtime fields or
host-performance claims. A failure stops this bounded unit; report the
discrepancy rather than expanding the mathematical search.
