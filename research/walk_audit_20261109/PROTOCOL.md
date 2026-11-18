# Exact walk census and formal boundaries

This addresses item10 of the original audit, whose complete acceptance map
remains ../theorem_census_20261109/README.md. Source review reads
Hildebrand1994 Theorems1–2 and Galbraith ch14 §14.2.5, with the latter's
bookp297 visually inspected. KB unavailable; primary/source fallback.
Hildebrand averages uniformly chosen distinct supports with fixed positive
weights. The collision penalty is heuristic. Neither source is refuted
by an arbitrary small-field deviation. [pcc:unverified]

Lean uses actual uniform-history PMFs and addition in Z/p, not an assumed
mixing table. For increments0,1,2,3 after t steps, an arc of at most3t+1
residues has probability1; its uniform probability is at most(3t+1)/p.
For6t+2≤p this rules out1/4event-distance mixing. Closed prime witness:
p101,t10, event mass1 versus uniform31/101. This formalizes a known
fixed-support obstruction; Hildebrand's averaged theorem remains qualified.
[pcc:unverified]

Lean also checks actual Z/3hash graphs with the two distinct nonzero
increments1,2. The entire hash space has cardinality8, normalized mean
in-degree1 and variance1/2. This contradicts a literal all-r reading of
the printed reciprocal-variance sentence on Galbraithp297 (r/(r−1)=2
at r=2). It does not refute the collision-time heuristic. A separate actual
function-space equivalence proves that full-support uniform hash tables
induce uniformly distributed endofunctions. [pcc:unverified]

Native census has two arms, each a single exhaustive population workload,
repeated twice. No random seed, sample selection, warmup, timing metric,
statistical population inference or confidence interval is involved.
The frozen source performs its analysis/verification inline; raw JSONL
and summary JSONL are preserved for both repetitions. [pcc:unverified]

- Mix: all distinct unordered supports of sizes2,3,4,6 wherever r≤p over
  p5,7,11,13, giving3930supports. Uniform iid steps from each support;
  start0. Exact integer histogram recurrence, compared every step against
  dense transition-matrix multiplication. Check mass and TV contraction.
  TV numerator is sum|p*count[x]−r^t|, denominator2*p*r^t. Record the first
  t with TV≤1/4, previous exact TV, final histogram, and a fixed cap4p².
  A cap hit is censored, not extended or silently dropped.
- Rho: all supports of sizes2,3 over p3,5,7, and every independent uniform
  label function on the p states, giving80supports and82034maps. Start0;
  x maps to x+support[label[x]]. Record first-repeat histograms/tail/cycle
  checks. Compare visited-array detection to Floyd for every function and
  enforce first repeat by step p. Record exact in-degree moments at0.
  Summarize both all supports and supports containing no zero increment.

Every support is counted once in increasing order; this is the uniform
unordered-support model. Ordering permutations with uniform step weights
leave the induced law unchanged. No comparison of nonuniform weights is
claimed. Mixing distributions use fresh iid choices; rho uses one fixed
hash map per walk. The two processes are deliberately kept separate.
[pcc:unverified]

Uniform labels mean the ensemble law at each state; an individual fixed
hash map need not have equal empirical partition sizes. No conditioning on
that partition-size vector or nonuniform p_i penalty is tested here. The
equal-weight factor is a finite descriptive reference only.
[pcc:unverified]

For rho, record the exact finite uniform-random-mapping first-repeat
baseline, E(T)=sum from k=0 to p−1 of product from i=1 to k of(1−i/p).
Record the observed/baseline ratio as a rational, with the heuristic
penalty squared r/(r−1) alongside it. The exact baseline is not the
asymptotic sqrt(pi*p/2). These small-field comparisons describe finite
values; no limiting constant or hypothesis rejection follows.
Full support at p3,r3 has the known mapping histogram9/12/6 and mean17/9.
[pcc:unverified]

Held-out preflight is disclosed as in-sample calibration: mix p3,r2
(all3supports; quarter time2, TV1/6) and p17,r2support{0,1}; rho p2,r2
(4maps, first-repeat histogram2/2, mean3/2) and p11,r3support{0,1,2}.
These probes lie outside their respective confirmatory populations.
All development generations/source snapshots are retained. Final preflight
timings appear in evidence/preflight-frozen-binary/ for budgeting only.
No confirmatory population has been run before the canonical freeze.
[pcc:unverified]

Use one approved local host and zero paid spend. One thread; timeout120s
per native call, no outcome-dependent retry, four calls total. Each complete
call is a local checkpoint; do not overwrite output. Both repetitions must
match byte-for-byte. Failures, timeouts, censored supports and comparison
failures remain in the export. No native source repair is allowed under
the frozen protocol after affected data; preserve originals and file a
labelled amendment if a later repair is needed. [pcc:unverified]
