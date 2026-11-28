# Dependency propagation v1: fixed development qualification

Frozen before source and control execution, 2026-11-16. This stage implements
an isolated partial public-equation solver, not a performance experiment.
The earlier original and indexed compilers and their evidence remain unchanged.

## Sources and question

KB unavailable: no KB MCP tool is loaded, and the repository wrapper's expected
backend was unavailable in this runtime. No catalog query or novelty conclusion
is inferred. Reviewed the sparse-elimination-v1 and indexed-circuit-v1 source,
protocols and NEXT files. Primary sources read: Mackworth, *Consistency in
Networks of Relations* (1977), pp.104–107, AC1/AC3 and queue deduplication,
https://www.cs.ubc.ca/~mack/Publications/AI77.pdf; HAC chapter3, section3.5.1,
Algorithms3.34–3.37, https://cacr.uwaterloo.ca/hac/about/chap3.pdf.
This applies classical dependency scheduling to partial polynomial substitution;
it is not an implementation or novelty claim for full AC3 domain consistency.

Question: does revising only equations incident on changed assignments preserve
the roots, deterministic candidate ordering and backtracking of a complete-sweep
arm using the SAME extraction and branching rules? A reverse chain should require
fewer equation revisions with the queue; this is a diagnostic prediction, not
a cold work, elapsed-time, memory or exponent prediction.

## Public boundary and semantics

Inputs: immutable public System, declared branching-variable indices and Limits.
No labels, signs, precomputed base list, witness or target scalar enters solve.
The target is already encoded in System. Univariate linear/quadratic extraction
uses prime-field arithmetic; primality is an external contract. No Buchberger/F4
fallback. A coupled or higher-degree residual may return Unsupported.
Unique roots propagate regardless of role; only declared semantic inputs may
branch on two roots. Derived range/borrow/product auxiliaries cannot be guessed.
All original equations remain present. Return a root only after original check.

Sweep and worklist share candidate priority (fewest roots, variable, equation)
and ascending roots. Worklist caches depend only on support assignments. Both
assignment and rollback dirty every incident equation; queue entries deduplicate.
Selection may still scan all cached statuses; count those scans explicitly.
Failed branches retain counters and decision trace. An unsupported branch cannot
be discarded as impossible; try remaining branches, then report incompleteness.
Cap, Unsupported and NoRootObservation are not absence certificates.

## Fixed controls, outputs and refusals

Run both schedules on a reverse64-variable linear chain; a forced-backtracking
Boolean/product system; triangular, inconsistent, nonsquare and cubic controls;
1100 declared variables with ONE constrained variable; malformed indices,
noncanonical coefficients, invalid modulus/role metadata and forced work, storage,
node, depth, variable, equation, degree and term caps. Compare completed answers
and complete branch traces, check every root and reject mutation/truncation.

Run the existing distinct actual-field q1/k2/arity2 public target fixture, both
original and indexed, with the same public target. Require original-system and
independent indexed-row/group replay of any returned root. This is two columns,
not the original huge base. Also use the fixed Fp101/a17/q3/k8 arity2 compiler
on all13 public eightfold-image targets. Retain every arm/outcome, including caps
or Unsupported; do not assume all targets finish. No known answers are solver
inputs. A target that exhausts this partial search is not called DLP absence.

Default per-arm limits:4096 variables,8192 equations, degree2,32768 input terms,
1024 branch nodes, depth128,10000000 charged diagnostic events,64MiB cumulative
modeled reservations. No retry with higher limits after seeing results in v1.
No original huge actual six-member search, timing, sampling, paid job, base
enumeration or broad root-library rerun. The full actual solve needs separate
published source/protocol/runner/budget admission.

## Accounting and acceptance

Record dependency-build incidences, equation revisions, selector scans, field
operations, assignments, undo, nodes, decisions, original-check calls and modeled
cumulative reservations. Work admission precedes counted operations. Model bytes
are NOT live memory, allocated bytes or RSS. Parsing, compilation, normalization
outside solve, BigUint/container overhead, replay and serialization remain
separate unpriced phases. No total cost/speed claim is admissible here.

Acceptance: fixed completed cases agree and replay; forced backtracking actually
undoes a failed branch; queue invalidation cannot retain a stale candidate; all
non-root outputs retain their precise class. A disagreement or invalid root is
an implementation failure, not a cryptanalytic result. Preserve source attempts,
raw logs, protocol, both systems, all reports/traces and source-bound binary
qualification. New discoveries, full cold accounting and matched admitted
performance comparison remain separate obligations.
