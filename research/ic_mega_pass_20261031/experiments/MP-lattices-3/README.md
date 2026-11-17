# MP-lattices-3: outcome REFUTES (accounting level, l<=2)

Joint lattice {S3(x1,x2,u), S3(x3,x4,v), S3(u,v,z)} mod p^l, x_i<=p^delta, u,v<=p, measured by LM-diagonal max-weight
closure (min-cut, scipy maximum_flow), 6 term orders, box deg<=2l per variable, multiplier u,v-degree s in {0,1,2l}.

| arm | l=1 | l=2 |
|---|---|---|
| A1 (s=0) | 0 (empty lattice) | 0 |
| A2 (s=1) | 0 | 0 |
| A3 (s=2l, multiplicity>=2) | 0 | 0 |
| S5 box-8 (W10 tool) | 0.000010 | 0.002455 |
delta_needed 0.169-0.2.

No positive-weight closure exists for any order: every g1/g2 shift carries a u^2 x^2 monomial costing p^2 per gain p^1, and
g3's LM u^2v^2 gains 1 per 4 units of u,v cost. Sanity: same code with u,v bound p^delta reproduces the S3 block (1/18, 1/10);
with u,v bound p^0.25 or larger it is already 0.

Controls (A0) reproduce lit2026_w10 exactly: univariate dense d=2,3,5 (1/3,2/5,4/9; 1/5,1/4,2/7; 1/9,1/7,1/6), BD 0.25 (l=1..4),
S3 block 1/18, 1/10, 0.11905, 0.12962 (tool 0.128889 at l=4: 0.6% gap; box differs). The first A0_univ rows used a sparse f and
over-report d=3,5; the dense rerun is authoritative.

Caveats: LM-diagonal accounting under triangular selection only. Groebner-optimal or non-triangular (Howell true det, arm C)
was not run, nor was l=3 (box 7^6 with g^3 products). The box is <=2l per variable. S5 was not recomputed; tool values were used.
Isolation (arm D) is moot because the lattice is empty.
Classification: accounting. Full-size internal nodes as lattice variables are strictly worse than eliminating them (S5),
which matches the hidden-variable-elimination expectation.
Next step at larger size: only Howell true-det or a non-triangular sublattice (BD-1999 style) at l=3 could change this. Low priority.
Files: PROTOCOL.md, lp.py, run.py, sanity.py, raw_output.txt, sanity_output.txt, RESULTS.json.
