# Point sieve real-stream correctness protocol, 2026-11-15

Class: established-method implementation plus engineering gate. Not a timed
experiment, novelty assertion or10xspeed result. Full mathematical objective
remains unchanged. Previous turn made progress: exact lazy affine forest and
joint cost derivation, published92c39473e4. No scientific process remains live.
Read docs/session-logs/2026-11-15.md and lazy SESSION/JOINT-COST before choosing
this next step. Shared resume remains owned elsewhere; use this scoped log.

## Literature receipt
KB ready, revision9141e386254340a9bdba3036f2bb706a, lexical only,
refresh2026-11-14T21:44:48Z predates local work. Queries `"Sarkar" "Singh"
"sieve"`, `"hyperelliptic" "sieve" "multiplicity"`. Nearest idea
IDEA-20260918-5a9a51 explicitly calls the incidence backend a comparison study,
not a new construction. No absence/novelty inference from unrelated hits.
Primary Section2.2/2.3and Algorithm1read at
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf.
Point keys, two signs per canonical place and touched buckets are prior art.
GTTD weighted/pinned cycles and the lazy invariant were checked in the previous
protocol; retain exact duplicate/cycle behavior rather than assume F2graphs.

## Frozen construction and checks before implementation
For degree-g D=[u,v]=aD1+bD2, choose t=v-lambda*u, form Q=(t²-f)/u monic.
Each small-base point with u(x)!=0 has key lambda=(v(x)-y)/u(x).
Use both signs, once for y0. Process only touched keys with >=g-1distinct
abscissae; divide those known factors once, then factor residual degree<=2.
Retain repeated roots through exact polynomial division. Orient all roots
with t(x). Deduplicate normalized complete modular relation rows, not buckets
alone. Whole-Jacobian gate: D+sum signed residual places=0, before projection.
Sparse row is sum signed place logs + b*k=-a. The expected k is harness-only.
Reject low-degree D explicitly and count it; do not apply degree-g formula.

1. On squarefree thread curves g2/3/4,p11/13,c1/2when valid,16deterministic
   generated divisors each, compare sieve output for ALLlambda to independent
   exhaustive root division, filtered by the SAME >=g-1distinct non-u-root
   small-hit policy. Full base and half base. Count omissions outside that
   policy separately; no completeness claim beyond it. Force zero and
   degree1D, y0, u(x)=0, repeated roots and lambda-sign controls; report actual
   coverage rather than infer it from random draws. Gates must fail if a
   group identity, factor equality, orientation or output-set equality fails.
2. Reuse the six committed family0fixtures from linear cold evidence, no
   selector rerun: g2p1009/16381,g3p127/1009,g4p31/127. Seed indices8/9/10.
   Public D2 created by harness from known k; worker accepts only D1,D2,ell.
   Deterministically choose walk labels without k, use128batches each target,
   H=min(FB,ceil(p^(1-1/g)))canonical places. No observed-cost tuning.
   Gate every admitted raw identity and public a,b label before modulusell.
3. Feed EXACTsame real relation stream to lazy and dense forests, widthH+2:
   Hsmall logs, constant D1 column, D2column. Compare Option<Row> and union/pin
   counters after every admission. Both known columns must be retained in
   potentials; constants cannot be dropped during recombination.
4. Certify a cofactor h with N=h*ell in a conservative Hasse interval,
   ell²>upperbound, h!=0modell, and Nannihilates every canonical FBplace.
   Project small places and D1/D2byh; assert ellannihilates each. Evaluate
   every emitted modular row in that projected group. Pass identical rows to
   existing incremental echelon: constant coefficient moves to RHSnegated,
   target column remaps H+1toH. Any recovered k must satisfy kD1=D2;
   harness separately compares known k. Report all unresolved128batch caps.
5. Count batches/lowdegree/skipped-u/buckets/candidates/repeated-root admissions/
   duplicates/unions/pins/pure rows/rank/recovered status and lazy expansion
   work/retained capacities. No clocks or throughput claims. No omitting failed
   targets, no timing arms, no statistical-confidence claim.

Native Rust standalone crate instrument using existing rlib and frozen
lazy/dense source includes. gate.sh compiles/runs; native jq may reduce counts.
No scientific Python or cloud worker. Preserve source/fixture hashes and all
terminal outputs. Exactness failure blocks timing. If all gates pass, require
a new frozen complete-cost protocol against strongest controls with A/A before
any speed claim. Conditional occupancy/rank assumptions remain unproven.

## Session
### What was done
Reviewed current worktree/logs, exact formula/sign and primary source; claimed
T-6083for this new directory before edits. Wrote this protocol before code.
### Decisions made
Implement known point-sieve with sparse challenge labels as the joint-cost
baseline. No prior benchmark record, admitted table or default will change.
### Open threads
Implement gates and preserve actual coverage, including low-degree and caps.
### Errors
None at protocol freeze. Shared prepared resume remains untouched.
