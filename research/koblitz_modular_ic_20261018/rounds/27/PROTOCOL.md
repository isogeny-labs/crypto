# R27 prospective exact computation

Continue every lead listed in R26/NEXT.md. Start with cusp valuations, which
are an input to the complete RR and actual Prym transfer construction; then
derive source-bound norm/collector identities and pursue node187 membership.
Each dependent extension requires its own prospective source freeze.

Reconciliation: new boot 8fc63b83-b328-41c5-9d73-c9bd94597e78. Approved receipt
and managed Diem inspected: deadline-expired,50/52,pendingfalse; legacycomplete.
EKS overview3succeeded2failed0active,unrelated; no submission/retry. Conductor
clears study/tools/problempaths; daily log held T729, use SESSION.md. R26
publication9b8a05bb4 is clean and complete, do not repeat its experiments.

Literature before method: R22 CM/base divisors/generic recovery; R23 CM
SS5/Z5; R24 charts/series; R25 quotient; R26 exact T source orbit/proof map.
Sutherland18.782 lecture22, Cor22.23:
https://www.ocw.mit.edu/courses/18-782-introduction-to-arithmetic-geometry-fall-2013/5859c5f7d2ad1eae5339d87dce144823_MIT18_782F13_lec22.pdf
The RR dimension requires the proper geometrically integral curve and genus
bridge. Singular official manual jet/truncation semantics reviewed via
https://www.singular.uni-kl.de/ftp/pub/Math/Singular/src/4-1-1/singular.pdf;
the selected latest manual URL was inaccessible. No new novelty claim.
Generic protocol checker/schema and lead-atlas unavailable in this checkout;
this remains the inherited custom exact-computation contract, not generic lint.

LAURENT27: start from the R24 four O-chart lifts,768 absolute series precision,
increased fixed lifting iterations; preserve original five residual checks.
Represent a Laurent approximation as (unit polynomial,exponent,absolute P),
meaning error O(w^P). For addition take min P; multiplication takes
min(Pa+vb,Pb+va), inversion P'=P-2v. Normalize after every operation.
All arithmetic is native Singular; Python/shell only assemble/store inputs.
Evaluate original source polynomials termwise with truncation and propagated
precision; require zero residual with strictly positive certified precision.
Iterate exact original T five times; record lambda,nu,S,N and all registered
entries at all20 cusp branches. Require the fifth iterate equals sigma at
positive certified precision and b,c orders agree with the complete R22
base table times e. Nonzero leading terms must lie below certified precision.
Helper controls include inversion of a pole, cancellation, multiplication
precision, and exact zero. A negative control changes the actual lambda
update by1; unchanged original source residual detector must reject it.
No global divisor completeness is inferred before degree/support exhaustion.

Planned2native calls positive/corrupt, each4GiB virtual/1200s fixed cap,
native Singular4.4.1 local inherited lane. Native exit0+finalmarker+no error
diagnostic required; failures retained even when process exits0. No random,
timed/comparative/statistical claim, sampling, A/A or power calculation applies
to these deterministic identities. Freeze amendments before affected data.
No new EKS,Diem,waiting-freeze,fifth Conductor claim,merge or auto-merge.
Checkpoint current source/raw to setec-astronomy-artifacts before each native
call. Never mutate repo concurrently with commit/checkpoint/gate. Native raw
must be durable before continuation. New faithful Lean identities will expose
geometric bridges rather than assuming missing geometry as axioms.

Prospective correction before LAURENT02:01 retains native exit0/late marker
but wrapperrejects the collision between seed poly entry and new int entry.
Rename only the new loop index to index27. No chart or precision change.
