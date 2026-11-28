# Initial-split baseline and matched discriminant candidate

This is an independent Python implementation of established squarefree
factorization and initial trace/Legendre splitting identified in FLINT's
primary source; it is not native FLINT and no novelty is claimed for it.
For squarefree H in characteristic two, T=sum X^(2^i) obeys
T^2+T=X^q+X. Thus gcd(H,T) and gcd(H,T+1) are disjoint products of precisely
all field-linear factors, already partitioned by trace. Their later trace
splitting skips the known constant first trace. For odd q, nonzero roots are
similarly characterized by X^((q-1)/2)=+/-1; zero is extracted separately.

Squarefree layers use the standard gcd/quotient loop and pth-root recursion
for residual pth powers, retaining characteristic3/5 multiplicities. The
binary candidate uses the exact Delta branches, then shares this stronger
initial-split extractor. Every resulting root/multiplicity is checked in the
original H. The old direct/radical comparators remain. Tiny literal oracles
exercise all five methods in characteristic two and all three in odd fields.

All Python files pass AST parsing before registration. The correctness worker
also executes both actual cold-wrapper entry points in --smoke mode: one
field/case/round, every configured method, same frozen panel/child paths and
independent count/multiplicity verification. Smoke outputs have distinct
exclusive names and are never included in the cold calibration panel.

Real cold panels retain all15fields, four cases and three repetitions; no
field or losing branch is selected away. Sources and frozen input hashes
are checked before run; each child inherits CPU0/4GiB. Binary outer budget
300seconds, odd150, with earlier internal/wrapper demotion boundaries. All
setup, input parsing, verification, root extraction and output count in cold
process wall time. Independent old-arithmetic fixture checks remain.
This is still a conditional quintic-root kernel, not the339-variable system,
not a field-family hardness proof and not a genuine F4/F5/F6 comparison.
