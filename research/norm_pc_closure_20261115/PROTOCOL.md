# Bounded intermediate-degree closure: fixed native correctness panel

Recorded before implementation and calls. Continue the non-tensor norm-system
route. This is a polynomial-calculus / MutantXL development instrument, not a
new elimination method or a scientific cost/yield experiment. Parent source
certificates and all failed flat-Macaulay candidates remain frozen. The owner
goal remains a complete classical subexponential prime-field ECDLP algorithm.
PCC-20261115-be28ec retains unverified standing. [pcc:unverified]

Source search: inherited_f4.rs and degree_reduction.rs already implement
Boolean-ring degree-fall machinery; their monomials and arithmetic cannot be
used as prime-field polynomials. Reuse the native canonical prime-field parser,
grevlex, polynomial arithmetic, quotient extraction and completion checks from
the parent studies. KB unavailable; this is not an exhaustive literature audit.

Start with the original generators and a forward echelon basis. For every new
independent pivot of actual total degree below D, queue its product by every
variable. Reduce those products by scalar combinations of existing pivots;
repeat until the queue empties or a fixed resource cap is reached. Under a
graded order, the low-degree part of the span is spanned by low-degree pivots:
the unique largest leading monomial of any nonzero combination cannot cancel.
This is the reason to queue low-degree pivots, rather than only input multiples.
A cap or a failed final check is UNKNOWN, never UNSAT.

Use a new PC-DAG-CERT-1 schema. Nodes are original-input references, multiplication
of an earlier node by one variable, or field-scalar linear combinations of
earlier nodes. Every reconstructed node must have actual degree at most D.
Serialize only nodes reachable from the final candidate basis, in topological
order. This certifies bounded intermediate degree, not a bound on the degrees
of expanded input multipliers in a flat Macaulay certificate. Log all generated
node/row/rank/reduction counts; the compact proof makes no negative rank claim.

A separately compiled verifier replays local node identities from the original
input, then checks each final basis polynomial, every S-polynomial and every
original generator. It does not call the producer's Gaussian elimination.
Acceptance requires source membership, Buchberger completion, and original
inclusion. Never weaken that conjunction or interpret a nonzero division
remainder before completion as proof of non-membership.

Fixed caps: characteristic p<2^31 as admitted by the existing parser, at most
20 variables, D in 1..6, 20000 attempted source/product rows, 40000 proof nodes,
2000000 live polynomial/proof entries, 1000000 elimination steps and 1000000
verifier reduction steps, 128 candidate basis polynomials, 1MiB certificate.
Each native solver plus exec'd verifier gets 20s CPU/wall and 2GiB address
space. No adaptive cap increase, retry, alternate scheduling, or target change.

Exactly one qualification-001 panel in a fresh directory:

- The parent's six exact ideals, unchanged degrees: F5 grid and nonreduced
  ideals at D3, F3 extension-only ideal at D3, F5 unit ideal at D1, F5 ideals
  (x) and (x,y) at D2. Compare complete rational sets and quotient dimensions.
- Four malformed/adverse proofs: a false linear-combination identity; a
  forward/self reference; a source-valid incomplete basis (x^2-y,xy-1); and
  basis (x) for input (x,y). All must reject for the intended reason.
- The same preserved natural positive norm input m3,t3,c1,R=(0,6), separately
  at D4,D5,D6. The reference GB is outer validation only, never producer input.
  On acceptance compare all rational assignments and quotient dimension with
  the retained parent basis. Otherwise preserve the candidate and UNKNOWN.

Capture source commit and source/binary hashes before calls. Preserve every
input, partial/candidate output, certificate, stdout/stderr, exit and cap.
Whole raw archives follow artifact policy; no splitting to evade size limits.
No field enumeration, guessed witness, scalar, planted norm assignment or
reference basis enters the producer or verifier. No clocks become measurements.
No cold ECDLP, useful random-target yield, T1/T2/T3 or speedup claim follows
from this panel. A new protocol is needed before plugging into cold recovery.
[pcc:unverified]

For v variables and degree D, rank is at most M=binom(v+D,D); each admitted
independent pivot queues at most v products. Ignoring implementation caps,
this gives at most s+vM attempted rows and polynomially many field operations
and DAG entries in s,v,M. This is a per-attempt envelope only. Neither a
uniform sufficient D nor useful relation yield is established. Arbitrary-prime
arithmetic, all cold phases and growing-family cost remain separate work.
[pcc:unverified]

## Amendment A1 — first complete cold recovery control

The first panel is complete: D4 passes source/completion/original checks and
the exact single-model comparison; D5 hits the outer cap and D6 the reduction
cap. Preserve all outcomes. Do not rerun the panel or change the producer.

Freeze one first cold control, public input Q=(0,6) on E/F17:y²=x³+2x+2,
P=(5,1), order19, seed1234. Reuse the unchanged core oracle and cold_solve:
algebraic factor-base construction, mixed point queries, signed relations,
left kernel, scalar recovery and final group check. Set PC_DEGREE_CAP=4 and
PC_VERIFIER to the separately compiled checker. No scalar, base logs, reference
models or decomposition assignments enter the candidate. This is one finite
software control, not an 18-target census or a public challenge/tier dataset.

The driver records the same seven-column summary and complete existing traces.
Extend the independent OpenSSL replayer with an explicit --cold-one mode
requiring exactly this single fixed target; leave full --cold census semantics
unchanged. Replay the preserved parent18-target traces once as a regression
check of that refactor, without producer/backend reruns.

Exactly one cold invocation in fresh cold-001, no retries on unknown. Preserve
all backend inputs, bases, DAGs, errors/statuses and group/kernel traces. Each
backend retains its20s CPU/wall and2GiB bounds. Whole cold driver gets1200s wall
and2GiB address space; stop on any unknown/mismatch. Replay successful output
with the independent OpenSSL binary. Capture source/binary hashes beforehand.
Large evidence uses the existing artifact store; never split a raw round to
fit Git. No clocks or query counts become comparative performance claims.

Build correction before any cold backend invocation: the driver's attempted
main-symbol macro conflicts with a nested legacy parser include. Replace it
with an explicit NORM_CORE_NO_MAIN guard, leaving the standalone core CLI and
all oracle/recovery logic unchanged. The first wrapper startup created an empty
cold-001 directory, then failed because no compiled cold build manifest existed.
No cold candidate or replay ran. Preserve these build/setup logs and use fresh
cold-002 for the still-single registered cold invocation. No algorithm or cap
changes and no extra target/call are introduced.
